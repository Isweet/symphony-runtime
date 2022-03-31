pub trait SecSendInput<Clear> {
    fn sec_send_input(&mut self, input: Clear);
}

pub trait SecRecvInput<Encrypted> {
    fn sec_recv_input(&mut self) -> Encrypted;
}

pub trait SecSendOutput<Encrypted> {
    fn sec_send_output(&mut self, output: Encrypted);
}

pub trait SecRecvOutput<Clear> {
    fn sec_recv_output(&mut self) -> Clear;
}

pub trait SecIO<Clear, Encrypted>:
    SecSendInput<Clear> + SecRecvInput<Encrypted> + SecSendOutput<Encrypted> + SecRecvOutput<Clear>
{
}
