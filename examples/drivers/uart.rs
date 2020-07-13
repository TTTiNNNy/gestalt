#![no_std]
#![no_main]

extern crate panic_halt;
extern crate nrf52840_drivers;
use gestalt_reference::uart::GestaltUart;
use nrf52840_drivers::uart::uart;

extern crate miscellaneous;
use miscellaneous::sleep;
use miscellaneous::time::*;

#[allow(unused_imports)]
use cortex_m::asm;
use cortex_m_rt::entry;
use gestalt_reference::interface::GestaltInerface;
use nrf52840_drivers::uart::uart::Nrf52Uart;


#[entry]
fn main() -> !
{

    let num: [u8; 9] = [49,50,51,52,53,54,55,56,57];

    let mess =  b"\n\rget de som numbs, bro: ".clone();;

    let u_0 = uart::new(uart::UartInst::Uart0);

    //let b = Bytes::from_static(b"hello");
    u_0.set_rx(nrf52840_drivers::gpio::gpio::GpioPin::Gpio14);
    u_0.set_tx(nrf52840_drivers::gpio::gpio::GpioPin::Gpio13);
    u_0.set_baud(uart::Baudrate::Baud9600);
    u_0.enable(true);


    let delay = time_h::from_millis(100);
    loop
    {

        u_0.set_tx_buf(&num);
        u_0.write();

        sleep::sleep::sleep(&delay);

        u_0.set_tx_buf(&mess);

        u_0.write();

        sleep::sleep::sleep(&delay);

    }
}
