// provides common data types, constants, and data

use std::alloc::Layout;

pub use serde::{Serialize, Deserialize};
pub type TResult<T> = Result<T, Box<dyn std::error::Error>>;

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
pub struct GameError {}

impl std::fmt::Display for GameError {
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
        Box::new(Self{})
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerData {
    pub has_password: bool,
    pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConnData<'a> {
    addr: &'a str,
    port: u16
}
impl<'a> ConnData<'a> {
    pub fn new(addr: &'a str, port: u16) -> ConnData<'a> {
        Self {addr,port}
    }
    pub fn to_str(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MoveUpdatePayload {
    x: u32,
    y: u32,
    team: u8
}

pub struct Threads {
    // threads: [std::thread::Thread;2],
    // inited: [bool;2],
    // locks: [Atomic]
}

pub struct GlobData {
    // pub threads: std::thread::Thread
    pub threads: Threads,
    pub value: u32
}

#[allow(non_upper_case_globals)]
static mut GLOBDATA_actual: GlobData = GlobData{value:0,threads:Threads{}};

pub fn get_globdata() -> &'static mut GlobData {
    unsafe {
        return &mut GLOBDATA_actual;
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItcError {
    pub msg: String
}

#[derive(Debug)]
pub enum ItcComm {
    Error(ItcError),
    Data(String),
    Invalid
}

impl ItcComm {
    pub fn to_string(&self) -> String {
        match self {
            Self::Error(e) => format!("ERROR{}", serde_json::to_string(e).unwrap()),
            Self::Data(d) => format!("DATA{}", d),
            Self::Invalid => "INVALID".to_owned()
        }
    }
    pub fn from_string(string: String) -> Self {
        if string.starts_with("ERROR") {
            return Self::Error(serde_json::from_str(&string[5..]).unwrap());
        }
        if string.starts_with("DATA") {
            return Self::Data((&string[4..]).to_owned());
        }
        if string.starts_with("OK") {
            return Self::Data(String::new());
        }
        return Self::Invalid;
    }
}