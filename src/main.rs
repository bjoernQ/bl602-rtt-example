#![no_std]
#![no_main]

use bl602_hal as hal;
use blash_target::{init_print, println};
use hal::{
    clock::{Strict, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
};

#[riscv_rt::entry]
fn main() -> ! {
    init_print!();

    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();

    // Set up all the clocks we need
    let _clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut parts.clk_cfg);

    println!("Start");

    // panic!("You should see a backtrace in blash in case of a panic!");

    // uncomment code below to trigger a "Load address misaligned" error
    // let _foo = unsafe {
    //     let foo = 0x22010001 as *mut u32;
    //     *foo as u32
    // };

    let mut i = 0;

    loop {
        println!("Hello BL602!");
        println!("Hello, world! Count is {}", i);
        i += 1;
        for _ in 0..250000 {}
    }
}
