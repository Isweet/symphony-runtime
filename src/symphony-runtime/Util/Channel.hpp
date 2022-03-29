#pragma once

#include <span>

namespace symphony::util {
  struct Channel {
    virtual void SendBytes(std::span<const std::byte> bytes) = 0;
    virtual void RecvBytes(std::span<std::byte> bytes) = 0;
  };
}
