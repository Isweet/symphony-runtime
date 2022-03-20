#include <cstddef>
#include <vector>

template <typename BaseBit>
class Bit : BaseBit {
public:
  using Context = typename BaseBit::Context;

  using BaseBit::BaseBit;

  // a | b ≜ (a ^ b) ^ (a & b)
  inline Bit Or(Context& context, const Bit& other) const {
    Bit a_xor_b = this->Xor(context, other);
    Bit a_and_b = this->And(context, other);
    Bit a_or_b  = a_xor_b.Xor(context, a_and_b);
    return a_or_b;
  }

  // ~a ≜ a ^ 1
  inline Bit Not(Context& context) const {
    Bit not_a = this->Xor(context, Bit::Embed(true)); // What do we do about this? Should `embed` be a conversion from replicated?
    return not_a;
  }

  // a == b ≜ ~(a ^ b)
  inline Bit Eq(Context& context, const Bit& other) const {
    Bit a_xor_b = this->Xor(context, other);
    Bit a_eq_b  = a_xor_b.Not(context);
    return a_eq_b;
  }

  // a != b ≜ ~(a == b) ≡ ~(~(a ^ b) ≡ a ^ b
  inline Bit Neq(Context& context, const Bit& other) const {
    Bit a_neq_b = this->Xor(context, other);
    return a_neq_b;
  }
};
