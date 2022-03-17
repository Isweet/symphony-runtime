#include <cstddef>
#include <span>
#include <vector>
#include <iostream>

/* ------------------ *
 * --- GMW Client --- *
 * ------------------ */

class GMWClient {};

/* ----------------- *
 * --- GMW Party --- *
 * ----------------- */

class GMWParty {};

/* --------------------- *
 * --- GMW : BaseBit --- *
 * --------------------- */

class GMWBaseBit {
public:
  using Context = GMWParty;

  GMWBaseBit()  = default;
  ~GMWBaseBit() = default;
  GMWBaseBit(const GMWBaseBit& other) = default;
  inline GMWBaseBit& operator=(const GMWBaseBit& r) = default;
  GMWBaseBit(GMWBaseBit&& other) = default;
  inline GMWBaseBit& operator=(GMWBaseBit&& r) = default;

  GMWBaseBit(Context& context, std::size_t client);
  static inline GMWBaseBit Constant(bool c);
  static inline void Reveal(Context& context, std::size_t client, const GMWBaseBit& v);

  static inline GMWBaseBit Xor(Context& context, const GMWBaseBit& l, const GMWBaseBit& r);
  static inline GMWBaseBit And(Context& context, const GMWBaseBit& l, const GMWBaseBit& r);

  inline bool From() const;
private:
  bool is_constant_;
  bool share_;
  GMWBaseBit(bool internal);
};

inline GMWBaseBit::GMWBaseBit(bool internal) : is_constant_(false), share_(internal) {};

inline GMWBaseBit::GMWBaseBit(Context& context, std::size_t client) : is_constant_(false) {
  assert(false); // TODO: Receive your share
};

inline GMWBaseBit GMWBaseBit::Constant(bool c) {
  GMWBaseBit ret;
  ret.share_ = c;
  ret.is_constant_ = true;
  return ret;
}

inline void GMWBaseBit::Reveal(Context& context, std::size_t client, const GMWBaseBit& v) {
  assert(false); // TODO: Send your share
}

inline GMWBaseBit GMWBaseBit::Xor(Context& context, const GMWBaseBit& l, const GMWBaseBit& r) {
  return GMWBaseBit(l.share_ ^ r.share_); // TODO: FIXME
}

inline GMWBaseBit GMWBaseBit::And(Context& context, const GMWBaseBit& l, const GMWBaseBit& r) {
  return GMWBaseBit(l.share_ & r.share_); // TODO: FIXME
}

inline bool GMWBaseBit::From() const {
  return share_;
}

/* ----------- *
 * --- Bit --- *
 * ----------- */

// A default implementation of the `Bit` (`Bool`) trait, based on an implementation of `BaseBit` trait
template <typename BaseBit>
class Bit : public BaseBit {
public:
  using Context = typename BaseBit::Context;

  Bit()  = default;
  ~Bit() = default;
  Bit(const Bit& other) = default;
  inline Bit& operator=(const Bit& r) = default;
  Bit(Bit&& other) = default;
  inline Bit& operator=(Bit&& r) = default;

  Bit(Context& context) : BaseBit(context) {};
  Bit(const GMWBaseBit& b) : GMWBaseBit(b) {};
  static inline Bit Constant(bool c) {
    return Bit(BaseBit::Constant(c));
  }

  // l | r ≜ (l ^ r) ^ (l & r)
  static inline Bit Or(Context& context, const Bit& l, const Bit& r) {
    GMWBaseBit l_xor_r = Xor(context, l, r);
    GMWBaseBit l_and_r = And(context, l, r);
    return Xor(context, l_xor_r, l_and_r);
  }

  // ~v ≜ v ^ 1
  static inline Bit Not(Context& context, const Bit& v) {
    return Xor(context, v, Constant(true));
  }

  // l == r ≜ ~(l ^ r)
  static inline Bit Eq(Context& context, const Bit& l, const Bit& r) {
    return Not(context, Xor(context, l, r));
  }

  // l != r ≜ ~(l == r) ≡ ~(~(l ^ r) ≡ l ^ r
  static inline Bit NEq(Context& context, const Bit& l, const Bit& r) {
    return Xor(context, l, r);
  }
};

/* ----------------- *
 * --- BitVector --- *
 * ----------------- */

// An implementation of a `BitVector` trait, based on an implementation of `Bit` trait
template <typename Bit>
class BitVector {
public:
  using Context = typename Bit::Context;

  BitVector()  = default;
  ~BitVector() = default;
  BitVector(const BitVector& other) = default;
  inline BitVector& operator=(const BitVector& r) = default;
  BitVector(BitVector&& other) = default;
  inline BitVector& operator=(BitVector&& r) = default;

  BitVector(std::size_t size) : bits_(size) {
    bits_.reserve(size);
  }

  BitVector(Context& context, std::size_t client, std::size_t size) : BitVector(size) {
    for (std::size_t i = 0; i < size; i++) {
      bits_[i] = Bit(context);
    }
  }

  BitVector(const std::vector<Bit>& bits) : bits_(bits) {};

  static inline BitVector Constant(std::vector<bool> c) {
    std::size_t size = c.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Constant(c[i]);
    }
    return BitVector(bits);
  }

  static inline void Reveal(Context& context, std::size_t client, const BitVector& v) {
    std::size_t size = v.bits_.size();
    for (std::size_t i = 0; i < size; i++) {
      Bit::Reveal(context, client, v.bits_[i]);
    }
  }

  inline Bit& operator[](std::size_t i) {
    return bits_[i];
  }

  inline Bit Read(std::size_t i) const {
    return bits_[i];
  }

  static inline BitVector Xor(Context& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Xor(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector And(Context& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::And(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector Or(Context& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::And(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector Not(Context& context, const BitVector& v) {
    std::size_t size = v.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Not(context, v);
    }
    return BitVector(bits);
  }

  static inline Bit Eq(Context& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    Bit ret = Bit::Constant(true);
    for (std::size_t i = 0; i < size; i++) {
      ret = Bit::And(context, ret, Bit::Eq(context, l.bits_[i], r.bits_[i]));
    }
    return ret;
  }

  static inline Bit NEq(Context& context, const BitVector& l, const BitVector& r) {
    return Bit::Not(context, Eq(context, l, r));
  }

private:
  std::vector<Bit> bits_; // TODO: Is there a better representation?
};

// TODO: How do I restrict this to only uint8_t, uint16_t, uint32_t, and uint64_t?
template <typename Bit, template<typename> typename BitVectorFr, typename T>
class BaseUInt {
public:
  static const std::size_t size = 8 * sizeof(T);

  using BitVector = BitVectorFr<Bit>;
  using Context   = typename BitVector::Context;

  BaseUInt() : bv_(size) {};
  ~BaseUInt() = default;
  BaseUInt(const BaseUInt& other) = default;
  inline BaseUInt& operator=(const BaseUInt& r) = default;
  BaseUInt(BaseUInt&& other) = default;
  inline BaseUInt& operator=(BaseUInt&& r) = default;

  BaseUInt(Context& context, std::size_t client) : bv_(BitVector(context, client, size)) {};
  BaseUInt(const BitVector& bv) : bv_(bv) {};

  static inline BaseUInt Constant(T t) {
    return BaseUInt(ToBits(t));
  }

  static inline void Reveal(Context& context, std::size_t client, const BaseUInt& v) {
    BitVector::Reveal(context, client, v.bv_);
  }

  static inline BaseUInt Add(Context& context, const BaseUInt& l, const BaseUInt& r) {
    BaseUInt ret;
    AddFull(context, &ret.bv_, nullptr, l.bv_, r.bv_, Bit::Constant(false), size);
    return ret;
  }

  static inline BaseUInt Sub(Context& context, const BaseUInt& l, const BaseUInt& r) {
    assert(false); // TODO
  }

  static inline BaseUInt Mul(Context& context, const BaseUInt &l, const BaseUInt& r) {
    assert(false); // TODO
  }

  static inline BaseUInt Div(Context& context, const BaseUInt &l, const BaseUInt& r) {
    assert(false); // TODO
  }

  static inline BaseUInt Mod(Context& context, const BaseUInt &l, const BaseUInt& r) {
    assert(false); // TODO
  }

  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/utils/utils.hpp#L36
  inline T FrBits() const {
    T ret = 0;
    for (std::size_t i = 0; i < size; i++) {
      T s = bv_.Read(i).From();
      s <<= i;
      ret |= s;
    }
    return ret;
  }

private:
  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/utils/utils.hpp#L47
  static inline BitVector ToBits(T t) {
    std::vector<bool> ret(size);
    ret.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      ret[i] = t & 1;
      t >>= 1;
    }
    return BitVector::Constant(ret);
  }

  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/circuits/integer.hpp#L1
  static inline void AddFull(Context& context, BitVector* sum, Bit* carry_out, const BitVector& a, const BitVector& b, const Bit& carry_in, std::size_t size) {
    if (size == 0 && carry_out) {
      *carry_out = carry_in;
      return;
    }

    Bit carry = carry_in;

    for (std::size_t i = 0; i < size; i++) {
      Bit axc   = Bit::Xor(context, a.Read(i), carry);
      Bit bxc   = Bit::Xor(context, b.Read(i), carry);
      (*sum)[i] = Bit::Xor(context, a.Read(i), bxc);
      Bit t     = Bit::And(context, axc, bxc);
      carry     = Bit::Xor(context, carry, t);
    }

    if (carry_out) {
      *carry_out = carry;
    }
  }

  BitVector bv_;
};

/* GMW */
using GMWBit    = Bit<GMWBaseBit>;
using GMWUInt8  = BaseUInt<GMWBit, BitVector, uint8_t>;
using GMWUInt64 = BaseUInt<GMWBit, BitVector, uint64_t>;

int main() {
  GMWParty context;

  uint64_t a = 2000000;
  uint64_t b = 3000000;
  auto a_share   = GMWUInt64::Constant(a);
  auto b_share   = GMWUInt64::Constant(b);
  auto sum_share = GMWUInt64::Add(context, a_share, b_share);
  std::cout << unsigned(a_share.FrBits()) << std::endl;
  std::cout << unsigned(b_share.FrBits()) << std::endl;
  std::cout << unsigned(sum_share.FrBits()) << std::endl;
}
