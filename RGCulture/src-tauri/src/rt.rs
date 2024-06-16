// receive/transmit using the JCulture Protocol
use std::net::TcpStream;
use std::net::Shutdown::Both;
use std::ops::{BitAnd, BitAndAssign, BitOrAssign};
use crate::common::*;
use crate::protocol::Protocol;
use std::io;
use std::io::{Read, Write};

macro_rules! rt_closure {
    ($stream:ident, $rty:ty, $tree:tt) => {
        |$stream:&mut TcpStream|->TResult<$rty>{$tree}
    };
    ($stream:expr, $rty:ty, $tree:tt) => {
        |$stream:&mut TcpStream|->TResult<$rty>{$tree}
    };
}
macro_rules! shutdown {
    ($stream:ident) => {
        println!("SHUTDOWN MACRO");
        let _=$stream.shutdown(Both);
    };
    ($stream1:ident, $stream2:ident) => {
        println!("SHUTDOWN MACRO");
        let _=$stream1.shutdown(Both);
        let _=$stream2.shutdown(Both);
    };
    ($stream:expr) => {
        println!("SHUTDOWN MACRO");
        let _=$stream.shutdown(Both);
    };
    ($stream1:expr, $stream2:expr) => {
        println!("SHUTDOWN MACRO");
        let _=$stream1.shutdown(Both);
        let _=$stream2.shutdown(Both);
    };
}

// type SRes<T> = Result<T, Box<GenErr<ErrCause>>>;
type SRes<T> = TResult<T>;

const SOCKETS_BARFLAG_SIZE: usize = 2;
const SOCKETS_SEQFLAG_SIZE: usize = 2;
pub struct Connection {
    sid: u16, // SID, Socket ID, used in S2 Handshake
    playid: PlayerId,
    teamid: TeamId,
    gamedata: TcpStream, // Socket 1, carries gamedata
    commdata: TcpStream, // Socket 2, carries overhead
    // ensure that certain actions are not repeated (eg. have one-way barriers)
    barrier_flags: [u8;SOCKETS_BARFLAG_SIZE],
    // ensure that certain actions are done in the correct sequence
    sequence_flags: [u8;SOCKETS_BARFLAG_SIZE],
    host_prot_ver: ProtVer
}

impl BitField for u8 {
    fn get_bit(&self, bit: usize) -> bool {
        if bit >= 8 {
            panic!("index out of range!");
        }
        return self.bitand(1<<bit)!=0;
    }

    fn set_bit(&mut self, bit: usize) -> () {
        if bit >= 8 {
            panic!("index out of range!");
        }
        self.bitor_assign(1<<bit);
    }

    fn clear_bit(&mut self, bit: usize) -> () {
        if bit >= 8 {
            panic!("index out of range!");
        }
        self.bitand_assign(!(1<<bit));
    }
}

impl BitField for [u8] {
    fn get_bit(&self, bit: usize) -> bool {
        if self.len() <= bit {
            panic!("index out of range!");
        }
        return self[bit/8].get_bit(bit%8);
    }

    fn set_bit(&mut self, bit: usize) -> () {
        if self.len() <= bit {
            panic!("index out of range!");
        }
        self[bit/8].set_bit(bit%8);
    }

    fn clear_bit(&mut self, bit: usize) -> () {
        if self.len() <= bit {
            panic!("index out of range!");
        }
        self[bit/8].clear_bit(bit%8);
    }
}

#[allow(non_camel_case_types)]
enum SOCKETS_BARRIER_IDS {
    S2HANDSHAKE = 0,
    GOTTEAMDATA
}

#[allow(non_camel_case_types)]
enum SOCKETS_SEQUENCE_IDS {
    S1HANDSHAKE = 0
}

#[allow(non_camel_case_types)]
type SOCKETS_SEQUENCE_VALUE = u8;
enum S1SEQ {
    POSTS2SHAKE = 0,
    POSTPWCHECK,
    POSTPWREJECT,
    POSTPWACCEPT,
    POSTNAMESENT,
    POSTTEAMGOT
}

#[derive(Debug)]
pub enum ErrCause {
    IO,
    SEQUENCE
}

impl Connection {
    fn get_barrier(&self, barrier_id: SOCKETS_BARRIER_IDS) -> bool {
        self.barrier_flags.get_bit(barrier_id as usize)
    }
    fn set_barrier(&mut self, barrier_id: SOCKETS_BARRIER_IDS) -> () {
        self.barrier_flags.set_bit(barrier_id as usize);
    }
    fn get_sequence(&self, sequence_id: SOCKETS_SEQUENCE_IDS) -> SOCKETS_SEQUENCE_VALUE {
        return self.sequence_flags[sequence_id as usize];
    }
    fn set_sequence(&mut self, sequence_id: SOCKETS_SEQUENCE_IDS, sequence_value: SOCKETS_SEQUENCE_VALUE) -> () {
        self.sequence_flags[sequence_id as usize] = sequence_value;
    }
    pub fn shutdown(&self) -> () {
        println!("CONNECTION SHUTDOWN");
        shutdown!(self.commdata);
    }
    pub fn matches_version(&self, test: ProtVer) -> bool {
        return self.host_prot_ver == test;
    }
}

impl Connection {
    fn new(sid: u16, gamedata: TcpStream, commdata: TcpStream, pver: ProtVer) -> Self {
        Self{sid, playid:0, teamid:0, gamedata, commdata, barrier_flags:[0;SOCKETS_BARFLAG_SIZE], sequence_flags:[0;SOCKETS_SEQFLAG_SIZE], host_prot_ver:pver}
    }
    pub fn connect_data(conn_data: &ConnData) -> TResult<ServerData> {
    // pub fn connect_data(conn_data: &str) -> Result<ServerData, ()> {
        let mut gd = match TcpStream::connect(conn_data.to_str()) {
        // let mut gd = match TcpStream::connect(conn_data) {
            Ok(s)=>s,
            Err(_) => {return Err(ConnError::boxed());}
        };
        gd.write(&[0x66])?;
        let r = Ok(ServerData{has_password:Protocol::read_byte(&mut gd)? != 0,name:String::from_utf8(Protocol::read_n_bytes(Protocol::read_byte(&mut gd)? as usize, &mut gd)?.to_vec()).unwrap(),version:ProtVer(Protocol::read_u16(&mut gd)?,Protocol::read_u16(&mut gd)?,Protocol::read_u16(&mut gd)?)});
        let _ = gd.shutdown(Both);
        return r;
    }
    pub fn connect_player(conn_data: &ConnData, prot_ver: ProtVer) -> TResult<Self> {
        let mut gd = match TcpStream::connect(conn_data.to_str()) {
            Ok(s)=>s,
            Err(_) => {return Err(ConnError::boxed());}
        };
        let r = |stream:&mut TcpStream|->TResult<u16> {
            Protocol::write_byte(0x44, stream)?;
            if Protocol::read_byte(stream)? == 2 {
                return Err(Box::new(io::Error::new(io::ErrorKind::ConnectionRefused, "host rejected client")));
            }
            return Protocol::read_u16(stream);
        }(&mut gd);
        let sid = match r {
            Ok(s)=>s,
            Err(e)=>{println!("{e}");let _=gd.shutdown(Both);return Err(e);}
        };
        let mut cd = match TcpStream::connect(conn_data.to_str()) {
            Ok(s)=>s,
            Err(_)=>{let _=gd.shutdown(Both);return Err(ConnError::boxed());}
        };
        let r = rt_closure!(stream, bool, {
            Protocol::write_byte(0x22, stream)?;
            Protocol::write_u16(sid, stream)?;
            return Ok(Protocol::read_byte(stream)? == 1);
        })(&mut cd);
        match r {
            Ok(true) => {},
            Ok(false) => {shutdown!(gd, cd);return Err(ConnError::boxed());},
            Err(e) => {shutdown!(gd, cd);println!("{e}");return Err(e);}
        };
        let mut ret = Self::new(sid,gd,cd,prot_ver);
        ret.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTS2SHAKE as u8);
        ret.set_barrier(SOCKETS_BARRIER_IDS::S2HANDSHAKE);
        return Ok(ret);
    }
    /**
     * returns Ok([value]) when the existance of a password was determined
     */
    pub fn check_password(&mut self) -> SRes<bool> {
        if self.get_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE) != S1SEQ::POSTS2SHAKE as u8 {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        let v = match Protocol::read_byte(&mut self.gamedata) {
            Ok(v) => v == 1,
            Err(_) => {self.shutdown();return Err(GenErr::boxed(ErrCause::IO));}
        };
        if !v {
            self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTPWACCEPT as u8);
        } else {
            self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTPWCHECK as u8);
        }
        return Ok(v);
    }
    pub fn submit_password(&mut self, pw: &str) -> SRes<bool> {
        if pw.len() > 255 {
            return Ok(false);
        }
        if self.get_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE) != S1SEQ::POSTPWCHECK as u8 {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        let r = rt_closure!(stream, bool, {
            Protocol::write_byte(pw.len() as u8, stream)?;
            stream.write_all(pw.as_bytes())?;
            return Ok(Protocol::read_byte(stream)? == 1);
        })(&mut self.gamedata);
        match r {
            Ok(true) => {self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTPWACCEPT as u8);Ok(true)},
            Ok(false) => {self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTPWREJECT as u8);Ok(false)},
            Err(_) => {self.shutdown();Err(GenErr::boxed(ErrCause::IO))}
        }
    }
    pub fn decide_pw_continue(&mut self, retry: bool) ->SRes<()> {
        if self.get_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE) != S1SEQ::POSTPWREJECT as u8 {
            if retry {
                return Ok(());
            }
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        if retry {
            match Protocol::write_byte(1, &mut self.gamedata) {
                Ok(()) => {self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTPWCHECK as u8);Ok(())},
                Err(_) => {self.shutdown();Err(GenErr::boxed(ErrCause::IO))}
            }
        } else {
            let _ = Protocol::write_byte(0, &mut self.gamedata);
            self.shutdown();
            return Ok(());
        }
    }
    pub fn send_name(&mut self, name: &str) -> SRes<()> {
        if self.get_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE) != S1SEQ::POSTPWACCEPT as u8 {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        if name.len() > 255 {
            return Err(InvarError::boxed());
        }
        match rt_closure!(stream, (), {
            Protocol::write_byte(name.len() as u8, stream)?;
            stream.write_all(name.as_bytes())?;
            Ok(())
        })(&mut self.gamedata) {
            Ok(_) => {self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTNAMESENT as u8);Ok(())},
            Err(_) => {self.shutdown();Err(GenErr::boxed(ErrCause::IO))}
        }
    }
    pub fn get_team(&mut self) -> SRes<()> {
        if self.get_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE) != S1SEQ::POSTNAMESENT as u8 || self.get_barrier(SOCKETS_BARRIER_IDS::GOTTEAMDATA) {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        match rt_closure!(stream, (PlayerId, TeamId), {
            let pn = Protocol::read_player_id(self.host_prot_ver, stream)?;
            let tn = Protocol::read_team(stream)?;
            Ok((pn, tn))
        })(&mut self.gamedata) {
            Ok(v) => {
                self.set_sequence(SOCKETS_SEQUENCE_IDS::S1HANDSHAKE, S1SEQ::POSTTEAMGOT as u8);
                self.set_barrier(SOCKETS_BARRIER_IDS::GOTTEAMDATA);
                self.playid = v.0;
                self.teamid = v.1;
                Ok(())
            },
            Err(_) => {self.shutdown();Err(GenErr::boxed(ErrCause::IO))}
        }
    }
    pub fn read_byte(&mut self) -> TResult<u8> {
        Protocol::read_byte(&mut self.gamedata)
    }
    pub fn write_byte(&mut self, byte: u8) -> TResult<()> {
        Protocol::write_byte(byte, &mut self.gamedata)
    }
    pub fn read_n_bytes(&mut self, k: usize) -> TResult<Box<[u8]>> {
        Protocol::read_n_bytes(k, &mut self.gamedata)
    }
    pub fn read_player_id(&mut self) -> TResult<PlayerId> {
        Protocol::read_player_id(self.host_prot_ver, &mut self.gamedata)
    }
    pub fn read_u16(&mut self) -> TResult<u16> {
        Protocol::read_u16(&mut self.gamedata)
    }
    pub fn read_size(&mut self) -> TResult<u32> {
        Ok(self.read_byte()? as u32)
    }
    pub fn read_score(&mut self) -> TResult<u32> {
        let b = Protocol::read_n_bytes(4, &mut self.gamedata)?;
        Ok(((b[0] as u32)<<24) | ((b[1] as u32)<<16) | ((b[2] as u32)<<8) | (b[3] as u32))
    }
    pub fn read_bsp(&mut self) -> TResult<(u8, u8)> {
        Ok((self.read_byte()?, self.read_byte()?))
    }
    pub fn try_move(&mut self, x: u32, y: u32) -> TResult<bool> {
        self.write_byte(x as u8)?;
        self.write_byte(y as u8)?;
        Ok(self.read_byte()? == 1)
    }
    pub fn read_move(&mut self) -> TResult<(u32, u32)> {
        Ok((self.read_byte()? as u32, self.read_byte()? as u32))
    }
    pub fn read_team(&mut self) -> TResult<TeamId> {
        self.read_byte()
    }
    pub fn teamid(&self) -> TResult<TeamId> {
        if !self.get_barrier(SOCKETS_BARRIER_IDS::GOTTEAMDATA) {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        Ok(self.teamid)
    }
    pub fn playerid(&self) -> TResult<PlayerId> {
        if !self.get_barrier(SOCKETS_BARRIER_IDS::GOTTEAMDATA) {
            return Err(GenErr::boxed(ErrCause::SEQUENCE));
        }
        Ok(self.playid)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("DROPPED THE CONNECTION");
    }
}

/*
this document defines the protocol for interfacing with session hosts as a client and the inverse

TYPES {
    u[X] - unsigned integer of [X] bytes
    s[X] - signed integer of [X] bytes
    byte - one byte signed integer
    short - two byte signed integer
    int - four byte signed integer
    long - eight byte singed integer
    ubyte - one byte unsigned integer
    ushort - two byte unsigned integer
    uint - four byte unsigned integer
    ulong - eight byte unsigned integer
    bool - one bit boolean
}

PARTIES {
    (defines the parties at each end of communication)
    H -> HOST
    C -> CLIENT
    C2 -> CLIENT via socket 2
    P -> CURRENT PLAYER [can be any of the clients or the host]
}

GET SERVER DATA {
    C>H: [0x66] // start communication
    H>C: bool PASS // if the host requires a password
    H>C: u1 HLEN // host name length
    H>C: <HLEN> // host name buffer
    TERMINATE SOCKET
}

S2HANDSHAKE {
    C2>H: [0x22] // signal S2 handshake
    C2>H: u2 SID // obtained from the other socket
    IF [SID VALID] {
        H>C2: 0x01
    } ELSE {
        H>C2: 0x00
    }
}

HOST HANDSHAKE {
    C>H: [0x44] // start communication
    IF [REFUSED] {
        H>C: 0x02
        TERMINATE SOCKET
    } ELSE {
        H>C: 0x00
    }
    H>C: u2 SID // unique id per connection
    DO S2HANDSHAKE
    WAIT UNTIL S2HANDSHAKE ENDS
    IF [HOST has PASSWORD] {
        H>C: 0x01
        C>H: u1 PLEN // length of password
        C>H: <PLEN> // password buffer
        IF [PASSWORD is VALID] {
            H>C: 0x01
        } else {
            H>C: 0x00
            IF [CLIENT TRY AGAIN] {
                C>H: 0x01
                RETRY
            } else {
                C>H: 0x00
                TERMINATE SOCKET
            }
        }
    } else {
        H>C: 0x00
    }
    C>H: u1 NLEN // length of name
    C>H: <NLEN> // name buffer
    H>C: u1 PLAYER // player id (given in the order in which players joined, defines turn order)
    H>C: u1 TEAM // team id for this client
    //H>C: [u1, u1, u1] COLOR // player color
    goto PREGAME LOOP
}

PREGAME LOOP (ON SOCKET 2) {
    C2>H: byte DCODE
    SWITCH [DCODE] {
        ON [0x00] {
            // CLIENT is leaving the session
            H>C2: 0x00 // only after this is recieved may the socket be closed
            HOST SIGNALS OTHERS OF LEAVING PLAYER
            TERMINATE SOCKET
        }
        ON [0x01] {
            NOTE: this is NOT VALID when the game is running
            POSSIBLE CONFLICT {
                in the event that the Host attempts to start the game and this player
                is in the options menu, then chooses to enter spectator mode,
                this code shall not be sent, instead the READY code is altered
                as described in PREGAME LOOP (ON SOCKET 1)
            }
            // CLIENT is toggling spectator mode
            HOST SIGNALS OTHERS OF MODE CHANGE
        }
    }
}

PREGAME LOOP (ON SOCKET 1) {
    H>C: byte DCODE // code for what kind of data is being sent
    SWITCH [DCODE] {
        ON [0x00] {
            // HOST is ending the session
            TERMINATE SOCKET
        }
        ON [0x01] {
            // HOST is starting the game
            C>H: SPECTATING ? 0x02 : 0x01 READY CODE
            //C>H: 0x01 READY code // CLIENT is ready to start
            //POSSIBLE CONFLICT {
            //    in the event the Host attempts to start the game and this player
            //    chooses to enter spectator mode before sending the READY code,
            //    the Client instead will send 0x02
            //}
            H>C: u1 SCNT // spectator count, number of players that are in
                        // spectator mode
            FOR EACH SPECTATOR {
                H>C: u1 PID
            }
            H>C: u1 W {RESTRICT 1 <= W <= 26} // board width
            H>C: u1 H {RESTRICT 1 <= H <= 26} // board height
            H>C: u1 P {RESTRICT 2 <= P <= 6} // player count
            goto GAME LOOP
        }
        ON [0x02] {
            // data on another CLIENT in the session
            H>C: u1 CLEN // length of other client's name
            H>C: <CLEN> // other name buffer
            H>C: u1 OPLAYER // other player's id
            H>C: u1 OTEAM // other player's team
            //H>C: [u1, u1, u1] OCOLOR // other player's color
        }
        ON [0x03] {
            // change team
            H>C: u1 PID // id of player changing teams
            H>C: u1 TID // new team id [must be an existing team]
        }
        ON [0x04] {
            // other player left
            H>C: u1 PID // id of player that left
        }
        ON [0x05] {
            // tileset change
            H>C: u1 SETID
        }
        ON [0x06] {
            do TILESET UPDATE
        }
        ON [0x07] {
            // spectating match because player joined as it was ongoing
            NOTE: THIS CODE MAY ONLY BE SENT IN THE SCENARIO DESCRIBED ABOVE
            NOTE: WHEN A PLAYER JOINS IN THIS WAY, NOTIFICATION OF OTHER PLAYERS IS DEFERRED
            goto BOARD_TRANSFER
            goto GAME_LOOP
        }
    }
    goto PREGAME LOOP
}

BOARD_TRANSFER {
    H>C: [u1, u1, u1] // width, height, and player count
    NOTE: "JOIN ORDER" is defined as the order in which default team assignments are given
            the order is as follows: RED, BLUE, GREEN, YELLOW, MAGENTA, CYAN
    FOR EACH TEAM IN JOIN ORDER {
        H>C: s32 SCORE
    }
    FOR ALL TILES {
        H>C: [u1, u1] // value and team
    }
}

TILESET UPDATE {
    NOTE: TILESET UPDATE will cause clients to behave as though they also recieved a command to change to SETID 0
    H>C: u1 NUMT // number of tilesets
    FOR EACH TILESET {
        H>C: [u2, u2, u2, u2] // four java characters
    }
}

GAME LOOP {
    H>P: 0x01 // signal it's that player's turn
    H>!P: 0x00 // signal to others that it's not their turn
    P>H: [u1, u1] POSITION
    IF [VALID POSITION] {
        H>P: 0x01
    } else {
        H>P: 0x00
        RETRY
    }
    FOR ALL CLIENTS {
        H>C: [u1, u1] POSITION
        H>C: u1 TEAM // team of player that just went
    }
    IF [PLAYER WON] {
        goto PREGAME LOOP
    }
}
*/