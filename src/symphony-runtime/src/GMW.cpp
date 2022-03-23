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

bool GMWContext::RandBool() {
  return prg_->RandBool();
}

inline GMWBaseBit GMWBaseBit::Embed(bool constant) {
  return GMWBaseBit(true, constant);
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

inline GMWBaseBit GMWBaseBit::Xor(Context& context, const GMWBaseBit& other) const {
  if (is_constant_ && other.is_constant_) {
    return GMWBaseBit(true, repr_ ^ other.repr_);
  }

  if (is_constant_ && context.Me() != 0) {
    return other;
  }

  if (other.is_constant_ && context.Me() != 0) {
    return *this;
  }

  return GMWBaseBit(false, repr_ ^ other.repr_);
}

inline GMWBaseBit GMWBaseBit::And(Context& context, const GMWBaseBit& other) const {
  if (is_constant_ && other.is_constant_) {
    return GMWBaseBit(true, repr_ & other.repr_);
  }

  if (is_constant_ || other.is_constant_) {
    return GMWBaseBit(false, repr_ & other.repr_);
  }

  assert(false); // TODO
}

GMWBaseBit::GMWBaseBit(bool is_constant, bool repr) : is_constant_(is_constant), repr_(repr) {};
