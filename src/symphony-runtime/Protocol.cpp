#include <cstddef>
#include <span>
#include <vector>
#include <iostream>

// Example of BaseBit trait
class GMWBaseBit {
public:
  using Context = struct GMWContext {};

  GMWBaseBit(Context& context) : context_(context) {};
  GMWBaseBit(Context& context, bool share) : context_(context), is_constant_(false), share_(share) {};
  GMWBaseBit(const GMWBaseBit& other) : context_(other.context_), is_constant_(other.is_constant_), share_(other.share_) {};

  inline GMWBaseBit& operator=(const GMWBaseBit& r) {
    context_     = r.context_;
    is_constant_ = r.is_constant_;
    share_       = r.share_;

    return *this;
  }

  static inline GMWBaseBit Zero(Context& context) {
    GMWBaseBit ret(context, false);
    ret.is_constant_ = true;
    return ret;
  }

  static inline GMWBaseBit One(Context& context) {
    GMWBaseBit ret(context, true);
    ret.is_constant_ = true;
    return ret;
  }

  inline bool From() const {
    return share_;
  }

  inline GMWBaseBit operator^(const GMWBaseBit& r) const {
    return GMWBaseBit(context_, share_ ^ r.share_); // TODO: FIXME
  }

  inline GMWBaseBit operator&(const GMWBaseBit& r) const {
    return GMWBaseBit(context_, share_ & r.share_); // TODO: FIXME
  }

  inline GMWBaseBit operator|(const GMWBaseBit& r) const {
    return (*this ^ r) ^ (*this & r);
  }

  inline GMWBaseBit operator~() const {
    return *this ^ One(context_);
  }

protected:
  Context& context_;
  bool is_constant_;
  bool share_;
};

// Booleans, based on an underlying secure bit
template <typename BaseBit>
class Bool : BaseBit {
public:
  using Context = typename BaseBit::Context;

  Bool(Context& context, bool share) : BaseBit(context, share) {};
  Bool(const Bool& other) : BaseBit(other) {};

  inline Bool& operator=(const Bool& r) {
    *this = r;
    return *this;
  }

  static inline Bool False(Context& context) {
    Bool(BaseBit::Zero(context));
  }

  static inline Bool True(Context& context) {
    Bool(BaseBit::One(context));
  }

  inline Bool operator!() const {
    return ~(*this);
  }

  inline Bool operator&&(const Bool& r) const {
    return *this & r;
  }

  inline Bool operator||(const Bool& r) const {
    return *this | r;
  }

  // g ? a : b ≜ (g & b) ^ (~g & b) ≡ g & (a ^ b) ^ b
  inline BaseBit Mux(const BaseBit& a, const BaseBit& b) const {
    return (*this & (a ^ b)) ^ b;
  }
};

// Bits, based on an underlying secure bit
template <typename BaseBit>
class Bit : public BaseBit {
public:
  using Bool    = Bool<BaseBit>;
  using Context = typename BaseBit::Context;

  Bit(Context& context) : BaseBit(context) {};
  Bit(Context& context, bool share) : BaseBit(context, share) {};
  Bit(const BaseBit& b) : BaseBit(b) {};
  Bit(const Bit& other) : BaseBit(other) {};

  static inline Bit Zero(Context& context) {
    return Bit(BaseBit::Zero(context));
  }

  static inline Bit One(Context& context) {
    return Bit(BaseBit::One(context));
  }

  inline Bool operator==(const Bit& r) const {
    return !Bool(*this ^ r);
  }

  // a != b ≜ !(a == b) ≡ !(!(a ^ b) ≡ ~(~(a ^ b)) ≡ a ^ b
  inline Bool operator!=(const Bit& r) const {
    return Bool(*this ^ r);
  }
};

// BitVectors, based on underlying secure bit
template <typename BaseBit, std::size_t size>
class BitVector {
public:
  using Bit     = Bit<BaseBit>;
  using Context = typename Bit::Context;

  BitVector(Context& context) : context_(context), bits_(size, Bit(context)) {};
  BitVector(Context& context, std::array<Bit, size> bits) : context_(context), bits_(bits) {};
  BitVector(const BitVector& other) : context_(other.context_), bits_(other.bits_) {};

  inline BitVector& operator=(const BitVector& r) {
    context_ = r.context_;
    bits_    = r.bits_;

    return *this;
  }

  static inline BitVector Zero(Context& context) {
    std::array<Bit, size> bits;
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Zero(context);
    }
    return BitVector(context, bits);
  }

  static inline BitVector One(Context& context) {
    std::array<Bit, size> bits;
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::One(context);
    }
    return BitVector(context, bits);
  }

  inline Bit& operator[](std::size_t i) {
    return bits_[i];
  }

  inline Bit Read(std::size_t i) const {
    return bits_[i];
  }

  inline BitVector operator^(const BitVector& r) const {
    BitVector ret(context_);
    for (std::size_t i = 0; i < size; i++) {
      ret.bits_[i] = bits_[i] ^ r.bits_[i];
    }
    return ret;
  }

  inline BitVector operator&(const BitVector& r) const {
    BitVector ret(context_);
    for (std::size_t i = 0; i < size; i++) {
      ret.bits_[i] = bits_[i] & r.bits_[i];
    }
    return ret;
  }

  inline BitVector operator|(const BitVector& r) const {
    BitVector ret(context_);
    for (std::size_t i = 0; i < size; i++) {
      ret.bits_[i] = bits_[i] | r.bits_[i];
    }
    return ret;
  }

  inline BitVector operator~() const {
    BitVector ret(context_);
    for (std::size_t i = 0; i < size; i++) {
      ret.bits_[i] = ~bits_[i];
    }
    return ret;
  }

  inline Bool<Bit> operator==(const BitVector& r) const {
    Bool<Bit> ret = Bool<Bit>::True;
    for (std::size_t i = 0; i < size; i++) {
      ret = ret & (bits_[i] == r.bits_[i]);
    }
    return ret;
  }

  inline Bool<Bit> operator!=(const BitVector& r) const {
    return !(*this == r);
  }

private:
  Context& context_;
  std::vector<Bit> bits_; // TODO: Could get better performance if this was std::array<bool, size> instead?
};

template <typename BaseBit, typename T> // TODO: How do I restrict this to only uint8_t, uint16_t, uint32_t, and uint64_t?
class BaseUInt {
public:
  static const std::size_t size = 8 * sizeof(T);

  using Bit       = Bit<BaseBit>;
  using BitVector = BitVector<BaseBit, size>;
  using Context   = typename BitVector::Context;

  BaseUInt(Context& context, BitVector bv) : context_(context), bv_(bv) {};

  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/utils/utils.hpp#L47
  static inline BitVector ToBits(Context& context, T t) {
    BitVector ret(context);
    for (std::size_t i = 0; i < size; i++) {
      ret[i] = Bit(context, t & 1);
      t >>= 1;
    }
    return ret;
  }

  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/utils/utils.hpp#L36
  inline T From() const {
    T ret = 0;
    for (std::size_t i = 0; i < size; i++) {
      T s = bv_.Read(i).From();
      s <<= i;
      ret |= s;
    }
    return ret;
  }

  inline BaseUInt operator+(const BaseUInt &r) const {
    BaseUInt ret(context_);
    AddFull(&ret.bv_, nullptr, bv_, r.bv_, Bit::Zero(context_), size);
    return ret;
  }

private:

  // https://github.com/emp-toolkit/emp-tool/blob/master/emp-tool/circuits/integer.hpp#L1
  static inline void AddFull(BitVector* sum, Bit* carry_out, const BitVector& a, const BitVector& b, const Bit& carry_in, std::size_t size) {
    if (size == 0 && carry_out) {
      *carry_out = carry_in;
      return;
    }

    Bit carry = carry_in;

    for (std::size_t i = 0; i < size; i++) {
      Bit axc   = a.Read(i) ^ carry;
      Bit bxc   = b.Read(i) ^ carry;
      (*sum)[i] = a.Read(i) ^ bxc;
      Bit t     = axc & bxc;
      carry     = carry ^ t;
    }

    if (carry_out) {
      *carry_out = carry;
    }
  }

  Context& context_;
  BitVector bv_;

  BaseUInt(Context& context) : context_(context), bv_(context) {};
};

using GMWBool      = Bool<GMWBaseBit>;
using GMWBit       = Bit<GMWBaseBit>;
template <std::size_t size>
using GMWBitVector = BitVector<GMWBaseBit, size>;
using GMWUInt8     = BaseUInt<GMWBaseBit, uint8_t>;
using GMWUInt64    = BaseUInt<GMWBaseBit, uint64_t>;

int main() {
  GMWBaseBit::Context context;

  uint64_t a = 2000000;
  uint64_t b = 3000000;
  auto a_share   = GMWUInt64(context, GMWUInt64::ToBits(context, a));
  auto b_share   = GMWUInt64(context, GMWUInt64::ToBits(context, b));
  auto sum_share = a_share + b_share;
  std::cout << unsigned(a_share.From()) << std::endl;
  std::cout << unsigned(b_share.From()) << std::endl;
  std::cout << unsigned(sum_share.From()) << std::endl;
}
