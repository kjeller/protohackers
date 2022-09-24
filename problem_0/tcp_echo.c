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
#include <pthread.h>
#include <sched.h>

#define MSGLEN 4096
#define PORT 0xbeef
#define   STACK_SIZE	(4096 * 1024)

void *echo(void *arg) {
  int cl_fd = *(int *)arg;
  char buffer[MSGLEN];

  int r;

  while( (r = read(cl_fd, buffer, MSGLEN)) > 0 ) {
    write(cl_fd, buffer, r);
  }
  close(cl_fd);
}

int main(int argc, char *argv[]) {
  int srv_fd = socket(AF_INET, SOCK_STREAM, 0);

  struct sockaddr_in addr = {
      .sin_family = AF_INET, .sin_addr.s_addr = INADDR_ANY, .sin_port = htons(PORT)};

  int addrlen = sizeof(addr);

  if (bind(srv_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
    exit(EXIT_FAILURE);
  }

  listen(srv_fd, 10);

  while (1) {
    int cl_fd =
        accept(srv_fd, NULL, NULL);

    if (cl_fd < 0) {
      continue;
    }

    pthread_t child_tid;
    pthread_create(&child_tid, NULL, echo, &cl_fd);
    pthread_detach(child_tid);
  }

  close(srv_fd);
  return 0;
}
