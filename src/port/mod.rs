use core::num::NonZeroU16;

use x86::io;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Port {
    portno: NonZeroU16,
}

impl Port {
    pub const unsafe fn new(portno: u16) -> Port {
        Port {
            portno: NonZeroU16::new_unchecked(portno),
        }
    }

    pub unsafe fn into_siblings(self) -> (Port, Port) {
        let portno = self.portno.get();
        (Port::new(portno), Port::new(portno + 1))
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe { io::inb(self.portno.get()) }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe { io::outb(self.portno.get(), byte) }
    }

    pub fn read_word(&mut self) -> u16 {
        unsafe { io::inw(self.portno.get()) }
    }

    pub fn write_word(&mut self, word: u16) {
        unsafe { io::outw(self.portno.get(), word) }
    }

    pub fn read_dword(&mut self) -> u32 {
        unsafe { io::inl(self.portno.get()) }
    }

    pub fn write_dword(&mut self, dword: u32) {
        unsafe { io::outl(self.portno.get(), dword) }
    }

    pub fn read(&mut self, buf: &mut [u8]) {
        for x in 0..buf.len() {
            buf[x] = self.read_byte();
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        for byte in buf.iter() {
            self.write_byte(*byte);
        }
    }

    pub unsafe fn read_byte_unsafe(&self) -> u8 {
        io::inb(self.portno.get())
    }

    pub unsafe fn read_word_unsafe(&self) -> u16 {
        io::inw(self.portno.get())
    }

    pub unsafe fn read_dword_unsafe(&self) -> u32 {
        io::inl(self.portno.get())
    }

    pub unsafe fn read_unsafe(&self, buf: &mut [u8]) {
        for x in 0..buf.len() {
            buf[x] = io::inb(self.portno.get());
        }
    }
}
