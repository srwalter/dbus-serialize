//! Implements the rustc_serialize::Encoder trait
use std::collections::HashMap;

use rustc_serialize::{Encoder,Encodable};

use types::{Value,BasicValue,Struct,Signature,Dictionary,Array};

pub struct DBusEncoder {
    val: Vec<Value>,
    key: Option<BasicValue>
}

#[derive(Debug,PartialEq)]
pub enum EncoderError {
    BadKeyType,
    Unsupported,
    EmptyArray,
    EmptyMap,
}

impl DBusEncoder {
    fn handle_struct (&mut self, len: usize) -> Result<(),EncoderError> {
        let mut objs = Vec::new();
        let mut sig = "(".to_string();
        let offset = self.val.len() - len;
        for v in self.val.drain(offset..) {
            sig.push_str(v.get_signature());
            objs.push(v);
        }
        sig.push(')');
        self.val.push(Value::Struct(Struct {
            objects: objs,
            signature: Signature(sig),
        }));
        Ok(())
    }

    fn handle_array (&mut self, len: usize) -> Result<(),EncoderError> {
        let mut objs = Vec::new();
        let offset = self.val.len() - len;
        for v in self.val.drain(offset..) {
            objs.push(v);
        }
        self.val.push(Value::Array(Array::new(objs)));
        Ok(())
    }

    pub fn new() -> DBusEncoder {
        DBusEncoder {
            val: Vec::new(),
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
        Ok(())
    }
    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint64(v)));
        Ok(())
    }
    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint32(v)));
        Ok(())
    }
    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Uint16(v)));
        Ok(())
    }
    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Byte(v)));
        Ok(())
    }
    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int64(v as i64)));
        Ok(())
    }
    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int64(v)));
        Ok(())
    }
    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int32(v)));
        Ok(())
    }
    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Int16(v)));
        Ok(())
    }
    fn emit_i8(&mut self, _v: i8) -> Result<(), Self::Error> {
        Err(EncoderError::Unsupported)
    }
    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Boolean(v)));
        Ok(())
    }
    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.val.push(Value::Double(v));
        Ok(())
    }
    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.val.push(Value::Double(v as f64));
        Ok(())
    }
    fn emit_char(&mut self, v: char) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::Byte(v as u8)));
        Ok(())
    }
    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.val.push(Value::BasicValue(BasicValue::String(v.to_string())));
        Ok(())
    }

    fn emit_struct<F>(&mut self, _name: &str, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct(len)
    }
    fn emit_struct_field<F>(&mut self, _f_name: &str, _f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }
    fn emit_tuple<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct(len)
    }
    fn emit_tuple_arg<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }
    fn emit_tuple_struct<F>(&mut self, _name: &str, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        try!(f(self));
        self.handle_struct(len)
    }
    fn emit_tuple_struct_arg<F>(&mut self, _f_idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    fn emit_seq<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        if len == 0 {
            return Err(EncoderError::EmptyArray)
        }
        try!(f(self));
        self.handle_array(len)
    }
    fn emit_seq_elt<F>(&mut self, _idx: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        f(self)
    }

    fn emit_map<F>(&mut self, len: usize, f: F) -> Result<(), Self::Error> where F: FnOnce(&mut Self) -> Result<(), Self::Error> {
        if len == 0 {
            return Err(EncoderError::EmptyMap)
        }
        // Yes, i'm intentionally creating a Dictionary with an invalid signature...
        let map : Dictionary = Dictionary::new_with_sig(HashMap::new(), "".to_string());
        self.val.push(Value::Dictionary(map));
        try!(f(self));

        // Fix up the signature now that the map hopefully has elements in it.
        let x = match self.val.pop().unwrap() {
            Value::Dictionary(x) => x.map,
            _ => panic!("Where'd my dictionary go?!")
        };
        self.val.push(Value::Dictionary(Dictionary::new(x)));
        Ok(())
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
            Value::Dictionary(ref mut x) => x.map.insert(key, val),
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

#[cfg(test)]
mod test {
    use rustc_serialize::{Encoder,Encodable};
    use std::collections::HashMap;
    use types::{Value,BasicValue,Struct,Signature,Dictionary,Array};
    use encoder::*;

    #[test]
    fn test_array() {
        let array : Vec<u32> = vec![1,2,3];
        let v = DBusEncoder::encode(&array).ok().unwrap();
        let a2 = vec![
            Value::BasicValue(BasicValue::Uint32(1)),
            Value::BasicValue(BasicValue::Uint32(2)),
            Value::BasicValue(BasicValue::Uint32(3)),
        ];
        assert_eq!(v, Value::Array(Array::new(a2)));
    }

    #[test]
    fn test_empty_array() {
        let array : Vec<u32> = vec![];
        assert_eq!(DBusEncoder::encode(&array), Err(EncoderError::EmptyArray));
    }

    #[test]
    fn test_nested_array() {
        let a1 : Vec<u32> = vec![1,2,3];
        let a2 : Vec<u32> = vec![16,17];
        let a3 : Vec<u32> = vec![9];
        let array : Vec<Vec<u32>> = vec![a1,a2,a3];
        let v = DBusEncoder::encode(&array).ok().unwrap();
        let expected_a1 = vec![
            Value::BasicValue(BasicValue::Uint32(1)),
            Value::BasicValue(BasicValue::Uint32(2)),
            Value::BasicValue(BasicValue::Uint32(3)),
        ];
        let expected_a2 = vec![
            Value::BasicValue(BasicValue::Uint32(16)),
            Value::BasicValue(BasicValue::Uint32(17)),
        ];
        let expected_a3 = vec![
            Value::BasicValue(BasicValue::Uint32(9)),
        ];
        let expected_array = vec![
            Value::Array(Array::new(expected_a1)),
            Value::Array(Array::new(expected_a2)),
            Value::Array(Array::new(expected_a3)),
        ];
        assert_eq!(v, Value::Array(Array::new(expected_array)));
    }

    #[test]
    fn test_map() {
        let mut map : HashMap<u32,u64> = HashMap::new();
        map.insert(1, 100);
        map.insert(2, 200);
        map.insert(3, 300);
        let v = DBusEncoder::encode(&map).ok().unwrap();
        let mut map2 : HashMap<BasicValue,Value> = HashMap::new();
        map2.insert(BasicValue::Uint32(1), Value::BasicValue(BasicValue::Uint64(100)));
        map2.insert(BasicValue::Uint32(2), Value::BasicValue(BasicValue::Uint64(200)));
        map2.insert(BasicValue::Uint32(3), Value::BasicValue(BasicValue::Uint64(300)));
        assert_eq!(v, Value::Dictionary(Dictionary::new(map2)));
    }

    #[test]
    fn test_empty_map() {
        let map : HashMap<u32,u64> = HashMap::new();
        assert_eq!(DBusEncoder::encode(&map), Err(EncoderError::EmptyMap));
    }

    #[test]
    fn test_bad_map_key() {
        let mut map : HashMap<(u32,u32),u32> = HashMap::new();
        map.insert((1,2), 100);
        assert_eq!(DBusEncoder::encode(&map), Err(EncoderError::BadKeyType));
    }

    #[test]
    fn test_nested_map() {
        let mut map1 : HashMap<u64,u16> = HashMap::new();
        map1.insert(1, 10);
        map1.insert(2, 20);
        map1.insert(3, 30);
        let mut map2 : HashMap<u64,u16> = HashMap::new();
        map2.insert(1, 10);
        map2.insert(2, 20);
        let mut map3 : HashMap<u64,u16> = HashMap::new();
        map3.insert(19, 190);
        let mut map : HashMap<i32,HashMap<u64,u16>> = HashMap::new();
        map.insert(-1, map1);
        map.insert(-2, map2);
        map.insert(-3, map3);
        let v = DBusEncoder::encode(&map).ok().unwrap();
        let mut expected_map1 : HashMap<BasicValue,Value> = HashMap::new();
        expected_map1.insert(BasicValue::Uint64(1), Value::BasicValue(BasicValue::Uint16(10)));
        expected_map1.insert(BasicValue::Uint64(2), Value::BasicValue(BasicValue::Uint16(20)));
        expected_map1.insert(BasicValue::Uint64(3), Value::BasicValue(BasicValue::Uint16(30)));
        let mut expected_map2 : HashMap<BasicValue,Value> = HashMap::new();
        expected_map2.insert(BasicValue::Uint64(1), Value::BasicValue(BasicValue::Uint16(10)));
        expected_map2.insert(BasicValue::Uint64(2), Value::BasicValue(BasicValue::Uint16(20)));
        let mut expected_map3 : HashMap<BasicValue,Value> = HashMap::new();
        expected_map3.insert(BasicValue::Uint64(19), Value::BasicValue(BasicValue::Uint16(190)));
        let mut expected_map : HashMap<BasicValue,Value> = HashMap::new();
        expected_map.insert(BasicValue::Int32(-1), Value::Dictionary(Dictionary::new(expected_map1)));
        expected_map.insert(BasicValue::Int32(-2), Value::Dictionary(Dictionary::new(expected_map2)));
        expected_map.insert(BasicValue::Int32(-3), Value::Dictionary(Dictionary::new(expected_map3)));
        assert_eq!(v, Value::Dictionary(Dictionary::new(expected_map)));
    }

    struct SimpleTestStruct {
        a: i32,
        b: u64,
    }

    impl Encodable for SimpleTestStruct {
        fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
            s.emit_struct("SimpleTestStruct", 2, |s| {
                try!(s.emit_struct_field("a", 0, |s| {
                    s.emit_i32(self.a)
                }));
                try!(s.emit_struct_field("b", 1, |s| {
                    s.emit_u64(self.b)
                }));
                Ok(())
            })
        }
    }

    struct EmptyTestStruct {
    }

    impl Encodable for EmptyTestStruct {
        fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
            s.emit_struct("EmptyTestStruct", 0, |_| { Ok(()) })
        }
    }

    struct NestedTestStruct {
        x: SimpleTestStruct,
        y: SimpleTestStruct,
        z: EmptyTestStruct,
    }

    impl Encodable for NestedTestStruct {
        fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
            s.emit_struct("NestedTestStruct", 3, |s| {
                try!(s.emit_struct_field("x", 0, |s| {
                    Encodable::encode(&self.x, s)
                }));
                try!(s.emit_struct_field("y", 1, |s| {
                    Encodable::encode(&self.y, s)
                }));
                try!(s.emit_struct_field("z", 2, |s| {
                    Encodable::encode(&self.z, s)
                }));
                Ok(())
            })
        }
    }

    #[test]
    fn test_struct() {
        let struc = SimpleTestStruct {
            a: 1,
            b: 2,
        };
        let v = DBusEncoder::encode(&struc).ok().unwrap();
        let expected_struct = Struct {
            objects: vec![
                Value::BasicValue(BasicValue::Int32(1)),
                Value::BasicValue(BasicValue::Uint64(2)),
            ],
            signature: Signature("(it)".to_string()),
        };
        assert_eq!(v, Value::Struct(expected_struct));
    }

    #[test]
    fn test_empty_struct() {
        let struc = EmptyTestStruct {};
        let v = DBusEncoder::encode(&struc).ok().unwrap();
        let expected_struct = Struct {
            objects: vec![],
            signature: Signature("()".to_string()),
        };
        assert_eq!(v, Value::Struct(expected_struct));
    }

    #[test]
    fn test_nested_struct() {
        let struc = NestedTestStruct {
            x: SimpleTestStruct {
                a: 1,
                b: 2,
            },
            y: SimpleTestStruct {
                a: 9,
                b: 10,
            },
            z: EmptyTestStruct {},
        };
        let v = DBusEncoder::encode(&struc).ok().unwrap();
        let inner_struct_x = Struct {
            objects: vec![
                Value::BasicValue(BasicValue::Int32(1)),
                Value::BasicValue(BasicValue::Uint64(2)),
            ],
            signature: Signature("(it)".to_string()),
        };
        let inner_struct_y = Struct {
            objects: vec![
                Value::BasicValue(BasicValue::Int32(9)),
                Value::BasicValue(BasicValue::Uint64(10)),
            ],
            signature: Signature("(it)".to_string()),
        };
        let inner_struct_z = Struct {
            objects: vec![],
            signature: Signature("()".to_string()),
        };
        let expected_struct = Struct {
            objects: vec![
                Value::Struct(inner_struct_x),
                Value::Struct(inner_struct_y),
                Value::Struct(inner_struct_z),
            ],
            signature: Signature("((it)(it)())".to_string()),
        };
        assert_eq!(v, Value::Struct(expected_struct));
    }
}
