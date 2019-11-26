#![allow(dead_code)]
#![feature(try_from)]
#![feature(trait_alias)]
#![feature(type_ascription)]
#![feature(async_await)]
#![feature(async_closure)]

extern crate specs;
#[macro_use]
extern crate cascade;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tokio;
#[macro_use]
extern crate futures;
extern crate bytes;
#[macro_use]
pub extern crate cpython;

pub mod network;
pub mod ecs;
pub mod script;

#[cfg(test)]
mod tests;