//! Central to this crate is the Value enum.  Value can be used to express any valid D-Bus data
//! structure (and some invalid ones).  Additionally, rustc_serialize can be used to convert from
//! standard rust data types to Value, and vice-versa.
extern crate rustc_serialize;

pub mod types;
pub mod decoder;
pub mod encoder;
