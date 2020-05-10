# Entrs

Entrs is a port of the classic entr utility


Entr is a file watch utilty that executes provided comments on file changes


## Installation

### Manual

```
git clone https://github.com/HallerPatrick/entrs.git && cd entrs

# Run directly or
cargo run

# Build binary (in ./target/release/entr )
cargo build --release

```

## Usage

```
USAGE:
    entrs [FLAGS] [utility]...

ARGS:
    <utility>...

FLAGS:
    -c               Clear screen before executing utility
    -h, --help       Prints help information
    -p               Execute utility first after files have changed
    -r               Watch for file changes recursively
    -u               Evaluate the first argument using the interpreter specified by the SHELL environment variable
    -V, --version    Prints version information
```


### Example

Rebuild the cargo project after file saving and clear the screen before executing `cargo build`

```

$ ls | entr -c cargo build

```
