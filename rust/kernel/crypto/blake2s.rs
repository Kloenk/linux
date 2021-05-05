
use crate::bindings;

#[repr(transparent)]
pub struct Blake2sState {
    //state: bindings::blake2s_state,
    inner: u16
}