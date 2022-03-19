#include "GMW.hpp"

std::size_t GMWContext::Me() const {
  return id_;
}

void GMWContext::Send(std::size_t id, bool b) {
  comm_[id]->Send(b);
}

bool GMWContext::RecvBool(std::size_t id) {
  return comm_[id]->RecvBool();
}

inline void GMWBaseBit::Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const {
  assert(!is_constant_); // Use replicated protocol instead.
  std::size_t num_sharees = sharees.size();

  bool sum = false;
  for (std::size_t i = 0; i < num_sharees - 1; i++) {
    bool share = local.RandBool();
    group.Send(sharees[i], share);
    sum ^= share;
  }

  group.Send(sharees[num_sharees - 1], repr_ ^ sum);
}

GMWBaseBit::GMWBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers) {
  std::size_t num_sharers = sharers.size();

  bool sum = false;
  for (std::size_t i = 0; i < num_sharers; i++) {
    sum ^= group.RecvBool(sharers[i]);
  }

  is_constant_ = false;
  repr_        = sum;
}

GMWBaseBit::GMWBaseBit(bool constant) : is_constant_(true), repr_(constant) {};

// TODO: Go back and add Context to all the functions, need it even for XOR to handle correctly
inline GMWBaseBit GMWBaseBit::operator^(const GMWBaseBit& other) const {
  if (is_constant_ && other.is_constant_) {
    return GMWBaseBit(repr_ ^ other.repr_);
  }

  if (is_constant_ && !context.IAm(0)) {
    return r;
  }

  if (r.is_constant_ && !context.IAm(0)) {
    return l;
  }

  return GMWBaseBit(l.share_ ^ r.share_);
  return GMWBaseBit(repr_ ^ other.repr_);
}

inline GMWBaseBit GMWBaseBit::And(Context& context, const GMWBaseBit& other) const {
  return GMWBaseBit(repr_ & other.repr_);
}

GMWBaseBit::GMWBaseBit(bool constant) : repr_(constant) {};
