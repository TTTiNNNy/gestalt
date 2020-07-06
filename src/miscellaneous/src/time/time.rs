 use time::time_h::*;

 impl Duration for Time
 {
    fn as_secs(&self) -> u32
    {
        self.secs
    }

    fn as_millis(&self) -> u32
    {
        (self.secs * 1000) + (self.nanos / 1000_000)
    }

    fn as_micros(&self) -> u32
    {
        (self.secs * 1000_000) + (self.nanos / 1000)
    }

    fn as_nanos(&self) -> u32
    {
        self.nanos + self.secs * 1000_000_000
    }
 }