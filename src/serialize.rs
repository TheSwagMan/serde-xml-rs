use std::io::Write;
use std::fmt::Display;
use serde::ser::{self, Impossible, Serialize};
use error::Error;


pub fn serialize<W: Write, S: Serialize>(value: S, writer: W) -> Result<(), Error> {
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

/// An XML Serializer.
pub struct Serializer<W>
    where W: Write
{
    writer: W,
}

impl<W> Serializer<W>
    where W: Write
{
    pub fn new(writer: W) -> Self {
        Self { writer: writer }
    }

    fn write_primitive<P: Display>(&mut self, primitive: P) -> Result<(), Error> {
        write!(self.writer, "{}", primitive)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn write_wrapped<S: Serialize>(&mut self, tag: &str, value: S) -> Result<(), Error> {
        write!(self.writer, "<{}>", tag)?;
        value.serialize(&mut *self)?;
        write!(self.writer, "</{}>", tag)?;
        Ok(())
    }
}


#[allow(unused_variables)]
impl<'w, W> ser::Serializer for &'w mut Serializer<W>
    where W: Write
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Map<'w, W>;
    type SerializeStruct = Struct<'w, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            write!(self.writer, "true")?;
        } else {
            write!(self.writer, "false")?;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(v)
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.write_primitive(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        // TODO: I imagine you'd want to use base64 here.
        // Not sure how to roundtrip effectively though...
        Err(Error::UnsupportedOperation("serialize_bytes".to_string()))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.write_wrapped(name, ())
    }

    fn serialize_unit_variant(self,
                              name: &'static str,
                              variant_index: usize,
                              variant: &'static str)
                              -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_unit_variant".to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                       name: &'static str,
                                                       value: &T)
                                                       -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_newtype_struct".to_string()))
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        variant_index: usize,
                                                        variant: &'static str,
                                                        value: &T)
                                                        -> Result<Self::Ok, Self::Error> {
        self.write_wrapped(name, value)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // TODO: Figure out how to constrain the things written to only be composites
        Err(Error::UnsupportedOperation("serialize_seq".to_string()))
    }

    fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_seq_fixed_size".to_string()))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_tuple".to_string()))
    }

    fn serialize_tuple_struct(self,
                              name: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_tuple_struct".to_string()))
    }

    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedOperation("serialize_tuple_variant".to_string()))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Map { parent: self })
    }

    fn serialize_struct(self,
                        name: &'static str,
                        len: usize)
                        -> Result<Self::SerializeStruct, Self::Error> {
        write!(self.writer, "<{}>", name)?;
        Ok(Struct {
               parent: self,
               name: name,
           })
    }

    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedOperation("Result".to_string()))
    }
}

/// An implementation of SerializeStruct for serializing to XML.
pub struct Struct<'w, W>
    where W: 'w + Write
{
    parent: &'w mut Serializer<W>,
    name: &'w str,
}

impl<'w, W> ser::SerializeStruct for Struct<'w, W>
    where W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              key: &'static str,
                                              value: &T)
                                              -> Result<(), Self::Error> {
        write!(self.parent.writer, "<{}>", key)?;
        value.serialize(&mut *self.parent)?;
        write!(self.parent.writer, "</{}>", key)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.parent.writer, "</{}>", self.name).map_err(|e| e.into())
    }
}

/// An implementation of SerializeMap for serializing to XML.
pub struct Map<'w, W>
    where W: 'w + Write
{
    parent: &'w mut Serializer<W>,
}

impl<'w, W> ser::SerializeMap for Map<'w, W>
    where W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        panic!("impossible to serialize the key on its own, please use serialize_entry()")
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        value.serialize(&mut *self.parent)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(&mut self,
                                                                     key: &K,
                                                                     value: &V)
                                                                     -> Result<(), Self::Error> {
        // TODO: Is it possible to ensure our key is never a composite type?
        // Anything which isn't a "primitive" would lead to malformed XML here...
        write!(self.parent.writer, "<")?;
        key.serialize(&mut *self.parent)?;
        write!(self.parent.writer, ">")?;

        value.serialize(&mut *self.parent)?;

        write!(self.parent.writer, "</")?;
        key.serialize(&mut *self.parent)?;
        write!(self.parent.writer, ">")?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serializer as SerSerializer;
    use serde::ser::{SerializeMap, SerializeStruct};

    #[test]
    fn test_serialize_bool() {
        let inputs = vec![(true, "true"), (false, "false")];

        for (src, should_be) in inputs {
            let mut buffer = Vec::new();

            {
                let mut ser = Serializer::new(&mut buffer);
                ser.serialize_bool(src).unwrap();
            }

            let got = String::from_utf8(buffer).unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn test_start_serialize_struct() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let _ = ser.serialize_struct("foo", 0).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>");
    }

    #[test]
    fn test_serialize_struct_field() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let mut struct_ser = Struct {
                parent: &mut ser,
                name: "baz",
            };
            struct_ser.serialize_field("foo", "bar").unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>bar</foo>");
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct Person {
            name: String,
            age: u32,
        }

        let bob = Person {
            name: "Bob".to_string(),
            age: 42,
        };
        let should_be = "<Person><name>Bob</name><age>42</age></Person>";
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            bob.serialize(&mut ser).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn test_serialize_map_entries() {
        let should_be = "<name>Bob</name><age>5</age>";
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let mut map = Map { parent: &mut ser };
            map.serialize_entry("name", "Bob").unwrap();
            map.serialize_entry("age", "5").unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn test_serialize_enum() {
        #[derive(Serialize)]
        enum Node {
            Boolean(bool),
            Number(f64),
            String(String),
        }

        let mut buffer = Vec::new();
        let should_be = "<Boolean>true</Boolean>";

        {
            let mut ser = Serializer::new(&mut buffer);
            let node = Node::Boolean(true);
            node.serialize(&mut ser).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, should_be);
    }
}
