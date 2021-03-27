#![feature(const_fn)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![feature(decl_macro)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate bitflags;
extern crate linked_list_allocator;
extern crate multiboot2;
#[macro_use]
extern crate once;
extern crate rlibc;
extern crate spin;
extern crate x86;
extern crate raw_cpuid;

use x86::irq;

pub mod macros;
#[cfg_attr(target_arch = "x86", path = "arch/x86/mod.rs")]
pub mod arch;
pub mod panic;
pub mod mem;
pub mod port;
pub mod drivers;
pub mod syscall;

#[path = "arch/x86/mod.rs"]
#[cfg(rustfmt)]
pub mod arch_x86;

use arch::Kinfo;
use drivers::vga;
use drivers::pic;
use macros::*;

// global_allocator doesn't work in modules
// tracking issue: #27389
// issue: #44113
use linked_list_allocator::LockedHeap;
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

pub fn kmain(kinfo: &Kinfo) {
    kprint!("paging... ");
    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");

    kprint!("global descriptor table... ");
    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");

    kprint!("interrupt descriptor table... ");
    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");

    kprint!("memory areas... ");
    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");
    kprintln!(
        "available memory: {}MB",
        kinfo.free_memory / (1024 * 1024),
    );

    kprint!("kernel heap... ");
    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");

    kprintln!(
        "heap size: {}kB",
        kinfo.heap_size() / 1024,
    );

    kprint!("video graphics array driver... ");

    vga::init();

    kprintln!("{green}[OK]{reset}", green = "\x1b[32m", reset = "\x1b[0m");

    kprint!("keyboard driver... ");
    kprintln!(
        "{yellow}[SKIP]{reset}",
        yellow = "\x1b[33m",
        reset = "\x1b[0m"
    );

    kprint!("programmable interrupt controller... ");

    {
        pic::init();
        let mut pic = pic::handle();
        pic.0.set_all();
        pic.1.set_all();
    }

    kprintln!(
        "{green}[OK]{reset}",
        green = "\x1b[32m",
        reset = "\x1b[0m"
    );

    kprint!("cpuid... ");
    if kinfo.cpuid.is_some() {
        kprintln!(
            "{green}[AVAILABLE]{reset}",
            green = "\x1b[32m",
            reset = "\x1b[0m"
        );
    } else {
        panic!("[NOT AVAILABLE]");
    }

    unsafe {
        irq::enable();
    }

    kprint!("> ");
}
