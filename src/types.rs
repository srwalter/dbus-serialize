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

impl<'a> From<&'a Value> for u8 {
    fn from(v: &'a Value) -> u8 {
        match v {
            &Value::BasicValue(BasicValue::Byte(x)) => x,
            _ => panic!("Not a byte")
        }
    }
}

impl<'a> From<&'a Value> for i16 {
    fn from(v: &'a Value) -> i16 {
        match v {
            &Value::BasicValue(BasicValue::Int16(x)) => x,
            _ => panic!("Not an Int16")
        }
    }
}

impl<'a> From<&'a Value> for i32 {
    fn from(v: &'a Value) -> i32 {
        match v {
            &Value::BasicValue(BasicValue::Int32(x)) => x,
            _ => panic!("Not an Int32")
        }
    }
}

impl<'a> From<&'a Value> for i64 {
    fn from(v: &'a Value) -> i64 {
        match v {
            &Value::BasicValue(BasicValue::Int64(x)) => x,
            _ => panic!("Not an Int64")
        }
    }
}

impl<'a> From<&'a Value> for u16 {
    fn from(v: &'a Value) -> u16 {
        match v {
            &Value::BasicValue(BasicValue::Uint16(x)) => x,
            _ => panic!("Not a Uint16")
        }
    }
}

impl<'a> From<&'a Value> for u32 {
    fn from(v: &'a Value) -> u32 {
        match v {
            &Value::BasicValue(BasicValue::Uint32(x)) => x,
            _ => panic!("Not a Uint32")
        }
    }
}

impl<'a> From<&'a Value> for u64 {
    fn from(v: &'a Value) -> u64 {
        match v {
            &Value::BasicValue(BasicValue::Uint64(x)) => x,
            _ => panic!("Not a Uint32")
        }
    }
}

impl<'a> From<&'a Value> for String {
    fn from(v: &'a Value) -> String {
        match v {
            &Value::BasicValue(BasicValue::String(ref x)) => x.to_string(),
            &Value::BasicValue(BasicValue::ObjectPath(Path(ref x))) => x.to_string(),
            &Value::BasicValue(BasicValue::Signature(Signature(ref x))) => x.to_string(),
            _ => panic!("Not a string type")
        }
    }
}

