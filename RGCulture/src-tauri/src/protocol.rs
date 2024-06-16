use std::{net::TcpStream, slice::{from_mut as slice_of_mut, from_ref as slice_of}};

use serde::Serialize;

use crate::{bytes, common::*};
use std::io::{Read, Write};

pub const PROTVER_MATCHALL: u16 = 65000;

const PVER_V001: ProtVer = ProtVer(0, 0, 1);
const PVER_V011: ProtVer = ProtVer(0, 1, 1);

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ProtVer(pub u16, pub u16, pub u16);
impl From<(u16, u16, u16)> for ProtVer {fn from(value: (u16, u16, u16)) -> Self {Self(value.0,value.1,value.2)}}
impl PartialEq for ProtVer {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == PROTVER_MATCHALL || other.0 == PROTVER_MATCHALL || self.0 == other.0) && (self.1 == PROTVER_MATCHALL || other.1 == PROTVER_MATCHALL || self.1 == other.1) && (self.2 == PROTVER_MATCHALL || other.2 == PROTVER_MATCHALL || self.2 == other.2)
    }
}
impl PartialOrd for ProtVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.2.partial_cmp(&other.2)
    }
}

pub struct Protocol();
impl Protocol {
    pub fn read_byte(stream: &mut TcpStream) -> TResult<u8> {
        let mut byte: u8 = 0;
        stream.read_exact(slice_of_mut(&mut byte))?;
        Ok(byte)
    }
    pub fn read_n_bytes(n: usize, stream: &mut TcpStream) -> TResult<Box<[u8]>> {
        let mut buf = byte_buf(n);
        stream.read_exact(buf.as_mut())?;
        Ok(buf)
    }
    pub fn read_u16(stream: &mut TcpStream) -> TResult<u16> {
        let buf = &mut [0;2];
        stream.read_exact(buf)?;
        Ok((buf[0] as u16) << 8 | buf[1] as u16)
    }
    pub fn read_u32(stream: &mut TcpStream) -> TResult<u32> {
        let buf = &mut [0;4];
        stream.read_exact(buf)?;
        Ok(((buf[0] as u32) << 24) | ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32))
    }
    pub fn write_byte(byte: u8, stream: &mut TcpStream) -> TResult<()> {
        Ok(stream.write_all(slice_of(&byte))?)
    }
    pub fn write_u16(n: u16, stream: &mut TcpStream) -> TResult<()> {
        Ok(stream.write_all(bytes![[(n>>8) as u8, (n&0xff) as u8]])?)
    }
    pub fn write_u32(n: u32, stream: &mut TcpStream) -> TResult<()> {
        Ok(stream.write_all(bytes![[(n>>24) as u8, ((n>>16)&0xff) as u8, ((n>>8)&0xff) as u8, (n&0xff) as u8]])?)
    }
    pub fn read_team(stream: &mut TcpStream) -> TResult<TeamId> {
        Self::read_byte(stream)
    }
    pub fn read_player_id(pver: ProtVer, stream: &mut TcpStream) -> TResult<PlayerId> {
        if pver <= PVER_V011 {
            return Ok(Self::read_byte(stream)? as PlayerId);
        } else {
            return Ok(Self::read_u16(stream)? as PlayerId);
        }
    }
    pub fn read_dimension(pver: ProtVer, stream: &mut TcpStream) -> TResult<Dimension> {
        if pver <= PVER_V011 {
            return Ok(Self::read_byte(stream)? as Dimension);
        } else {
            return Ok(Self::read_u32(stream)? as Dimension);
        }
    }
    pub fn write_dimension(dim: Dimension, pver: ProtVer, stream: &mut TcpStream) -> TResult<()> {
        if pver <= PVER_V011 {
            return Self::write_byte(dim as u8, stream);
        } else {
            return Self::write_u32(dim as u32, stream);
        }
    }
    pub fn read_board_transfer(pver: ProtVer, stream: &mut TcpStream) -> TResult<BoardInfo> {
        let w = Self::read_dimension(pver, stream)?;
        let h = Self::read_dimension(pver, stream)?;
        let mut scores = [0u32; 6];
        for i in 0..6 {
            scores[i] = Self::read_u32(stream)?;
        }
        let len = (w as usize) * (h as usize);
        let mut bytes = byte_buf(len * 2);
        for i in 0..len {
            bytes[i*2] = Self::read_byte(stream)?;
            bytes[i*2 + 1] = Self::read_team(stream)?;
        }
        return Ok(BoardInfo{w, h, tiles:bytes, scores});
    }
    pub fn get_is_move(stream: &mut TcpStream) -> TResult<bool> {
        return Ok(Self::read_byte(stream)? == 1);
    }
    /// sends a move to the host, returns Ok(true) if it was valid
    pub fn send_move(pmove: PMove, pver: ProtVer, stream: &mut TcpStream) -> TResult<bool> {
        Self::write_dimension(pmove.0, pver, stream)?;
        Self::write_dimension(pmove.1, pver, stream)?;
        return Ok(Self::read_byte(stream)? == 1);
    }
    pub fn read_move(pver: ProtVer, stream: &mut TcpStream) -> TResult<TMove> {
        return Ok((Self::read_dimension(pver, stream)?, Self::read_dimension(pver, stream)?, Self::read_team(stream)?));
    }
}
