#![no_std]
//#![feature(const_mut_refs)]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod instruction;
pub mod state;

pinocchio_pubkey::declare_id!("GTaQUFVc7fmZ4XHiNJtPJoHj3AJrUjckcNCFnCDVBVxe");