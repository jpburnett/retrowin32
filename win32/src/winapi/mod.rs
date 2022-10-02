use crate::x86;
use crate::X86;

pub mod kernel32;
pub mod user32;

// winapi is stdcall, which means args are right to left and callee-cleaned.
// The caller of winapi functions is responsible for pushing/popping the
// return address, because some callers actually 'jmp' directly.

// This winapi macro generates shim wrappers of winapi functions, taking their
// input args off the stack and forwarding their return values via eax.
// It also generates a resolve() function that maps symbol names to shims.

#[macro_export]
macro_rules! winapi {
    ($(fn $name:ident($($param:ident: $type:ident),* $(,)?);)*) => {
        mod shims {
            use super::X86;
            $(
                #[allow(non_snake_case)]
                pub fn $name(x86: &mut X86) {
                    $(let $param: $type = x86.pop();)*
                    x86.regs.eax = super::$name(x86, $($param),*);
                }
            )*
        }

        pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
            Some(match name {
                $(stringify!($name) => shims::$name,)*
                _ => return None,
            })
        }
    }
}

pub fn resolve(dll: &str, sym: &str) -> Option<fn(&mut X86)> {
    match dll {
        "kernel32.dll" => kernel32::resolve(sym),
        "user32.dll" => user32::resolve(sym),
        _ => None,
    }
}

pub struct State {
    pub kernel32: kernel32::State,
}
impl State {
    pub fn new() -> Self {
        State {
            kernel32: kernel32::State::new(),
        }
    }
}