#include <iostream>

#include "../symphony-runtime/Traits/Group.hpp"
#include "../symphony-runtime/Boolean.hpp"

using namespace symphony::runtime::traits::group;

void test_zero() {
  assert(Zero<bool> == false);
}

void test_neg() {
  assert(Neg(false) == false);
  assert(Neg(true) == true);
}

void test_add() {
  assert(Add(false, false) == false);
  assert(Add(false, true)  == true);
  assert(Add(true, false) == true);
  assert(Add(true, true) == false);
}

void test_sub() {
  bool bools[2] = { false, true };
  for (size_t i = 0; i < 2; i++) {
    for (size_t j = 0; j < 2; j++) {
      assert(Add(bools[i], bools[j]) == Sub(bools[i], bools[j]));
    }
  }
}

int main() {
  test_zero();
  test_neg();
  test_add();
  test_sub();

  return 0;
}
