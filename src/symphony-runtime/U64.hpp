#pragma once

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

#include "Traits/Group.hpp"
#include "Traits/Random.hpp"
#include "Traits/Comm.hpp"

namespace symphony::traits {
  template<>
  bool Zero<uint64_t> = 0;

  template<>
  inline uint64_t Neg<uint64_t>(const uint64_t& v) {
    return -v;
  }

  template<>
  inline uint64_t Add<uint64_t>(const uint64_t& lhs, const uint64_t& rhs) {
    return lhs + rhs;
  }

  template<>
  inline uint64_t Random(PRG& prg) {
    uint64_t ret;
    prg.RandBytes(std::span<std::byte> { static_cast<std::byte*>(static_cast<void*>(&ret)), sizeof(uint64_t) });
    return ret;
  }

  template<>
  inline void Send(Channel& channel, uint64_t v) {
    channel.SendBytes(std::span<const std::byte> { static_cast<const std::byte*>(static_cast<void*>(&v)), sizeof(uint64_t) });
  }

  template<>
  inline uint64_t Recv(Channel& channel) {
    uint64_t v;
    channel.RecvBytes(std::span<std::byte> { static_cast<std::byte*>(static_cast<void*>(&v)), sizeof(uint64_t) });
    return v;
  }
}
