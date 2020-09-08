use super::gpio;

pub enum GpioBufState
{
    Connect,
    Disconnect
}

pub trait gpio_none_gestalt
{
    fn set_inp_buf(&self, _:gpio::GpioPin, _:GpioBufState);
}