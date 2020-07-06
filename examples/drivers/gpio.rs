#![no_std]
#![no_main]

extern crate panic_halt;
extern crate nrf52840_drivers;
use gestalt_reference::gpio::*;
use nrf52840_drivers::gpio::gpio;

extern crate miscellaneous;
use miscellaneous::sleep;
use miscellaneous::time::*;

#[allow(unused_imports)]
use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> !
{

    let p_0 = gpio::new(gpio::GpioPort::P0);

    p_0.set_derection(gpio::GpioPin::Gpio20,gestalt_reference::gpio::GpioDir::OUT);
    p_0.set_derection(gpio::GpioPin::Gpio22,gestalt_reference::gpio::GpioDir::OUT);
    p_0.set_derection(gpio::GpioPin::Gpio23, gestalt_reference::gpio::GpioDir::OUT);
    p_0.set_derection(gpio::GpioPin::Gpio24, gestalt_reference::gpio::GpioDir::OUT);



    let delay = time_h::from_millis(200);
    loop
    {

        p_0.set_high(gpio::GpioPin::Gpio22);
        sleep::sleep::sleep(&delay);

        p_0.set_high(gpio::GpioPin::Gpio23);
        sleep::sleep::sleep(&delay);

        p_0.set_state(gpio::GpioPin::Gpio24,gestalt_reference::gpio::GpioState::UP);
        sleep::sleep::sleep(&delay);

        p_0.set_state(gpio::GpioPin::Gpio20,gestalt_reference::gpio::GpioState::UP);
        sleep::sleep::sleep(&delay);

        p_0.set_low(gpio::GpioPin::Gpio22);
        sleep::sleep::sleep(&delay);

        p_0.set_low(gpio::GpioPin::Gpio23);
        sleep::sleep::sleep(&delay);

        p_0.set_state(gpio::GpioPin::Gpio24,gestalt_reference::gpio::GpioState::DOWN);
        sleep::sleep::sleep(&delay);

        p_0.set_state(gpio::GpioPin::Gpio20,gestalt_reference::gpio::GpioState::DOWN);
        sleep::sleep::sleep(&delay);


    }
}
