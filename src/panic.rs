use core::{fmt::{Write}, panic::PanicInfo};

use drivers::vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::try_handle().map(|mut vga| {
        let _ = vga.write_str("\x1b[0;31mkernel panicked at '");

        if let Some(s) = info.payload().downcast_ref::<&str>() {
            let _ = vga.write_str(s);
        } else {
            let _ = vga.write_str("<payload not a &str>");
        }

        if let Some(l) = info.location() {
            let _ = write!(vga, "', {}:{}\x1b[0m", l.file(), l.line());
        } else {
            let _ = write!(vga, "', <location unavailable>\x1b[0m");
        }
    });

    loop {}
}

