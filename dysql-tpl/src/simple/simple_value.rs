
use chrono::{NaiveDateTime, DateTime, Utc, Local, FixedOffset};
use paste::paste;
use uuid::Uuid;

use super::SimpleError;
use super::SimpleInnerError;

#[derive(Debug)]
pub struct RawStr(pub * const str);

unsafe impl Send for RawStr {}
unsafe impl Sync for RawStr {}

impl RawStr {
    /// 指针转字符串切片
    pub fn as_str(&self) -> Result<&str, SimpleError> {
        let val = self.0;
        let val: &str = unsafe {&*val};
        Ok(val)
    }
}

#[derive(Debug)]
pub struct RawString(pub * const String);

unsafe impl Send for RawString {}
unsafe impl Sync for RawString {}

impl RawString {
    /// 指针转字符串引用
    pub fn as_string(&self) -> Result<&String, SimpleError> {
        let val = self.0;
        let val: &String = unsafe {&*val};
        Ok(val)
    }
}

macro_rules! impl_simple_value_varaint {
    (
        $($vtype: ty),*
    ) => {
        paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum SimpleValue {
                $(
                    [<t_ $vtype>]($vtype),
                )*
                t_str(RawStr),
                t_String(RawString),
                t_Utc(DateTime<Utc>),
                t_DateTime_Local(DateTime<Local>),
                t_DateTime_FixedOffset(DateTime<FixedOffset>),
                None(Option<i32>),
            }
        }
    }
}

impl_simple_value_varaint!(usize, isize, i64, u64, i32, u32, i16, u16, i8, u8, i128, u128, f32, f64, bool, char, Uuid, NaiveDateTime);

impl SimpleValue {
    /// 指针转字符串切片
    pub fn as_str(&self) -> Result<&str, SimpleError> {
        if let SimpleValue::t_str(val) = self {
            let val = val.0;
            let val: &str = unsafe {&*val};
            Ok(val)
        } else {
            Err(SimpleInnerError(format!("value: '{:?}' convert to &str failed", self)).into())
        }
    }

    /// 指针转字符串引用
    pub fn as_string(&self) -> Result<&String, SimpleError> {
        if let SimpleValue::t_String(val) = self {
            let val = val.0;
            let val: &String = unsafe {&*val};
            Ok(val)
        } else {
            Err(SimpleInnerError(format!("value: '{:?}' convert to &String failed", self)).into())
        }
    }
}