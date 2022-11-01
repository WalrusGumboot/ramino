#![deny(missing_docs)]

//! ramino is a Rust engine for the traditional Italian card game Ramino.
//! It can be used both as a referee for the rules and an emulator for a game.

extern crate rand;

/// The amount of cards dealt to create a Hand at the beginning of the game.
pub const HAND_SIZE: u8 = 13;

pub mod card;
pub mod hand;
pub mod utils;
pub mod run;

pub mod tests;
