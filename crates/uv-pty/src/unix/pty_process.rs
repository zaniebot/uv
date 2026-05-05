pub(super) use nix::sys::{signal, wait};
use nix::{
    self,
    fcntl::{OFlag, open},
    pty::{PtyMaster, Winsize, grantpt, posix_openpt, unlockpt},
    sys::termios::{InputFlags, Termios},
    sys::{stat, termios},
    unistd::{ForkResult, Pid, fork, setsid},
};
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::{self, fs::File, io, os::unix::process::CommandExt, process::Command, thread, time};

#[cfg(target_os = "linux")]
use nix::pty::ptsname_r;

/// Start a process in a forked tty so you can interact with it the same as you would
/// within a terminal.
///
/// The process and pty session are killed upon dropping `PtyProcess`.
pub struct PtyProcess {
    pub pty: PtyMaster,
    pub child_pid: Pid,
    kill_timeout: Option<time::Duration>,
}

#[cfg(target_os = "macos")]
/// `ptsname_r` is a Linux extension but `ptsname` isn't thread-safe.
/// Instead of using a static mutex this calls ioctl with `TIOCPTYGNAME` directly.
/// Based on <https://blog.tarq.io/ptsname-on-osx-with-rust/>.
fn ptsname_r(fd: &PtyMaster) -> nix::Result<String> {
    use nix::libc::{TIOCPTYGNAME, ioctl};
    use std::ffi::CStr;

    // The buffer size on macOS is 128, defined by sys/ttycom.h.
    let mut buf: [i8; 128] = [0; 128];

    // SAFETY: We pass a correctly-sized buffer and check the return value.
    unsafe {
        match ioctl(fd.as_raw_fd(), u64::from(TIOCPTYGNAME), &mut buf) {
            0 => {
                let res = CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned();
                Ok(res)
            }
            _ => Err(nix::Error::last()),
        }
    }
}

#[derive(Default)]
pub struct PtyProcessOptions {
    pub echo: bool,
    pub window_size: Option<Winsize>,
}

impl PtyProcess {
    /// Start a process in a forked pty.
    pub fn new(mut command: Command, opts: PtyProcessOptions) -> nix::Result<Self> {
        // Open a new PTY master.
        let master_fd = posix_openpt(OFlag::O_RDWR)?;

        // Allow a slave to be generated for it.
        grantpt(&master_fd)?;
        unlockpt(&master_fd)?;

        // On Linux this is the libc function, on macOS this is our implementation.
        let slave_name = ptsname_r(&master_fd)?;

        // Get the current window size if it was not specified.
        let window_size = opts.window_size.unwrap_or_else(|| {
            let mut size: libc::winsize = unsafe { std::mem::zeroed() };
            // SAFETY: Querying terminal dimensions via ioctl is safe with a valid fd.
            unsafe { libc::ioctl(io::stdout().as_raw_fd(), libc::TIOCGWINSZ, &mut size) };
            size
        });

        // SAFETY: We immediately diverge in the child (via exec) and only access
        // async-signal-safe operations before that point.
        match unsafe { fork()? } {
            ForkResult::Child => {
                // Avoid leaking master fd. We use libc::close directly because
                // after fork the OwnedFd destructor behavior is unspecified.
                // SAFETY: We own this fd in the child process and it's valid.
                unsafe { libc::close(master_fd.as_raw_fd()) };

                setsid()?; // create new session with child as session leader
                let slave_fd = open(
                    std::path::Path::new(&slave_name),
                    OFlag::O_RDWR,
                    stat::Mode::empty(),
                )?;

                let slave_raw = slave_fd.as_raw_fd();

                // Assign stdin, stdout, stderr to the tty.
                // SAFETY: Valid fds from open() and well-known constants.
                unsafe {
                    libc::dup2(slave_raw, libc::STDIN_FILENO);
                    libc::dup2(slave_raw, libc::STDOUT_FILENO);
                    libc::dup2(slave_raw, libc::STDERR_FILENO);
                }

                // Avoid leaking slave fd.
                if slave_raw > libc::STDERR_FILENO {
                    // SAFETY: Valid fd, needs to be closed to avoid leak.
                    unsafe { libc::close(slave_raw) };
                }

                // Set `echo` and `window_size` for the pty.
                set_echo(io::stdin(), opts.echo)?;
                set_window_size(io::stdout().as_raw_fd(), window_size)?;

                let _ = command.exec();
                Err(nix::Error::last())
            }
            ForkResult::Parent { child: child_pid } => Ok(Self {
                pty: master_fd,
                child_pid,
                kill_timeout: None,
            }),
        }
    }

    /// Get handle to pty fork for reading/writing.
    pub fn get_file_handle(&self) -> nix::Result<File> {
        // Needed because otherwise fd is closed both by dropping process and reader/writer.
        let fd: OwnedFd = self
            .pty
            .as_fd()
            .try_clone_to_owned()
            .map_err(|_| nix::Error::last())?;
        Ok(File::from(fd))
    }

    /// Get status of child process, non-blocking.
    ///
    /// This method runs `waitpid` on the process.
    /// If you ran `exit()` before or `status()` returned a terminal status,
    /// this method will return `None`.
    pub fn status(&self) -> Option<wait::WaitStatus> {
        wait::waitpid(self.child_pid, Some(wait::WaitPidFlag::WNOHANG)).ok()
    }

    /// Regularly exit the process. This method is blocking until the process is dead.
    pub fn exit(&mut self) -> nix::Result<wait::WaitStatus> {
        self.kill(signal::SIGTERM)
    }

    /// Kill the process with a specific signal. This method blocks until the process is dead.
    ///
    /// Repeatedly sends the signal to the process until it exits.
    /// If `kill_timeout` is set and the process has not exited within that duration,
    /// `SIGKILL` is sent.
    pub fn kill(&mut self, sig: signal::Signal) -> nix::Result<wait::WaitStatus> {
        let start = time::Instant::now();
        loop {
            match signal::kill(self.child_pid, sig) {
                Ok(()) => {}
                // Process was already killed before -> ignore.
                Err(nix::errno::Errno::ESRCH) => {
                    return Ok(wait::WaitStatus::Exited(Pid::from_raw(0), 0));
                }
                Err(e) => return Err(e),
            }

            match self.status() {
                Some(status) if status != wait::WaitStatus::StillAlive => return Ok(status),
                Some(_) | None => thread::sleep(time::Duration::from_millis(100)),
            }
            // Send SIGKILL if timeout is reached.
            if let Some(timeout) = self.kill_timeout
                && start.elapsed() > timeout
            {
                signal::kill(self.child_pid, signal::Signal::SIGKILL)?;
            }
        }
    }

    /// Set raw mode on stdin and return the original mode.
    pub fn set_raw(&self) -> nix::Result<Termios> {
        let original_mode = termios::tcgetattr(io::stdin())?;
        let mut raw_mode = original_mode.clone();
        raw_mode.input_flags.remove(
            InputFlags::BRKINT
                | InputFlags::ICRNL
                | InputFlags::INPCK
                | InputFlags::ISTRIP
                | InputFlags::IXON,
        );
        raw_mode.output_flags.remove(termios::OutputFlags::OPOST);
        raw_mode
            .control_flags
            .remove(termios::ControlFlags::CSIZE | termios::ControlFlags::PARENB);
        raw_mode.control_flags.insert(termios::ControlFlags::CS8);
        raw_mode.local_flags.remove(
            termios::LocalFlags::ECHO
                | termios::LocalFlags::ICANON
                | termios::LocalFlags::IEXTEN
                | termios::LocalFlags::ISIG,
        );

        raw_mode.control_chars[termios::SpecialCharacterIndices::VMIN as usize] = 1;
        raw_mode.control_chars[termios::SpecialCharacterIndices::VTIME as usize] = 0;

        termios::tcsetattr(io::stdin(), termios::SetArg::TCSAFLUSH, &raw_mode)?;

        Ok(original_mode)
    }

    /// Restore the terminal to the given mode.
    pub fn set_mode(&self, original_mode: Termios) -> nix::Result<()> {
        termios::tcsetattr(io::stdin(), termios::SetArg::TCSAFLUSH, &original_mode)?;
        Ok(())
    }

    /// Set the window size of the pty.
    pub fn set_window_size(&self, window_size: Winsize) -> nix::Result<()> {
        set_window_size(self.pty.as_raw_fd(), window_size)
    }
}

/// Set the window size on a file descriptor.
#[allow(clippy::unnecessary_wraps)]
pub(super) fn set_window_size(raw_fd: i32, window_size: Winsize) -> nix::Result<()> {
    // SAFETY: Setting window size via ioctl is safe with a valid PTY fd.
    unsafe { libc::ioctl(raw_fd, nix::libc::TIOCSWINSZ, &window_size) };
    Ok(())
}

/// Set echo mode on a file descriptor.
pub(super) fn set_echo<Fd: AsFd>(fd: Fd, echo: bool) -> nix::Result<()> {
    let mut flags = termios::tcgetattr(&fd)?;
    if echo {
        flags.local_flags.insert(termios::LocalFlags::ECHO);
    } else {
        flags.local_flags.remove(termios::LocalFlags::ECHO);
    }
    termios::tcsetattr(&fd, termios::SetArg::TCSANOW, &flags)?;
    Ok(())
}

impl Drop for PtyProcess {
    fn drop(&mut self) {
        if let Some(wait::WaitStatus::StillAlive) = self.status() {
            if let Err(err) = self.exit() {
                eprintln!("Failed to exit PTY process: {err}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::sys::{signal, wait};
    use std::io::{BufRead, BufReader, LineWriter, Write};

    #[test]
    /// Open cat, write string, read back string twice, send Ctrl+C and check that cat exited.
    fn test_cat() -> std::io::Result<()> {
        let process = PtyProcess::new(
            Command::new("cat"),
            PtyProcessOptions {
                echo: false,
                window_size: Default::default(),
            },
        )
        .expect("could not execute cat");
        let f = process.get_file_handle().unwrap();
        let mut writer = LineWriter::new(&f);
        let mut reader = BufReader::new(&f);
        let _ = writer.write(b"hello cat\n")?;
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        assert_eq!(buf, "hello cat\r\n");

        // This sleep solves an edge case where cat is not yet ready for the signal.
        thread::sleep(time::Duration::from_millis(100));
        writer.write_all(&[3])?; // send ^C
        writer.flush()?;
        let should = wait::WaitStatus::Signaled(process.child_pid, signal::Signal::SIGINT, false);
        assert_eq!(should, wait::waitpid(process.child_pid, None).unwrap());
        Ok(())
    }
}
