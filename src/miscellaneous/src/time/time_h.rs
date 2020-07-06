

pub trait Duration
{
    fn as_secs  (&self) -> u32;
    fn as_millis(&self) -> u32;
    fn as_micros(&self) -> u32;
    fn as_nanos (&self) -> u32;
}

pub struct Time {
    pub(crate) secs:   u32,
    pub(crate) nanos:  u32,
}



pub const fn from_secs(secs: u32) -> Time{Time{secs:secs,nanos:0}}

pub const fn from_millis(millis: u32) -> Time{Time{secs:(millis/1000), nanos:((millis%1000) * 1000_000) as u32}}

pub const fn from_micros(micros: u32) -> Time{Time{secs:(micros/1000_000), nanos:((micros%1000_000) * 1000) as u32}}

pub const fn from_nanos(nanos: u32) -> Time{Time{secs:0,nanos:nanos}}