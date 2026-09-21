use serde::Deserialize;

use stdio::*;
use transport::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Stdio,
}

pub enum TransportEnum {
    Stdio(TransportStdio),
}

macro_rules! delegate_transport {
    ($enum:ident { $($variant:ident),* $(,)? }) => {
        impl transport::Transport for $enum {
            fn id(&self) -> u32 {
                return match self {
                    $( $enum::$variant(inner) => {
                        inner.id()
                    } ),*
                }
            }

            async fn send(self, data: &[u8])
                -> (Self, Result<usize, Error>)
            {
                return match self {
                    $( $enum::$variant(inner) => {
                        let res = inner.send(data).await;
                        ($enum::$variant(res.0), res.1)
                    } ),*
                }
            }

            async fn receive(self, data: &mut [u8])
                -> (Self, Result<usize, Error>)
            {
                return match self {
                    $( $enum::$variant(inner) => {
                        let res = inner.receive(data).await;
                        ($enum::$variant(res.0), res.1)
                    } ),*
                }
            }
        }
    };
}

delegate_transport! {
    TransportEnum {
        Stdio,
    }
}

pub struct TransportHandle {
    to: Vec<u32>,
    rxbuf: Vec<u8>,
    txbuf: Vec<u8>,
}
