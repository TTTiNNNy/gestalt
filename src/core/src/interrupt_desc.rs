use crate::interrupt_manager;
use interrupt_manager::Vector;
use crate::sheduler::Node;
use crate::sheduler::def_fn;
use peripheral_access;
use core::any::Any;

#[path = "../../desc.rs"]
mod desc;

pub use peripheral_access::Interrupt;


fn POWER_CLOCK()    { unsafe { (interrupt_sectors_arr[0]).exec(); } }
fn RADIO()          { unsafe { (interrupt_sectors_arr[1]).exec(); } }
fn UARTE0_UART0()   { unsafe { (interrupt_sectors_arr[2]).exec(); } }
fn SPI0_TWI0()      { unsafe { (interrupt_sectors_arr[3]).exec(); } }
fn SPI1_TWI1()      { unsafe { (interrupt_sectors_arr[4]).exec(); } }
fn NFCT()           { unsafe { (interrupt_sectors_arr[5]).exec(); } }
fn GPIOTE()         { unsafe { (interrupt_sectors_arr[6]).exec(); } }
fn SAADC()          { unsafe { (interrupt_sectors_arr[7]).exec(); } }
fn TIMER0()         { unsafe { (interrupt_sectors_arr[8]).exec(); } }
fn TIMER1()         { unsafe { (interrupt_sectors_arr[9]).exec(); } }
fn TIMER2()         { unsafe { (interrupt_sectors_arr[10]).exec(); } }
fn RTC0()           { unsafe { (interrupt_sectors_arr[11]).exec(); } }
fn TEMP()           { unsafe { (interrupt_sectors_arr[12]).exec(); } }
fn RNG()            { unsafe { (interrupt_sectors_arr[13]).exec(); } }

pub fn qwe(){}
static  interrupt_arr: [ crate::sheduler::Node; desc::INTERRUPT_NUM * desc::TASK_NUM] =
	[
		Node { elem: def_fn, next: 0 as *mut Node, prev: 0 as *mut Node };
		desc::INTERRUPT_NUM * desc::TASK_NUM
	];

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; desc::INTERRUPT_NUM] = [
	Vector {
		_handler: POWER_CLOCK,
	},
	Vector { _handler: RADIO },
	Vector {
		_handler: UARTE0_UART0,
	},
	Vector {
		_handler: SPI0_TWI0,
	},
	Vector {
		_handler: SPI1_TWI1,
	},
	Vector { _handler: NFCT },
	Vector { _handler: GPIOTE },
	Vector { _handler: SAADC },
	Vector { _handler: TIMER0 },
	Vector { _handler: TIMER1 },
	Vector { _handler: TIMER2 },
	Vector { _handler: RTC0 },
	Vector { _handler: TEMP },
	Vector { _handler: RNG },

];
pub static POWER_CLOCK_TASK: [Node; desc::TASK_NUM] =
	[Node{ elem: def_fn , next: 0 as *mut Node, prev: 0 as *mut Node }; desc::TASK_NUM];

pub static TASK_CONT: [Node; desc::TASK_NUM] =
	[Node{ elem: def_fn , next: 0 as *mut Node, prev: 0 as *mut Node }; desc::TASK_NUM];

pub(crate) static mut interrupt_sectors_arr:[crate::sheduler::List;desc::INTERRUPT_NUM] =
	[crate::sheduler::List
	{
		node_crate: [Node
		{
			elem: def_fn ,
			next: 0 as *mut Node,
			prev: 0 as *mut Node
		}; desc::TASK_NUM],

		head: (0 as *const Node) as *mut Node,
		cur: (0 as *const Node) as *mut Node

	}; desc::INTERRUPT_NUM];


