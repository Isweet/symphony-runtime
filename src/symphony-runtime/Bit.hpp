#include <cstddef>
#include <vector>

template <typename BaseBit>
class Bit : BaseBit {
public:
  using Context = typename BaseBit::Context;

  using BaseBit::BaseBit;

  // a | b ≜ (a ^ b) ^ (a & b)
  inline Bit Or(Context& context, const Bit& other) const {
    Bit l_xor_r = *this ^ other;
    Bit l_and_r = this->And(context, other);
    return l_xor_r ^ l_and_r;
  }

  // ~a ≜ a ^ 1
  inline Bit operator~() const {
    return *this ^ Bit(true); // What do we do about this? `embed` should really be a conversion from replicated
  }

  // a == b ≜ ~(a ^ b)
  inline Bit operator==(const Bit& other) const {
    return ~(*this ^ other);
  }

  // a != b ≜ ~(a == b) ≡ ~(~(a ^ b) ≡ a ^ b
  inline Bit operator!=(const Bit& other) const {
    return *this ^ other;
  }
};
