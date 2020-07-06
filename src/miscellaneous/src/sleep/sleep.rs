use time::time_h::*;
#[path = "../../../socs/nordic/nrf52840/desc.rs"]
 mod desc;
pub fn sleep<T: Duration>(dur: &T)
{
    let per = (1_000_000_000 / desc::FREQ_CORE) as i32;
    let mut time:i32= dur.as_nanos() as i32;
    while time>0
    {
        time-= per;
    }

}