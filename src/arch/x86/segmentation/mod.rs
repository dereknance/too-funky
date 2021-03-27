pub mod gdt;

use x86::segmentation::{load_ds, load_es, load_fs, load_gs, load_ss,
                                load_cs, SegmentSelector};

use self::gdt::Gdtr;

pub unsafe fn lgdt(gdtr: &Gdtr) {
    llvm_asm!("lgdtl   $0" : : "*m"(gdtr) : "memory" : "volatile");
}

pub unsafe fn reload_segments(code: u16, data: u16) {
    load_cs(SegmentSelector::from_raw(code));
    load_ds(SegmentSelector::from_raw(data));
    load_es(SegmentSelector::from_raw(data));
    load_fs(SegmentSelector::from_raw(data));
    load_gs(SegmentSelector::from_raw(data));
    load_ss(SegmentSelector::from_raw(data));
}
