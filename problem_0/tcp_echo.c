// TCP Echo Service from RFC 862
// The solution to https://protohackers.com/problem/0
//
// This was used as a template for the x86_64 assembly code.
//
// Neither this file or tcp_echo.asm shouldn't be used in any production
// code base. More error handling is required for that.

#include <netinet/in.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/socket.h>
#include <unistd.h>

#define MSGLEN 512
#define PORT 0xbeef

int main(int argc, char *argv[]) {
  int srv_fd = socket(AF_INET, SOCK_STREAM, 0);

  struct sockaddr_in addr = {
      .sin_family = AF_INET, .sin_addr.s_addr = INADDR_ANY, .sin_port = htons(PORT)};

  int addrlen = sizeof(addr);

  if (bind(srv_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
    exit(EXIT_FAILURE);
  }

  listen(srv_fd, 5);

  char buffer[MSGLEN] = {0};

  while (1) {
    int cl_fd =
        accept(srv_fd, 0, 0);

    if (cl_fd < 0) {
      continue;
    }

    pid_t ret = fork();

    // 0 is returned in the child thread
    if (ret == 0) {
      while (1) {
        read(cl_fd, buffer, MSGLEN);
        write(cl_fd, buffer, MSGLEN);
      }
    } else {
      // Main thread does nothing but accept more connections
    }
  }

  return 0;
}