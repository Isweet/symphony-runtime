#include <cstddef>

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

// Arithmetic modulo 2^l
template <size_t l>
struct Modular;

// Arithmetic modulo 2 (Booleans)
template <>
struct Modular<1> {
  static const Modular Zero;

  static Modular Random(PRG& prg) {
    bool rand;
    prg.RandBytes(std::span<std::byte> { static_cast<std::byte*>(&rand), sizeof(bool) });
    return Modular(
  }

  inline Modular operator+(const Modular other) {
    Modular(repr_ ^ other.repr_);
  }

  inline Modular operator-(const Modular other) {
    Modular(repr_ ^ other.repr_);
  }

  inline Modular operator*(const Modular other) {
    Modular(repr_ & other.repr_);
  }
private:
  Modular(bool v) : repr_(v) {};
  bool repr_;
};

const Modular<1> Modular<1>::Zero = Modular(false);
