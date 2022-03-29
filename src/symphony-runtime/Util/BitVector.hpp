#pragma once

#include <cstddef>
#include <vector>

#include "../Backends/MpSpdz.hpp"

namespace symphony::util {
  using namespace GC;

  struct BitVector {
    std::vector<bool> bits;

    BitVector(bool b) : bits(1, b) {}

    BitVector(std::size_t size) : bits(size, false) {}

    explicit BitVector(long l) {
      assert(false); // TODO
    }
    explicit operator long() {
      assert(false); // TODO
    }

    explicit BitVector(const BitVec& b) : BitVector(b.get()) {}
    explicit operator BitVec() {
      return BitVec(static_cast<long>(*this));
    }

    explicit BitVector(const SemiSecret& sh) : BitVector(static_cast<BitVec>(sh)) {}
    explicit operator SemiSecret() {
      SemiSecret(static_cast<BitVec>(*this));
    }3w
  };

}
