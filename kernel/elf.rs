use platform::{io, cpu};
use core::mem::{transmute, size_of};
use core::ptr::offset;
use core::c_types::*;
use kernel::int;
use kernel::rt::{memset, memcpy};

// rust-bindgen generated bindings
pub type Elf32_Half = c_ushort;
pub type Elf64_Half = c_ushort;
pub type Elf32_Word = c_uint;
pub type Elf32_Sword = c_int;
pub type Elf64_Word = c_uint;
pub type Elf64_Sword = c_int;
pub type Elf32_Xword = c_ulong;
pub type Elf32_Sxword = c_long;
pub type Elf64_Xword = c_ulong;
pub type Elf64_Sxword = c_long;
pub type Elf32_Addr = c_uint;
pub type Elf64_Addr = c_ulong;
pub type Elf32_Off = c_uint;
pub type Elf64_Off = c_ulong;
pub type Elf32_Section = c_ushort;
pub type Elf64_Section = c_ushort;
pub type Elf32_Symndx = c_uint;
pub type Elf64_Symndx = c_ulong;
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
pub struct Elf64_Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Elf64_Half,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half,
}
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
pub struct Elf64_Shdr {
    sh_name: Elf64_Word,
    sh_type: Elf64_Word,
    sh_flags: Elf64_Xword,
    sh_addr: Elf64_Addr,
    sh_offset: Elf64_Off,
    sh_size: Elf64_Xword,
    sh_link: Elf64_Word,
    sh_info: Elf64_Word,
    sh_addralign: Elf64_Xword,
    sh_entsize: Elf64_Xword,
}
pub struct Elf32_Sym {
    st_name: Elf32_Word,
    st_value: Elf32_Addr,
    st_size: Elf32_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf32_Section,
}
pub struct Elf64_Sym {
    st_name: Elf64_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf64_Section,
    st_value: Elf64_Addr,
    st_size: Elf64_Xword,
}
pub struct Elf32_Syminfo {
    si_boundto: Elf32_Half,
    si_flags: Elf32_Half,
}
pub struct Elf64_Syminfo {
    si_boundto: Elf64_Half,
    si_flags: Elf64_Half,
}
pub struct Elf32_Rel {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
}
pub struct Elf64_Rel {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
}
pub struct Elf32_Rela {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
    r_addend: Elf32_Sword,
}
pub struct Elf64_Rela {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
    r_addend: Elf64_Sxword,
}

#[packed]
pub struct Elf32_Phdr {
    p_type: HeaderType,
    p_offset: Elf32_Off,
    p_vaddr: Elf32_Addr,
    p_paddr: Elf32_Addr,
    p_filesz: Elf32_Word,
    p_memsz: Elf32_Word,
    p_flags: Elf32_Word,
    p_align: Elf32_Word,
}
pub struct Elf64_Phdr {
    p_type: Elf64_Word,
    p_flags: Elf64_Word,
    p_offset: Elf64_Off,
    p_vaddr: Elf64_Addr,
    p_paddr: Elf64_Addr,
    p_filesz: Elf64_Xword,
    p_memsz: Elf64_Xword,
    p_align: Elf64_Xword,
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
pub struct Union_Unnamed2 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed2 {
    pub fn d_val(&mut self) -> *mut Elf64_Xword {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf64_Addr {
        unsafe { transmute(self) }
    }
}
pub struct Elf64_Dyn {
    d_tag: Elf64_Sxword,
    d_un: Union_Unnamed2,
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
pub struct Elf64_Verdef {
    vd_version: Elf64_Half,
    vd_flags: Elf64_Half,
    vd_ndx: Elf64_Half,
    vd_cnt: Elf64_Half,
    vd_hash: Elf64_Word,
    vd_aux: Elf64_Word,
    vd_next: Elf64_Word,
}
pub struct Elf32_Verdaux {
    vda_name: Elf32_Word,
    vda_next: Elf32_Word,
}
pub struct Elf64_Verdaux {
    vda_name: Elf64_Word,
    vda_next: Elf64_Word,
}
pub struct Elf32_Verneed {
    vn_version: Elf32_Half,
    vn_cnt: Elf32_Half,
    vn_file: Elf32_Word,
    vn_aux: Elf32_Word,
    vn_next: Elf32_Word,
}
pub struct Elf64_Verneed {
    vn_version: Elf64_Half,
    vn_cnt: Elf64_Half,
    vn_file: Elf64_Word,
    vn_aux: Elf64_Word,
    vn_next: Elf64_Word,
}
pub struct Elf32_Vernaux {
    vna_hash: Elf32_Word,
    vna_flags: Elf32_Half,
    vna_other: Elf32_Half,
    vna_name: Elf32_Word,
    vna_next: Elf32_Word,
}
pub struct Elf64_Vernaux {
    vna_hash: Elf64_Word,
    vna_flags: Elf64_Half,
    vna_other: Elf64_Half,
    vna_name: Elf64_Word,
    vna_next: Elf64_Word,
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
pub struct Union_Unnamed4 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed4 {
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
pub struct Elf64_auxv_t {
    a_type: c_long,
    a_un: Union_Unnamed4,
}
pub struct Elf32_Nhdr {
    n_namesz: Elf32_Word,
    n_descsz: Elf32_Word,
    n_type: Elf32_Word,
}
pub struct Elf64_Nhdr {
    n_namesz: Elf64_Word,
    n_descsz: Elf64_Word,
    n_type: Elf64_Word,
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
pub struct Elf64_Lib {
    l_name: Elf64_Word,
    l_time_stamp: Elf64_Word,
    l_checksum: Elf64_Word,
    l_version: Elf64_Word,
    l_flags: Elf64_Word,
}
pub type Elf32_Conflict = Elf32_Addr;

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

impl Elf32_Phdr {
    unsafe fn load(&self, header: *Elf32_Ehdr) {
        use cpu::paging;
        let vaddr = self.p_vaddr as *mut u8;
        let p_offset = self.p_offset as int;
        let p_filesz = self.p_filesz as int;

        paging::map(vaddr);

        if self.p_memsz > self.p_filesz {
            memset(offset(vaddr as *u8, p_offset + p_filesz) as *mut u8, 0, self.p_memsz - self.p_filesz);
        }
        memcpy(vaddr, offset(header as *u8, p_offset) as *u8, p_filesz);
    }
}

pub unsafe fn load_elf(header: *Elf32_Ehdr) -> extern "C" fn() {
    //TODO: Verify file integrity
    let pheader = offset(header as *u8, (*header).e_phoff as int) as *Elf32_Phdr;

    int::range(0, (*header).e_phnum as uint, |i| {
        match (*pheader).p_type {
            PT_LOAD => (*pheader).load(header),
            _ => {}
        }
    });
    // return entry address
    transmute((*header).e_entry)
}

pub fn exec() {
    unsafe {
        let ptr = &initram as *u8 as *Elf32_Ehdr;
        let entry = load_elf(ptr);
        // jump into the module
        entry();
    }
}

extern { static initram: u8; }
