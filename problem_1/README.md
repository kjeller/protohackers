# Problem 1: Prime Time

Protohackers problem 1 solution written in Rust

## Build, Run and Test

- `cargo build` to build application
- `cargo test` to run unit tests.
- `cargo run` to run application (nicely wrapped in cargo)

## Test

There are two ways to test the application against tcp clients:

- Protohackers solution checker provides a way to test solutions against a tcp client online.

- Testing locally can be done using the following command when the tcp server is running.

### Testing locally

Run the following command (create a tcp client using ncat)
```
echo "{\"method\":\"isPrime\",\"number\":123}" | nc -v localhost 48879

# test is successful if the response is:
{"method":"isPrime","prime":false}\n
```
