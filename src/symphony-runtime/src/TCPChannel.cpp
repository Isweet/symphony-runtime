#include <cstddef>

#include "TCPChannel.hpp"

void TCPChannel::SendBytes(const std::span<std::byte>& bytes) {
  return;
}

void TCPChannel::RecvBytes(std::span<std::byte>& bytes) {
  return;
}

void TCPChannel::Flush() {
  return;
}
