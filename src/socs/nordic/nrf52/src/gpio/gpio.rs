use peripheral_access::p0;
use peripheral_access::gpiote;

use gestalt_reference::gpio::*;

use gestalt_reference::gpio::GestaltGpio;
use gestalt_reference::interrupt_handler;
use crate::gpio::gpio_none_gestalt::*;
use core::mem::transmute;
use num_enum;
use gestalt_reference::interrupt_handler::interrupt_handle;
//use gestalt_reference::gpio::GestaltGpioInterrupt;
use core::borrow::BorrowMut;
pub use gestalt_reference;

#[path = "../../../nrf52832/drivers/src/gpio/gpio.rs"]
pub(crate) mod gpio;
pub use gpio::GpioInst;

const EVENT_SIZE: usize = 8;
const INTERRUPT_ADRESS: usize = 0x40006000;
const PSEL_MASK: u32 = 0b0000_0000_0000_0000_0001_1111_0000_0000;

#[derive(Clone, Copy)]
pub enum  GpioPin
{
	Gpio0	= 0,
	Gpio1	= 1,
	Gpio2	= 2,
	Gpio3	= 3,
	Gpio4	= 4,
	Gpio5	= 5,
	Gpio6	= 6,
	Gpio7	= 7,
	Gpio8	= 8,
	Gpio9	= 9,
	Gpio10	= 10,
	Gpio11	= 11,
	Gpio12	= 12,
	Gpio13	= 13,
	Gpio14	= 14,
	Gpio15	= 15,
	Gpio16	= 16,
	Gpio17	= 17,
	Gpio18	= 18,
	Gpio19	= 19,
	Gpio20	= 20,
	Gpio21	= 21,
	Gpio22	= 22,
	Gpio23	= 23,
	Gpio24	= 24,
	Gpio25	= 25,
	Gpio26	= 26,
	Gpio27	= 27,
	Gpio28	= 28,
	Gpio29	= 29,
	Gpio30	= 30,
	Gpio31	= 31,
}

pub struct Port
{
	pub base_pin:   *const p0::RegisterBlock,
	base_interrupt: *const gpiote::RegisterBlock,
	is_inter_free:  [bool; EVENT_SIZE],
}

pub fn new	(p: gpio::GpioInst) -> Port
{
	let p = Port{ base_pin: ((p as u32) as  *const p0::RegisterBlock ), base_interrupt: INTERRUPT_ADRESS as *const gpiote::RegisterBlock, is_inter_free: [true; EVENT_SIZE] };
	p
}



impl gestalt_reference::gpio::GestaltGpio for Port
{
	type Port		= gpio::GpioInst;
	type Pin		= self::GpioPin;
	type Dir		= self::GpioDir;
	type Pull		= self::GpioPull;
	type State		= self::GpioState;
	type PortLength	= usize;

	fn set_state	(&self, pin: GpioPin, state: Self::State)
	{
		let st = state as u32;
		let p = 1 << (pin as u32);
		let r = self.base_pin as u32;
		let s = r as *const p0::RegisterBlock;

		unsafe
		{
			let v = ((p * st)) | ((*s).out.read().bits()  & !(p as u32));
			(*s).out.write(|w|w.bits(v));
		}
	}

	fn set_high		(&self, pin: GpioPin)
	{
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe{(*s).outset.write(|w|w.bits(1<<(pin as u32)));}
	}

	fn set_low		(&self, pin: GpioPin)
	{
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe{ (*s).outclr.write(|w|w.bits(1<<(pin as u32))); }

	}

	fn set_derection(&self, pin: GpioPin, dir: Self::Dir)
	{
		self.set_inp_buf(pin,crate::gpio::gpio_none_gestalt::GpioBufState::Connect);

		let st = dir as u32;
		let p = 1<<(pin as u32);
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe
		{
			let v = (p * st) | ((*s).dir.read().bits()  & !(p as u32));
			(*s).dir.write(|w| w.bits(v));
		}
	}

	fn set_pull	(&self, pin: GpioPin, pull: Self::Pull)
	{
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe
		{
			(*s).pin_cnf[pin as usize].write(|w| w.pull().bits(pull as u8));
		}
	}

	fn set_pull_up(&self, pin: GpioPin)
	{

		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().pullup())}
	}

	fn set_pull_down(&self, pin: GpioPin)
	{

		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().pulldown())}
	}

	fn set_pull_none(&self, pin: GpioPin)
	{

		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;

		unsafe{(*s).pin_cnf[pin as usize].write(|w| w.pull().disabled())}
	}

	fn set_port	(&self){}

	fn tougle	(&self, pin: GpioPin)
	{
		let p 		= 1<<(pin as u32);
		let r 		= {self.base_pin as u32};
		let s		= r as *const p0::RegisterBlock;

		unsafe
		{
			let state	= (*s).in_.read().bits();
			let mask = state & p;
			let port = (p ^ mask) | (state & (!p));
			(*s).out.write(| w|{w.bits(port)})
		}

	}

	fn get(&self, pin: Self::Pin) -> Self::State
	{
		let p = 1<<(pin as u32);
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;
		let state;
		unsafe{state = (*s).in_.read().bits() & p;}
		let state = state >> (pin as u32);
		let num;
		unsafe {
			num = transmute::<u32, Self::State>(state);
		};
		num
	}

	fn get_port(&self) -> Self::PortLength {
		0
	}
}

impl gpio_none_gestalt for Port
{
	fn set_inp_buf(&self, pin: GpioPin, state: self::super::gpio_none_gestalt::GpioBufState)
	{
		let r = {self.base_pin as u32};
		let s = r as *const p0::RegisterBlock;
		let state_as_bool;
		unsafe
			{

				if let crate::gpio::gpio_none_gestalt::GpioBufState::Connect = state{state_as_bool=false}
				else{state_as_bool=true}

				(*s).pin_cnf[pin as usize].write(|w| w.input().bit(state_as_bool));
//				let adr =(0x50000000 + 0x704) as *mut u32;
//				*adr=0;
			}
	}
}

impl interrupt_handler::interrupt_handle for Port{
	type Interrupts = GestaltGpioInterrupt;
	type Pin = self::GpioPin;

	fn set_interrupt(&mut self, interrupt: Self::Interrupts, pin: Self::Pin)
	{
		let mut i:usize = 0;
		while self.is_inter_free[i] == false {i+=1}
		self.is_inter_free[i] = false;
		unsafe
		{
			(*self.base_interrupt).intenset.write(|w|w.bits(1<<i));
			match interrupt
			{
				Self::Interrupts::CHANGE =>
				{
					(*self.base_interrupt).config[i].write(|w|
						w.psel().bits(pin as u8).mode().event().polarity().toggle());
				}

				Self::Interrupts::RISE =>
				{
					(*self.base_interrupt).config[i].write(|w|
						w.psel().bits(pin as u8).mode().event().polarity().lo_to_hi());
				}

				Self::Interrupts::FALL =>
				{
					(*self.base_interrupt).config[i].write(|w|
						w.psel().bits(pin as u8).mode().event().polarity().hi_to_lo());
				}
			}
		}
	}


	fn clear_interrupt(&mut self, pin:self::GpioPin)
	{
		unsafe
			{
				let mut i:usize = 0;

				while ((*self.base_interrupt).config[i].read().bits() & PSEL_MASK) == (pin as u32) {i+=1}
				self.is_inter_free[i] = true;

				(*self.base_interrupt).intenclr.write(|w|w.bits(1<<i));



			}
	}
}

