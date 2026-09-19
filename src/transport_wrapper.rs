use serde::Deserialize;
use ambassador::Delegate;

use stdio::*;
use transport::*;
use transport::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TransportKind {
    Stdio,
}

#[derive(Delegate)]
#[delegate(Transport)]
pub enum TransportEnum {
    Stdio(TransportStdio),
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
