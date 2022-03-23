#include <iostream>

#include "../symphony-runtime/Util/LocalChannel.hpp"

#include "../symphony-runtime/GMW.hpp"
#include "../symphony-runtime/Boolean.hpp"
#include "../symphony-runtime/Traits/Share.hpp"

using namespace symphony::runtime::util::channel;
using namespace symphony::runtime::gmw;
using namespace symphony::runtime::traits::share;

void test_encrypt() {
  Context c;
  c.channels.push_back(std::make_shared<LocalChannel>());
  c.channels.push_back(std::make_shared<LocalChannel>());
  c.prg = std::make_unique<DummyPRG>();
  std::vector<std::size_t> receivers = { 0, 1 };
  SendEncrypted(c, receivers, false);
}

int main() {
  test_encrypt();

  return 0;
}
