#include <cstddef>

#include <PRG.hpp>

struct Boolean {
  bool repr;
  Boolean(bool repr) : repr(repr) {};

  static const Boolean Zero;

  static Boolean Random(PRG&);


};
