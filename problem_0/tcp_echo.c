// TCP Echo Service from RFC 862
// The solution to https://protohackers.com/problem/0
//
// This was used as a template for the x86_64 assembly code.
//
// Neither this file or tcp_echo.asm shouldn't be used in any production
// code base. More error handling is required for that.
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <string.h>
#include <errno.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <signal.h>
#include <sys/wait.h>

#define MSGLEN 4096
#define PORT 0xbeef

void echo(int cl_fd) {
  char buffer[MSGLEN];
  int r;

  while( (r = read(cl_fd, buffer, MSGLEN)) > 0 ) {
    write(cl_fd, buffer, r);
  }
}

int main(int argc, char *argv[]) {
  int srv_fd = socket(AF_INET, SOCK_STREAM, 0);

  struct sockaddr_in addr = {
      .sin_family = AF_INET, .sin_addr.s_addr = INADDR_ANY, .sin_port = htons(PORT)};

  int addrlen = sizeof(addr);

  if (bind(srv_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
    exit(EXIT_FAILURE);
  }

  listen(srv_fd, 511);

  while (1) {
    int cl_fd =
        accept(srv_fd, NULL, NULL);

    if (cl_fd < 0) {
      continue;
    }
    
    pid_t pid = fork();
    if (pid == -1) {
      exit(0);
    }
    
    if (pid == 0) {
      // in child process
      close(srv_fd);
      echo(cl_fd);
      close(cl_fd);
      
    } else if (pid > 0) {
      // in parent process
      close(cl_fd);
    }
  }

  close(srv_fd);
  return 0;
}
