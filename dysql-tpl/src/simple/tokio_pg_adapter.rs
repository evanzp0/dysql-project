use bytes::BytesMut;
use postgres_types::{ToSql, to_sql_checked};

use super::{RawStr, RawString};

impl ToSql for RawStr {
    fn to_sql(&self, ty: &postgres_types::Type, w: &mut BytesMut) 
        -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized 
    {
        let rf = unsafe { &*self.0 };
        // <&str as ToSql>::to_sql(&self.0.as_ref(), ty, w)
        <&str as ToSql>::to_sql(&rf.as_ref(), ty, w)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized 
    {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

impl ToSql for RawString {
    fn to_sql(&self, ty: &postgres_types::Type, w: &mut BytesMut) 
        -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized 
    {
        let rf = unsafe { &*self.0 };
        // <&str as ToSql>::to_sql(&self.0.as_ref(), ty, w)
        <&str as ToSql>::to_sql(&rf.as_ref(), ty, w)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized 
    {
        <&str as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}