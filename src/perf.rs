use crate::result::TockResult;
use crate::syscalls::{command, command2};

const DRIVER_NUMBER: usize = 0x90004;

mod command_nr {
    pub const COUNT: usize = 0;
    pub const CYCLES: usize = 1;
}

pub struct Perf;

impl Perf {
    pub fn count(&self) -> TockResult<usize> {
        Ok(command(DRIVER_NUMBER, command_nr::COUNT, 0, 0)?)
    }

    pub fn cycles(&self) -> TockResult<usize> {
        Ok(command2(DRIVER_NUMBER, command_nr::CYCLES, 0, 0)?)
    }
}
