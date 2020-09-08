
pub trait interrupt_handle
{
	type Interrupts;
	type Pin;

	fn set_interrupt    (&mut self, _: Self::Interrupts, _: Self::Pin);
	fn clear_interrupt  (&mut self, _: Self::Pin);

}



