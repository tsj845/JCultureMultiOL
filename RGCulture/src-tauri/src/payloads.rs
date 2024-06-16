use serde::{Serialize, Deserialize};

use crate::{protocol::ProtVer, common::*};

#[derive(Clone, Serialize, Deserialize)]
pub struct UIPlayerMovePayload {
    pub x: Dimension,
    pub y: Dimension
}

#[derive(Clone, Deserialize)]
pub struct PasswordPayload {
    pub canceled: bool,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct BoardInfo {
    pub w: Dimension,
    pub h: Dimension,
    pub tiles: Box<[u8]>,
    pub scores: [u32; 6]
}

#[derive(Clone, Serialize)]
pub struct BoardDims {
    pub w: Dimension,
    pub h: Dimension
}

#[derive(Clone, Serialize)]
pub struct PlayerInfo {
    pub name: String,
    pub pid: PlayerId,
    pub tid: TeamId
}

#[derive(Clone, Serialize)]
pub struct MoveInfo {
    pub x: Dimension,
    pub y: Dimension,
    pub team: TeamId
}

#[derive(Clone, Serialize, Debug)]
pub struct ServerData {
    pub has_password: bool,
    pub name: String,
    pub version: ProtVer
}

#[derive(Clone, Deserialize)]
pub struct JoinData {
    pub cdat: ConnData,
    pub name: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConnData {
    pub addr: String,
    pub port: u16
}
impl ConnData {
    pub fn new(addr: String, port: u16) -> ConnData {
        Self {addr,port}
    }
    pub fn to_str(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MoveUpdatePayload {
    pub x: Dimension,
    pub y: Dimension,
    pub team: TeamId
}