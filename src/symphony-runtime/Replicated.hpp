#include <cstddef>

#include "Channel.hpp"

#include "Bit.hpp"

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

  static inline ReplicatedBaseBit Embed(bool constant);
  inline void Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const;
  ReplicatedBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers);

  inline ReplicatedBaseBit Xor(Context& context, const ReplicatedBaseBit& other) const;
  inline ReplicatedBaseBit And(Context& context, const ReplicatedBaseBit& other) const;
private:
  ReplicatedBaseBit(bool repr);
  bool repr_;
};

using ReplicatedBit = Bit<ReplicatedBaseBit>;
