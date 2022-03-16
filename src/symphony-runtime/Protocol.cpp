#include <cstddef>
#include <span>
#include <vector>

// Rings

struct BooleanRing {
  using T = bool;

  static T Zero;
  static T One;

  static T Add(T l, T r) {
    return l ^ r;
  }

  static T Neg(T v) {
    return v;
  }

  static T Mul(T l, T r) {
    return l & r;
  }
};

bool BooleanRing::Zero = false;
bool BooleanRing::One  = true;

template <size_t BitWidth>
struct BitVector {};

template <size_t BitWidth>
struct ArithmeticRing {
  using T = BitVector<BitWidth>;

  static T Zero;
  static T One;

  static T Add(T l, T r) {
    return T::Add(l, r);
  }
};

//template <size_t BitWidth>
//BitVector<BitWidth> ArithmeticRing<BitWidth>::Zero = BitVector<BitWidth>(0);

//template <size_t BitWidth>
//BitVector<BitWidth> ArithmeticRing<BitWidth>::One = BitVector<BitWidth>(1);

template <>
struct ArithmeticRing <8> {
  using T = uint8_t;

  static T Zero;
  static T One;

  static T Add(T l, T r) {
    return l + r;
  }
};

uint8_t ArithmeticRing<8>::Zero = 0;
uint8_t ArithmeticRing<8>::One  = 1;

// Protocols

template <typename T, typename Enable = void>
struct RingOf {
};

template <>
struct RingOf <bool, std::true_type> {
  using Type = BooleanRing;
};

template <typename T>
struct RingOf <T, typename std::enable_if<std::is_unsigned<T>::value>::type> {
  using Type = ArithmeticRing<8 * sizeof(T)>;
};

template <typename R>
struct Beaver {
  using Ring = typename RingOf<R>::Type;

  struct Context {};

  template <typename T>
  using Share = T;

  template <typename T>
  static Share<T> Constant(Context& context, T clear) {
    assert(false); // TODO
  }

  template <typename T>
  static Share<T> Zero(Context& context) {
    return Constant(context, Ring::Zero);
  }

  template <typename T>
  static Share<T> One(Context& context) {
    return Constant(context, Ring::One);
  }

  template <typename T>
  static Share<T> Add(Context& context, Share<T> l, Share<T> r) {
    return Ring::Add(l, r);
  }

  template <typename T>
  static Share<T> Neg(Context& context, Share<T> v) {
    return Ring::Neg(v);
  }

  template <typename T>
  static Share<T> Mul(Context& context, Share<T> l, Share<T> r) {
    assert(false); // TODO
  }
};

template <typename P>
struct Boolean {
  using Context = typename P::Context;

  template <typename T>
  using Share = typename P::template Share<T>;

  static Share<bool> False(Context& context) {
    return P::Zero(context);
  }

  static Share<bool> True(Context& context) {
    return P::One(context);
  }

  static Share<bool> Xor(Context& context, Share<bool> l, Share<bool> r) {
    return P::Add(context, l, r);
  }

  static Share<bool> Not(Context& context, Share<bool> v) {
    return P::Neg(context, v);
  }

  static Share<bool> And(Context& context, Share<bool> l, Share<bool> r) {
    return P::Mul(context, l, r);
  }

  static Share<bool> Mux(Context& context, Share<bool> g, Share<bool> tt, Share<bool> ff) {
    return Xor(context, And(context, g, tt), And(context, Not(context, g), ff));
  }
};

template <typename P>
struct Arith {
  using Context = typename P::Context;

  template <typename T>
  using Share = typename P::template Share<T>;

  template <typename T>
  static Share<T> Zero(Context& context) {
    return P::template Zero<T>(context);
  }
};
/*
using BeaverB = Beaver<bool>;
using BB = Boolean<BeaverB>;

BB::Share<bool> foo1(BB::Context& c) {
  return BB::False(c);
}
*/
using Beaver8 = Beaver<uint8_t>;
using A8 = Arith<Beaver8>;

A8::Share<uint8_t> foo2(A8::Context& c) {
  return A8::Zero<uint8_t>(c);
}

//A8::Share<bool> foo(A8::Context& c) {
//  return A8::Zero(c);
//}

/* TODO: How can we defined GMW once, based on an underlying cleartext ring? E.g.
     template <typename R>
     struct GMW { ... };

     I guess something like this?
template <typename R>
struct GMW {
  static_assert(is_ring<R>::value, "GMW is defined over a ring.");
  // TODO: Context

  template <typename T>
  struct Share {
    static_assert(std::is_same<T, R::Set>::value, "GMW only supports sharing elements of the underlying ring.");
    T repr;

    Share(T v) : repr(v) {};
  };
};
*/
/*
template <typename R>
struct is_protocol <GMW<R>> {
  static const bool value =
}

struct SPDZContext {};

struct GMWBoolRing {
  typedef SPDZContext Context;

  template <typename T>
  struct Share {
    static_assert(std::is_same<T, bool>::value, "GMWBool is a Boolean protocol.");
    T repr;

    Share(T v) : repr(v) {};
  };

  static bool Zero;
  static bool One;

  static Share<bool> Add(Context& context, Share<bool> l, Share<bool> r) {
    return Share<bool>(l.repr ^ r.repr);
  }

  static Share<bool> Neg(Context& context, Share<bool> v) {
    return Share<bool>(!v.repr);
  }

  static Share<bool> Mul(Context& context, Share<bool> l, Share<bool> r) {
    assert(false); // TODO
  }
};

bool GMWBoolRing::Zero = false;
bool GMWBoolRing::One  = true;

struct GMWArithRing {
  typedef SPDZContext Context;

  template <typename T>
  struct Share {
    static_assert(std::is_same<T, uint64_t>::value, "GMWArith is a Arithmetic protocol."); // TODO: How can I ask for types T that are in { (u)int(8 | 16 | 32 | 64) }?
    uint64_t repr;

    Share(uint64_t n) : repr(n) {};
  };

  static uint64_t Zero;
  static uint64_t One;

  static Share<uint64_t> Add(Context& context, Share<uint64_t> l, Share<uint64_t> r) {
    return Share<uint64_t>(l.repr ^ r.repr);
  }

  static Share<uint64_t> Neg(Context& context, Share<uint64_t> v) {
    return Share<uint64_t>(!v.repr);
  }

  static Share<uint64_t> Mul(Context& context, Share<uint64_t> l, Share<uint64_t> r) {
    assert(false);
    return Share<uint64_t>(l.repr & r.repr); // TODO
  }
};

uint64_t GMWArithRing::Zero = 0;
uint64_t GMWArithRing::One  = 1;

template <typename Ring>
struct Boolean {
  typedef typename Ring::Context Context;

  template <typename T>
  using Share = typename Ring::template Share<T>;

  static bool False;
  static bool True;

  static Share<bool> Xor(Context& context, Share<bool> l, Share<bool> r) {
    return Ring::Add(context, l, r);
  }

  static Share<bool> Not(Context& context, Share<bool> v) {
    return Ring::Neg(context, v);
  }

  static Share<bool> And(Context& context, Share<bool> l, Share<bool> r) {
    return Ring::And(context, l, r);
  }

  static Share<bool> Mux(Context& context, Share<bool> g, Share<bool> a, Share<bool> b) {
    Share<bool> tt = And(context, g, a);
    Share<bool> ff = And(context, Not(context, g), b);
    return Xor(context, tt, ff);
  }

  // TODO: more operations ...
};

template <typename Ring>
bool Boolean<Ring>::False = Ring::Zero;

template <typename Ring>
bool Boolean<Ring>::True = Ring::One;

*/

/*


template <typename Ring, size_t Width>
struct StaticVectorRing {
  // Convenience
  typedef std::array<typename Ring::Share, Width> VShare;
  // ----------
  typedef typename Ring::Context Context;
  typedef typename std::array<Ring::Share, Width> Share;


  static VShare Add(Context& context, VShare l, VShare r) {
    VShare ret;
    for (std::size_t i = 0; i < Width; i++) {
      ret[i] = Ring::Add(context, l[i], r[i]);
    }
    return ret;
  }



  template <typename T>
  using Share<T> = std::array<Ring::Share


  typedef





  template <typename T, size_t Width>
  struct Vectorized {
    static
  }

  template <typename T, size_t Width>
  using Share<T> = std::array<T, Width>;

};

typedef StaticVectorRing<GMWRing> GMWVecRing;
*/
/*
template <class Ring>
class Boolean {
public:
  typedef typename Ring::Context Context;

  template <size_t BW>
  using Share<BW> = Ring::Share<BitVector<BW>>;

  static BitVector False = Boolean::Zero;
  static BitVector True  = Boolean::One;

  static Share False(Context& context) {
    return Boolean::Constant(context, Boolean::Zero);
  }

  static Share True(Context& context) {
    return Boolean::Constant(context, Boolean::One);
  }

  static Share Xor(Context& context, Share l, Share r) {
    return Boolean::Add(context, l, r);
  }

  static Share Not(Context& context, Share v) {
    return Boolean::Neg(context, v);
  }

  static Share And(Context& context, Share l, Share r) {
    return Boolean::Mul(context, l, r);
  }

  static Share Mux(Context& context, Share g, Share a, Share b) {
    Share l = Boolean::And(context, g, a);
    Share r = Boolean::And(context, Boolean::Not(context, g), b);
    return Boolean::Xor(l, r);
  }

  // ...
};



template
class GMWRing {
public:
  typedef Void Context;
  typedef bool Clear;
  typedef bool Share;

  static Clear Zero = false;
  static Clear One  = true;

  static Share Constant(Clear clear) {
    return clear;
  }

  static Share Add(Context& context, Share l, Share r) {
    return l ^ r;
  }

  static Share Mul(Context& context, Share l, Share r) {
    return l & r;
  }
};


struct GMWRing {
  typedef
}

*/
/*
template <typename Context, template <typename> class Share, typename T>
struct Ring {
  typedef
  static T Zero;
  static T One;

  static Share<T> RingAdd(Context&, Share<T> l, Share<T> r);
  static Share<T> RingNeg(Context&, Share<T> l, Share<T> r);
  static Share<T> RingMul(Context&, Share<T> l, Share<T> r);
};
*/

// M : * -> *
// T : *
template <template <typename> typename M, typename T>
M<T> foo(T t) {
  return t.toM(t);
}

// R : (* -> *) -> *
template <template <typename, typename> typename R, template <typename> typename M, typename T>
R<M<T>,T> bar(T t) {
  return t.toRM(t);
}

/*



// MP-SPDZ
#include "Machines/Semi.hpp"
#include "Protocols/RepRingOnlyEdabitPrep.hpp"
#include "Protocols/Semi2kShare.h"
#include "Protocols/SemiPrep2k.h"
#include "Protocols/ProtocolSet.h"

class Channel {
public:
  virtual void Send(std::span<const std::byte> bytes) = 0;
  virtual std::vector<std::byte> Recv(std::size_t num_bytes) = 0;

  template <typename T>
  void Send(const T& v) {
    Send(T::ToBytes(v));
  }

  template <typename T>
  T Recv() {
    T::FromBytes(Recv(sizeof(T)));
  }
};

class PRG {
public:
  virtual std::vector<std::byte> Random(std::size_t num_bytes) = 0;

  template <typename T>
  T Random() {
    T::FromBytes(Random(sizeof(T)));
  }
};

template <typename SS>
class SecretShare {
public:
  class ClientContext {
  public:
    std::size_t num_parties;
    std::vector<std::shared_ptr<Channel>>& channels;
    PRG& prg;
  };

  class PartyContext {
  public:
    ClientContext& client_context;
    std::size_t num_clients;
    std::vector<std::shared_ptr<Channel>>& channels;
  };

  typedef typename SS::Clear Clear;
  typedef typename SS::Share Share;

  static void ClientShare(ClientContext& context, Clear clear) {
    std::vector<Clear> shares;
    shares.reserve(context.num_parties);

    Clear sum;
    std::size_t i;
    for (i = 0; i < context.num_parties - 1; i++) {
      shares[i] = context.prg.template Random<Clear>();
      sum += shares[i];
    }
    shares[i] = clear - sum;

    for (i = 0; i < context.num_parties; i++) {
      context.channels[i]->template Send<Share>(Share(shares[i]));
    }
  }

  static Share PartyShare(PartyContext& context, std::size_t client_id) {
    return context.channels[client_id]->template Recv<Share>();
  }

  static void ClientReshare(ClientContext& context, Share share) {
    Clear reified_share = share;
    ClientShare(context, reified_share);
  }

  static Share PartyReshare(PartyContext& context, std::vector<std::size_t>& client_ids) {
    Share ret;

    for (std::size_t i = 0; i < client_ids.size(); i++) {
      ret += context.channels[client_ids[i]]->template Recv<Share>();
    }

    return ret;
  }

  static Clear ClientReveal(ClientContext& context) {
    Share ret;

    for (std::size_t i = 0; i < context.num_parties; i++) {
      ret += context.channels[i]->template Recv<Share>();
    }

    return Clear(ret);
  }

  static void PartyReveal(PartyContext& context, std::size_t client_id, Share share) {
    context.channels[client_id]->template Send<Share>(share);
  }
};



class GMW {
public:
  class Context {
  public:
    Names& names;
    Player& party;
    GC::SemiSecret::Protocol& protocol;
  };

  typedef BitVec Clear;
  typedef GC::SemiSecret Share;



  static Share Constant(PartyContext& context, BitVec clear) {
    return Share::constant(clear, context.player.my_num());
  }

  static Share Xor(PartyContext& context, Share l, Share r) {
    return l ^ r;
  }

  static Share And(PartyContext& context, Share l, Share r) {
    return context.prot.mul(l, r);
  }
};

class SecureClient {
protected:
  std::size_t num_parties;
  std::vector<std::shared_ptr<Channel>> channels;
public:
  ClientParty(std::vector<std::shared_ptr<Channel>> channels) : channels(channels) {};
  ~ClientParty() {};

  virtual void share(T clear) = 0;
  virtual void reshare(Share<T> share) = 0;
  virtual T reveal() = 0;
};

template <template <typename> class Share, class T>
class ServerParty : public ClientParty<Share, T> {
protected:
  std::vector<std::shared_ptr<Channel>> clients_;
public:
  ServerParty(std::vector<std::shared_ptr<Channel>> parties) : ClientParty<Share, T>(parties), clients_(parties) {};

  std::size_t addClient(std::shared_ptr<Channel> client) {
    std::size_t id = clients_.size();
    clients_.push_back(client);
    return id;
  }

  virtual Share<T> constant(T clear) = 0;

  virtual Share<T> shareFr(std::size_t client) = 0;
  virtual Share<T> reshareFr(std::vector<std::size_t> clients) = 0;
  virtual void revealTo(std::size_t client, Share<T> share) = 0;
};

template <template <typename> class Share>
class BoolParty : public ServerParty<Share, bool> {
  BoolParty(std::vector<std::shared_ptr<Channel>> parties) : ServerParty<Share, bool>(parties) {};
  ~BoolParty() {};

  virtual Share<bool> bFalse() = 0;
  virtual Share<bool> bXor(Share<bool> l, Share<bool> r) = 0;
  virtual Share<bool> bNot(Share<bool> v) = 0;
  virtual Share<bool> bTrue() = 0;
  virtual Share<bool> bAnd(Share<bool> l, Share<bool> r) = 0;
};

// TODO: Generalize to all unsigned types
template <template <typename> class Share>
class ArithParty : public ServerParty<Share, uint64_t> {
  ArithParty(std::vector<std::shared_ptr<Channel>> parties) : ServerParty<Share, uint64_t>(parties) {};
  ~ArithParty() {};

  virtual Share<uint64_t> aZero() = 0;
  virtual Share<uint64_t> aAdd(Share<uint64_t> l, Share<uint64_t> r) = 0;
  virtual Share<uint64_t> aNeg(Share<uint64_t> v) = 0;
  virtual Share<uint64_t> aOne() = 0;
  virtual Share<uint64_t> aMul() = 0;
};

template <template <typename> class Share>
class ArithWithBoolParty : public BoolParty<Share>, public ArithParty<Share> {
  ArithWithBoolParty(std::vector<std::shared_ptr<Channel>> parties) : BoolParty<Share>(parties), ArithParty<Share>(parties) {};
};


// Is this useful? Should be a commutative ring with identity
template <template <typename> class Share, class T>
class ComputeParty : public ServerParty<Share, T> {
public:
  ComputeParty(std::vector<std::shared_ptr<Channel>> parties) : ServerParty<Share, T>(parties) {};
  ~ComputeParty() {};

  virtual Share<T> zero() = 0;
  virtual Share<T> add(Share<T> l, Share<T> r) = 0;
  virtual Share<T> neg(Share<T> v) = 0;

  virtual Share<T> one() = 0;
  virtual Share<T> mul(Share<T> l, Share<T> r) = 0;

  // l - r = l + (-r)
  virtual Share<T> sub(Share<T> l, Share<T> r) {
    return add(l, neg(r));
  }

  // g ? a : b = g * a + (1 - g) * b
  virtual Share<T> cond(Share<T> g, Share<T> a, Share<T> b) {
    return add(mul(g, a), mul(sub(one(), g), b));
  }
};

*/
