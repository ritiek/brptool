use anyhow::Result;
use binrw::BinRead;
use std::io::Read;

use crate::consts::BaMessage;
use crate::error::BrpError;
use crate::huffman::Huffman;
use crate::session::handle_session_message;

pub const BRP_FILE_ID: u32 = 83749;
pub const TARGET_PROTOCOL_VERSION: u16 = 33;

#[derive(BinRead, Debug)]
pub struct BrpHeader {
    pub file_id: u32,
    pub protocol_version: u16,
}

pub struct BaMessages<T: Read>(T);

impl<T: Read> Iterator for BaMessages<T> {
    type Item = BaMessage;

    fn next(&mut self) -> Option<Self::Item> {
        let message = load_replay_message(&mut self.0);
        message.ok()
    }
}

pub struct Replay<T: Read> {
    pub messages: BaMessages<T>,
    pub header: BrpHeader,
}

fn read_message_length<T: Read>(stream: &mut T) -> Result<u32> {
    // The first byte represents the actual size if the value is < 254
    // if it is 254, the 2 bytes after it represent size
    // if it is 255, the 4 bytes after it represent size
    // (from original ballistica source, logic/session/replay_client_session.cc)

    let mut buf = [0; 1];
    stream.read_exact(&mut buf)?;
    let len = u8::from_le_bytes(buf);
    if len < 254 {
        Ok(len.into())
    } else if len == 254 {
        let mut buf = [0; 2];
        stream.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf).into())
    } else if len == 255 {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    } else {
        unreachable!();
    }
}

fn load_replay_message<T: Read>(mut stream: T) -> Result<BaMessage> {
    let huffman = Huffman::build();
    let length = read_message_length(&mut stream)?;
    let mut buf = vec![0; length as usize];
    stream.read_exact(&mut buf)?;
    let data = huffman.decompress(&buf);
    Ok(handle_session_message(&data))
}

pub fn load_replay<T: Read>(mut stream: T) -> Result<Replay<T>> {
    let mut file_id_le: [u8; 4] = [0; 4];
    stream.read_exact(&mut file_id_le)?;
    let file_id = u32::from_le_bytes(file_id_le);

    let mut protocol_version_le: [u8; 2] = [0; 2];
    stream.read_exact(&mut protocol_version_le)?;
    let protocol_version = u16::from_le_bytes(protocol_version_le);

    let header = BrpHeader {
        file_id,
        protocol_version,
    };

    if header.file_id != BRP_FILE_ID {
        return Err(BrpError::NotABrpFile.into());
    }

    if header.protocol_version != TARGET_PROTOCOL_VERSION {
        return Err(BrpError::UnsupportedProtocolVersion(header.protocol_version).into());
    }

    let messages = BaMessages(stream);
    let replay = Replay {
        messages,
        header,
    };

    Ok(replay)
}
