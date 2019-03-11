//! This crate contains most python logic.
//!
//! - Compilation
//! - Bytecode
//! - Import mechanics
//! - Base objects

#[macro_use]
extern crate bitflags;
#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[cfg(target_arch = "wasm32")]
extern crate base64;
#[macro_use]
extern crate lazy_static;
extern crate lexical;
#[macro_use]
extern crate log;
// extern crate env_logger;
extern crate num_bigint;
extern crate num_complex;
extern crate num_integer;
extern crate num_traits;
extern crate serde;
extern crate serde_json;
extern crate statrs;
extern crate indexmap;

extern crate rustpython_parser;

//extern crate eval; use eval::eval::*;
// use py_code_object::{Function, NativeType, PyCodeObject};

// This is above everything else so that the defined macros are available everywhere
#[macro_use]
pub mod macros;

mod builtins;
pub mod bytecode;
pub mod compile;
pub mod error;
pub mod eval;
mod exceptions;
pub mod format;
mod frame;
pub mod import;
pub mod obj;
pub mod pyobject;
pub mod stdlib;
mod sysmodule;
mod traceback;
pub mod util;
mod vm;

// pub use self::pyobject::Executor;
pub use self::exceptions::print_exception;
pub use self::vm::VirtualMachine;
