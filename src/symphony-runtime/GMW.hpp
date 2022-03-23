#pragma once

#include "Util/PRG.hpp"
#include "Util/Channel.hpp"

#include "Traits/Group.hpp"
#include "Traits/Random.hpp"
#include "Traits/Comm.hpp"
#include "Traits/Share.hpp"

namespace symphony::runtime {
  namespace gmw {
    using namespace util::channel;
    using namespace util::prg;

    struct Context {
      std::vector<std::shared_ptr<Channel>> channels;
      std::unique_ptr<PRG> prg;
    };

    template <typename T>
    struct Encrypted {
      T repr;
      bool is_constant;
    };
  }

  namespace traits::share {
    using namespace traits::group;
    using namespace traits::random;
    using namespace traits::comm;

    template<typename Clear>
    void SendEncrypted(const gmw::Context& context, const std::vector<std::size_t>& receivers, const Clear& clear) {
      std::size_t num_receivers = receivers.size();

      Clear sum = Zero<Clear>;

      for (std::size_t i = 1; i < num_receivers; i++) {
        Clear share = Random<Clear>(*context.prg);
        Send(*context.channels[receivers[i]], share);
        sum = Add(sum, share);
      }

      Send(*context.channels[receivers[0]], Sub(clear, sum));
    }
  }
}
