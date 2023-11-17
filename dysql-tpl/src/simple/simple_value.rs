use paste::paste;
use chrono::NaiveDateTime;
use chrono::DateTime;
use uuid::Uuid;
use chrono::Utc;

macro_rules! impl_simple_value_varaint {
    (
        $($vtype: ty),*
    ) => {
        paste! {
            #[allow(non_camel_case_types)]
            pub enum SimpleValue<'a> {
                $(
                    [<t_ $vtype>]($vtype),
                    [<option_ $vtype>](Option<$vtype>),
                )*
                t_str(&'a str),
                option_str(Option<&'a str>),
                t_Uuid(Uuid),
                option_Uuid(Option<Uuid>),
                t_NaiveDateTime(NaiveDateTime),
                option_NaiveDateTime(Option<NaiveDateTime>),
                t_Utc(DateTime<Utc>),
                option_Utc(Option<DateTime<Utc>>),
            }
        }
    }
}

impl_simple_value_varaint!(usize, i64, u64, i32, u32, i16, u16, i8, u8, bool, char);

// pub enum SimpleValue<'a> {
//     Usize(usize),
//     I64(i64),
//     U64(u64),
//     I32(i32),
//     U32(u32),
//     I16(i16),
//     U16(u16),
//     I8(i8),
//     U8(u8),
//     Bool(bool),
//     Str(&'a str),
//     Char(char),
//     Uuid(uuid::Uuid),
//     NaiveDateTime(NaiveDateTime),
//     Utc(DateTime<Utc>),
//     OptionUsize(Option<usize>),
//     OptionI64(Option<i64>),
//     OptionU64(Option<u64>),
//     OptionI32(Option<i32>),
//     OptionU32(Option<u32>),
//     OptionI16(Option<i16>),
//     OptionU16(Option<u16>),
//     OptionI8(Option<i8>),
//     OptionU8(Option<u8>),
//     OptionBool(Option<bool>),
//     OptionStr(Option<String>),
//     OptionChar(Option<char>),
//     OptionUuid(Option<Uuid>),
//     OptionNaiveDateTime(Option<NaiveDateTime>),
//     OptionUtc(Option<DateTime<Utc>>),
// }