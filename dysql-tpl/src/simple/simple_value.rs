
use paste::paste;
use chrono::NaiveDateTime;
use chrono::DateTime;
use uuid::Uuid;
use chrono::Utc;

use super::SimpleError;
use super::SimpleInnerError;

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
                    [<option_ $vtype>](Option<$vtype>),
                )*
                t_str(*const str),
                // option_str(Option<*const str>),
                t_String(*const String),
                // option_String(Option<*const String>),
                t_Utc(DateTime<Utc>),
                // option_Utc(Option<DateTime<Utc>>),
                Null(Option<()>),
                // t_unknown,
            }
        }
    }
}

impl_simple_value_varaint!(usize, isize, i64, u64, i32, u32, i16, u16, i8, u8, i128, u128, f32, f64, bool, char, Uuid, NaiveDateTime);

impl SimpleValue {
    /// 指针转字符串切片
    pub fn as_str(&self) -> Result<&str, SimpleError> {
        if let SimpleValue::t_str(val) = self {
            let val: &str = unsafe {&**val};
            Ok(val)
        } else {
            Err(SimpleInnerError(format!("value: '{:?}' convert to &str failed", self)).into())
        }
    }

    /// 指针转字符串引用
    pub fn as_string(&self) -> Result<&String, SimpleError> {
        if let SimpleValue::t_String(val) = self {
            let val: &String = unsafe {&**val};
            Ok(val)
        } else {
            Err(SimpleInnerError(format!("value: '{:?}' convert to &String failed", self)).into())
        }
    }

    // pub fn as_option_str(&self) -> Result<Option<&str>, SimpleError> {
    //     if let SimpleValue::option_str(val) = self {
    //         let val = val.map(|t| unsafe { (&*t) as &str });
    //         Ok(val)
    //     } else {
    //         Err(SimpleInnerError(format!("value: '{:?}' convert to Option<&str> failed", self)).into())
    //     }
    // }

    // pub fn as_option_string(&self) -> Result<Option<&String>, SimpleError> {
    //     if let SimpleValue::option_String(val) = self {
    //         let val = val.map(|t| unsafe { (&*t) as &String });
    //         Ok(val)
    //     } else {
    //         Err(SimpleInnerError(format!("value: '{:?}' convert to Option<&String> failed", self)).into())
    //     }
    // }
}