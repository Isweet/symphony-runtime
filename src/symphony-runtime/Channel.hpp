class Channel {
public:
  virtual void Send(bool b) = 0;
  virtual bool RecvBool() = 0;
};
