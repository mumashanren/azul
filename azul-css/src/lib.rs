//! Provides datatypes used to describe an application's style using the Azul GUI framework.

use std::fmt;

#[macro_export]
macro_rules! impl_vec {($struct_type:ident, $struct_name:ident) => (

    #[repr(C)]
    pub struct $struct_name {
        ptr: *mut $struct_type,
        len: usize,
        cap: usize,
    }

    impl $struct_name {

        pub fn new() -> Self {
            Vec::<$struct_type>::new().into()
        }

        pub fn clear(&mut self) {
            let mut v: Vec<$struct_type> = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
            v.clear();
            std::mem::forget(v);
        }

        pub fn sort_by<F: FnMut(&$struct_type, &$struct_type) -> std::cmp::Ordering>(&mut self, compare: F) {
            let v1: &mut [$struct_type] = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };
            v1.sort_by(compare);
        }

        pub fn with_capacity(cap: usize) -> Self {
            Vec::<$struct_type>::with_capacity(cap).into()
        }

        pub fn push(&mut self, val: $struct_type) {
            let mut v: Vec<$struct_type> = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
            v.push(val);
            std::mem::forget(v);
        }

        pub fn iter(&self) -> std::slice::Iter<$struct_type> {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            v1.iter()
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<$struct_type> {
            let v1: &mut [$struct_type] = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };
            v1.iter_mut()
        }

        pub fn into_iter(self) -> std::vec::IntoIter<$struct_type> {
            let v1: Vec<$struct_type> = unsafe { std::vec::Vec::from_raw_parts(self.ptr, self.len, self.cap) };
            println!("Vec::<{}>::drop!", stringify!($struct_type));
            std::mem::forget(self); // do not run destructor of self
            v1.into_iter()
        }

        pub fn as_ptr(&self) -> *const $struct_type {
            self.ptr as *const $struct_type
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        pub fn cap(&self) -> usize {
            self.cap
        }

        pub fn get(&self, index: usize) -> Option<&$struct_type> {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let res = v1.get(index);
            res
        }

        pub fn foreach<U, F: FnMut(&$struct_type) -> Result<(), U>>(&self, mut closure: F) -> Result<(), U> {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            for i in v1.iter() { closure(i)?; }
            Ok(())
        }

        /// Same as Vec::into_raw_parts(self), prevents destructor from running
        fn into_raw_parts(mut v: Vec<$struct_type>) -> (*mut $struct_type, usize, usize) {
            let ptr = v.as_mut_ptr();
            let len = v.len();
            let cap = v.capacity();
            std::mem::forget(v);
            (ptr, len, cap)
        }
    }

    impl Default for $struct_name {
        fn default() -> Self {
            Vec::<$struct_type>::default().into()
        }
    }

    impl std::iter::FromIterator<$struct_type> for $struct_name {
        fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = $struct_type> {
            let v: Vec<$struct_type> = Vec::from_iter(iter);
            println!("Vec::<{}>::new!", stringify!($struct_type));
            v.into()
        }
    }

    impl AsRef<[$struct_type]> for $struct_name {
        fn as_ref(&self) -> &[$struct_type] {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    impl From<Vec<$struct_type>> for $struct_name {
        fn from(input: Vec<$struct_type>) -> $struct_name {
            let (ptr, len, cap) = $struct_name::into_raw_parts(input);
            println!("Vec::<{}>::new!", stringify!($struct_type));
            $struct_name { ptr, len, cap }
        }
    }

    impl From<$struct_name> for Vec<$struct_type> {
        fn from(input: $struct_name) -> Vec<$struct_type> {
            let v = unsafe { Vec::from_raw_parts(input.ptr, input.len, input.cap) };
            println!("Vec::<{}>::drop!", stringify!($struct_type));
            std::mem::forget(input); // don't run the destructor of "input"
            v
        }
    }

    impl Drop for $struct_name {
        fn drop(&mut self) {
            println!("Vec::<{}>::drop!", stringify!($struct_name));
            let _v: Vec<$struct_type> = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
            // let v drop here
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_as_hashmap {($struct_type:ident, $struct_name:ident) => (
    impl $struct_name {
        pub fn insert_hm_item(&mut self, item: $struct_type) {
            if !self.contains_hm_item(&item) {
                self.push(item);
            }
        }

        pub fn contains_hm_item(&self, searched: &$struct_type) -> bool {
            let v1: &mut [$struct_type] = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };
            v1.iter().any(|i| i == searched)
        }

        pub fn remove_hm_item(&mut self, remove_key: &$struct_type) {
            let mut v: Vec<$struct_type> = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
            v.retain(|v| v == remove_key);
            std::mem::forget(v);
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_debug {($struct_type:ident, $struct_name:ident) => (
    impl std::fmt::Debug for $struct_name {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let res = v1.fmt(f);
            res
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_partialord {($struct_type:ident, $struct_name:ident) => (
    impl PartialOrd for $struct_name {
        fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let v2: &[$struct_type] = unsafe { std::slice::from_raw_parts(rhs.ptr, rhs.len) };
            v1.partial_cmp(&v2)
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_ord {($struct_type:ident, $struct_name:ident) => (
    impl Ord for $struct_name {
        fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let v2: &[$struct_type] = unsafe { std::slice::from_raw_parts(rhs.ptr, rhs.len) };
            v1.cmp(&v2)
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_clone {($struct_type:ident, $struct_name:ident) => (
    impl Clone for $struct_name {
        fn clone(&self) -> Self {
            let v: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let v2 = v.to_vec();
            let (ptr, len, cap) = $struct_name::into_raw_parts(v2);
            $struct_name { ptr, len, cap }
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_partialeq {($struct_type:ident, $struct_name:ident) => (
    impl PartialEq for $struct_name {
        fn eq(&self, other: &Self) -> bool {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            let v2: &[$struct_type] = unsafe { std::slice::from_raw_parts(other.ptr, other.len) };
            v1.eq(v2)
        }
    }
)}

#[macro_export]
macro_rules! impl_vec_eq {($struct_type:ident, $struct_name:ident) => (
    impl Eq for $struct_name { }
)}

#[macro_export]
macro_rules! impl_vec_hash {($struct_type:ident, $struct_name:ident) => (
    impl std::hash::Hash for $struct_name {
        fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
            let v1: &[$struct_type] = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
            v1.hash(state);
        }
    }
)}

#[macro_export]
macro_rules! impl_option_inner {
    ($struct_type:ident, $struct_name:ident) => (

    impl From<$struct_name> for Option<$struct_type> {
        fn from(o: $struct_name) -> Option<$struct_type> {
            match o {
                $struct_name::None => None,
                $struct_name::Some(t) => Some(t),
            }
        }
    }

    impl From<Option<$struct_type>> for $struct_name {
        fn from(o: Option<$struct_type>) -> $struct_name {
            match o {
                None => $struct_name::None,
                Some(t) => $struct_name::Some(t),
            }
        }
    }

    impl Default for $struct_name {
        fn default() -> $struct_name { $struct_name::None }
    }

    impl $struct_name {
        pub fn as_option(&self) -> Option<&$struct_type> {
            match self {
                $struct_name::None => None,
                $struct_name::Some(t) => Some(t),
            }
        }
        pub fn is_some(&self) -> bool {
            match self {
                $struct_name::None => false,
                $struct_name::Some(_) => true,
            }
        }
        pub fn is_none(&self) -> bool {
            !self.is_some()
        }
    }
)}

#[macro_export]
macro_rules! impl_option {
    ($struct_type:ident, $struct_name:ident, copy = false, clone = false, [$($derive:meta),* ]) => (
        $(#[derive($derive)])*
        #[repr(C, u8)]
        pub enum $struct_name {
            None,
            Some($struct_type)
        }

        impl $struct_name {
            pub fn into_option(self) -> Option<$struct_type> {
                match self {
                    $struct_name::None => None,
                    $struct_name::Some(t) => Some(t),
                }
            }
        }

        impl_option_inner!($struct_type, $struct_name);
    );
    ($struct_type:ident, $struct_name:ident, copy = false, [$($derive:meta),* ]) => (
        $(#[derive($derive)])*
        #[repr(C, u8)]
        pub enum $struct_name {
            None,
            Some($struct_type)
        }

        impl $struct_name {
            pub fn into_option(&self) -> Option<$struct_type> {
                match self {
                    $struct_name::None => None,
                    $struct_name::Some(t) => Some(t.clone()),
                }
            }
        }

        impl_option_inner!($struct_type, $struct_name);
    );
    ($struct_type:ident, $struct_name:ident, [$($derive:meta),* ]) => (
        $(#[derive($derive)])*
        #[repr(C, u8)]
        pub enum $struct_name {
            None,
            Some($struct_type)
        }

        impl $struct_name {
            pub fn into_option(&self) -> Option<$struct_type> {
                match self {
                    $struct_name::None => None,
                    $struct_name::Some(t) => Some(*t),
                }
            }
        }

        impl_option_inner!($struct_type, $struct_name);
    );
}

#[macro_export]
macro_rules! impl_result_inner {
    ($ok_struct_type:ident, $err_struct_type:ident, $struct_name:ident) => (

    impl From<$struct_name> for Result<$ok_struct_type, $err_struct_type> {
        fn from(o: $struct_name) -> Result<$ok_struct_type, $err_struct_type> {
            match o {
                $struct_name::Ok(o) => Ok(o),
                $struct_name::Err(e) => Err(e),
            }
        }
    }

    impl From<Result<$ok_struct_type, $err_struct_type>> for $struct_name {
        fn from(o: Result<$ok_struct_type, $err_struct_type>) -> $struct_name {
            match o {
                Ok(o) => $struct_name::Ok(o),
                Err(e) => $struct_name::Err(e),
            }
        }
    }

    impl $struct_name {
        pub fn as_result(&self) -> Result<&$ok_struct_type, &$err_struct_type> {
            match self {
                $struct_name::Ok(o) => Ok(o),
                $struct_name::Err(e) => Err(e),
            }
        }
    }
)}

#[macro_export]
macro_rules! impl_result {
    ($ok_struct_type:ident, $err_struct_type:ident, $struct_name:ident, copy = false, clone = false) => (
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C, u8)]
        pub enum $struct_name {
            Ok($ok_struct_type),
            Err($err_struct_type),
        }

        impl $struct_name {
            pub fn into_result(self) -> Result<$ok_struct_type, $err_struct_type> {
                match self {
                    $struct_name::Ok(o) => Ok(o),
                    $struct_name::Err(e) => Err(e),
                }
            }
        }

        impl_result_inner!($ok_struct_type, $err_struct_type, $struct_name);
    );
    ($ok_struct_type:ident, $err_struct_type:ident, $struct_name:ident, copy = false) => (
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C, u8)]
        pub enum $struct_name {
            Ok($ok_struct_type),
            Err($err_struct_type),
        }

        impl $struct_name {
            pub fn into_result(&self) -> Result<$ok_struct_type, $err_struct_type> {
                match self {
                    $struct_name::Ok(o) => Ok(o.clone()),
                    $struct_name::Err(e) => Err(e.clone()),
                }
            }
        }

        impl_result_inner!($ok_struct_type, $err_struct_type, $struct_name);
    );
    ($ok_struct_type:ident, $err_struct_type:ident, $struct_name:ident) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C, u8)]
        pub enum $struct_name {
            Ok($ok_struct_type),
            Err($err_struct_type),
        }

        impl $struct_name {
            pub fn into_result(&self) -> Result<$ok_struct_type, $err_struct_type> {
                match self {
                    $struct_name::Ok(o) => Ok(*o),
                    $struct_name::Err(e) => Err(*e),
                }
            }
        }

        impl_result_inner!($ok_struct_type, $err_struct_type, $struct_name);
    );
}

#[repr(C)]
pub struct AzString { vec: U8Vec }

impl AsRef<str> for AzString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for AzString {
    fn default() -> Self {
        String::new().into()
    }
}

impl fmt::Debug for AzString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Display for AzString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AzString {
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) }
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.vec.as_ref()
    }
    #[inline]
    pub fn into_string(self) -> String {
        String::from(self)
    }
    #[inline]
    pub fn into_bytes(self) -> U8Vec {
        let self_vec = U8Vec { ptr: self.vec.ptr, len: self.vec.len, cap: self.vec.cap };
        std::mem::forget(self); // don't run destructor
        self_vec
    }
}

impl From<AzString> for String {
    fn from(input: AzString) -> String {
        let s = unsafe { String::from_raw_parts(input.vec.ptr, input.vec.len, input.vec.cap) };
        std::mem::forget(input);
        s
    }
}

impl From<String> for AzString {
    fn from(mut input: String) -> AzString {
        let ptr = input.as_mut_ptr();
        let len = input.len();
        let cap = input.capacity();
        std::mem::forget(input);
        AzString { vec: U8Vec { ptr, len, cap } }
    }
}

impl PartialOrd for AzString {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        let v1: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) };
        let v2: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(rhs.vec.ptr, rhs.vec.len)) };
        v1.partial_cmp(&v2)
    }
}

impl Ord for AzString {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        let v1: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) };
        let v2: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(rhs.vec.ptr, rhs.vec.len)) };
        v1.cmp(&v2)
    }
}

impl Clone for AzString {
    fn clone(&self) -> Self {
        let v: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) };
        let mut v2 = v.to_owned();

        let ptr = v2.as_mut_ptr();
        let len = v2.len();
        let cap = v2.capacity();
        std::mem::forget(v2);

        AzString { vec: U8Vec { ptr, len, cap } }
    }
}

impl PartialEq for AzString {
    fn eq(&self, other: &Self) -> bool {
        let v1: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) };
        let v2: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(other.vec.ptr, other.vec.len)) };
        v1.eq(v2)
    }
}

impl Eq for AzString { }

impl std::hash::Hash for AzString {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        let v1: &str = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.vec.ptr, self.vec.len)) };
        v1.hash(state)
    }
}

impl Drop for AzString {
    fn drop(&mut self) {
        // NOTE: dropping self.vec would lead to a double-free,
        // since U8Vec::drop() is automatically called here
    }
}

impl_vec!(u8, U8Vec);
impl_vec_debug!(u8, U8Vec);
impl_vec_partialord!(u8, U8Vec);
impl_vec_ord!(u8, U8Vec);
impl_vec_clone!(u8, U8Vec);
impl_vec_partialeq!(u8, U8Vec);
impl_vec_eq!(u8, U8Vec);
impl_vec_hash!(u8, U8Vec);

impl_vec!(AzString, StringVec);
impl_vec_debug!(AzString, StringVec);
impl_vec_partialord!(AzString, StringVec);
impl_vec_ord!(AzString, StringVec);
impl_vec_clone!(AzString, StringVec);
impl_vec_partialeq!(AzString, StringVec);
impl_vec_eq!(AzString, StringVec);
impl_vec_hash!(AzString, StringVec);

impl_vec!(GradientStopPre, GradientStopPreVec);
impl_vec_debug!(GradientStopPre, GradientStopPreVec);
impl_vec_partialord!(GradientStopPre, GradientStopPreVec);
impl_vec_ord!(GradientStopPre, GradientStopPreVec);
impl_vec_clone!(GradientStopPre, GradientStopPreVec);
impl_vec_partialeq!(GradientStopPre, GradientStopPreVec);
impl_vec_eq!(GradientStopPre, GradientStopPreVec);
impl_vec_hash!(GradientStopPre, GradientStopPreVec);

impl From<Vec<String>> for StringVec {
    fn from(v: Vec<String>) -> StringVec {
        let new_v: Vec<AzString> = v.into_iter().map(|s| s.into()).collect();
        new_v.into()
    }
}

impl From<StringVec> for Vec<String> {
    fn from(v: StringVec) -> Vec<String> {
        let v: Vec<AzString> = v.into();
        v.into_iter().map(|s| s.into()).collect()
    }
}

mod css;
mod css_properties;

pub use crate::css::*;
pub use crate::css_properties::*;
