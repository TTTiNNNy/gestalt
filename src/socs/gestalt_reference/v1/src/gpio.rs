
trait GpioG
{
	fn as_u16(&self);
	fn as_u32(&self);
}

#[allow(dead_code)]
pub enum GpioDir
{
	IN	=	0,
	OUT	=	1,
}

#[allow(dead_code)]
pub enum GpioState
{
	UP		=	1,
	DOWN	=	0,
}

#[allow(dead_code)]
enum GpioInterrupt
{
	RISE,
	FALL,
}

#[allow(dead_code)]
pub enum GpioPull
{
	UP		= 3,
	DOWN	= 1,
	NONE	= 0,
}



pub trait GestaltGpio
{
	type Port;
	type Pin;
	type Dir;
	type Pull;
	type State;
	type PortLength;

    fn set_state	(&self, _: Self::Pin, _: Self::State);
	fn set_high		(&self, _: Self::Pin);
	fn set_low		(&self, _: Self::Pin);
	fn set_derection(&self, _: Self::Pin, _: Self::Dir);
    fn set_pull		(&self,	_: Self::Pin, _: Self::Pull);
	fn set_port		(&self);
	fn set_pull_up	(&self,	_: Self::Pin);
	fn set_pull_down(&self,	_: Self::Pin);
	fn set_pull_none(&self,	_: Self::Pin);
	fn tougle_pin	(&self,	_: Self::Pin);
    fn init();
}
