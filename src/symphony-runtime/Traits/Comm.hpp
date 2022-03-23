#pragma once

#include "../Util/Channel.hpp"

namespace symphony::runtime::traits::comm {
  using namespace util::channel;

  template <typename T>
  void Send(Channel&, T);

  template <typename T>
  T Recv(Channel&);
}
