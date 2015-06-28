use std::collections::HashMap;

use rustc_serialize::{Encoder,Encodable};

use types::{Value,BasicValue,Struct,Signature,Dictionary};

pub struct DBusEncoder {
    val: Vec<Value>,
    signature: String,
    key: Option<BasicValue>
}

#[derive(Debug,PartialEq)]
pub enum EncoderError {
    BadKeyType,
    Unsupported,
    EmptyArray
}

impl DBusEncoder {
    fn handle_struct (&mut self) -> Result<(),EncoderError> {
        let mut objs = Vec::new();
        {
            let val = &self.val;
            for i in val {
                objs.push(i.clone());
            }
        }
        let s = Struct {
            objects: objs,
            signature: Signature("(".to_string() + &self.signature + ")")
        };
        self.signature = "".to_string();
        self.val.push(Value::Struct(s));
        Ok(())
    }

    fn handle_array (&mut self) -> Result<(),EncoderError> {
        let mut objs = Vec::new();
        for i in 0..self.val.len() {
            self.val.push(Value::BasicValue(BasicValue::Byte(0)));
            objs.push(self.val.swap_remove(i));
        }
        self.val.clear();
        self.val.push(Value::Array(objs));
        Ok(())
    }

    pub fn new() -> DBusEncoder {
        DBusEncoder {
            val: Vec::new(),
            signature: "".to_string(),
            key: None
        }
    }
    
    pub fn encode<T: Encodable>(obj: &T) -> Result<Value,EncoderError> {
        let mut encoder = DBusEncoder::new();
        try!(obj.encode(&mut encoder));
        Ok(encoder.val.remove(0))
    }
}

impl<T: Encodable> From<T> for Value {
    fn from(x: T) -> Value {
        DBusEncoder::encode(&x).unwrap()
    }
}

impl Encoder for DBusEncoder {
    type Error = EncoderError;

    fn emit_nil(&mut self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint64(v as u64)));
        self.signature.push_str("n");
        Ok(())
    }
    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint64(v)));
        self.signature.push_str("n");
        Ok(())
    }
    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint32(v)));
        self.signature.push_str("u");
        Ok(())
    }
    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint16(v)));
        self.signature.push_str("q");
        Ok(())
    }
    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Byte(v)));
        self.signature.push_str("y");
        Ok(())
    }
    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int64(v as i64)));
        self.signature.push_str("x");
        Ok(())
    }
    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int64(v)));
        self.signature.push_str("x");
        Ok(())
    }
    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int32(v)));
        self.signature.push_str("i");
        Ok(())
    }
    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int16(v)));
        self.signature.push_str("n");
        Ok(())
    }
    fn emit_i8(&mut self, _v: i8) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Boolean(v)));
        self.signature.push_str("b");
        Ok(())
    }
    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.val.push(Value::Double(v));
        self.signature.push_str("d");
        Ok(())
    }
    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.val.push(Value::Double(v as f64));
        self.signature.push_str("d");
        Ok(())
    }
    fn emit_char(&mut self, v: char) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Byte(v as u8)));
        self.signature.push_str("y");
        Ok(())
    }
    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::String(v.to_string())));
        self.signature.push_str("s");
        Ok(())
    }

    fn emit_struct<F>(&mut self, _name: &str, _len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct()
    }
    fn emit_struct_field<F>(&mut self, _f_name: &str, _f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }
    fn emit_tuple<F>(&mut self, _len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct()
    }
    fn emit_tuple_arg<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }
    fn emit_tuple_struct<F>(&mut self, _name: &str, _len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct()
    }
    fn emit_tuple_struct_arg<F>(&mut self, _f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    fn emit_seq<F>(&mut self, _len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_array()
    }
    fn emit_seq_elt<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    fn emit_map<F>(&mut self, _len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        let map : Dictionary = HashMap::new();
        self.val.push(Value::Dictionary(map));
        f(self)
    }
    fn emit_map_elt_key<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.key = match self.val.pop().unwrap() {
            Value::BasicValue(x) => Some(x),
            _ => return Err(EncoderError::BadKeyType)
        };
        Ok(())
    }
    fn emit_map_elt_val<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        let key : BasicValue = self.key.take().unwrap();
        try!(f(self));
        let val : Value = self.val.pop().unwrap();
        let mut map = self.val.pop().unwrap();
        match map {
            Value::Dictionary(ref mut x) => x.insert(key, val),
            _ => panic!("No dictionary on stack")
        };
        self.val.push(map);
        Ok(())
    }

    fn emit_option<F>(&mut self, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_option_none(&mut self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_option_some<F>(&mut self, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_enum<F>(&mut self, _name: &str, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_enum_variant<F>(&mut self, _v_name: &str, _v_id: usize, _len: usize, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_enum_variant_arg<F>(&mut self, _a_idx: usize, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_enum_struct_variant<F>(&mut self, _v_name: &str, _v_id: usize, _len: usize, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_enum_struct_variant_field<F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
}

#[test]
fn test_array() {
    let array : Vec<u32> = vec![1,2,3];
    let v = DBusEncoder::encode(&array).ok().unwrap();
    let a2 = vec![
        Value::BasicValue(BasicValue::Uint32(1)),
        Value::BasicValue(BasicValue::Uint32(2)),
        Value::BasicValue(BasicValue::Uint32(3)),
    ];
    assert_eq!(v, Value::Array(a2));
}
