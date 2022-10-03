# Problem 0: Smoke Test
Protohackers problem 0 solution written in x86_64 NASM and C.

## Build

Build and execution instructins for tcp_echo

### Prerequisite(s)

The tcp server code is written in x86_64 with NASM syntax.

Example of installing nasm using apt:
```
sudo apt install nasm
```

### Build and run
```
# Build and link tcp server
make asm

# Run tcp server
./bin/tcp_server
```

## Test

Two ways to test the TCP echo server:

- Protohackers solution checker provides a way to test solutions against a tcp client online.

- Testing locally can be done using the following command when the tcp server is running.

### Testing locally

Run the following command (create a tcp client using ncat)
```
echo "test" | nc -v localhost 48879
```

If test is returned from the tcp server the test is successful.
