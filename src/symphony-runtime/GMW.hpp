#pragma once

#include <cstddef>
#include <vector>

#include "Util/PRG.hpp"
#include "Util/Channel.hpp"
#include "Util/BitVector.hpp"

#include "Traits/Random.hpp"
#include "Traits/Comm.hpp"

#include "Backends/MpSpdz.hpp"

namespace symphony::mpc::gmw {
  using namespace util;
  using namespace traits;
  using namespace GC;

  struct ClientInputContext {
    std::shared_ptr<PRG> prg;
    std::vector<std::shared_ptr<Channel>> receivers;
  };

  struct PartyContext {
    std::shared_ptr<Channel> client;
  };

  struct ClientOutputContext {
    std::vector<std::shared_ptr<Channel>> senders;
  };

  template <typename T>
  using Sec = SemiSecret;

  template <>
  void SendInput(const ClientInputContext& context, const BitVector& input) {
    std::size_t num_receivers = context.receivers.size();

    BitVector masked = input;
    for (std::size_t i = 1; i < num_receivers; i++) {
      BitVector r = Random<BitVector>(*context.prg);
      Send(*context.receivers[i], r);
      masked -= r;
    }

    Send(*context.receivers[0], masked);
  }

  Sec<BitVector> RecvInput(const PartyContext& context) {
    return static_cast<SemiSecret>(Recv<BitVector>(*context.client));
  }

  void SendOutput(const PartyContext& context, const Sec<BitVector>& output) {
    Send(*context.client, static_cast<BitVector>(output));
  }

  BitVector RecvOutput(const ClientOutputContext& context) {
    BitVector ret(CHAR_BIT * sizeof(long));

    for (auto sender : context.senders) {
      ret += Recv<BitVector>(*sender);
    }

    return ret;
  }

  // Each party i involved in P2 chooses random mask ri
  // Each party i calls P1.share(ri)
  // x <- P1.reveal(st + r0 + r1)
  // P2.share(x)
  // P2.share(ri)
  // P2: compute x - r0 - r1

  void SendReshare(const InputContext& context, const Sec<BitVector>& shared) {
    ...
  }

  Sec<BitVector> RecvReshare(const OutputContext& context) {
    ...
  }
}
