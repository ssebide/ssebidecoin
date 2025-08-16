use uint::construct_uint;

construct_uint! {
    //construct an unsigned 256-bit integer
    //consisting of 4 x 64-bit words
    pub struct u256(4);
}

pub mod crypto;
pub mod sha256;
pub mod types;
pub mod utils;
