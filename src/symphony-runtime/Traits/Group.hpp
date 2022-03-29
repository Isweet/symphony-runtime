#pragma once

namespace symphony::traits {
  template <typename T>
  const T Zero;

  template <typename T>
  T Neg(const T&);

  template <typename T>
  T Add(const T&, const T&);

  template <typename T>
  T Sub(const T& lhs, const T& rhs) {
    return Add(lhs, Neg(rhs));
  }
}
