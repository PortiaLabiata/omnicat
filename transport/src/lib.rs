pub mod error;

use std::future::Future;
use error::*;

#[derive(Default)]
pub struct CreationHandle {
    path: String,
}

#[allow(warnings)]
pub trait Transport: Sized {
    async fn send(self, data: &[u8])
        -> (Self, Result<usize, Error>);

    async fn receive(self, receive: &mut [u8])
        -> (Self, Result<usize, Error>);
}

