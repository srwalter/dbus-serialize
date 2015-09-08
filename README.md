dbus-serialize
==============
This package aims to implement a set of standard types for representing
D-Bus objects in Rust.  Additionally, it contains utilities for
converting between D-Bus objects and native Rust objects, particularly
through supporting the Encoder and Decoder traits from rustc_serialize.
Other packages could use these types as a standard basis for encoding
D-Bus objects either directly into a bytestream, or through FFI with
dbus_message_iter.
