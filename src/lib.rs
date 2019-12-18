#![allow(dead_code)]
#![feature(trait_alias)]
#![feature(type_ascription)]
#![feature(async_closure)]
//#![warn(clippy::pedantic)]

extern crate specs;
extern crate cascade;
extern crate lazy_static;
extern crate tokio;
extern crate futures;
extern crate bytes;
pub extern crate cpython;

pub mod network;
pub mod ecs;
pub mod script;
pub mod logger;

#[cfg(test)]
mod tests;