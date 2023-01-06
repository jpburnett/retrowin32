//! Types exposed by the Windows API.

use crate::memory::Pod;

use super::shims::{FromX86, ToX86};

pub type WORD = u16;
pub type DWORD = u32;

// Handles like HFILE etc. are just u32s.
// I looked at using PhantomData to declare distinct types but I think using
// a macro generates an equivalent amount of code and it's way less confusing
// to work with.
macro_rules! declare_handle {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        #[repr(transparent)]
        pub struct $name(pub u32);
        unsafe impl Pod for $name {}
        impl FromX86 for $name {
            fn from_raw(raw: u32) -> Self {
                $name(raw)
            }
        }
        impl ToX86 for $name {
            fn to_raw(&self) -> u32 {
                self.0
            }
        }
    };
}

declare_handle!(HFILE);
declare_handle!(HMODULE);

/// UTF-16 string view.
pub struct Str16<'a>(&'a [u16]);

impl<'a> Str16<'a> {
    pub fn from_nul_term(mem: &'a [u16]) -> Self {
        let end = mem.iter().position(|&c| c == 0).unwrap();
        Str16(&mem[..end])
    }

    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|&c| {
                if c > 0xFF {
                    // TODO
                    panic!("unhandled non-ascii {:?}", char::from_u32(c as u32));
                }
                c as u8 as char
            })
            .collect()
    }
}

impl<'a> std::fmt::Debug for Str16<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.to_string()))
    }
}
