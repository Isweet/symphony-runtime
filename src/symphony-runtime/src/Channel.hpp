#include <cstddef>
#include <vector>
#include <span>

class Channel {
public:
  virtual void SendBytes(const std::span<std::byte>& bytes) = 0;
  virtual void RecvBytes(std::span<std::byte> bytes) = 0;
  virtual void Flush() = 0;

  bool RecvBool() {
    std::array<std::byte, sizeof(bool)> bytes;
    RecvBytes(bytes);
    return bool(bytes);
  }
};
