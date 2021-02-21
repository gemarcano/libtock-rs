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
    let perf = drivers.perf;

    let a = perf.cycles()?;
    let b = perf.cycles()?;
    println!("Perf1: {}, Perf2: {}, diff: {}", a, b, b - a);
    Ok(())
}
