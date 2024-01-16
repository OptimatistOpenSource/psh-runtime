use crate::event::{Event, EventScope};
use alloc::vec::Vec;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Default, PartialEq, Eq, Debug, Clone)]
pub struct ExtraConfig {
    pub pinned: bool,
    pub exclusive: bool,

    pub inherit: bool,
    pub inherit_stat: bool,
    pub inherit_thread: bool,

    pub enable_on_exec: bool,
    pub remove_on_exec: bool,
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum Cpu {
    Any,
    On { cpu: u32 },
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub enum Process {
    Any,
    Calling,
    On { pid: u32 },
}

#[derive(Archive, Deserialize, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Config {
    pub cpu: Cpu,
    pub process: Process,
    pub event: Event,
    pub scopes: Vec<EventScope>,
    pub extra_config: ExtraConfig,
}