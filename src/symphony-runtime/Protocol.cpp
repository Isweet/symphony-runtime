#include <cstddef>
#include <span>
#include <vector>
#include <iostream>

struct SPDZ {};

struct GMWBaseBool {
  using Base    = GMWBaseBool;
  using Context = SPDZ;
  using Clear   = bool;
  using Share   = bool;

  static Share Constant(Context& context, Clear clear) {
    return clear; // TODO: FIXME, provided by MP-SPDZ
  }

  static Share Add(Context& context, Share l, Share r) {
    return l ^ r;
  }

  static Share Neg(Context& context, Share v) {
    return v;
  }

  static Share Mul(Context& context, Share l, Share r) {
    return l & r; // TODO: FIXME, provided by MP-SPDZ
  }
};

template <typename P>
struct BoolFrBoolBase {
  using Base    = P;
  using Context = typename Base::Context;
  using Clear   = typename Base::Clear;
  using Share   = typename Base::Share;

  static Share Constant(Context& context, Clear clear) {
    return Base::Constant(context, clear);
  }

  static Share Xor(Context& context, Share l, Share r) {
    return Base::Add(context, l, r);
  }

  static Share Not(Context& context, Share v) {
    return Base::Neg(context, v);
  }

  static Share And(Context& context, Share l, Share r) {
    return Base::Mul(context, l, r);
  }

  static Share Or(Context& context, Share l, Share r) {
    return Xor(context, Xor(context, l, r), And(context, l, r));
  }

  static Share Mux(Context& context, Share g, Share l, Share r) {
    return Xor(context, And(context, g, l), And(context, Not(context, g), r));
  }
};

template <typename P>
struct BoolVecFrBool {
  using Base    = P;
  using Context = typename Base::Context;
  template <size_t bw>
  using Clear   = std::array<typename Base::Clear, bw>;
  template <size_t bw>
  using Share   = std::array<typename Base::Share, bw>;

  template <size_t bw>
  static Share<bw> Constant(Context& context, Clear<bw> clear) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::Constant(context, clear[i]);
    }
    return ret;
  }

  template <size_t bw>
  static Share<bw> Xor(Context& context, Share<bw> l, Share<bw> r) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::Xor(context, l[i], r[i]);
    }
    return ret;
  }

  template <size_t bw>
  static Share<bw> Not(Context& context, Share<bw> v) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::Not(context, v[i]);
    }
    return ret;
  }

  template <size_t bw>
  static Share<bw> And(Context& context, Share<bw> l, Share<bw> r) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::And(context, l[i], r[i]);
    }
    return ret;
  }

  template <size_t bw>
  static Share<bw> Or(Context& context, Share<bw> l, Share<bw> r) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::Or(context, l[i], r[i]);
    }
    return ret;
  }

  template <size_t bw>
  static Share<bw> Mux(Context& context, Share<bw> g, Share<bw> l, Share<bw> r) {
    Share<bw> ret;
    for (std::size_t i = 0; i < bw; i++) {
      ret[i] = Base::Mux(context, g[i], l[i], r[i]);
    }
    return ret;
  }
};

template <typename T>
constexpr std::size_t bits() {
  return 8 * sizeof(T);
}

template <typename P>
struct ArithBaseFrBoolVec {
  using Base    = P;
  using Context = typename Base::Context;
  template <typename Z>
  using Clear   = Z;
  template <typename Z>
  using Share   = typename Base::template Share<bits<Z>()>;

  using Bool      = typename Base::Base;
  using BoolShare = typename Bool::Share;

  template <typename Z>
  using BaseClear = typename Base::template Clear<bits<Z>()>;

  template <typename Z>
  static Share<Z> Constant(Context& context, Z z) {
    std::size_t bits = ::bits<Z>();
    BaseClear<Z> clear;
    for (std::size_t i = 0; i < bits; i++) {
      clear[i] = z & 1;
      z >>= 1;
    }
    return Base::Constant(context, clear);
  }

  static std::pair<BoolShare, BoolShare> FullAdd(Context& context, BoolShare a, BoolShare b, BoolShare cin) {
    BoolShare axb  = Bool::Xor(context, a, b);
    BoolShare sum  = Bool::Xor(context, axb, cin);
    BoolShare cout = Bool::Or(context, Bool::And(context, axb, cin), Bool::And(context, a, b));
    return std::make_pair(sum, cout);
  }

  template <typename Z>
  static Share<Z> Add(Context& context, Share<Z> l, Share<Z> r) {
    std::size_t bits = ::bits<Z>();
    Share<Z> ret;
    BoolShare sum;
    BoolShare carry = Bool::Constant(context, false);
    for (std::size_t i = 0; i < bits; i++) {
      std::pair<BoolShare, BoolShare> p = FullAdd(context, l[i], r[i], carry);
      sum    = p.first;
      carry  = p.second;
      ret[i] = sum;
    }
    return ret;
  }
};

using GMWBoolBool      = BoolFrBoolBase<GMWBaseBool>;
using GMWBoolBoolVec   = BoolVecFrBool<GMWBoolBool>;
using GMWBoolArithBase = ArithBaseFrBoolVec<GMWBoolBoolVec>;

uint8_t frBool(std::array<bool, 8> bs) {
  uint8_t ret = 0;
  for (std::size_t i = 0; i < 8; i++) {
    bool b = bs[i];
    uint8_t s = b ? 1 : 0;
    s <<= i;
    ret |= s;
  }
  return ret;
}

int main() {
  SPDZ context;
  uint8_t a = 2;
  std::array<bool, 8> a_share = GMWBoolArithBase::Constant(context, a);
  uint8_t b = 3;
  std::array<bool, 8> b_share = GMWBoolArithBase::Constant(context, b);
  std::array<bool, 8> sum = GMWBoolArithBase::Add<uint8_t>(context, a_share, b_share);
  std::cout << unsigned(frBool(a_share)) << std::endl;
  std::cout << unsigned(frBool(b_share)) << std::endl;
  std::cout << unsigned(frBool(sum)) << std::endl;
  return 0;
}

//using GMWBoolArith     = ArithFrArithBase<GMWBoolArithBase>;
//using GMWBoolArithVec  = ArithVecFrArith<GMWBoolArith>;

//struct GMW : public GMWBoolBool, public GMWBoolBoolVec, public GMWBoolArith, GMWBoolArithVec {};
