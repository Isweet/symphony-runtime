#include <cstddef>

#include "Channel.hpp"
#include "PRG.hpp"

#include "Bit.hpp"

class GMWContext {
public:
  GMWContext(std::size_t id, std::vector<std::shared_ptr<Channel>> comm, std::shared_ptr<PRG> prg) : id_(id), comm_(comm), prg_(prg) {};

  std::size_t Me() const;
  void Send(std::size_t id, bool b);
  bool RecvBool(std::size_t b);
  bool RandBool();
private:
  std::size_t id_;
  std::vector<std::shared_ptr<Channel>> comm_;
  std::shared_ptr<PRG> prg_;
};

// GMW : BaseBit
class GMWBaseBit {
public:
  using Context = GMWContext;

  static inline GMWBaseBit Embed(bool constant);
  inline void Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const;
  GMWBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers);

  inline GMWBaseBit Xor(Context& context, const GMWBaseBit& other) const;
  inline GMWBaseBit And(Context& context, const GMWBaseBit& other) const;
private:
  GMWBaseBit(bool is_constant, bool repr);
  bool is_constant_;
  bool repr_;
};

using GMWBit = Bit<GMWBaseBit>;
