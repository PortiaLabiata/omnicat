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
            async fn create(handle: CreationHandle)
                -> Result<Self, Error>
            {
                return match self {
                    $( $enum::$variant(inner) => inner.create(handle).await ),*
                }
            }


            async fn send(self, data: &[u8])
                -> (Self, Result<usize, Error>)
            {
                return match self {
                    $( $enum::$variant(inner) => inner.send(data).await ),*
                }
            }

            async fn receive(self, receive: &mut [u8])
                -> (Self, Result<usize, Error>)
            {
                return match self {
                    $( $enum::$variant(inner) => inner.receive(data).await ),*
                }
            }
        }
    }
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
