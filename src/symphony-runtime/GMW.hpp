#include <cstddef>

#include "Channel.hpp"

#include "Bit.hpp"

class GMWContext {
public:
  GMWContext(std::size_t id, std::vector<std::shared_ptr<Channel>> comm) : id_(id), comm_(comm) {};

  std::size_t Me() const;
  void Send(std::size_t id, bool b);
  bool RecvBool(std::size_t b);
  bool RandBool();
private:
  std::size_t id_;
  std::vector<std::shared_ptr<Channel>> comm_;
  PRG prg;
};

// GMW : BaseBit
class GMWBaseBit {
public:
  using Context = GMWContext;

  inline void Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const;
  GMWBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers);
  GMWBaseBit(bool constant);

  inline GMWBaseBit operator^(const GMWBaseBit& other) const;
  inline GMWBaseBit And(Context& context, const GMWBaseBit& other) const;

  GMWBaseBit(bool v, bool is_constant);
private:
  bool is_constant_;
  bool repr_
};

using GMWBit = Bit<GMWBaseBit>;
