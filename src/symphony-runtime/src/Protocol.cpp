#include <cstddef>
#include <span>
#include <vector>
#include <iostream>

class GMWBaseBit;

/* -------------------------- *
 * --- Replicated Context --- *
 * -------------------------- */

class RepContext {
private:
  std::size_t id_;
  std::vector<std::shared_ptr<Channel>> parties_;
public:
  std::size_t Me() const;
  template <typename T>
  void Send(std::size_t id, T);
  template <typename T>
  T Recv(std::size_t id);
};

/* ---------------------------- *
 * --- Replicated : BaseBit --- *
 * ---------------------------- */

class RepBaseBit {
public:
  using Context = RepContext;

  RepBaseBit()                        = default;
  RepBaseBit(const RepBaseBit& other) = default;
  RepBaseBit(RepBaseBit&& other)      = default;
  ~RepBaseBit()                       = default;

  inline RepBaseBit& operator=(const RepBaseBit& r) = default;
  inline RepBaseBit& operator=(RepBaseBit&& r)      = default;

  RepBaseBit(bool v) : value_(v) {};
  static inline bool From(const RepBaseBit& v);

  static inline void       ShareTo(Context& local, Context& group, std::vector<std::size_t> to, const RepBaseBit& v);
  static inline RepBaseBit ShareFr(Context& local, Context& group, std::vector<std::size_t> fr);

  static inline RepBaseBit Xor(const RepBaseBit& l, const RepBaseBit& r);
  static inline RepBaseBit And(Context& context, const RepBaseBit& l, const RepBaseBit& r);
private:
  bool value_;
};

inline bool RepBaseBit::From(const RepBaseBit& v) {
  return v.value_;
}

inline void RepBaseBit::ShareTo(Context& local, Context& group, std::vector<std::size_t> sharees, const RepBaseBit& v) {
  if (!(local.Me() == 0)) {
    return;
  }

  std::size_t num_sharees = sharees.size();

  for (std::size_t i = 0; i < num_sharees; i++) {
    group.Send(sharees[i], v.value_);
  }
}

inline RepBaseBit RepBaseBit::ShareFr(Context& local, Context& group, std::vector<std::size_t> sharers) {
  return RepBaseBit(group.Recv<bool>(sharers[0]));
}

inline RepBaseBit RepBaseBit::Xor(const RepBaseBit& l, const RepBaseBit& r) {
  return RepBaseBit(l.value_ ^ r.value_);
}

inline RepBaseBit RepBaseBit::And(Context& context, const RepBaseBit& l, const RepBaseBit& r) {
  return RepBaseBit(l.value_ & r.value_);
}

/* ------------------- *
 * --- GMW Context --- *
 * ------------------- */

class GMWContext {
private:
  std::vector<std::shared_ptr<Channel>> parties_;
public:
  inline bool IAm(std::size_t party);
  template <typename T>
  void Send(std::size_t client, T v);
  template <typename T>
  T Recv(std::size_t client);
};

/* --------------------- *
 * --- GMW : BaseBit --- *
 * --------------------- */

// TODO: Document `BaseBit` trait -- basically the public portion of GMWBaseBit below

class GMWBaseBit {
public:
  using Context = GMWContext;

  GMWBaseBit()                        = default;
  GMWBaseBit(const GMWBaseBit& other) = default;
  GMWBaseBit(GMWBaseBit&& other)      = default;
  ~GMWBaseBit()                       = default;

  inline GMWBaseBit& operator=(const GMWBaseBit& r) = default;
  inline GMWBaseBit& operator=(GMWBaseBit&& r)      = default;

  static inline void       ShareTo(Context& local, Context& group, std::vector<std::size_t> to, const GMWBaseBit& v);
  static inline GMWBaseBit ShareFr(Context& local, Context& group, std::vector<std::size_t> fr);

  static inline GMWBaseBit Xor(const GMWBaseBit& l, const GMWBaseBit& r);
  static inline GMWBaseBit And(Party& context, const GMWBaseBit& l, const GMWBaseBit& r);

  inline bool From() const; // TODO: Remove, just here for testing
private:
  GMWBaseBit(bool internal);

  bool is_constant_;
  bool share_;
};

inline GMWBaseBit::GMWBaseBit(bool internal) : is_constant_(false), share_(internal) {};

inline void GMWBaseBit::ShareTo(Client& context, bool b) {
  std::size_t num_parties = context.NumParties();
  bool sum = false;
  for (std::size_t i = 0; i < num_parties - 1; i++) {
    bool share = context.Rand<bool>();
    context.Send(i, share);
    sum ^= share;
  }

  context.Send<bool>(num_parties - 1, b ^ sum);
}

inline void GMWBaseBit::ShareTo(Client& context, const GMWBaseBit& v) {
  GMWBaseBit::ShareTo(context, v.share_);
}

// TODO: Should this be a constructor? It is pleasant to me to have the symmetry with `ShareTo`
inline GMWBaseBit GMWBaseBit::ShareFr(Party& context, std::size_t client) {
  return GMWBaseBit(context.Recv<bool>(client));
}

inline GMWBaseBit GMWBaseBit::ShareFr(Party& context, std::vector<std::size_t> clients) {
  std::size_t num_clients = clients.size();
  bool share = false;
  for (std::size_t i = 0; i < num_clients; i++) {
    share ^= context.Recv<bool>(clients[i]);
  }
  return GMWBaseBit(share);
}

inline GMWBaseBit GMWBaseBit::Constant(bool c) {
  GMWBaseBit ret;
  ret.share_ = c;
  ret.is_constant_ = true;
  return ret;
}

inline void GMWBaseBit::RevealTo(Party& context, std::size_t client, const GMWBaseBit& v) {
  if (v.is_constant_) {
    if (context.IAm(0)) {
      context.Send<bool>(client, v.share_);
    } else {
      context.Send<bool>(client, false);
    }

    return;
  }

  context.Send<bool>(client, v.share_);
}

inline bool GMWBaseBit::RevealFr(Client& context) {
  std::size_t num_parties = context.NumParties();
  bool ret = false;
  for (std::size_t i = 0; i < num_parties; i++) {
    ret ^= context.Recv<bool>(i);
  }
  return ret;
}

inline GMWBaseBit GMWBaseBit::Xor(Party& context, const GMWBaseBit& l, const GMWBaseBit& r) {
  if (l.is_constant_ && r.is_constant_) {
    return Constant(l.share_ ^ r.share_);
  }

  if (l.is_constant_ && !context.IAm(0)) {
    return r;
  }

  if (r.is_constant_ && !context.IAm(0)) {
    return l;
  }

  return GMWBaseBit(l.share_ ^ r.share_);
}

inline GMWBaseBit GMWBaseBit::And(Party& context, const GMWBaseBit& l, const GMWBaseBit& r) {
  if (l.is_constant_ && r.is_constant_) {
    return Constant(l.share_ & r.share_);
  }

  if (l.is_constant_ || r.is_constant_) {
    return GMWBaseBit(l.share_ & r.share_);
  }

  return GMWBaseBit(l.share_ & r.share_); // TODO: FIXME
}

inline bool GMWBaseBit::From() const {
  return share_;
}

/* ----------- *
 * --- Bit --- *
 * ----------- */

// TODO: Document `Bit` trait -- basically the public portion of Bit below

// A default implementation of the `Bit` (`Bool`) trait, based on an implementation of `BaseBit` trait
template <typename BaseBit>
class Bit : public BaseBit {
public:
  using Client = typename BaseBit::Client;
  using Party  = typename BaseBit::Party;

  Bit()                 = default;
  Bit(Bit&& other)      = default;
  Bit(const Bit& other) = default;
  ~Bit()                = default;

  inline Bit& operator=(const Bit& r) = default;
  inline Bit& operator=(Bit&& r)      = default;

  Bit(const GMWBaseBit& b) : GMWBaseBit(b) {};

  static inline void ShareTo(Client& context, bool b) {
    BaseBit::ShareTo(context, b);
  }

  static inline void ShareTo(Client& context, const Bit& v) {
    BaseBit::ShareTo(context, v);
  }

  static inline Bit ShareFr(Party& context, std::size_t client) {
    return Bit(BaseBit::ShareFr(context, client));
  }

  static inline Bit ShareFr(Party& context, std::vector<std::size_t> clients) {
    return Bit(BaseBit::ShareFr(context, clients));
  }

  static inline Bit Constant(bool c) {
    return Bit(BaseBit::Constant(c));
  }

  static inline void RevealTo(Party& context, std::size_t client, const Bit& v) {
    BaseBit::RevealTo(context, client, v);
  }

  static inline bool RevealFr(Client& context) {
    BaseBit::RevealFr(context);
  }

  // l | r ≜ (l ^ r) ^ (l & r)
  static inline Bit Or(Party& context, const Bit& l, const Bit& r) {
    GMWBaseBit l_xor_r = Xor(context, l, r);
    GMWBaseBit l_and_r = And(context, l, r);
    return Xor(context, l_xor_r, l_and_r);
  }

  // ~v ≜ v ^ 1
  static inline Bit Not(Party& context, const Bit& v) {
    return Xor(context, v, Constant(true));
  }

  // l == r ≜ ~(l ^ r)
  static inline Bit Eq(Party& context, const Bit& l, const Bit& r) {
    return Not(context, Xor(context, l, r));
  }

  // l != r ≜ ~(l == r) ≡ ~(~(l ^ r) ≡ l ^ r
  static inline Bit NEq(Party& context, const Bit& l, const Bit& r) {
    return Xor(context, l, r);
  }
};

/* ----------------- *
 * --- BitVector --- *
 * ----------------- */

// TODO: Document `BitVector` trait -- basically the public portion of BitVector below

// An implementation of a `BitVector` trait, based on an implementation of `Bit` trait
template <typename Bit>
class BitVector {
public:
  using Client = typename Bit::Client;
  using Party  = typename Bit::Party;

  BitVector()                       = default;
  BitVector(const BitVector& other) = default;
  BitVector(BitVector&& other)      = default;
  ~BitVector()                      = default;

  inline BitVector& operator=(const BitVector& r) = default;
  inline BitVector& operator=(BitVector&& r)      = default;

  BitVector(std::size_t size) : bits_(size) {};
  BitVector(const std::vector<Bit>& bits) : bits_(bits) {};

  inline std::size_t& Size() const {
    return bits_.size();
  }

  inline Bit& operator[](std::size_t i) {
    return bits_[i];
  }

  inline Bit Read(std::size_t i) const {
    return bits_[i];
  }

  static inline void ShareTo(Client& context, std::vector<bool> b) {
    std::size_t num_parties = context.NumParties();
    std::size_t num_bits    = b.size();

    for (std::size_t i = 0; i < num_parties; i++) {
      context.Send(i, num_bits);
    }

    for (std::size_t i = 0; i < num_bits; i++) {
      Bit::ShareTo(context, b[i]);
    }
  }

  static inline void ShareTo(Client& context, const BitVector& v) {
    std::size_t num_parties = context.NumParties();
    std::size_t num_bits    = v.Size();

    for (std::size_t i = 0; i < num_parties; i++) {
      context.Send(i, num_bits);
    }

    for (std::size_t i = 0; i < num_bits; i++) {
      Bit::ShareTo(context, v[i]);
    }
  }

  static inline GMWBaseBit ShareFr(Party& context, std::size_t client) {
    std::size_t num_bits = context.template Recv<std::size_t>(client);
    std::vector<Bit> bits(num_bits);
    for (std::size_t i = 0; i < num_bits; i++) {
      bits[i] = Bit::ShareFr(context, client);
    }
    return BitVector(bits);
  }

  static inline GMWBaseBit ShareFr(Party& context, std::vector<std::size_t> clients) {
    std::size_t num_parties = context.NumParties();
    std::size_t num_bits;

    for (std::size_t i = 0; i < num_parties; i++) {
      num_bits = context.template Recv<std::size_t>(clients[i]); // TODO: FIX ... this is dumb
    }

    std::vector<Bit> bits(num_bits);
    for (std::size_t i = 0; i < num_bits; i++) {
      bits[i] = Bit::ShareFr(context, clients);
    }

    return BitVector(bits);
  }

  static inline BitVector Constant(std::vector<bool> c) {
    std::size_t size = c.size();
    std::vector<Bit> bits(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Constant(c[i]);
    }
    return BitVector(bits);
  }

  static inline void RevealTo(Party& context, std::size_t client, const BitVector& v) {
    std::size_t num_bits = v.Size();
    context.Send(client, num_bits);
    for (std::size_t i = 0; i < num_bits; i++) {
      Bit::RevealTo(context, client, v[i]);
    }
  }

  static inline std::vector<bool> RevealFr(Client& context) {
    std::size_t num_parties = context.NumParties();
    std::size_t num_bits;

    for (std::size_t i = 0; i < num_parties; i++) {
      num_bits = context.template Recv<std::size_t>(i); // TODO: FIX ... this is dumb
    }

    std::vector<bool> ret(num_bits);
    for (std::size_t i = 0; i < num_bits; i++) {
      ret[i] = Bit::RevealFr(context);
    }

    return ret;
  }

  static inline BitVector Xor(Party& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Xor(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector And(Party& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::And(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector Or(Party& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::And(context, l, r);
    }
    return BitVector(bits);
  }

  static inline BitVector Not(Party& context, const BitVector& v) {
    std::size_t size = v.bits_.size();
    std::vector<Bit> bits(size);
    bits.reserve(size);
    for (std::size_t i = 0; i < size; i++) {
      bits[i] = Bit::Not(context, v);
    }
    return BitVector(bits);
  }

  static inline Bit Eq(Party& context, const BitVector& l, const BitVector& r) {
    assert(l.bits_.size() == r.bits_.size());
    std::size_t size = l.bits_.size();
    Bit ret = Bit::Constant(true);
    for (std::size_t i = 0; i < size; i++) {
      ret = Bit::And(context, ret, Bit::Eq(context, l.bits_[i], r.bits_[i]));
    }
    return ret;
  }

  static inline Bit NEq(Party& context, const BitVector& l, const BitVector& r) {
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
  // TODO: Remove, just here for testing
  // TODO: Move FrBits / ToBits to a utility class, have them go to/from std::vector<bool>
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
