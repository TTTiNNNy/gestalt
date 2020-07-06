extern crate nrf52840_peripheral_access;


extern crate gestalt_reference;


#[allow(dead_code)]
#[derive(Debug)]
pub enum UartInst
{
	Uart0		= 0x40002000,
	Uart1		= 0x40028000,
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

pub struct Uart
{
	pub base: *const nrf52840_peripheral_access::uart0::RegisterBlock,
}

pub fn new	(inst: UartInst) -> Uart
{
	let i = Uart{base : ((inst as u32) as  *const nrf52840_peripheral_access::uart0::RegisterBlock )};
	i
}
