#include <cstddef>

#include "Util/Channel.hpp"
#include "Util/PRG.hpp"

struct SharingContext {
  inline std::size_t Me() const;
  inline Channel& GetChannel(std::size_t id) const;
  inline PRG& GetPRG() const;
private:
  std::size_t id_;
  std::vector<std::shared_ptr<Channel>> channels_;
  PRG prg_;
};
