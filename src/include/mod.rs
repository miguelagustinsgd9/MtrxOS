#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub use core::arch::asm;
pub use core::fmt::Write;
pub use core::slice;
pub use uefi::prelude::*;
pub use uefi::proto::console::gop::GraphicsOutput;
pub use uefi::proto::console::text::{Color, Key, ScanCode};
pub use uefi::table::runtime::ResetType;


// Addons

// NULL

// ---

// Drivers

// NULL

// ---

// Shell

#[path = "shell/shell.rs"]
pub mod shell;

// ---

// Commands

#[path = "commands/commands.rs"]
pub mod commands;

// ---

// Audio

#[path = "audio/song1.rs"]
pub mod song1;

// ---

// Kernel

#[path = "kernel/ocpu.rs"]
pub mod ocpu;

#[path = "kernel/sound.rs"]
pub mod sound;

#[path = "kernel/panic.rs"]
pub mod panic;

// ---

// LADE

#[path = "lade/lade.rs"]
pub mod lade;

// ---

// LADE Apps

#[path = "laapps/monitor.rs"]
pub mod monitor;

#[path = "laapps/calc.rs"]
pub mod calculadora;

// Shell Apps

#[path = "shapps/zim.rs"]
pub mod zim;

// ---

// Games

#[path = "games/buggy.rs"]
pub mod buggy;

#[path = "games/raycaster.rs"]
pub mod raycaster;

#[path = "games/flappy.rs"]
pub mod flappy;

#[path = "games/geometry.rs"]
pub mod geometry;

// ---

// Sys

#[path = "sys/sysconf.rs"]
pub mod sysconf;

#[path = "sys/intro.rs"]
pub mod intro;

// ---

// UI

#[path = "ui/sdmlle1.rs"]
pub mod sdmlle1;

// ---

// Simple DirectMedia Layer Lade

#[path = "sdmll/sdmll.rs"]
pub mod sdmll;

// ---

// GL

#[path = "gl/mtrx_gl.rs"]
pub mod mtrx_gl;

#[path = "gl/gltest.rs"]
pub mod gltest;

#[path = "gl/gltest2.rs"]
pub mod gltest2;

#[path = "gl/gltest3.rs"]
pub mod gltest3;

// ---

pub use crate::mtrx_gl::{MtrxGl, Vec2};
