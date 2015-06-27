use std::collections::HashMap;

#[derive(PartialEq,Eq,Debug,Hash,Clone)]
pub enum BasicValue {
    Byte(u8),
    Boolean(bool),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    String(String),
    ObjectPath(Path),
    Signature(Signature),
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct Path(pub String);

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct Signature(pub String);

#[derive(PartialEq,Debug,Clone)]
pub struct Struct {
    pub objects: Vec<Value>,
    pub signature: Signature
}

pub type Dictionary = HashMap<BasicValue,Value>;

#[derive(PartialEq,Debug,Clone)]
pub enum Value {
    BasicValue(BasicValue),
    Double(f64),
    Array(Vec<Value>),
    Variant(Box<Value>),
    Struct(Struct),
    Dictionary(Dictionary)
}
