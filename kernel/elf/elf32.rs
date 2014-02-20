use core::mem::transmute;
use core::ptr::offset;
use core::c_types::{c_ushort, c_uint, c_int, c_ulong, c_long};

use util::int;
use util::ptr::mut_offset;

// rust-bindgen generated bindings
pub type Elf32_Half = c_ushort;
pub type Elf32_Word = c_uint;
pub type Elf32_Sword = c_int;
pub type Elf32_Xword = c_ulong;
pub type Elf32_Sxword = c_long;
pub type Elf32_Addr = c_uint;
pub type Elf32_Off = c_uint;
pub type Elf32_Section = c_ushort;
pub type Elf32_Symndx = c_uint;
type c_uchar = u8;
type c_void = uint;

#[packed]
pub struct Elf32_Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Elf32_Half,
    e_machine: Elf32_Half,
    e_version: Elf32_Word,
    e_entry: Elf32_Addr,
    e_phoff: Elf32_Off,
    e_shoff: Elf32_Off,
    e_flags: Elf32_Word,
    e_ehsize: Elf32_Half,
    e_phentsize: Elf32_Half,
    e_phnum: Elf32_Half,
    e_shentsize: Elf32_Half,
    e_shnum: Elf32_Half,
    e_shstrndx: Elf32_Half,
}

#[packed]
pub struct Elf32_Phdr {
    p_type: super::HeaderType,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr,
    p_filesz: Elf32_Word,
    p_memsz: Elf32_Word,
    p_flags: Elf32_Word,
    p_align: Elf32_Word,
}

#[packed]
pub struct Elf32_Shdr {
    sh_name: Elf32_Word,
    sh_type: Elf32_Word,
    sh_flags: Elf32_Word,
    sh_addr: Elf32_Addr,
    sh_offset: Elf32_Off,
    sh_size: Elf32_Word,
    sh_link: Elf32_Word,
    sh_info: Elf32_Word,
    sh_addralign: Elf32_Word,
    sh_entsize: Elf32_Word,
}

pub struct Elf32_Sym {
    st_name: Elf32_Word,
    st_value: Elf32_Addr,
    st_size: Elf32_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf32_Section,
}

pub struct Elf32_Syminfo {
    si_boundto: Elf32_Half,
    si_flags: Elf32_Half,
}

pub struct Elf32_Rel {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
}

pub struct Elf32_Rela {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
    r_addend: Elf32_Sword,
}

pub struct Union_Unnamed1 {
    data: [c_uchar, ..4u],
}
impl Union_Unnamed1 {
    pub fn d_val(&mut self) -> *mut Elf32_Word {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf32_Addr {
        unsafe { transmute(self) }
    }
}
pub struct Elf32_Dyn {
    d_tag: Elf32_Sword,
    d_un: Union_Unnamed1,
}

pub struct Elf32_Verdef {
    vd_version: Elf32_Half,
    vd_flags: Elf32_Half,
    vd_ndx: Elf32_Half,
    vd_cnt: Elf32_Half,
    vd_hash: Elf32_Word,
    vd_aux: Elf32_Word,
    vd_next: Elf32_Word,
}

pub struct Elf32_Verdaux {
    vda_name: Elf32_Word,
    vda_next: Elf32_Word,
}

pub struct Elf32_Verneed {
    vn_version: Elf32_Half,
    vn_cnt: Elf32_Half,
    vn_file: Elf32_Word,
    vn_aux: Elf32_Word,
    vn_next: Elf32_Word,
}

pub struct Elf32_Vernaux {
    vna_hash: Elf32_Word,
    vna_flags: Elf32_Half,
    vna_other: Elf32_Half,
    vna_name: Elf32_Word,
    vna_next: Elf32_Word,
}

pub struct Union_Unnamed3 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed3 {
    pub fn a_val(&mut self) -> *mut c_long {
        unsafe { transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        unsafe { transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern "C" fn() {
        unsafe { transmute(self) }
    }
}
pub struct Elf32_auxv_t {
    a_type: c_int,
    a_un: Union_Unnamed3,
}

pub struct Elf32_Nhdr {
    n_namesz: Elf32_Word,
    n_descsz: Elf32_Word,
    n_type: Elf32_Word,
}

pub struct Struct_Unnamed5 {
    gt_current_g_value: Elf32_Word,
    gt_unused: Elf32_Word,
}
pub struct Struct_Unnamed6 {
    gt_g_value: Elf32_Word,
    gt_bytes: Elf32_Word,
}
pub struct Elf32_gptab {
    data: [c_uchar, ..8u],
}
impl Elf32_gptab {
    pub fn gt_header(&mut self) -> *mut Struct_Unnamed5 {
        unsafe { transmute(self) }
    }
    pub fn gt_entry(&mut self) -> *mut Struct_Unnamed6 {
        unsafe { transmute(self) }
    }
}
pub struct Elf32_RegInfo {
    ri_gprmask: Elf32_Word,
    ri_cprmask: [Elf32_Word, ..4u],
    ri_gp_value: Elf32_Sword,
}
pub struct Elf_Options {
    kind: c_uchar,
    size: c_uchar,
    section: Elf32_Section,
    info: Elf32_Word,
}
pub struct Elf_Options_Hw {
    hwp_flags1: Elf32_Word,
    hwp_flags2: Elf32_Word,
}
pub struct Elf32_Lib {
    l_name: Elf32_Word,
    l_time_stamp: Elf32_Word,
    l_checksum: Elf32_Word,
    l_version: Elf32_Word,
    l_flags: Elf32_Word,
}

pub type Elf32_Conflict = Elf32_Addr;

impl super::Ehdr for Elf32_Ehdr {
    // unsafe fn load(&self, buffer: *u8) -> extern "C" fn();
    unsafe fn load(&self) -> extern "C" fn() {
        //TODO: Verify file integrity
        let buffer: *u8 = transmute(self);
        let pheader = offset(buffer, self.e_phoff as int) as *Elf32_Phdr;

        int::range(0, self.e_phnum as uint, |_| {
            match (*pheader).p_type {
                PT_LOAD => (*pheader).load(buffer),
                _ => {}
            }
        });
        // return entry address
        transmute(self.e_entry)
    }
}
