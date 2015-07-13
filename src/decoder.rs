//! Implements the rustc_serialize::Decoder trait
use std;

use rustc_serialize::{Decoder,Decodable};

use types::{BasicValue,Value};

#[derive(Debug,PartialEq)]
pub enum DecodeError {
    BadSignature,
    NotSupported,
    IntTooNarrow
}

pub struct DBusDecoder {
    value: Value,
    map_val: Option<Value>
}

impl DBusDecoder {
    fn get_unsigned_int (v: &BasicValue) -> Result<u64,DecodeError> {
        let val = match v {
            &BasicValue::Byte(x) => x as u64,
            &BasicValue::Uint16(x) => x as u64,
            &BasicValue::Uint32(x) => x as u64,
            &BasicValue::Uint64(x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
	Ok(val)
    }

    fn get_signed_int (v: &BasicValue) -> Result<i64,DecodeError> {
        let val = match v {
            &BasicValue::Int16(x) => x as i64,
            &BasicValue::Int32(x) => x as i64,
            &BasicValue::Int64(x) => x as i64,
            _ => return Err(DecodeError::BadSignature)
        };
	Ok(val)
    }

    fn read_unsigned_int (v: &Value, max: usize) -> Result<u64,DecodeError> {
        let basic_val = match v {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };

        // Make sure the value will fit
	let x = try!(DBusDecoder::get_unsigned_int(basic_val));
	if x > (max as u64) {
            return Err(DecodeError::IntTooNarrow);
	}
        Ok(x)
    }

    fn read_signed_int (v: &Value, max: isize, min: isize) -> Result<i64,DecodeError> {
        let basic_val = match v {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
	let x = try!(DBusDecoder::get_signed_int(basic_val));

        // Make sure the value will fit
	if x > (max as i64) {
            return Err(DecodeError::IntTooNarrow);
	}
	if x < (min as i64) {
            return Err(DecodeError::IntTooNarrow);
	}
        Ok(x)
    }

    pub fn new (v: Value) -> DBusDecoder {
        DBusDecoder{
            value: v,
            map_val: None
        }
    }

    pub fn decode<T: Decodable>(v: Value) -> Result<T,DecodeError> {
        let mut decoder = DBusDecoder::new(v);
        T::decode(&mut decoder)
    }
}

impl Decoder for DBusDecoder {
    type Error = DecodeError;
    
    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        let basic_val = match &self.value {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
	let x = try!(DBusDecoder::get_unsigned_int(basic_val));
        Ok(x as usize)
    }
    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        let val = try!(self.read_usize());
        Ok(val as u64)
    }
    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(try!(DBusDecoder::read_unsigned_int(&self.value, std::u32::MAX as usize)) as u32)
    }
    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        Ok(try!(DBusDecoder::read_unsigned_int(&self.value, std::u16::MAX as usize)) as u16)
    }
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        Ok(try!(DBusDecoder::read_unsigned_int(&self.value, std::u8::MAX as usize)) as u8)
    }

    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        let basic_val = match &self.value {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
	let x = try!(DBusDecoder::get_signed_int(basic_val));
        Ok(x as isize)
    }
    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        let val = try!(self.read_isize());
        Ok(val as i64)
    }
    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        Ok(try!(DBusDecoder::read_signed_int(&self.value, std::i32::MAX as isize, std::i32::MIN as isize)) as i32)
    }
    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        Ok(try!(DBusDecoder::read_signed_int(&self.value, std::i16::MAX as isize, std::i16::MIN as isize)) as i16)
    }
    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        Ok(try!(DBusDecoder::read_signed_int(&self.value, std::i8::MAX as isize, std::i8::MIN as isize)) as i8)
    }
    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        let basic_val = match &self.value {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
        let x = match basic_val {
            &BasicValue::Boolean(x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
        Ok(x)
    }
    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        match &self.value {
            &Value::Double(x) => Ok(x),
            _ => return Err(DecodeError::BadSignature)
        }
    }
    fn read_char(&mut self) -> Result<char, Self::Error> {
        let val = try!(self.read_u8());
        Ok(val as char)
    }
    fn read_str(&mut self) -> Result<String, Self::Error> {
        let basic_val = match &self.value {
            &Value::BasicValue(ref x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
        let x = match basic_val {
            &BasicValue::String(ref x) => x.to_string(),
            &BasicValue::ObjectPath(ref x) => x.0.to_string(),
            &BasicValue::Signature(ref x) => x.0.to_string(),
            _ => return Err(DecodeError::BadSignature)
        };
        Ok(x)
    }

    fn read_seq<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        let len = match self.value {
            Value::Array(ref x) => x.objects.len(),
            _ => return Err(DecodeError::BadSignature)
        };
        f(self, len)
    }
    fn read_seq_elt<T, F>(&mut self, idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        let val = match self.value {
            Value::Array(ref mut x) => {
                x.objects.push(Value::BasicValue(BasicValue::Byte(0)));
                x.objects.swap_remove(idx)
            },
            _ => return Err(DecodeError::BadSignature)
        };
        let mut subdecoder = DBusDecoder::new(val);
        f(&mut subdecoder)
    }

    fn read_map<T, F>(&mut self, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self, usize) -> Result<T, Self::Error> {
        let len = match self.value {
            Value::Dictionary(ref x) => x.map.keys().len(),
            _ => return Err(DecodeError::BadSignature)
        };
        f(self, len)
    }
    fn read_map_elt_key<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        let dict = match self.value {
            Value::Dictionary(ref mut x) => x,
            _ => return Err(DecodeError::BadSignature)
        };
        let key = {
            dict.map.keys().next().unwrap().clone()
        };
        self.map_val = Some(dict.map.remove(&key).unwrap());

        let mut subdecoder = DBusDecoder::new(Value::BasicValue(key));
        f(&mut subdecoder)
    }
    fn read_map_elt_val<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        let val = self.map_val.take().unwrap();
        let mut subdecoder = DBusDecoder::new(val);
        f(&mut subdecoder)
    }

    fn read_struct<T, F>(&mut self, _s_name: &str, _len: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        match self.value {
            Value::Struct(_) => (),
            _ => return Err(DecodeError::BadSignature)
        };
        f(self)
    }
    fn read_struct_field<T, F>(&mut self, _f_name: &str, f_idx: usize, f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        let val = match self.value {
            Value::Struct(ref mut x) => {
                x.objects.push(Value::BasicValue(BasicValue::Byte(0)));
                x.objects.swap_remove(f_idx)
            },
            _ => return Err(DecodeError::BadSignature)
        };
        let mut subdecoder = DBusDecoder::new(val);
        f(&mut subdecoder)
    }

    fn read_enum<T, F>(&mut self, _name: &str, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_enum_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_enum_variant_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_enum_struct_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, usize) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_enum_struct_variant_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_tuple<T, F>(&mut self, _len: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_tuple_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_tuple_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_tuple_struct_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Self::Error> where F: FnOnce(&mut Self) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_option<T, F>(&mut self, _f: F) -> Result<T, Self::Error> where F: FnMut(&mut Self, bool) -> Result<T, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_nil(&mut self) -> Result<(), Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        Err(DecodeError::NotSupported)
    }
    fn error(&mut self, _err: &str) -> Self::Error {
        DecodeError::NotSupported
    }
}

#[cfg(test)]
mod test {
    use rustc_serialize::{Decoder,Decodable};
    use types::{BasicValue,Value,Path,Struct,Signature,Array};
    use decoder::*;

    #[test]
    fn test_array () {
        let vec = vec![
            Value::BasicValue(BasicValue::Uint32(1)),
            Value::BasicValue(BasicValue::Uint32(2)),
            Value::BasicValue(BasicValue::Uint32(3)),
        ];
        let val = Value::Array(Array::new(vec));
        let arr : Vec<u32> = DBusDecoder::decode(val).ok().unwrap();
        assert_eq!(vec![1,2,3], arr);
    }

    #[test]
    fn test_int () {
        let v = Value::BasicValue(BasicValue::Uint32(1024));
        let i : u32 = DBusDecoder::decode(v).ok().unwrap();
        assert_eq!(i, 1024);

        let x = Value::BasicValue(BasicValue::Uint32(1024));
        let err = DBusDecoder::decode::<u8>(x).err().unwrap();
        assert_eq!(err, DecodeError::IntTooNarrow);
    }

    #[test]
    fn test_string () {
        let v = Value::BasicValue(BasicValue::String("foo".to_string()));
        let i : String = DBusDecoder::decode(v).ok().unwrap();
        assert_eq!(i, "foo");

        let v = Value::BasicValue(BasicValue::Signature(Signature("foo".to_string())));
        let i : String = DBusDecoder::decode(v).ok().unwrap();
        assert_eq!(i, "foo");

        let v = Value::BasicValue(BasicValue::ObjectPath(Path("foo".to_string())));
        let i : String = DBusDecoder::decode(v).ok().unwrap();
        assert_eq!(i, "foo");
    }

    #[derive(PartialEq,Debug)]
    struct TestStruct {
        foo: u8,
        bar: u32,
        baz: String,
    }

    impl Decodable for TestStruct {
        fn decode<S: Decoder>(s: &mut S) -> Result<Self, S::Error> {
            s.read_struct("TestStruct", 3, |s: &mut S| {
                let foo = try!(s.read_struct_field("foo", 0, |s: &mut S| {
                    s.read_u8()
                }));
                let bar = try!(s.read_struct_field("bar", 1, |s: &mut S| {
                    s.read_u32()
                }));
                let baz = try!(s.read_struct_field("baz", 2, |s: &mut S| {
                    s.read_str()
                }));
                Ok(TestStruct {
                    foo: foo,
                    bar: bar,
                    baz: baz
                })
            })
        }
    }

    #[test]
    fn test_struct () {
        let objects = vec![
            Value::BasicValue(BasicValue::Byte(1)),
            Value::BasicValue(BasicValue::Uint32(10)),
            Value::BasicValue(BasicValue::String("baz".to_string()))
        ];
        let s = Struct {
            objects: objects,
            signature: Signature("(yus)".to_string())
        };
        let v = Value::Struct(s);

        let x : TestStruct = DBusDecoder::decode(v).unwrap();
        assert_eq!(x, TestStruct {
            foo: 1,
            bar: 10,
            baz: "baz".to_string()
        });
    }
}

