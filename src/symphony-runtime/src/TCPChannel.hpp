#include <cstddef>

#include "Channel.hpp"

class TCPChannel : Channel {
public:
  void SendBytes(const std::span<std::byte>& bytes);
  void RecvBytes(std::span<std::byte>& bytes);
  void Flush();
};
