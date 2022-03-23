#include "TraitTest.hpp"

template<>
bool Zero<bool> = false;

template <>
bool Neg<bool>(const bool b) {
  return ~b;
}
