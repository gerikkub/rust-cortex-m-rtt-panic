
extern crate cortex_m_rtt;

use core::fmt::{self, Write};

use cortex_m_rtt::rtt;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    let mut my_rtt = rtt::get_chan(0);

    my_rtt.write_fmt(format_args!("{}\n", info));

    loop {}

}
