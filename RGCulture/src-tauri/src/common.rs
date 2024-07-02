// provides common data types, constants, and data
pub use crate::payloads::*;
pub use crate::protocol::{ProtVer,PROTVER_MATCHALL};
use std::error::Error;
use std::{alloc::Layout, fmt::{Debug, Display}};

pub use serde::{Serialize, Deserialize};
pub type TResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type TeamId = u8;
pub type PlayerId = u16;
pub type Dimension = u32;
pub type PMove = (Dimension, Dimension);
pub type TMove = (Dimension, Dimension, TeamId);



#[macro_export]
macro_rules! bytes {
    ($content:tt) => {
        &mut vec!$content[..]
    };
}

pub trait BitField {
    fn get_bit(&self, bit: usize) -> bool;
    fn set_bit(&mut self, bit: usize) -> ();
    fn clear_bit(&mut self, bit: usize) -> ();
}

pub fn byte_buf(size: usize) -> Box<[u8]> {
    unsafe {
        let raw_buf = std::slice::from_raw_parts_mut(std::alloc::alloc_zeroed(Layout::array::<u8>(size).unwrap()), size);
        let buf: Box<[u8]> = Box::from_raw(raw_buf);
        return buf;
    };
}

#[derive(Debug)]
pub struct VersionError {
    ver: ProtVer,
    reason: bool
}
impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Version {} too {}", self.ver, match self.reason{true=>"old".to_string(),false=>"young".to_string()}))
    }
}
impl Error for VersionError {}
impl VersionError {
    pub fn new(ver: ProtVer, reason: bool) -> Self {
        Self{ver, reason}
    }
    pub fn boxed(ver: ProtVer, reason: bool) -> Box<Self> {
        Box::new(Self::new(ver, reason))
    }
}

#[derive(Debug)]
pub struct GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
impl std::error::Error for GameError {}

impl GameError {
    pub fn new() -> Self {
        Self{}
    }
    pub fn boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

macro_rules! DISP {
    ($sn:ident) => {
        impl Display for $sn {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{:?}", self))
            }
        }
        impl std::error::Error for $sn {}
    };
}

#[derive(Debug)]
pub struct InvarError {}
DISP!(InvarError);
impl InvarError {
    pub fn new() -> Self{Self{}}
    pub fn boxed() -> Box<Self> {Box::new(Self::new())}
}

#[derive(Debug)]
pub struct ConnError {}
impl Display for ConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
impl std::error::Error for ConnError {}
impl ConnError {
    pub fn new() -> Self {Self{}}
    pub fn boxed() -> Box<Self> {Box::new(Self::new())}
}

#[derive(Debug)]
/// for when the user cancels something, not an actual error, just a way to represent a user cancelling an interaction
pub struct CancellationError {}
impl Display for CancellationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
impl std::error::Error for CancellationError {}
impl CancellationError {
    pub fn new() -> Self{Self{}}
    pub fn boxed() -> Box<Self> {Box::new(Self::new())}
}

#[derive(Debug)]
pub struct GenErr<T: Debug> {pub src: T}
impl<T: Debug> Display for GenErr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
impl<T: Debug> std::error::Error for GenErr<T> {}
impl<T: Debug> GenErr<T> {
    pub fn new(src: T) -> Self{Self{src}}
    pub fn boxed(src: T) -> Box<Self> {Box::new(Self::new(src))}
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct ItcError {
//     pub msg: String
// }

// #[derive(Debug)]
// pub enum ItcComm {
//     Error(ItcError),
//     Data(String),
//     Invalid
// }

// impl ItcComm {
//     pub fn to_string(&self) -> String {
//         match self {
//             Self::Error(e) => format!("ERROR{}", serde_json::to_string(e).unwrap()),
//             Self::Data(d) => format!("DATA{}", d),
//             Self::Invalid => "INVALID".to_owned()
//         }
//     }
//     pub fn from_string(string: String) -> Self {
//         if string.starts_with("ERROR") {
//             return Self::Error(serde_json::from_str(&string[5..]).unwrap());
//         }
//         if string.starts_with("DATA") {
//             return Self::Data((&string[4..]).to_owned());
//         }
//         if string.starts_with("OK") {
//             return Self::Data(String::new());
//         }
//         return Self::Invalid;
//     }
// }