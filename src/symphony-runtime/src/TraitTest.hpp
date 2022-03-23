template <typename T>
T Zero;

// Negation
template <typename T>
T Neg(const T);

// Addition
template <typename T>
T Add(const T, const T);

// Subtraction
template <typename T>
T Sub(const T, const T);

template <typename T>
T Sub(const T lhs, const T rhs) {
  return Add<T>(lhs, Neg<T>(rhs));
}

// Booleans
template <>
bool Zero<bool> = false;

template <>
bool Neg<bool>(const bool b) {
  return b;
}

template <>
bool Add<bool>(const bool lhs, const bool rhs) {
  return lhs ^ rhs;
}
