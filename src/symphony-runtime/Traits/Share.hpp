#pragma once

#include <cstddef>
#include <vector>

namespace symphony::runtime::traits::encrypt {
  template <typename Context, typename Clear>
  void SendEncrypted(const Context&, const std::vector<std::size_t>&, const Clear&);

  template <typename Context, typename Encrypted>
  Encrypted RecvEncrypted(Context&, std::size_t);

  template <typename Context, typename Encrypted>
  void SendDecrypted(const Context&, std::size_t, const Encrypted&);

  template <typename Context, typename Clear>
  Clear RecvDecrypted(const Context&, const std::vector<std::size_t>&);
}
