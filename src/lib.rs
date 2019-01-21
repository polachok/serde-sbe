use serde::Deserializer;
use serde::de::IntoDeserializer;
use serde::de::Visitor;

use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct SbeDeser<R: Read> {
    reader: R,
}

impl<R: Read> SbeDeser<R> {
    pub fn from_reader(reader: R) -> Self {
        SbeDeser { reader }
    }
}

#[derive(Debug)]
pub enum Error {
    NotSupported,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        println!("{}", msg);
        unimplemented!()
    }
}

pub type Result<V> = std::result::Result<V, Error>;

impl<'de, R> Deserializer<'de> for &mut SbeDeser<R> where R: Read {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u8()?;
        match v {
            0 => visitor.visit_bool(false),
            1 => visitor.visit_bool(true),
            _ => unimplemented!(),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_i8()?;
        visitor.visit_i8(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u8()?;
        visitor.visit_u8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_i16::<LittleEndian>()?;
        visitor.visit_i16(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u16::<LittleEndian>()?;
        visitor.visit_u16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_i32::<LittleEndian>()?;
        visitor.visit_i32(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u32::<LittleEndian>()?;
        visitor.visit_u32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_i64::<LittleEndian>()?;
        visitor.visit_i64(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u64::<LittleEndian>()?;
        visitor.visit_u64(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_f64::<LittleEndian>()?;
        visitor.visit_f64(v)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.reader.read_u8()?;
        visitor.visit_char(v as char)
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        struct Access<'a, R: Read + 'a> {
            deserializer: &'a mut SbeDeser<R>,
            len: usize,
        }

        impl<
            'de,
            'a,
            'b: 'a,
            R: Read
        > serde::de::SeqAccess<'de> for Access<'a, R> {
            type Error = Error;

            fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
            where
                T: serde::de::DeserializeSeed<'de>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let value = serde::de::DeserializeSeed::deserialize(
                        seed,
                        &mut *self.deserializer,
                    )?;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: self,
            len: len,
        })
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)    
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        impl<'de, 'a, R> serde::de::VariantAccess<'de> for &'a mut SbeDeser<R>
        where R: Read {
            type Error = Error;

            fn unit_variant(self) -> Result<()> {
                Ok(())
            }

            fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
                where T: serde::de::DeserializeSeed<'de>,
            {
                serde::de::DeserializeSeed::deserialize(seed, self)
            }

            fn tuple_variant<V>(self,
                              len: usize,
                              visitor: V) -> Result<V::Value>
                where V: serde::de::Visitor<'de>,
            {
                serde::de::Deserializer::deserialize_tuple(self, len, visitor)
            }

            fn struct_variant<V>(self,
                               fields: &'static [&'static str],
                               visitor: V) -> Result<V::Value>
                where V: serde::de::Visitor<'de>,
            {
                serde::de::Deserializer::deserialize_tuple(self, fields.len(), visitor)
            }
        }

        impl<'de, 'a, R: 'a> serde::de::EnumAccess<'de> for &'a mut SbeDeser<R>
        where R: Read {
            type Error = Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
                where V: serde::de::DeserializeSeed<'de>,
            {
                let idx: u8 = serde::de::Deserialize::deserialize(&mut *self)?;
                let idx = idx - b'1';
                let val: Result<_> = seed.deserialize(idx.into_deserializer());
                Ok((val?, self))
            }
        }
        visitor.visit_enum(self)
        //Err(Error::NotSupported)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }
}

#[cfg(test)]
mod tests {
    use super::SbeDeser;
    use std::io::Cursor;
    use serde::Deserialize;

    // SBE 1.0 with errata July 27, 2018, p.68
    #[test]
    fn sbe_1_0_p68() {
        use serde_derive::Deserialize;

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct SimpleOpenFramingHeader {
            message_length: u32,
            encoding_type: u16,
        }

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct StandardHeader {
            block_length: u16,
            template_id: u16,
            schema_id: u16,
            version: u16,
        }

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct CIOrdId([u8; 8]);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Account([u8; 8]);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Symbol([u8; 8]);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct TransactTime(u64);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Qty(u32);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Price(i64);

        #[repr(u8)]
        #[derive(Debug, Deserialize, Eq, PartialEq)]
        enum OrderType {
            Market = b'1',
            Limit = b'2',
            Stop = b'3',
            StopLimit = b'4',
        }

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct OrderType2(char);

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct NewOrderSingle {
            order_id: CIOrdId,
            account: Account,
            security_id: Symbol,
            side: char,
            time: TransactTime,
            order_qty: Qty,
            order_type: OrderType,
            price: Price,
        }

        #[derive(Debug, Deserialize, Eq, PartialEq)]
        struct Test1 {
            framing: SimpleOpenFramingHeader,
            standard: StandardHeader,
            command: NewOrderSingle,
        }

        let bytes = [
            0x00, 0x00, 0x00, 0x44, 0xeb, 0x50, 0x36, 0x00, 0x63, 0x00, 0x5b, 00, 00, 00, 0x4f, 0x52,
            0x44, 0x30, 0x30, 0x30, 0x30, 0x31, 0x41, 0x43, 0x43, 0x54, 0x30, 0x31, 00, 00, 0x47, 0x45,
            0x4d, 0x34, 00, 00, 00, 00, 0x31, 0x80, 0x16, 0xb3, 0x3b, 0x13, 0x65, 0x29, 0x15, 0x07,
            0, 0, 0, 0x32, 0x1a, 0x85, 0x1, 00, 00, 00, 00, 00, 00, 00, 00, 00,
            0, 0, 0, 80
        ];

        let mut deser = SbeDeser::from_reader(Cursor::new(&bytes[..]));
        let t = Test1::deserialize(&mut deser).unwrap();
        println!("{:?}", t);
        assert_eq!(t.command, NewOrderSingle {
            order_id: CIOrdId([b'O', b'R', b'D', b'0', b'0', b'0', b'0', b'1']),
            account: Account([b'A', b'C', b'C', b'T', b'0', b'1', 0, 0]),
            security_id: Symbol([b'G', b'E', b'M', b'4', 0, 0, 0, 0]),
            side: '1',
            time: TransactTime(1524861082122000000) // todo,
            order_qty: Qty(7),
            order_type: OrderType::Limit,
            price: Price(99610),
        });

    }
}
