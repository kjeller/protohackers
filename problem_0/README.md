# Problem 0: Smoke Test

Protohackers problem 0 solution(s) written in:
- [x86_64 NASM & C](asm_c/README.md)
- [Rust](rust/README.md)

## Test

There are two ways to test the application against tcp clients:

- Protohackers solution checker provides a way to test solutions against a tcp client online.

- Testing locally can be done using the following command when the tcp server is running.

### Testing locally

Run the following command (create a tcp client using ncat)
```
echo "test" | nc -v localhost 48879
```

If test is returned from the tcp server the test is successful.
