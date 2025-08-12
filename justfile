alias q := quiet
alias v := verbose

bin := "./target/debug/tic"

# run the application
run args:
    ./target/debug/tic {{args}}

# build main
build:
    cargo build

# show help
help:
    @{{bin}} --help

# run in quiet mode and print exit code
quiet:
    @{{bin}} -q
    @echo "Exited with code $?"

# run in verbose mode
verbose:
    @{{bin}} -v

# run with rust backtrace
trace args:
    @RUST_BACKTRACE=1 {{bin}} {{args}}