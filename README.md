# Operation FIRE STORM

My solution to the **Operation FIRE STORM** challenge by CoreTech Security. The objective is to intercept an incoming network stream, extract the sensitive data being transmitted under the "CTFS" protocol, and forward everything else on untouched.

This Rust program listens on TCP port 5733, reassembles the CTFS payloads into a PDF, and forwards all other traffic to port 5744.

## Requirements
- Rust 1.85 or newer

## Build
```bash
cargo build --release
```
## Run

```bash
cargo run --release
```

It listens on 5733, forwards non-CTFS traffic to 5744, writes `output.pdf` to the
current directory, and exits when the sender closes the connection.

## Verify

Using the challenge's [test harness](https://github.com/CoreTechSecurity/fire-storm) (`test.py`), start the program in one terminal
and run the test in another from the directory containing `test.py`:

```bash
# Terminal 1
cargo run --release

# Terminal 2
python3 test.py
```

A passing run prints `OK`. Then open `output.pdf` in a PDF reader to confirm the document reconstructed correctly.