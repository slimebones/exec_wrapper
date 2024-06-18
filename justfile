set shell := ["nu", "-c"]

build:
    @cargo build -r
    @cp target/release/exec_wrapper.exe target/release/exec_wrapper

test:
    @cargo run -q "python hello.py" tests/hello
    @cd tests; hello arg1 arg2 arg3 "spaced arg4"
