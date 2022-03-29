#pragma once

#include <cstddef>
#include <vector>

#include "Traits/Random.hpp"

namespace symphony::runtime::traits::secio {
  template <typename ClientInputContext, typename Clear>
  void SendInput(const ClientInputContext&, const Clear&);

  template <typename PartyOutputContext, typename Encrypted>
  Encrypted RecvInput(const PartyOutputContext&);

  template <typename PartyInputContext, typename Encrypted>
  void SendOutput(const PartyInputContext&, const Encrypted&);

  template <typename ClientOutputContext, typename Clear>
  Clear RecvOutput(const Context&);
}
