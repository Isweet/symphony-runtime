#pragma once

#include "../Util/PRG.hpp"

namespace symphony::runtime::traits::random {
  using namespace util::prg;

  template <typename T>
  T Random(PRG&);
}
