use super::*;
use quick_protobuf::sizeofs::*;
use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Result, Writer};
use std::io::Write;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumTestLiteRuntime {
    ONE = 1,
    TWO = 2,
}

impl Default for EnumTestLiteRuntime {
    fn default() -> Self {
        EnumTestLiteRuntime::ONE
    }
}

impl From<i32> for EnumTestLiteRuntime {
    fn from(i: i32) -> Self {
        match i {
            1 => EnumTestLiteRuntime::ONE,
            2 => EnumTestLiteRuntime::TWO,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EnumTestLiteRuntime {
    fn from(s: &'a str) -> Self {
        match s {
            "ONE" => EnumTestLiteRuntime::ONE,
            "TWO" => EnumTestLiteRuntime::TWO,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TestLiteRuntime {
    pub v: Option<i32>,
}

impl<'a> MessageRead<'a> for TestLiteRuntime {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.v = Some(r.read_int32(bytes)?),
                Ok(t) => {
                    r.read_unknown(bytes, t)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TestLiteRuntime {
    fn get_size(&self) -> usize {
        0 + self
            .v
            .as_ref()
            .map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.v {
            w.write_with_tag(8, |w| w.write_int32(*s))?;
        }
        Ok(())
    }
}
