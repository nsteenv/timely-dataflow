#![feature(unsafe_destructor)]
#![feature(core)]
#![feature(std_misc)]
#![feature(collections)]
#![feature(io)]
#![feature(hash)]
#![feature(libc)]

#![allow(dead_code)]
#![allow(missing_copy_implementations)]

// extern crate serialize;
extern crate core;

pub mod networking;
// pub mod serialization;
pub mod progress;
pub mod example;
pub mod communication;
