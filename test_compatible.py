#!/usr/bin/env python3
import subprocess
import sys
import tempfile
import os

def main():
    # Create a temporary directory
    with tempfile.TemporaryDirectory() as tmpdir:
        # Create requirements.in
        req_file = os.path.join(tmpdir, "requirements.in")
        with open(req_file, "w") as f:
            f.write("numpy==1.26.4\n")
        
        # Path to the uv binary (assuming it's built in target/debug)
        uv_bin = "./target/debug/uv"
        if not os.path.exists(uv_bin):
            uv_bin = "./target/release/uv"
            if not os.path.exists(uv_bin):
                print("ERROR: uv binary not found. Please build it first.")
                sys.exit(1)
        
        # Test 1: Run with --hashes all (default)
        print("=== Testing --hashes all ===")
        result = subprocess.run(
            [uv_bin, "pip", "compile", req_file, "--generate-hashes", "--hashes", "all"],
            capture_output=True,
            text=True
        )
        all_hashes = result.stdout.count("--hash=")
        print(f"Hash count with 'all': {all_hashes}")
        
        # Test 2: Run with --hashes compatible
        print("\n=== Testing --hashes compatible ===")
        result = subprocess.run(
            [uv_bin, "pip", "compile", req_file, "--generate-hashes", "--hashes", "compatible"],
            capture_output=True,
            text=True,
            env={**os.environ, 'RUST_LOG': 'debug'}
        )
        compatible_hashes = result.stdout.count("--hash=")
        print(f"Hash count with 'compatible': {compatible_hashes}")
        
        if "DEBUG:" in result.stderr:
            print("\nDEBUG output found:")
            for line in result.stderr.split('\n'):
                if 'hash_mode' in line:
                    print(f"  {line}")
        
        # Show comparison
        print(f"\n=== Results ===")
        print(f"All mode: {all_hashes} hashes")
        print(f"Compatible mode: {compatible_hashes} hashes")
        print(f"Difference: {all_hashes - compatible_hashes}")
        
        if compatible_hashes >= all_hashes:
            print("\nERROR: Compatible mode should have fewer hashes than all mode!")
            print("\nCompatible output sample:")
            print(result.stdout[:500])
        else:
            print("\nSUCCESS: Compatible mode has fewer hashes as expected!")

if __name__ == "__main__":
    main()