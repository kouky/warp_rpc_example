# Warp RPC Example

An example of how to build Rust HTTP RPC services with [warp_rpc](https://github.com/kouky/warp_rpc)
macros.

## Running

Build the project.

        cargo build

Start the server for the example `GreetingService`.

        RUST_LOG=info cargo run -p greeting_server

Run the executable which uses a service `GreetingClient` to talk to the server.

        RUST_LOG=info cargo run

## License

MIT
