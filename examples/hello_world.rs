// This example just prints "Hello Tock World" to the terminal.
// Run `tockloader listen`, or use any serial program of your choice
//  (e.g. `screen`, `minicom`) to view the message.

#![no_std]
#![feature(asm)]

use libtock::println;
use libtock::result::TockResult;

libtock_core::stack_size! {0x400}

#[libtock::main]
async fn main() -> TockResult<()> {
    let drivers = libtock::retrieve_drivers()?;

    drivers.console.create_console();

    let num : u32;
    unsafe{
        asm!("RDCYCLE {}", out(reg) num);
      }

    println!("Hello Tock World {}", num);

    Ok(())
}
