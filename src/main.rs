#![no_std]
#![no_main]

use bl602_hal as hal;
use core::fmt::Write;
use hal::{
    clock::{Strict, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
};
use panic_halt as _;

extern crate jlink_rtt;

#[riscv_rt::entry]
fn main() -> ! {
    let mut output = jlink_rtt::Output::new();

    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();

    // Set up all the clocks we need
    let _clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut parts.clk_cfg);

    writeln!(output, "Start").ok();

    let mut i = 0;

    loop {
        writeln!(output, "Hello BL602!").ok();
        writeln!(output, "Hello, world! Count is {}", i).ok();
        i += 1;
        for _ in 0..250000 {}
    }
}
