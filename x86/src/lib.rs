pub mod debug;
mod icache;
pub mod ops;
mod registers;
mod x86;

pub use x86::{CPU, X86};
