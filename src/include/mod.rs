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

// addons

// ---

// apps

#[path = "apps/notes.rs"]
pub mod notes;

#[path = "apps/sndmker.rs"]
pub mod sndmker;

#[path = "apps/zim.rs"]
pub mod zim;

#[path = "apps/imageviewer.rs"]
pub mod imageviewer;

// ---

// audio

#[path = "audio/sound.rs"]
pub mod sound;

// ---

// audios

#[path = "audios/song1.rs"]
pub mod song1;

// ---

// commands

#[path = "commands/commands.rs"]
pub mod commands;

// ---

// drivers

// ---

// games

#[path = "games/buggy.rs"]
pub mod buggy;

#[path = "games/flappy.rs"]
pub mod flappy;

#[path = "games/geometry.rs"]
pub mod geometry;

#[path = "games/raycaster.rs"]
pub mod raycaster;

#[path = "games/flightsm.rs"]
pub mod flightsm;

// ---

// gl

#[path = "gl/gltest.rs"]
pub mod gltest;

#[path = "gl/gltest2.rs"]
pub mod gltest2;

#[path = "gl/gltest3.rs"]
pub mod gltest3;

#[path = "gl/mtrx_gl.rs"]
pub mod mtrx_gl;

// kernel

#[path = "kernel/ocpu.rs"]
pub mod ocpu;

#[path = "kernel/panic.rs"]
pub mod panic;

// ---

// laaps

#[path = "laapps/calculadora.rs"]
pub mod calculadora;

#[path = "laapps/monitor.rs"]
pub mod monitor;

// ---

// lade

#[path = "lade/lade.rs"]
pub mod lade;

// ---

// sdmll

#[path = "sdmll/sdmll.rs"]
pub mod sdmll;

// ---

// shell

#[path = "shell/shell.rs"]
pub mod shell;

// ---

// sys

#[path = "sys/intro.rs"]
pub mod intro;

#[path = "sys/sysconf.rs"]
pub mod sysconf;

// ---

// ui

#[path = "ui/sdmlle1.rs"]
pub mod sdmlle1;

// ---

pub use crate::mtrx_gl::{MtrxGl, Vec2};
