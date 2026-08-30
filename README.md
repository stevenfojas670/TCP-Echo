# TCP Echo

A TCP echo server written in Rust. It listens on localhost, reads whatever you send it, and immediately responds with the same string.

## Overview

The server uses Rust's `TcpListener` to accept incoming TCP connections on `127.0.0.1:<any_port>`. For each connection it reads the request and writes the same bytes straight back.

## Environment Setup

### Server

Rust application hosted on `127.0.0.1:<any_port>`.

```
cargo run
```

### Requestor

Utilize `PuTTy` and connect via `Raw` to make any requests.

Then connect:

```
127.0.0.1 7878
```

## Test

Send simple messages and wait for a response. The server should echo back exactly what was sent.

## License

See [LICENSE](LICENSE).
