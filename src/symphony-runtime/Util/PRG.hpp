#pragma once

#include <cstddef>
#include <vector>
#include <span>

namespace symphony::runtime::util::prg {
  struct PRG {
    virtual std::vector<std::byte> RandBytes(std::size_t num_bytes) = 0;
  };

  struct DummyPRG : PRG {
    std::vector<std::byte> RandBytes(std::size_t num_bytes) {
      return std::vector<std::byte>(num_bytes, std::byte {0});
    }
  };
}
