#include <cstddef>

#include "Channel.hpp"

#include "Bit.hpp"
//#include "BitVector.hpp"
//#include "UInt.hpp"
//#include "Int.hpp"

class ReplicatedContext {
public:
  ReplicatedContext(std::size_t id, std::vector<std::shared_ptr<Channel>> comm) : id_(id), comm_(comm) {};

  std::size_t Me() const;
  void Send(std::size_t id, bool b);
  bool RecvBool(std::size_t b);
private:
  std::size_t id_;
  std::vector<std::shared_ptr<Channel>> comm_;
};

// Replicated : BaseBit
class ReplicatedBaseBit {
public:
  using Context = ReplicatedContext;

  inline void Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const;
  ReplicatedBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers);
  ReplicatedBaseBit(bool constant);

  inline ReplicatedBaseBit operator^(const ReplicatedBaseBit& other) const;
  inline ReplicatedBaseBit And(Context& context, const ReplicatedBaseBit& other) const;

  inline bool Reveal() const;
private:
  bool repr_;
};

using ReplicatedBit       = Bit<ReplicatedBaseBit>;
//using ReplicatedBitVector = BitVector<ReplicatedBit>;
//using ReplicatedUInt8     = UInt<ReplicatedBaseUInt<8>>;
//using ReplicatedUInt16    = UInt<ReplicatedBaseUInt<16>>;
//using ReplicatedUInt32    = UInt<ReplicatedBaseUInt<32>>;
//using ReplicatedUInt64    = UInt<ReplicatedBaseUInt<64>>;
