use serde::Deserialize;

use stdio::*;
use transport::*;
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

delegate_transport!{
    TransportEnum {
        Stdio,
    }
}

pub struct TransportStruct {
    pub name: String,
    pub to: Vec<String>,
    pub rxbuf: Vec<u8>,
    pub txbuf: Vec<u8>,
    pub transport: TransportEnum,
}

impl std::cmp::PartialEq for TransportStruct {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

impl std::cmp::Eq for TransportStruct {}

impl std::hash::Hash for TransportStruct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
