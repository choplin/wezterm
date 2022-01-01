use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

mod macos;
mod windows;
mod linux;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum LocalProcessStatus {
    Idle,
    Run,
    Sleep,
    Stop,
    Zombie,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    Parked,
    LockBlocked,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub executable: PathBuf,
    pub argv: Vec<String>,
    pub cwd: PathBuf,
    pub status: LocalProcessStatus,
    pub children: HashMap<u32, LocalProcessInfo>,
    pub start_time: SystemTime,
}
luahelper::impl_lua_conversion!(LocalProcessInfo);

impl LocalProcessInfo {
    pub fn flatten_to_exe_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();

        fn flatten(item: &LocalProcessInfo, names: &mut HashSet<String>) {
            if let Some(exe) = item.executable.file_name() {
                names.insert(exe.to_string_lossy().into_owned());
            }
            for proc in item.children.values() {
                flatten(proc, names);
            }
        }

        flatten(self, &mut names);
        names
    }
}