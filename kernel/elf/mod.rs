use core::ptr::{copy_nonoverlapping_memory, set_memory};
use core::mem::{transmute, size_of};
use core::ptr::offset;
use core::c_types::*;
use core::option::{Option, Some, None};

use kernel::int;
use kernel::ptr::mut_offset;
use platform::{io, cpu};

use self::elf32::Elf32_Ehdr;
use self::elf64::Elf64_Ehdr;

mod elf32;
mod elf64;

#[repr(u32)]
enum HeaderType {
    PT_NULL = 0,
    PT_LOAD = 1,
    PT_DYNAMIC = 2,
    PT_INTERP = 3,
    PT_NOTE = 4,
    PT_SHLIB = 5,
    PT_PHDR = 6,
    PT_TLS = 7,
    PT_LOOS = 0x60000000,
    PT_HIOS = 0x6fffffff,
    PT_LOPROC = 0x70000000,
    PT_HIPROC = 0x7fffffff
}

pub trait Ehdr {
    unsafe fn load(&self) -> extern "C" fn();
}

#[packed]
struct ELFIdent {
    ei_mag: [u8, ..4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8, ..7]
}

impl elf32::Elf32_Phdr {
    unsafe fn load(&self, buffer: *u8) {
        use kernel::memory::virtual;

        let vaddr = self.p_vaddr as *mut u8;
        let mem_size = self.p_memsz as uint;
        let file_pos = self.p_offset as int;
        let file_size = self.p_filesz as uint;

        virtual::map(vaddr);

        copy_nonoverlapping_memory(vaddr, offset(buffer, file_pos), file_size);
        set_memory(mut_offset(vaddr, file_pos + file_size as int), 0, mem_size - file_size);
    }
}

impl elf64::Elf64_Phdr {
    unsafe fn load(&self, buffer: *u8) {
        use kernel::memory::virtual;

        let vaddr = self.p_vaddr as *mut u8;
        let mem_size = self.p_memsz as uint;
        let file_pos = self.p_offset as int;
        let file_size = self.p_filesz as uint;

        virtual::map(vaddr);

        copy_nonoverlapping_memory(vaddr, offset(buffer, file_pos), file_size);
        set_memory(mut_offset(vaddr, file_pos + file_size as int), 0, mem_size - file_size);
    }
}

impl ELFIdent {
    /*
    unsafe fn file(&self) -> Option<&Ehdr> {
        // TODO: check validity, check endianness
        let e32: &Elf32_Ehdr = transmute(self);
        let e64: &Elf64_Ehdr = transmute(self);
        match self.ei_class {
            1 => Some(e32 as &Ehdr),
            2 => Some(e64 as &Ehdr),
            _ => None
        }
    }
    */
    unsafe fn load(&self) -> Option<extern "C" fn()> {
        // TODO: check validity, check endianness
        let e32: &Elf32_Ehdr = transmute(self);
        let e64: &Elf64_Ehdr = transmute(self);
        match self.ei_class {
            1 => Some(e32.load()),
            2 => Some(e64.load()),
            _ => None
        }
    }
}

pub fn exec(buffer: *u8) {
    unsafe {
        let ident: &ELFIdent = transmute(buffer);
        /*ident.file().map(|header| {
            // jump into the module
            header.load()();
        });*/
        ident.load().map(|e| { e() });
    }
}
