extern crate nrf52840_peripheral_access;


extern crate gestalt_reference;


#[allow(dead_code)]
#[derive(Debug)]
pub enum GpioInst
{
	P0		= 0x5000_0000 + 0x504,
	P1		= 0x5000_0300,
}

pub enum  GpioPin
{
	Gpio0	=	0,
	Gpio1	=	1,
	Gpio2	=	2,
	Gpio3	=	3,
	Gpio4	=	4,
	Gpio5	=	5,
	Gpio6	=	6,
	Gpio7	=	7,
	Gpio8	=	8,
	Gpio9	=	9,
	Gpio10	=	10,
	Gpio11	=	11,
	Gpio12	=	12,
	Gpio13	=	13,
	Gpio14	=	14,
	Gpio15	=	15,
	Gpio16	=	16,
	Gpio17	=	17,
	Gpio18	=	18,
	Gpio19	=	19,
	Gpio20	=	20,
	Gpio21	=	21,
	Gpio22	=	22,
	Gpio23	=	23,
	Gpio24	=	24,
	Gpio25	=	25,
	Gpio26	=	26,
	Gpio27	=	27,
	Gpio28	=	28,
	Gpio29	=	29,
	Gpio30	=	30,
	Gpio31	=	31,
}

pub struct Port
{
	pub base: *const nrf52840_peripheral_access::p0::RegisterBlock,
}

pub fn new	(p: GpioInst) -> Port
{
	let p = Port{base : ((p as u32) as  *const nrf52840_peripheral_access::p0::RegisterBlock )};
	p
}

impl gestalt_reference::gpio::GestaltGpio for Port
{
	type Port		= GpioInst;
	type Pin		= GpioPin;
	type Dir		= gestalt_reference::gpio::GpioDir;
	type Pull		= gestalt_reference::gpio::GpioPull;
	type State		= gestalt_reference::gpio::GpioState;
	type PortLength	= usize;

	fn set_state	(&self, pin: GpioPin, state: gestalt_reference::gpio::GpioState)
	{
		let st = state as u32;
		let p = 1 << (pin as u32);
		let r = self.base as u32;
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe
		{
			let v = ((p * st)) | ((*s).out.read().bits()  & !(p as u32));
			(*s).out.write(|w|w.bits(v));
		}
	}

	fn set_high		(&self, pin: GpioPin)
	{
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe{(*s).outset.write(|w|w.bits(1<<(pin as u32)));}
	}

	fn set_low		(&self, pin: GpioPin)
	{
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe{ (*s).outclr.write(|w|w.bits(1<<(pin as u32))); }

	}

	fn set_derection(&self, pin: GpioPin, dir: gestalt_reference::gpio::GpioDir)
	{
		let st = dir as u32;
		let p = 1<<(pin as u32);
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe
		{
			let v = (p * st) | ((*s).dir.read().bits()  & !(p as u32));
			(*s).dir.write(|w| w.bits(v));
		}
	}

	fn set_pull	(&self, pin: GpioPin, pull: gestalt_reference::gpio::GpioPull)
	{
		let p = 1<<(pin as usize);
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe
		{
			let v = ((pull as u32)<<2) | ((*s).pin_cnf[p].read().bits()  & !(p as u32));
			(*s).pin_cnf[p].write(|w| w.bits(v));
		}
	}

	fn set_pull_up(&self, pin: GpioPin)
	{
		let p = 1<<(pin as usize);
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe{(*s).pin_cnf[p].write(|w| w.pull().pullup())}
	}

	fn set_pull_down(&self, pin: GpioPin)
	{
		let p = 1<<(pin as usize);
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe{(*s).pin_cnf[p].write(|w| w.pull().pulldown())}
	}

	fn set_pull_none(&self, pin: GpioPin)
	{
		let p = 1<<(pin as usize);
		let r = {self.base as u32};
		let s = r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe{(*s).pin_cnf[p].write(|w| w.pull().disabled())}
	}

	fn set_port	(&self)
	{
		
	}

	fn tougle_pin	(&self, pin: GpioPin)
	{
		let p 		= 1<<(pin as u32);
		let r 		= {self.base as u32};
		let s		= r as *const nrf52840_peripheral_access::p0::RegisterBlock;

		unsafe
		{
			let state	= (*s).out.read().bits();

			let v = (p * state) | ((*s).out.read().bits()  & !(p as u32));
			(*s).out.write(|w| w.bits(v));
		}

	}

	fn init()
	{

	}
}


