#pragma once

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

#include "Traits/Group.hpp"
#include "Traits/Random.hpp"

namespace symphony::runtime {
  namespace traits::group {
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
  }

  namespace traits::random {
    template<>
    inline bool Random(PRG& prg) {
      std::vector<std::byte> bytes = prg.RandBytes(1);
      return static_cast<bool>(bytes[0]);
    }
  }

  namespace traits::comm {
    template<>
    inline void Send(Channel& channel, bool b) {
      channel.SendBytes(std::span<const std::byte> { static_cast<const std::byte*>(static_cast<void*>(&b)), 1 });
    }

    template<>
    inline bool Recv(Channel& channel) {
      bool b;
      channel.RecvBytes(std::span<std::byte> { static_cast<std::byte*>(static_cast<void*>(&b)), 1 });
      return b;
    }
  }
}
