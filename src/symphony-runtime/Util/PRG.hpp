#pragma once

#include <cstddef>
#include <vector>
#include <span>
#include <random>

namespace symphony::util {
  struct PRG {
    virtual void RandBytes(std::span<std::byte> bytes) = 0;
  };

  struct PlainPRG : PRG {
    using rbe = std::independent_bits_engine<std::default_random_engine, CHAR_BIT * sizeof(short), unsigned short>;
    rbe gen;

    void RandBytes(std::span<std::byte> bytes) {
      std::size_t num_bytes = bytes.size();
      std::vector<unsigned short> rands(num_bytes);
      std::generate(rands.begin(), rands.end(), std::ref(gen));
      for (std::size_t i = 0; i < num_bytes; i++) {
        bytes[i] = static_cast<std::byte>(rands[i]);
      }
    }
  };
}
