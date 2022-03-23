#include <cstddef>

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

#include "Sharing/Context.hpp"

inline std::size_t SharingContext::Me() const {
  return id_;
}

inline Channel& SharingContext::GetChannel(std::size_t id) const {
  return *channels_[id];
}

inline PRG& GetPRG() const {
  return prg_;
}
