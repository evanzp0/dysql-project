use std::{fmt, borrow::Cow, collections::HashMap, hash::BuildHasher, time::SystemTime, net::IpAddr};

pub trait ToSql: fmt::Debug { }

impl<'a, T> ToSql for &'a T
where
    T: ToSql
{ }

impl<T: ToSql> ToSql for Option<T> { }

impl<'a, T: ToSql> ToSql for &'a [T] { }

impl<'a> ToSql for &'a [u8] { }

impl<T: ToSql, const N: usize> ToSql for [T; N] { }

impl<T: ToSql> ToSql for Vec<T> { }

impl<T: ToSql> ToSql for Box<[T]> { }

impl ToSql for Vec<u8> { }

impl<'a> ToSql for &'a str { }

impl<'a> ToSql for Cow<'a, str> { }

impl ToSql for String { }

impl ToSql for Box<str> { }

impl ToSql for bool { }
impl ToSql for i8 { }
impl ToSql for i16 { }
impl ToSql for i32 { }
impl ToSql for u32 { }
impl ToSql for i64 { }
impl ToSql for f32 { }
impl ToSql for f64 { }

impl<H> ToSql for HashMap<String, Option<String>, H>
where
    H: BuildHasher,
{ }

impl ToSql for SystemTime { }

impl ToSql for IpAddr { }