//! # Serializer
//!
//! The serialization module.

// TODO: use itoa (int) and ryu (float) for fast serialization
// TODO: consider const generics
use serde::{ser, Serialize};

use crate::error::{CsvError, Result as CustomResult};

pub struct CsvSerializer {
    // TODO: not sure if i want to keep it this way
    output: String,
    /// The delimiter to use between each field.
    /// Defaults to a comma.
    delimiter: &'static str,
}

impl Default for CsvSerializer {
    fn default() -> Self {
        Self {
            output: String::new(),
            delimiter: ",",
        }
    }
}

impl CsvSerializer {
    /// Pushes a string to the output buffer. **Doesn't try to escape it.**
    ///
    /// Only use this when serializing a string without any characters that need escaping.
    pub(crate) fn push_str_unchecked(&mut self, s: &str) {
        self.output.push_str(s)
    }
}

pub fn to_string<T>(value: &T) -> CustomResult<String>
where
    T: Serialize,
{
    let mut serializer = CsvSerializer::default();
    value.serialize(&mut serializer)?;
    todo!()
    //Ok(serializer.output) // you need to modify the string lol
}

impl<'a> ser::Serializer for &'a mut CsvSerializer {
    type Ok = ();
    type Error = CsvError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.output.push_str("true"),
            false => self.output.push_str("false"),
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&itoa::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&ryu::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.push_str_unchecked(&ryu::Buffer::new().format(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.push(v);
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // if a field contains quotes, we need to escape them
        let has_quotes = v.contains("\"");

        // if we escape anything, we'll need to put quotes around the whole field
        let escape = ["\n", "\r", self.delimiter].iter().any(|s| v.contains(s)) || has_quotes;

        if escape {
            self.output.push('"'); // leading quote for field

            // if we have quotes, we gotta do an allocation :p
            if has_quotes {
                self.output.reserve(v.len()); // expand the buffer
                v.chars().for_each(|c| match c {
                    '"' => self.output.push_str("\"\""), // escape quotes
                    _ => self.output.push(c),            // otherwise, just push it
                });
            } else {
                self.output.push_str(v); // no allocation needed. just push it as-is
            }

            self.output.push('"'); // trailing quote for field
        } else {
            self.output.push_str(v); // no quotes, no allocation. straight to the buffer
        }

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        todo!()
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut CsvSerializer {
    type Ok = ();
    type Error = CsvError;

    fn serialize_element<T>(&mut self, value: &T) -> CustomResult<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }
}
