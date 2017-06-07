#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;

use cortex_m::asm;
use stm32f103xx::RCC;
use stm32f103xx::GPIOB;

fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let rcc = RCC.borrow(cs);
            let gpiob = GPIOB.borrow(cs);
            rcc.apb2enr.modify(|_, w| w.iopben().enabled());
            gpiob.crh.modify(|_, w| w.mode11().output50());
            gpiob.crh.modify(|_, w| w.cnf11().push());
            gpiob.bsrr.write(|w| w.bs11().set());
        }
    );
    hprintln!("Hello, world!");
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
