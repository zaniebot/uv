FROM archlinux:latest
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/
WORKDIR /app
RUN uv init -p 3.12 test-cffi
WORKDIR /app/test-cffi
RUN uv add cffi
CMD ["uv", "run", "-p", "3.12", "python", "-c", "import cffi; print(cffi.__version__); print(cffi.FFI())"]
