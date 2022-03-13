#include <cstddef>
#include <vector>
#include <bitset>

// Stuff from MP-SPDZ
#include "Tools/random.h"
#include "Math/BitVec.h"
#include "GC/SemiSecret.h"

class Channel {
public:
  virtual void Send(const std::vector<std::byte>& bytes) = 0;
  virtual std::vector<std::byte> Recv(size_t num_bytes) = 0;
  virtual void Flush() = 0;

  void Send(bool b) {
    std::vector<std::byte> bytes = { std::byte(b) };
    Send(bytes);
  }

  void Send(std::uint8_t n) {
    std::byte *bytes = static_cast<std::byte *>(static_cast<void *>(&n));
    Send(std::vector<std::byte>(bytes, bytes + sizeof(std::uint8_t)));
  }

  // ...
};

template <template <typename> class Share, class T>
class Client {
protected:
  std::vector<std::shared_ptr<Channel>> parties_;
public:
  typedef typename Share<T>::Clear Clear;

  Client(std::vector< std::shared_ptr<Channel> > parties) : parties_(parties) {};
  ~Client() {};

  virtual void share(Clear clear) = 0;
  virtual void reshare(Share<T> share) = 0;
  virtual Clear reveal() = 0;
};

template <template <typename> class Share, class T>
class ServerFoo : public Client<Share, T> {
protected:
  std::vector<std::shared_ptr<Channel>> clients_;
public:
  ServerFoo(std::vector<std::shared_ptr<Channel>> parties) : Client<Share, T>(parties), clients_(parties) {};

  std::size_t addClient(std::shared_ptr<Channel> client) {
    std::size_t id = clients_.size();
    clients_.push_back(client);
    return id;
  }

  virtual Share<T> shareFr(std::size_t client) = 0;
  virtual Share<T> reshareFr(std::vector<std::size_t> clients) = 0;
  virtual void revealTo(std::size_t client, Share<T> share) = 0;
};


/* GMW */
// typedef GMWContext ProtocolSet<??>
// Also need the Player
class GMWContext {};

template <class T>
class GMWShare {
private:
  GMWContext& context_;
  GC::SemiSecret share_;
public:
  typedef BitVec Clear;
  GMWShare(GMWContext& context, BitVec sh) : context_(context), share_(sh) {};
};


template <class T>
class GMWClient : public Client<GMWShare, T> {
private:
  SeededPRNG prg;
public:
  typedef typename GMWShare<T>::Clear Clear;

  GMWClient(std::vector<std::shared_ptr<Channel>> parties) : Client<GMWShare, T>(parties) {};
  ~GMWClient() {};

  GMWShare<T> share(Clear clear) {
    std::size_t num_parties = this->parties_.size();

    Clear sum;
    std::vector<Clear> shares(num_parties);

    for (std::size_t i = 0; i < num_parties - 1; i++) {
      shares[i] = prg.get<Clear>();
      sum += shares[i];
    }

    shares[num_parties - 1] = clear - sum;

    for (std::size_t i = 0; i < num_parties; i++) {
      this->parties_[i].Send(shares[i]);
    }
  }


};
