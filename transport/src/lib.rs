use std::future::Future;

pub trait CreationHandle: Sized {}

pub trait Transport<T, H>: Sized
where 
    T: std::error::Error,
    H: CreationHandle,
{
    fn create(handle: H)
        -> impl Future<Output = Result<Self, T>>;

    fn send(self, data: &[u8], timeout: u32)
        -> impl Future<Output = (Self, Result<usize, T>)>;

    fn receive(self, receive: &mut [u8], timeout: u32)
        -> impl Future<Output = (Self, Result<usize, T>)>;
}

