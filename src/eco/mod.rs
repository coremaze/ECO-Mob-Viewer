

pub use sysinfo::Process;
use sysinfo::System;

use read_process_memory::{copy_address, ProcessHandle};

mod monsters;
pub use monsters::monster_by_id;
pub use monsters::{Properties};

pub fn read_u32_le(handle: &ProcessHandle, address: usize) -> Option<u32> {
    let value = copy_address(address, 4, handle).ok()?;
    let &q1 = value.first()?;
    let &q2 = value.get(1)?;
    let &q3 = value.get(2)?;
    let &q4 = value.get(3)?;
    Some(u32::from_le_bytes([q1, q2, q3, q4]))
}

pub fn get_hovered_monster_id(handle: &ProcessHandle) -> Option<u32> {
    let ptr1 = read_u32_le(handle, 0x400000usize + 0x0330A888usize)? as usize;
    let ptr2 = ptr1 + 0x019FFC;
    let mob_ptr = read_u32_le(handle, ptr2)? as usize;
    let mob_id = read_u32_le(handle, mob_ptr + 0x14)?;
    Some(mob_id)
}

pub fn get_eco() -> Option<ProcessHandle> {
    // Create a new System instance
    let mut system = System::new_all();

    // First we update all information of our system struct
    system.refresh_all();

    let eco_pid = i32::try_from(
        system
            .processes_by_exact_name("eco.exe")
            .next()
            .map(Process::pid)?
            .as_u32(),
    )
    .ok()?;

    ProcessHandle::try_from(eco_pid).ok()
}
