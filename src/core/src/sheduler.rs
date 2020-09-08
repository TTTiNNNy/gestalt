use core::option::Option;
use core::option::Option::*;

use core::panic::PanicInfo;
use core::alloc::Layout;


use cortex_m_rt::entry;
use core::ptr;

use core::option;

#[path = "../../desc.rs"]
mod desc;

pub(crate)fn def_fn(){}
#[derive(Copy, Clone)]
pub struct List
{
    pub(crate) node_crate: [Node; desc::TASK_NUM],
	pub(crate) head:       *mut Node,
	pub(crate) cur:        *mut Node,

}

#[derive(Debug, Clone,Copy)]
pub struct Node
{
    pub(crate) elem:  fn(),
    pub(crate) next: *mut Node,
	pub(crate) prev: *mut Node,
}


pub fn new() -> List
{

	let mut ar = [Node{ elem: def_fn , next: 0 as *mut Node, prev: 0 as *mut Node }; desc::TASK_NUM];
	ar[0].next  = (&(ar[1]) as *const Node) as *mut Node;
	ar[0].prev  = ((&ar[1] as *const Node) as *mut Node);

	let mut list =  List{node_crate: ar, head: (&ar[0] as *const Node) as *mut Node, cur: &ar[1] as *const Node as *mut Node };

	list.head   = &list.node_crate[0] as *const Node as *mut Node;
	list.cur    = &list.node_crate[1] as *const Node as *mut Node;
	unsafe
	{
		(*list.head).next = list.cur;
		(*list.head).prev = list.cur;
		(*list.cur).next = list.head;
		(*list.cur).prev = list.head;
	}
	list

}

unsafe impl Sync for Node{}

impl List
{

	pub fn exec(&self)
	{
		//unsafe {((*self.cur).elem)() ;}
		let mut el = self.node_crate[0].next;
		while el != &self.node_crate[0] as *const Node as *mut Node
		{
			unsafe
			{
				((*el).elem)();
				el = (*el).next;
			}

		}
	}

    pub fn push(&mut self, func: fn())
    {
	    let mut i= 2;
	    while self.node_crate[i].elem != def_fn {i=i+1;}
	    let mut new_el = &mut self.node_crate[i] as *const Node as *mut Node;;
	    let head = &self.node_crate[0] as *const Node as *mut Node;

	    unsafe
		    {
			    (*new_el).elem = func;
			    (*new_el).next = head;
			    (*new_el).prev = (*head).prev;

			    (*(*head).prev).next = new_el;
			    (*head).prev = new_el;
		    }
    }

    pub fn pop(&mut self, id: usize)
    {
		self.node_crate[id].elem=(def_fn );
	    unsafe
		    {
			    (*self.node_crate[id].prev).next    = self.node_crate[id].next;
			    (*self.node_crate[id].next).prev    = self.node_crate[id].prev;
	        }
    }
}

impl Iterator for List {
	type Item =  *const Node;

	fn next(&mut self) -> Option<Self::Item>
	{
		unsafe {self.cur = (*self.cur).next;}
		(Some(self.cur as *mut Node))
	}
}


