#include <iostream>

#include "../symphony-runtime/Util/LocalChannel.hpp"

#include "../symphony-runtime/GMW.hpp"

using namespace symphony::util;
using namespace symphony::mpc::gmw;

template <typename Clear>
void test_encrypt_decrypt_with(Clear input) {
  auto chAB = std::make_shared<LocalChannel>();
  auto chAC = std::make_shared<LocalChannel>();
  auto chBD = std::make_shared<LocalChannel>();
  auto chCD = std::make_shared<LocalChannel>();

  for (std::size_t i = 0; i < 4; i++) {
    if (i == 0) {
      ClientInputContext c;
      c.prg = std::make_shared<PlainPRG>();
      c.receivers.resize(2);
      c.receivers[0] = chAB;
      c.receivers[1] = chAC;

      SendInput(c, input);
    } else if (i == 1) {
      PartyContext cA;
      cA.client = chAB;

      PartyContext cD;
      cD.client = chBD;

      SendOutput(cD, RecvInput(cA));
    } else if (i == 2) {
      PartyContext cA;
      cA.client = chAC;

      PartyContext cD;
      cD.client = chCD;

      SendOutput(cD, RecvInput(cA));
    } else {
      ClientOutputContext c;
      c.senders.resize(2);
      c.senders[0] = chBD;
      c.senders[1] = chCD;

      Clear output = RecvOutput(c);
      assert(input == output);
    }
  }
}

void test_encrypt_decrypt() {
  test_encrypt_decrypt_with<BitVector>(BitVector(true));
}

int main() {
  test_encrypt_decrypt();

  return 0;
}
