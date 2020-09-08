


use crate::interrupt_desc;

#[path = "../../desc.rs"]
pub(crate) mod desc;


use core::any::Any;
use crate::sheduler::{Node, def_fn};
use core::mem::{transmute, transmute_copy};

//unsafe { nrf52832_peripheral_access::NVIC::unmask(nrf52832_peripheral_access::Interrupt::GPIOTE) };

pub union Vector {
	pub(crate) _handler: fn(),
	_reserved: u32,
}

pub trait InterruptManagerTrait
{
	type Interrupt;

	fn push(&mut self, func: fn(), interrupt: Self::Interrupt);
	fn pop(&mut self, index: usize, interrupt: Self::Interrupt);

}

pub fn new()-> InterruptManager
{
	let sectors = [Node{ elem: def_fn , next: 0 as *mut Node, prev: 0 as *mut Node }; desc::TASK_NUM];

	let manger;
		unsafe
		{

			let mut i = 0;

			manger = InterruptManager
			{
				interrupt_table:  &crate::interrupt_desc::interrupt_sectors_arr
				as *const [crate::sheduler::List ; desc::INTERRUPT_NUM]
				as *mut [crate::sheduler::List;
				desc::INTERRUPT_NUM]
			};

			while i != desc::INTERRUPT_NUM
			{
				(*manger.interrupt_table)[i].node_crate[0].prev =
					&(*manger.interrupt_table)[i].node_crate[1] as
						*const Node as *mut Node;

				(*manger.interrupt_table)[i].node_crate[0].next =
					&(*manger.interrupt_table)[i].node_crate[1] as
						*const Node as *mut Node;

				(*manger.interrupt_table)[i].node_crate[1].prev =
					&(*manger.interrupt_table)[i].node_crate[0] as
						*const Node as *mut Node;

				(*manger.interrupt_table)[i].node_crate[1].next =
					&(*manger.interrupt_table)[i].node_crate[0] as
						*const Node as *mut Node;
				i+=1;
			}

		}

	manger

}

pub struct InterruptManager
{
	pub(crate) interrupt_table: *mut [crate::sheduler::List;1 * desc::INTERRUPT_NUM],
}

#[derive(Debug, Clone,Copy)]
pub struct InterruptSectorManager
{
	//pub(crate) interrupt_sector_table: &'static[Node; desc::TASK_NUMB],
	pub(crate) interrupt_sector_table: [Node; desc::TASK_NUM],
}

impl InterruptSectorManager
{
	pub(crate) fn execute(&mut self)
	{
		let mut i = 1;
		while self.interrupt_sector_table[i].elem == def_fn { i+=1; }
		let start = &self.interrupt_sector_table[i] as *const Node as *mut Node;

		unsafe
		{
			((*start).elem)();
			let mut point = (*start).next;
			while point != start
			{
				((*point).elem)();
				point = (*point).next;
			}
		}
	}

}

impl InterruptManagerTrait for InterruptManager
{
	type Interrupt = peripheral_access::Interrupt;

	fn push(&mut self, func: fn(), interrupt: Self::Interrupt)
	{
		let mut pos;
		unsafe {pos = transmute::<Self::Interrupt , u8>(interrupt) as usize;}

		let mut i= 1;

		unsafe
		{
			(*self.interrupt_table)[pos].push(func);

			// while (*self.interrupt_table)[pos].node_crate[i].elem != def_fn {i=i+1;}

			// (*self.interrupt_table)[pos].node_crate[i].elem = func;
			// (*self.interrupt_table)[pos].node_crate[i].next =
			// 	&(*self.interrupt_table)[pos].node_crate[0] as
			// 		*const Node as *mut Node;
			// (*self.interrupt_table)[pos].node_crate[i].prev = (*self.interrupt_table)[pos].node_crate[0].prev;
			//
			// (*(*self.interrupt_table)[pos].node_crate[0].prev).next   = (&(*self.interrupt_table)[pos].node_crate[i] as *const Node) as *mut Node;
			// (*self.interrupt_table)[pos].node_crate[0].prev = (&(*self.interrupt_table)[pos].node_crate[i] as *const Node) as *mut Node;
		}

		// unsafe
		// {
		// let mut origin_index= 0;
		// let mut neighbour_index = 0;
		//
		// while (*self.interrupt_table)[pos].interrupt_sector_table[origin_index].elem != def_fn { origin_index = origin_index + 1; }
		// 	(*self.interrupt_table)[pos].interrupt_sector_table[origin_index].elem = func;
		//
		// 	while (*self.interrupt_table)[pos].interrupt_sector_table[neighbour_index].elem == def_fn { neighbour_index += 1; }
		//
		// 	(*self.interrupt_table)[pos].interrupt_sector_table[origin_index].prev = &(*(*self.interrupt_table)[pos].interrupt_sector_table[neighbour_index].prev) as *const Node as *mut Node;
		// 	(*self.interrupt_table)[pos].interrupt_sector_table[origin_index].next = &(*self.interrupt_table)[pos].interrupt_sector_table[neighbour_index] as *const Node as *mut Node ;
		//
		// 	(*(*self.interrupt_table)[pos].interrupt_sector_table[neighbour_index].prev).next = &(*self.interrupt_table)[pos].interrupt_sector_table[origin_index] as *const Node as *mut Node;
		// 	(*self.interrupt_table)[pos].interrupt_sector_table[neighbour_index].prev = &(*self.interrupt_table)[pos].interrupt_sector_table[origin_index] as *const Node as *mut Node;
		// }
	}

	fn pop(&mut self, index: usize, interrupt: Self::Interrupt)
	{
		let mut pos:usize;
		unsafe {//pos = transmute::<Self::Interrupt , usize>(interrupt);
		pos = transmute_copy(&interrupt)}
		unsafe
			{
				(*self.interrupt_table)[pos].node_crate[index].elem = def_fn;

				(*(*self.interrupt_table)[pos].node_crate[index].prev).next = (*self.interrupt_table)[pos].node_crate[index].next;
				(*(*self.interrupt_table)[pos].node_crate[index].next).prev = (*self.interrupt_table)[pos].node_crate[index].prev;
			}

	}
}

unsafe impl core::marker::Sync for InterruptManager{}
unsafe  impl core::marker::Sync for InterruptSectorManager{}
