use std::marker::PhantomData;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use paste::paste;
use serde::ser::{Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple};
use serde::{ser, Serialize};

use varint_rs::VarintWriter;

use crate::{EndiannessImpl, FieldType, NbtError, NetworkLittleEndian, Variant};

/// Returns a `not supported` error.
macro_rules! forward_unsupported {
    ($($ty: ident),+) => {
        paste! {$(
            #[inline]
            fn [<serialize_ $ty>](self, _v: $ty) -> Result<(), NbtError> {
                Err(NbtError::Unsupported(concat!(
                    "Serialization of `", stringify!($ty), "` is not supported"
                )))
            }
        )+}
    }
}

/// Returns a `not supported` error.
macro_rules! forward_unsupported_field {
    ($($ty: ident),+) => {
        paste! {$(
            #[inline]
            fn [<serialize_ $ty>](self, _v: $ty) -> Result<bool, NbtError> {
                Err(NbtError::Unsupported(concat!(
                    "Serialization of `", stringify!($ty), "` is not supported"
                )))
            }
        )+}
    }
}

/// Serializes the given data in any endian format.
///
/// See [`to_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbtx::to_bytes::<nbtx::BigEndian>(&data).unwrap();
/// # }
/// ```
pub fn to_bytes<E>(v: &(impl Serialize + ?Sized)) -> Result<Vec<u8>, NbtError>
where
    E: EndiannessImpl,
{
    let mut ser = Serializer::<_, E>::new(Vec::new());
    v.serialize(&mut ser)?;

    Ok(ser.into_inner())
}

/// Serializes the given data in any endian format.
///
/// See [`to_bytes`] for an alternative just returns a new buffer, instead of using an existing writer.
///
/// # Example
///
/// ```rust
/// # use std::io::Cursor;
/// # fn main() {
/// #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Cursor::new(Vec::new());
///
///  nbtx::to_bytes_in::<nbtx::BigEndian>(&mut writer, &data).unwrap();
/// # }
/// ```
pub fn to_bytes_in<E>(
    writer: &mut impl WriteBytesExt,
    v: &(impl Serialize + ?Sized),
) -> Result<(), NbtError>
where
    E: EndiannessImpl,
{
    let mut ser = Serializer::<_, E>::new(writer);
    v.serialize(&mut ser)?;

    Ok(())
}

/// Serializes the given data in network little endian format.
///
/// This is the format used by Minecraft: Bedrock Edition.
///
/// See [`to_net_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbtx::to_net_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_net_bytes<T>(v: &T) -> Result<Vec<u8>, NbtError>
where
    T: ?Sized + Serialize,
{
    to_bytes::<NetworkLittleEndian>(v)
}

/// Serializes the given data in network little endian format.
///
/// This is the format used by Minecraft: Bedrock Edition.
///
/// See [`to_net_bytes`] for an alternative just returns a new buffer, instead of using an existing writer.
///
/// # Example
///
/// ```rust
/// # use std::io::Cursor;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Cursor::new(Vec::new());
///
///  let encoded = nbtx::to_net_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_net_bytes_in<T, W>(writer: &mut W, v: &T) -> Result<(), NbtError>
where
    W: WriteBytesExt,
    T: ?Sized + Serialize,
{
    to_bytes_in::<NetworkLittleEndian>(writer, v)
}

/// Serializes the given data in big endian format.
///
/// This is the format used by Minecraft: Java Edition.
///
/// See [`to_be_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbtx::to_be_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_be_bytes<T>(v: &T) -> Result<Vec<u8>, NbtError>
where
    T: ?Sized + Serialize,
{
    to_bytes::<BigEndian>(v)
}

/// Serializes the given data in big endian format.
///
/// This is the format used by Minecraft: Java Edition.
///
/// See [`to_be_bytes`] for an alternative just returns a new buffer, instead of using an existing writer.
///
/// # Example
///
/// ```rust
/// # use std::io::Cursor;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Cursor::new(Vec::new());
///
///  let encoded = nbtx::to_be_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_be_bytes_in<T, W>(writer: &mut W, v: &T) -> Result<(), NbtError>
where
    W: WriteBytesExt,
    T: ?Sized + Serialize,
{
    to_bytes_in::<BigEndian>(writer, v)
}

/// Serializes the given data in little endian format.
///
/// This is the format used by Minecraft: Bedrock Edition.
///
/// See [`to_be_bytes_in`] for an alternative that serializes into the given writer, instead
/// of producing a new one.
///
/// # Example
///
/// ```rust
/// # fn main() {
/// #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let encoded = nbtx::to_le_bytes(&data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_le_bytes<T>(v: &T) -> Result<Vec<u8>, NbtError>
where
    T: ?Sized + Serialize,
{
    to_bytes::<LittleEndian>(v)
}

/// Serializes the given data in little endian format.
///
/// This is the format used by Minecraft: Bedrock Edition.
///
/// See [`to_le_bytes_in`] for an alternative just returns a new buffer, instead of using an existing writer.
///
/// # Example
///
/// ```rust
/// # use std::io::Cursor;
/// # fn main() {
///  #[derive(serde::Serialize, serde::Deserialize)]
///  struct Data {
///     value: String
///  }
///
///  let data = Data { value: "Hello, World!".to_owned() };
///  let mut writer = Cursor::new(Vec::new());
///
///  let encoded = nbtx::to_le_bytes_in(&mut writer, &data).unwrap();
/// # }
/// ```
#[inline]
pub fn to_le_bytes_in<T, W>(writer: &mut W, v: &T) -> Result<(), NbtError>
where
    W: WriteBytesExt,
    T: ?Sized + Serialize,
{
    to_bytes_in::<LittleEndian>(writer, v)
}

/// NBT data serializer.
#[derive(Debug)]
pub struct Serializer<W, E>
where
    W: WriteBytesExt,
    E: EndiannessImpl,
{
    writer: W,
    /// Whether this is the first data to be written.
    /// This makes sure that the name and type of the root compound are written.
    is_initial: bool,
    /// Stores the length of the list that is currently being serialised.
    len: usize,
    _marker: PhantomData<E>,
}

impl<W, E> Serializer<W, E>
where
    W: WriteBytesExt,
    E: EndiannessImpl,
{
    /// Creates a new and empty serializer.
    #[inline]
    pub const fn new(w: W) -> Serializer<W, E> {
        Serializer {
            writer: w,
            is_initial: true,
            len: 0,
            _marker: PhantomData,
        }
    }

    /// Consumes the serialiser and returns the inner writer.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W, E> ser::Serializer for &mut Serializer<W, E>
where
    E: EndiannessImpl,
    W: WriteBytesExt,
{
    type Ok = ();
    type Error = NbtError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<(), NbtError>;
    type SerializeTupleVariant = Impossible<(), NbtError>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<(), NbtError>;

    forward_unsupported!(char, u8, u16, u32, u64, i128);

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<(), NbtError> {
        self.writer.write_u8(v as u8)?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<(), NbtError> {
        self.writer.write_i8(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_i16::<BigEndian>(v)?,
            Variant::LittleEndian | Variant::NetworkEndian => {
                self.writer.write_i16::<LittleEndian>(v)?
            }
        };

        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_i32::<BigEndian>(v)?,
            Variant::LittleEndian => self.writer.write_i32::<LittleEndian>(v)?,
            Variant::NetworkEndian => self.writer.write_i32_varint(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_i64::<BigEndian>(v)?,
            Variant::LittleEndian => self.writer.write_i64::<LittleEndian>(v)?,
            Variant::NetworkEndian => self.writer.write_i64_varint(v)?,
        };

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_f32::<BigEndian>(v)?,
            Variant::LittleEndian | Variant::NetworkEndian => {
                self.writer.write_f32::<LittleEndian>(v)?
            }
        };

        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_f64::<BigEndian>(v)?,
            Variant::LittleEndian | Variant::NetworkEndian => {
                self.writer.write_f64::<LittleEndian>(v)?
            }
        };

        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_u16::<BigEndian>(v.len() as u16),
            Variant::LittleEndian => self.writer.write_u16::<LittleEndian>(v.len() as u16),
            Variant::NetworkEndian => self.writer.write_u32_varint(v.len() as u32),
        }?;

        self.writer.write_all(v.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<(), NbtError> {
        match E::AS_ENUM {
            Variant::BigEndian => self.writer.write_i32::<BigEndian>(v.len() as i32),
            Variant::LittleEndian => self.writer.write_i32::<LittleEndian>(v.len() as i32),
            Variant::NetworkEndian => self.writer.write_i32_varint(v.len() as i32),
        }?;

        self.writer.write_all(v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing Options is not supported",
        ))
    }

    fn serialize_some<T: Serialize + ?Sized>(self, _value: &T) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing Options is not supported",
        ))
    }

    fn serialize_unit(self) -> Result<(), NbtError> {
        Err(NbtError::Unsupported("Serializing () is not supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing unit structs is not supported",
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing unit variants is not supported",
        ))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing newtype structs is not supported",
        ))
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), NbtError> {
        Err(NbtError::Unsupported(
            "Serializing newtype variants is not supported",
        ))
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if let Some(len) = len {
            self.len = len;
            Ok(self)
        } else {
            Err(NbtError::Unsupported("Dynamically sized sequences is not supported. If you are trying to serialize an iterator, call `Iterator::collect` to create a sequence with known size."))
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.len = len;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing tuple structs is not supported",
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing tuple variants is not supported",
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // nbt::Value does not distinguish between maps and structs.
        // Therefore, this is also necessary here
        if self.is_initial {
            self.writer.write_u8(FieldType::Compound as u8)?;
            self.serialize_str("")?;
            self.is_initial = false;
        }

        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.is_initial {
            self.writer.write_u8(FieldType::Compound as u8)?;
            self.serialize_str(name)?;
            self.is_initial = false;
        }

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing struct variants is not supported",
        ))
    }
}

impl<W, F> SerializeSeq for &mut Serializer<W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = ();
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        if self.len != 0 {
            let ty_serializer = FieldTypeSerializer::new(self);
            element.serialize(ty_serializer)?;

            match F::AS_ENUM {
                Variant::BigEndian => self.writer.write_i32::<BigEndian>(self.len as i32),
                Variant::LittleEndian => self.writer.write_i32::<LittleEndian>(self.len as i32),
                Variant::NetworkEndian => self.writer.write_i32_varint(self.len as i32),
            }?;
            self.len = 0;
        }

        element.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        Ok(())
    }
}

impl<W, M> SerializeTuple for &mut Serializer<W, M>
where
    W: WriteBytesExt,
    M: EndiannessImpl,
{
    type Ok = ();
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        if self.len != 0 {
            let ty_serializer = FieldTypeSerializer::new(self);
            element.serialize(ty_serializer)?;

            match M::AS_ENUM {
                Variant::BigEndian => self.writer.write_i32::<BigEndian>(self.len as i32),
                Variant::LittleEndian => self.writer.write_i32::<LittleEndian>(self.len as i32),
                Variant::NetworkEndian => self.writer.write_i32_varint(self.len as i32),
            }?;
            self.len = 0;
        }

        element.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        Ok(())
    }
}

impl<W, M> SerializeMap for &mut Serializer<W, M>
where
    W: WriteBytesExt,
    M: EndiannessImpl,
{
    type Ok = ();
    type Error = NbtError;

    /// This function *must* not be used. Use [`serialize_key`](Self::serialize_key) instead.
    fn serialize_key<K>(&mut self, _key: &K) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
    {
        Err(NbtError::Unsupported(
            "Serializer::serialize_key is not supported. Use Serializer::serialize_entry instead",
        ))
    }

    /// This function *must* not be used. Use [`serialize_key`](Self::serialize_key) instead.
    fn serialize_value<V>(&mut self, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Err(NbtError::Unsupported(
            "Serializer::serialize_value is not supported. Use Serializer::serialize_entry instead",
        ))
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
        V: ?Sized + Serialize,
    {
        let ty_serializer = FieldTypeSerializer::new(self);
        value.serialize(ty_serializer)?;

        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        self.writer.write_u8(FieldType::End as u8)?;
        Ok(())
    }
}

impl<W, M> SerializeStruct for &mut Serializer<W, M>
where
    W: WriteBytesExt,
    M: EndiannessImpl,
{
    type Ok = ();
    type Error = NbtError;

    fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        let ty_serializer = FieldTypeSerializer::new(self);
        let should_skip = value.serialize(ty_serializer)?;

        if !should_skip {
            match M::AS_ENUM {
                Variant::LittleEndian => self.writer.write_u16::<LittleEndian>(key.len() as u16),
                Variant::BigEndian => self.writer.write_u16::<BigEndian>(key.len() as u16),
                Variant::NetworkEndian => self.writer.write_u32_varint(key.len() as u32),
            }?;

            self.writer.write_all(key.as_bytes())?;
            value.serialize(&mut **self)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn end(self) -> Result<(), NbtError> {
        self.writer.write_u8(FieldType::End as u8)?;
        Ok(())
    }
}

/// Separate serialiser that writes data types to the writer.
///
/// Serde does not provide any type information, hence this exists.
///
/// This serialiser writes the data type of the given value and does not consume it.
struct FieldTypeSerializer<'a, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    ser: &'a mut Serializer<W, F>,
}

impl<'a, W, F> FieldTypeSerializer<'a, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    pub fn new(ser: &'a mut Serializer<W, F>) -> Self {
        Self { ser }
    }
}

impl<W, F> ser::Serializer for FieldTypeSerializer<'_, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = bool; // Whether the field should be skipped
    type Error = NbtError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Impossible<bool, Self::Error>;
    type SerializeTupleVariant = Impossible<bool, Self::Error>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Impossible<bool, Self::Error>;

    forward_unsupported_field!(char, u8, u16, u32, u64, i128);

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<bool, Self::Error> {
        self.ser.writer.write_u8(FieldType::Byte as u8)?;
        Ok(false)
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Byte as u8)?;
        Ok(false)
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Short as u8)?;
        Ok(false)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Int as u8)?;
        Ok(false)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Long as u8)?;
        Ok(false)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Float as u8)?;
        Ok(false)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::Double as u8)?;
        Ok(false)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::String as u8)?;
        Ok(false)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ser.writer.write_u8(FieldType::ByteArray as u8)?;
        Ok(false)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(true) // Skip field
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)?;
        Ok(false)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(NbtError::Unsupported("Serializing () is not supported"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing unit structs is not supported",
        ))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing unit variants is not supported",
        ))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing newtype structs is not supported",
        ))
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing newtype variants is not supported",
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ser.writer.write_u8(FieldType::List as u8)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ser.writer.write_u8(FieldType::List as u8)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing tuple structs is not supported",
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing tuple variants is not supported",
        ))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.ser.writer.write_u8(FieldType::Compound as u8)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.ser.writer.write_u8(FieldType::Compound as u8)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(NbtError::Unsupported(
            "Serializing struct variants is not supported",
        ))
    }
}

impl<W, F> SerializeSeq for FieldTypeSerializer<'_, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, _element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<W, F> SerializeTuple for FieldTypeSerializer<'_, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_element<T>(&mut self, _element: &T) -> Result<(), NbtError>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<W, F> SerializeMap for FieldTypeSerializer<'_, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_key<K>(&mut self, _key: &K) -> Result<(), NbtError>
    where
        K: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn serialize_value<V>(&mut self, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl<W, F> SerializeStruct for FieldTypeSerializer<'_, W, F>
where
    W: WriteBytesExt,
    F: EndiannessImpl,
{
    type Ok = bool;
    type Error = NbtError;

    #[inline]
    fn serialize_field<V>(&mut self, _key: &'static str, _value: &V) -> Result<(), NbtError>
    where
        V: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}
