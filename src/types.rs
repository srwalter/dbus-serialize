//! Contains the Value and BasicValue enums, as well as traits and helper types for them
use std::collections::HashMap;

/// BasicValue covers the "basic" D-Bus types, that is those that are allowed to be used as keys in
/// a dictionary.
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

impl BasicValue {
    /// Returns the D-Bus type signature that corresponds to the Value
    pub fn get_signature(&self) -> &str {
        match self {
            &BasicValue::Byte(_) => "y",
            &BasicValue::Boolean(_) => "b",
            &BasicValue::Int16(_) => "n",
            &BasicValue::Uint16(_) => "q",
            &BasicValue::Int32(_) => "i",
            &BasicValue::Uint32(_) => "u",
            &BasicValue::Int64(_) => "x",
            &BasicValue::Uint64(_) => "t",
            &BasicValue::String(_) => "s",
            &BasicValue::ObjectPath(_) => "o",
            &BasicValue::Signature(_) => "g",
        }
    }
}

/// A Struct is an ordered sequence of Value objects, which may be of different varieties.
/// signature must be of the form "(<type>)", where <type> is the signature of contents of
/// objects.
#[derive(PartialEq,Debug,Clone)]
pub struct Struct {
    pub objects: Vec<Value>,
    pub signature: Signature
}

/// A Variant is a boxed type-erased value.  It is trasmitted on the wire with its signature.
/// It is useful for arrays with varying types and for allowing DBus method argument types to be
/// determined at runtime.  signature contains the signature of the boxed value.
#[derive(PartialEq,Debug,Clone)]
pub struct Variant {
    pub object: Box<Value>,
    pub signature: Signature
}

impl Variant {
    /// Create a new variant to wrap the given value.  s must be the signature of v.
    pub fn new (v: Value, s: &str) -> Variant {
        Variant {
            object: Box::new(v),
            signature: Signature(s.to_string())
        }
    }
}

/// An Array is an ordered sequence of Value objects which must all be of the same variety.  That
/// is, it is not value to have a Uint8 and a Uint32 as elements of the same Array.
#[derive(Clone,Debug,PartialEq)]
pub struct Array {
    pub objects: Vec<Value>,
    signature: Signature
}

impl Array {
    /// Create a new array from the given vector of Value.  This function may only be used when it
    /// is never possible for the input vector to be empty.  The reason is that it is impossible to
    /// determine the type signature for an empty vector.  Use new_with_sig instead.
    ///
    /// # Panics
    /// If objects.len() is 0, this function will panic.
    pub fn new(objects: Vec<Value>) -> Array {
        let inner_sig = objects.iter().next().unwrap().get_signature().to_string();
        let sig = "a".to_string() + &inner_sig;
        Array {
            objects: objects,
            signature: Signature(sig)
        }
    }

    /// Create a new array from the given vector.  If sig is not of the form a<type>
    /// or if objects is non-empty and the inner type does not match the type of the contents,
    /// the resulting value will be invalid and will not encode correctly.
    pub fn new_with_sig(objects: Vec<Value>, sig: String) -> Array {
        Array {
            objects: objects,
            signature: Signature(sig)
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct Dictionary {
    pub map: HashMap<BasicValue,Value>,
    signature: Signature
}

impl Dictionary {
    /// Create a new Dictionary from the given map.  This function may only be used when it
    /// is never possible for the input map to be empty.  The reason is that it is impossible to
    /// determine the type signature for an empty vector.  Use new_with_sig instead.
    ///
    /// # Panics
    /// If map.len() is 0, this function will panic.
    pub fn new(map: HashMap<BasicValue,Value>) -> Dictionary {
        let key_type = map.keys().next().unwrap().get_signature().to_string();
        let val_type = map.values().next().unwrap().get_signature().to_string();
        let sig = "a{".to_string() + &key_type + &val_type + "}";
        Dictionary {
            map: map,
            signature: Signature(sig)
        }
    }

    /// Create a new Dictionary from the given map.  If sig is not of the form a{<type><type>}
    /// or if map is non-empty and the inner types do not match the type of the map's contents,
    /// the resulting value will be invalid and will not encode correctly.
    pub fn new_with_sig(map: HashMap<BasicValue,Value>, sig: String) -> Dictionary {
        Dictionary {
            map: map,
            signature: Signature(sig)
        }
    }
}

/// Root type for any D-Bus value
#[derive(PartialEq,Debug,Clone)]
pub enum Value {
    BasicValue(BasicValue),
    Double(f64),
    Array(Array),
    Variant(Variant),
    Struct(Struct),
    Dictionary(Dictionary)
}

impl Value {
    /// Returns the D-Bus type signature that corresponds to the Value
    pub fn get_signature(&self) -> &str {
        match self {
            &Value::BasicValue(ref x) => x.get_signature(),
            &Value::Double(_) => "d",
            &Value::Array(ref x) => &x.signature.0,
            &Value::Variant(_) => "v",
            &Value::Struct(ref x) => &x.signature.0,
            &Value::Dictionary(ref x) => &x.signature.0
        }
    }
}

#[test]
fn test_from () {
    let x = Value::from(12);
    assert_eq!(x, Value::BasicValue(BasicValue::Int32(12)));
    let y = Value::from("foobar");
    assert_eq!(y, Value::BasicValue(BasicValue::String("foobar".to_string())));
}
