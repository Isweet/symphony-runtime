#include <cstddef>

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

#include "Sharing/Context.hpp"

template <typename T>
struct Sharing {
  using Context   = SharingContext;
  using Clear     = T;
  using Encrypted = T;

  static inline Sharing Constant(Clear clear) {
    return Sharing(clear, true);
  }

  static inline Sharing Pre(Encrypted cipher) {
    return Sharing(cipher, false);
  }

  static inline void SendEncrypted(Context& context, const std::vector<std::size_t>& receivers, Clear clear) {
    std::size_t num_receivers = receivers.size();

    T sum = T::Zero;

    for (std::size_t i = 0; i < num_receivers - 1; i++) {
      T share = T::Random(context.GetPRG());
      T::Send(context.GetChannel(receivers[i]), share);
      sum += share;
    }

    T::Send(context.GetChannel(receivers[num_receivers - 1]), clear - sum);
  }

  static inline Encrypted RecvEncrypted(Context& context, std::size_t sender) {
    T::Recv(context.GetChannel(sender));
  }

  static inline void SendClear(Context& context, std::size_t receiver, Encrypted cipher) {
    T::Send(context.GetChannel(receiver));
  }

  static inline Clear RecvClear(Context& context, const std::vector<std::size_t>& senders) {
    std::size_t num_senders = senders.size();

    Type sum = T::Zero;

    for (std::size_t i = 0; i < num_senders; i++) {
      sum += T:Recv(context.GetChannel(senders[i]));
    }

    return sum;
  }

  static inline Sharing Add(Context& context, const Sharing& lhs, const Sharing& rhs) {
    if (lhs.is_constant_ && rhs.is_constant_) {
      return Constant(lhs.share_ + rhs.share_);
    }

    Type l = !lhs.is_constant || context.Me() == 0 ? lhs.share_ : T::Zero;
    Type r = !rhs.is_constant || context.Me() == 0 ? rhs.share_ : T::Zero;

    return Shared(l + r);
  }

  static inline Sharing Mul(Context& context, const Sharing& lhs, const Sharing& rhs) {
    if (lhs.is_constant_ || rhs.is_constant_) {
      return Shared(lhs.share_ * rhs.share_);
    }

    assert(false); // TODO
  }
private:
  Sharing(T share, bool is_constant) : share_(share), is_constant_(is_constant) {};

  T share_;
  bool is_constant_;
};
