#pragma once

#include <span>
#include <vector>

#include "Channel.hpp"

namespace symphony::runtime::util::channel {
  struct LocalChannel : Channel {
    std::vector<std::byte> buf;
    void SendBytes(std::span<const std::byte> bytes) {
      buf.insert(buf.begin(), bytes.begin(), bytes.end());
    }

    void RecvBytes(std::span<std::byte> bytes) {
      std::copy(buf.begin(), buf.begin() + bytes.size(), bytes.begin());
      buf.erase(buf.begin(), buf.begin() + bytes.size());
    }
  };
}
