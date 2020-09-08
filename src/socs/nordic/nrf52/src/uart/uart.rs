use peripheral_access;

use peripheral_access::uarte0;

use gestalt_reference;
use crate::gpio::gpio;

use core::borrow::Borrow;
use gestalt_reference::uart::GestaltUart;
use gestalt_reference::interface::DMAGestaltInterface;


#[allow(dead_code)]
#[derive(Debug)]
pub enum UartInst
{
    Uart0		= 0x4000_2000,
    Uart1		= 0x4002_8000,
}

pub enum  Baudrate
{
    ///1200 baud (actual rate: 1205)
    Baud1200	= 0x0004F000,

    ///2400 baud (actual rate: 2396)
    Baud2400	= 0x0009D000,
    ///4800 baud (actual rate: 4808)
    Baud4800	= 0x0013B000,
    ///9600 baud (actual rate: 9598)
    Baud9600	= 0x00275000,
    ///14400 baud (actual rate: 14401)
    Baud14400	= 0x003AF000,
    ///19200 baud (actual rate: 19208)
    Baud19200	= 0x004EA000,
    ///28800 baud (actual rate: 28777)
    Baud28800	= 0x0075C000,
    ///31250 baud
    Baud31250	= 0x00800000,
    ///38400 baud (actual rate: 38369)
    Baud38400	= 0x009D0000,
    ///56000 baud (actual rate: 55944)
    Baud56000	= 0x00E50000,
    ///57600 baud (actual rate: 57554)
    Baud57600	= 0x00EB0000,
    ///76800 baud (actual rate: 76923)
    Baud76800	= 0x013A9000,
    ///115200 baud (actual rate: 115108)
    Baud115200	= 0x01D60000,
    ///230400 baud (actual rate: 231884)
    Baud230400	= 0x03B00000,
    ///250000 baud
    Baud250000	= 0x04000000,
    ///460800 baud (actual rate: 457143)
    Baud460800	= 0x07400000,
    ///921600 baud (actual rate: 941176)
    Baud921600	= 0x0F000000,
    ///1 Mega baud
    Baud1M		= 0x10000000,
}
pub trait Nrf52Uart
{
    fn enable(&self, _:bool);
}
pub struct Uart
{
    pub base: *const uarte0::RegisterBlock,
    pub write_type: fn(&Uart),
}

impl gestalt_reference::interface::DMAGestaltInterface for Uart
{
    fn dma_write(&self){unsafe {(*self.base).tasks_starttx.write(|w|{w.bits(1)})}}
    fn dma_read(&self){unsafe {(*self.base).tasks_startrx.write(|w|{w.bits(1)})}}
}

impl gestalt_reference::interface::GestaltInerface for Uart
{
    fn write(&self)
    {
        unsafe
            {
                (*self.base).events_endtx.write(|w|{w.bits(0)});
            }
        self.dma_write();
    }
    fn read(&self){(self.write_type)(self);}
}

impl Nrf52Uart for Uart
{
    fn enable(&self, state: bool)
    {
        unsafe {(*self.base).enable.write(|w|{w.bits(state as u32*8)});}
    }
}

pub fn new	(inst: UartInst) -> Uart
{
    let i = Uart{base : ((inst as u32)
        as  *const uarte0::RegisterBlock), write_type: Uart::dma_write};
    i
}

impl gestalt_reference::uart::GestaltUart for Uart
{
    type TxPin	= gpio::GpioPin;
    type RxPin	= gpio::GpioPin;
    type Baud	= self::Baudrate;
    type TxBuf	= u8;
    type RxBuf	= u8;

    fn set_rx       (&self, pin: Self::RxPin)
    {
        unsafe {(*self.base).psel.rxd.write(|w|{w.bits(pin as u32)});}
    }

    fn set_tx       (&self, pin: Self::TxPin)
    {
        unsafe {(*self.base).psel.txd.write(|w|{w.bits(pin as u32)});}
    }

    fn set_baud     (&self, baud: Self::Baud)
    {
        unsafe {(*self.base).baudrate.write(|w|{w.bits(baud as u32)})};
    }

    fn set_tx_buf   (&self, arr: &[Self::TxBuf])
    {
        unsafe
        {
            (*self.base).txd.maxcnt.write	(|w| {w.bits(arr.len() as u32)});
            (*self.base).txd.ptr.write		(|w| {w.bits(arr.as_ptr() as u32)});
        }
    }

    fn set_rx_buf   (&self, arr: &[Self::RxBuf])
    {
        unsafe
        {
            (*self.base).rxd.maxcnt.write	(|w| {w.bits(arr.len() as u32)});
            (*self.base).rxd.ptr.write		(|w| {w.bits(arr.as_ptr() as u32)});
        }
    }
}

