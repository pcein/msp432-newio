#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate msp432p401r;


use cortex_m::asm;

fn main() {

    let p = msp432p401r::Peripherals::take().unwrap();

    // The Digital I/O module
    let dio = p.DIO;

    // PORTA consists of two ports P1 and P2. Red LED of the 
    // MSP432P401R launchpad is on P2.0. Green and Blue LED's
    // on P2.1 and P2.2. Simply set the direction register bit
    // to 1 and write a 1 to the output register to put ON the LED.
    dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 1) });

    dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() | 1) });

    loop {}

}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
