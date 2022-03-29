#pragma once

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

#include "Traits/Group.hpp"
#include "Traits/Random.hpp"

namespace symphony::traits {
  template<>
  bool Zero<bool> = false;

  template<>
  inline bool Neg<bool>(const bool& b) {
    return b;
  }

  template<>
  inline bool Add<bool>(const bool& lhs, const bool& rhs) {
    return lhs ^ rhs;
  }

  template<>
  inline bool Random(PRG& prg) {
    bool ret;
    prg.RandBytes(std::span<std::byte> { static_cast<std::byte*>(static_cast<void*>(&ret)), sizeof(bool) });
    return ret;
  }

  template<>
  inline void Send(Channel& channel, bool b) {
    channel.SendBytes(std::span<const std::byte> { static_cast<const std::byte*>(static_cast<void*>(&b)), sizeof(bool) });
  }

  template<>
  inline bool Recv(Channel& channel) {
    bool b;
    channel.RecvBytes(std::span<std::byte> { static_cast<std::byte*>(static_cast<void*>(&b)), sizeof(bool) });
    return b;
  }
}
