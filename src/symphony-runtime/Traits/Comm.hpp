#pragma once

#include "../Util/Channel.hpp"

namespace symphony::traits {
  using namespace util;

  template <typename T>
  void Send(Channel&, T);

  template <typename T>
  T Recv(Channel&);
}
