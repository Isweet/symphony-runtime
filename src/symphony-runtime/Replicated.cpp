#include "Replicated.hpp"

std::size_t ReplicatedContext::Me() const {
  return id_;
}

void ReplicatedContext::Send(std::size_t id, bool b) {
  comm_[id]->Send(b);
}

bool ReplicatedContext::RecvBool(std::size_t id) {
  return comm_[id]->RecvBool();
}

inline ReplicatedBaseBit ReplicatedBaseBit::Embed(bool constant) {
  return ReplicatedBaseBit(constant);
}

inline void ReplicatedBaseBit::Share(Context& local, Context& group, const std::vector<std::size_t>& sharees) const {
  if (local.Me() != 0) {
    return;
  }

  std::size_t num_sharees = sharees.size();

  for (std::size_t i = 0; i < num_sharees; i++) {
    group.Send(sharees[i], repr_);
  }
}

ReplicatedBaseBit::ReplicatedBaseBit(Context& local, Context& group, const std::vector<std::size_t>& sharers) : repr_(group.RecvBool(sharers[0])) {}

inline ReplicatedBaseBit ReplicatedBaseBit::Xor(Context& context, const ReplicatedBaseBit& other) const {
  return ReplicatedBaseBit(repr_ ^ other.repr_);
}

inline ReplicatedBaseBit ReplicatedBaseBit::And(Context& context, const ReplicatedBaseBit& other) const {
  return ReplicatedBaseBit(repr_ & other.repr_);
}

ReplicatedBaseBit::ReplicatedBaseBit(bool repr) : repr_(repr) {};
