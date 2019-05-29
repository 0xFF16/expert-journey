#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>

#define BUFFER 1024

int main(int argc, char** argv)
{
  if (argc < 2) {
    printf("Modo de Uso:\n"
           "\t %s <host> <port>\n"
           "Ex:\t %s 192.168.0.1 22\n", argv[0], argv[0]);
    exit(1);
  } else if (argc > 3)
    exit(1);

  int desc_socket;
  struct sockaddr_in host;
  char buff[BUFFER];

  host.sin_family = AF_INET;
  host.sin_addr.s_addr = inet_addr(argv[1]);
  host.sin_port = htons((uint16_t)atoi(argv[2]));

  memset(&(host.sin_zero), 0x0, sizeof(host.sin_zero));

  if ((desc_socket = socket(AF_INET, SOCK_STREAM, 0)) < 0) {
    perror("socket()");
    exit(1);
  }

  if ((connect(desc_socket, (struct sockaddr *)&host, sizeof(host))) < 0) {
    perror("connect()");
    exit(1);
  } else
    printf("Scanning port: %d\n", (uint16_t)atoi(argv[2]));

  if ((recv(desc_socket, buff, BUFFER, 0)) < 0) {
    perror("recv()");
    exit(1);
  } else {
    printf("Service running at port %d: %s", (uint16_t)atoi(argv[2]), buff);
    exit(1);
  }
}
