//! Protocol message definitions (manually defined to match protobuf)
//! 
//! This replaces the prost-generated code with manual implementations
//! that are wire-compatible with the Go protobuf definitions.

use bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use prost::{DecodeError, Message};

/// Envelope message containing either Ping or Packet
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Envelope {
    pub r#type: Option<envelope::Type>,
}

pub mod envelope {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    pub enum Type {
        Ping(super::MessagePing),
        Packet(super::MessagePacket),
    }
}

impl Message for Envelope {
    fn encode_raw(&self, buf: &mut impl BufMut)
    where
        Self: Sized,
    {
        if let Some(ref r#type) = self.r#type {
            match r#type {
                envelope::Type::Ping(ping) => {
                    prost::encoding::message::encode(1, ping, buf);
                }
                envelope::Type::Packet(packet) => {
                    prost::encoding::message::encode(2, packet, buf);
                }
            }
        }
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            1 => {
                let mut ping = MessagePing::default();
                prost::encoding::message::merge(wire_type, &mut ping, buf, ctx)?;
                self.r#type = Some(envelope::Type::Ping(ping));
                Ok(())
            }
            2 => {
                let mut packet = MessagePacket::default();
                prost::encoding::message::merge(wire_type, &mut packet, buf, ctx)?;
                self.r#type = Some(envelope::Type::Packet(packet));
                Ok(())
            }
            _ => prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        match &self.r#type {
            Some(envelope::Type::Ping(ping)) => prost::encoding::message::encoded_len(1, ping),
            Some(envelope::Type::Packet(packet)) => prost::encoding::message::encoded_len(2, packet),
            None => 0,
        }
    }

    fn clear(&mut self) {
        self.r#type = None;
    }
}

/// Ping message for heartbeat/routing
#[derive(Clone, PartialEq, Debug, Default)]
pub struct MessagePing {
    pub timestamp: i64,
    pub local_addr: String,
    pub local_private_addr: String,
    pub ip: String,
    pub dc: String,
}

impl Message for MessagePing {
    fn encode_raw(&self, buf: &mut impl BufMut)
    where
        Self: Sized,
    {
        if self.timestamp != 0 {
            prost::encoding::int64::encode(1, &self.timestamp, buf);
        }
        if !self.local_addr.is_empty() {
            prost::encoding::string::encode(2, &self.local_addr, buf);
        }
        if !self.local_private_addr.is_empty() {
            prost::encoding::string::encode(3, &self.local_private_addr, buf);
        }
        if !self.ip.is_empty() {
            prost::encoding::string::encode(4, &self.ip, buf);
        }
        if !self.dc.is_empty() {
            prost::encoding::string::encode(5, &self.dc, buf);
        }
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            1 => prost::encoding::int64::merge(wire_type, &mut self.timestamp, buf, ctx),
            2 => prost::encoding::string::merge(wire_type, &mut self.local_addr, buf, ctx),
            3 => prost::encoding::string::merge(wire_type, &mut self.local_private_addr, buf, ctx),
            4 => prost::encoding::string::merge(wire_type, &mut self.ip, buf, ctx),
            5 => prost::encoding::string::merge(wire_type, &mut self.dc, buf, ctx),
            _ => prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        let mut len = 0;
        if self.timestamp != 0 {
            len += prost::encoding::int64::encoded_len(1, &self.timestamp);
        }
        if !self.local_addr.is_empty() {
            len += prost::encoding::string::encoded_len(2, &self.local_addr);
        }
        if !self.local_private_addr.is_empty() {
            len += prost::encoding::string::encoded_len(3, &self.local_private_addr);
        }
        if !self.ip.is_empty() {
            len += prost::encoding::string::encoded_len(4, &self.ip);
        }
        if !self.dc.is_empty() {
            len += prost::encoding::string::encoded_len(5, &self.dc);
        }
        len
    }

    fn clear(&mut self) {
        self.timestamp = 0;
        self.local_addr.clear();
        self.local_private_addr.clear();
        self.ip.clear();
        self.dc.clear();
    }
}

/// Packet message for IP packet payload
#[derive(Clone, PartialEq, Debug, Default)]
pub struct MessagePacket {
    pub payload: Vec<u8>,
}

impl Message for MessagePacket {
    fn encode_raw(&self, buf: &mut impl BufMut)
    where
        Self: Sized,
    {
        if !self.payload.is_empty() {
            prost::encoding::bytes::encode(1, &self.payload, buf);
        }
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            1 => prost::encoding::bytes::merge(wire_type, &mut self.payload, buf, ctx),
            _ => prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        if !self.payload.is_empty() {
            prost::encoding::bytes::encoded_len(1, &self.payload)
        } else {
            0
        }
    }

    fn clear(&mut self) {
        self.payload.clear();
    }
}
