use super::DynamicSectionType;

pub type Word = libc::Elf32_Word;
// manual impl, signed word is i32;
pub type SignedWord = i32;
pub type Half = libc::Elf32_Half;
pub type Addr = libc::Elf32_Addr;
pub type ProgramHeader = libc::Elf32_Phdr;

#[repr(C)]
#[derive(Debug)]
pub struct DynEntry {
    pub d_tag: self::Word,
    /// Either a value (Elf64_Xword) or an address (Elf64_Addr)
    pub d_val_ptr: self::Word,
}

#[repr(C)]
#[derive(Debug)]
pub struct DynSym {
    pub st_name: self::Word,
    pub st_value: self::Addr,
    pub st_size: self::Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: self::Half,
}

#[repr(C)]
#[derive(Debug)]
pub struct DynRel {
    pub r_offset: self::Addr,
    pub r_info: self::Word,
}

impl DynRel {
    pub fn symbol_index(&self) -> self::Word {
        (self.r_info >> 8) as self::Word
    }
    pub fn symbol_type(&self) -> self::Word {
        (self.r_info & 0x0ff) as self::Word
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DynRela {
    pub r_offset: self::Addr,
    pub r_info: self::Word,
    pub r_addend: self::SignedWord,
}

impl DynRela {
    pub fn symbol_index(&self) -> self::Word {
        (self.r_info >> 8) as self::Word
    }
    pub fn symbol_type(&self) -> self::Word {
        (self.r_info & 0x0ff) as self::Word
    }
}

/// An unknown Dynamic Section Type was observed
#[derive(Debug, thiserror::Error)]
#[error("Unknown Dynamic section type witnessed: `{0}`")]
pub struct DynTypeError(self::Word);

impl TryFrom<self::Word> for DynamicSectionType {
    type Error = DynTypeError;
    fn try_from(value: self::Word) -> Result<Self, Self::Error> {
        use DynamicSectionType::*;
        Ok(match value {
            0 => DT_NULL,

            2 => DT_PLTRELSZ,
            3 => DT_PLTGOT,
            20 => DT_PLTREL,

            5 => DT_STRTAB,
            6 => DT_SYMTAB,
            11 => DT_SYMENT,

            17 => DT_REL,
            18 => DT_RELSZ,
            19 => DT_RELENT,

            7 => DT_RELA,
            8 => DT_RELASZ,
            9 => DT_RELAENT,

            10 => DT_STRSZ,
            23 => DT_JMPREL,

            tag => return Err(DynTypeError(tag)),
        })
    }
}
