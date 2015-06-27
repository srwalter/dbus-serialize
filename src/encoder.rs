use rustc_serialize::{Encoder,Encodable};
use types::{Value,BasicValue,Struct,Signature};

pub struct DBusEncoder {
    val: Vec<Value>,
    signature: String,
    handling_key: bool
}

#[derive(Debug,PartialEq)]
pub enum EncoderError {
    BadKeyType,
    Unsupported,
    EmptyArray
}

impl DBusEncoder {
    fn handle_struct (&self) -> Result<(),EncoderError> {
        let objs = Vec::new();
        for i in self.val.into_iter() {
            objs.push(i);
        }
        let s = Struct {
            objects: objs,
            signature: Signature(self.signature.to_string())
        };
        self.signature = "".to_string();
        self.val.push(Value::Struct(s));
        Ok(())
    }

    pub fn new() -> DBusEncoder {
        DBusEncoder {
            val: Vec::new(),
            signature: "".to_string(),
            handling_key: false
        }
    }
    
    pub fn encode<T: Encodable>(obj: &T) -> Result<Value,EncoderError> {
        let mut encoder = DBusEncoder::new();
        try!(obj.encode(&mut encoder));
        Ok(encoder.val.take().unwrap())
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

    fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        if len == 0 {
            return Err(EncoderError::EmptyArray);
        }
        self.handle_dbus_array(f)
    }
    fn emit_seq_elt<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        let val = f(self);
        val
    }

    fn emit_map<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        if len == 0 {
            return Err(EncoderError::EmptyArray);
        }
        self.handle_dbus_array(f)
    }
    fn emit_map_elt_key<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        self.add_to_sig("{");
        self.handling_key = true;
        let val = try!(f(self));
        self.handling_key = false;
        Ok(val)
    }
    fn emit_map_elt_val<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        let val = try!(f(self));
        self.add_to_sig("}");
        Ok(val)
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

