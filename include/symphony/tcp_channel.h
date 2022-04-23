#ifndef SYMPHONY_TCP_CHANNEL_H__
#define SYMPHONY_TCP_CHANNEL_H__

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

  struct tcp_channel;
  typedef struct tcp_channel tcp_channel_t;

  tcp_channel_t *tcp_channel_create_client(const char *addr, uint16_t port);
  tcp_channel_t *tcp_channel_create_server(uint16_t port);
  void tcp_channel_destroy(tcp_channel_t *self);

#ifdef __cplusplus
}
#endif

#endif
