use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;

pub enum SimpleValue {
    Usize(usize),
    I64(i64),
    U64(u64),
    I32(i32),
    U32(u32),
    I16(i16),
    U16(u16),
    I8(i8),
    U8(u8),
    Bool(bool),
    Str(String),
    Char(char),
    Uuid(uuid::Uuid),
    NaiveDateTime(NaiveDateTime),
    Utc(DateTime<Utc>),
    OptionUsize(Option<usize>),
    OptionI64(Option<i64>),
    OptionU64(Option<u64>),
    OptionI32(Option<i32>),
    OptionU32(Option<u32>),
    OptionI16(Option<i16>),
    OptionU16(Option<u16>),
    OptionI8(Option<i8>),
    OptionU8(Option<u8>),
    OptionBool(Option<bool>),
    OptionStr(Option<String>),
    OptionChar(Option<char>),
    OptionUuid(Option<Uuid>),
    OptionNaiveDateTime(Option<NaiveDateTime>),
    OptionUtc(Option<DateTime<Utc>>),
}