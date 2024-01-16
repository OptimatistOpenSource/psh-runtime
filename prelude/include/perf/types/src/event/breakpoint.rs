use rkyv::{Archive, Deserialize, Serialize};
use crate::event::Event;

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct BreakpointEvent {
    pub bp_type: BreakpointType,
}

impl BreakpointEvent {
    pub const fn new(bp_type: BreakpointType) -> Self {
        Self { bp_type }
    }
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum BreakpointType {
    R { addr: u64, len: BreakpointLen },
    W { addr: u64, len: BreakpointLen },
    Rw { addr: u64, len: BreakpointLen },
    X { addr: u64 },
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum BreakpointLen {
    Len1,
    Len2,
    Len4,
    Len8,
}

impl From<BreakpointEvent> for Event {
    fn from(value: BreakpointEvent) -> Self {
        Self::Breakpoint(value)
    }
}