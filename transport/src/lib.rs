pub mod error;

use std::future::Future;
use error::*;

#[derive(Default)]
pub struct CreationHandle {
    pub path: String,
    pub id: u32,
}

#[allow(warnings)]
pub trait Transport: Sized {
    fn id(&self) -> u32;

    async fn send(self, data: &[u8])
        -> (Self, Result<usize, Error>);

    async fn receive(self, receive: &mut [u8])
        -> (Self, Result<usize, Error>);
}

