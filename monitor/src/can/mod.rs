mod udp;

use std::time::Instant;
use common::can::CanPacket;


#[derive(Debug, Clone, Copy)]
pub struct CanPacketIn(pub CanPacket, pub Instant);
impl Event for CanPacketIn {}

#[derive(Debug, Clone, Copy)]
pub struct CanPacketOut(pub CanPacket, pub Instant);
impl Event for CanPacketOut {}

pub use udp::*;

use crate::Event;