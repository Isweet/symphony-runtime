use crate::bitvec::*;
use rand::CryptoRng;
use rand::Rng;
use std::io::Read;
use std::io::Write;

use crate::gmw::Share;

use crate::sec::SecIO;
use crate::sec::SecRecvInput;
use crate::sec::SecRecvOutput;
use crate::sec::SecSendInput;
use crate::sec::SecSendOutput;

pub struct InputClient<RNG: Rng + CryptoRng, W: Write> {
    pub prg: RNG,
    pub receivers: Vec<W>,
}

impl<RNG: Rng + CryptoRng, W: Write, T: Share> SecSendInput<T> for InputClient<RNG, W> {
    fn sec_send_input(&mut self, input: T) {
        let mut masked: BV = input.into();
        let input_len = masked.len();

        let mut share = BV::zero(input_len);
        for receiver in self.receivers.iter_mut().skip(1) {
            share.randomize(&mut self.prg);
            share.send(receiver);
            masked ^= &share;
        }

        masked.send(&mut self.receivers[0]);
    }
}

pub struct InputServer<R: Read> {
    pub sender: R,
}

impl<R: Read> SecRecvInput<BV> for InputServer<R> {
    fn sec_recv_input(&mut self) -> BV {
        BV::recv(&mut self.sender)
    }
}

pub struct OutputServer<W: Write> {
    pub receiver: W,
}

impl<W: Write> SecSendOutput<BV> for OutputServer<W> {
    fn sec_send_output(&mut self, output: BV) {
        output.send(&mut self.receiver);
    }
}

pub struct OutputClient<R: Read> {
    pub senders: Vec<R>,
}

impl<R: Read, T: Share> SecRecvOutput<T> for OutputClient<R> {
    fn sec_recv_output(&mut self) -> T {
        let mut clear = BV::recv(&mut self.senders[0]);
        for sender in self.senders.iter_mut().skip(1) {
            clear ^= BV::recv(sender)
        }
        clear.into()
    }
}

pub enum Eval<RNG: Rng + CryptoRng, R: Read, W: Write> {
    IC(InputClient<RNG, W>),
    IS(InputServer<R>),
    OS(OutputServer<W>),
    OC(OutputClient<R>),
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write> Eval<RNG, R, W> {
    fn in_client_mut(&mut self) -> &mut InputClient<RNG, W> {
        if let Eval::IC(ic) = self {
            ic
        } else {
            panic!()
        }
    }

    fn in_server_mut(&mut self) -> &mut InputServer<R> {
        if let Eval::IS(is) = self {
            is
        } else {
            panic!()
        }
    }

    fn out_server_mut(&mut self) -> &mut OutputServer<W> {
        if let Eval::OS(os) = self {
            os
        } else {
            panic!()
        }
    }

    fn out_client_mut(&mut self) -> &mut OutputClient<R> {
        if let Eval::OC(oc) = self {
            oc
        } else {
            panic!()
        }
    }
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write, T: Share> SecSendInput<T> for Eval<RNG, R, W> {
    fn sec_send_input(&mut self, input: T) {
        self.in_client_mut().sec_send_input(input)
    }
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write> SecRecvInput<BV> for Eval<RNG, R, W> {
    fn sec_recv_input(&mut self) -> BV {
        self.in_server_mut().sec_recv_input()
    }
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write> SecSendOutput<BV> for Eval<RNG, R, W> {
    fn sec_send_output(&mut self, output: BV) {
        self.out_server_mut().sec_send_output(output)
    }
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write, T: Share> SecRecvOutput<T> for Eval<RNG, R, W> {
    fn sec_recv_output(&mut self) -> T {
        self.out_client_mut().sec_recv_output()
    }
}

impl<RNG: Rng + CryptoRng, R: Read, W: Write, T: Share> SecIO<T, BV> for Eval<RNG, R, W> {}

#[cfg(test)]
mod tests {
    use super::*;
    use scuttlebutt::AesRng;

    fn send_recv<T: Share + Clone + Eq + std::fmt::Debug>(expected: T) {
        let prg = AesRng::new();
        let mut channel1: Vec<u8> = Vec::new();
        let mut channel2: Vec<u8> = Vec::new();
        let mut send_ctx = InputClient {
            prg,
            receivers: vec![&mut channel1, &mut channel2],
        };
        send_ctx.sec_send_input(expected.clone());

        let mut recv_ctx1 = InputServer {
            sender: channel1.as_slice(),
        };
        let mut recv_ctx2 = InputServer {
            sender: channel2.as_slice(),
        };

        let bv1 = recv_ctx1.sec_recv_input();
        let bv2 = recv_ctx2.sec_recv_input();

        let mut send_ctx1 = OutputServer {
            receiver: &mut channel1,
        };

        let mut send_ctx2 = OutputServer {
            receiver: &mut channel2,
        };

        send_ctx1.sec_send_output(bv1);
        send_ctx2.sec_send_output(bv2);

        let mut recv_ctx = OutputClient {
            senders: vec![channel1.as_slice(), channel2.as_slice()],
        };

        let result: T = recv_ctx.sec_recv_output();

        assert_eq!(result, expected);
    }

    #[test]
    fn send_recv_u32() {
        send_recv(42);
    }

    #[test]
    fn send_recv_bit() {
        send_recv(false);
    }

    #[test]
    fn send_recv_bits() {
        let mut prg = AesRng::new();
        for i in 0..=128 {
            send_recv(BV::random(i, &mut prg));
        }
    }
}
