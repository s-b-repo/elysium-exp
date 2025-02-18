#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(abi_thiscall)]
#![feature(decl_macro)]
#![feature(const_convert)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(array_methods)]
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_metadata)]
#![feature(strict_provenance)]

pub use animation_layer::AnimationLayer;
pub use animation_state::AnimationState;
pub use atomic_mut::AtomicMut;
pub use beam::{Beam, BeamInfo, ViewRenderBeams};
pub use client::Client;
pub use client_mode::ClientMode;
pub use console::{Console, Var, VarKind, Vars};
pub use engine::{Engine, PlayerInfo};
pub use entity::EntityList;
pub use frame::Frame;
pub use game_console::GameConsole;
pub use globals::Globals;
pub use hit_group::HitGroup;
pub use id::SteamId;
pub use input::{Command, Input};
pub use input_system::InputSystem;
pub use interfaces::{Interface, InterfaceKind, Interfaces, LibraryKind};
pub use item_kind::ItemKind;
pub use network::{Flow, NetworkChannel};
pub use render::{OverrideKind, Render};
pub use sound::{ActiveChannels, Channel};
pub use steam::SteamAPIContext;
pub use surface::Surface;
pub use trace::{Filter, Trace, TraceKind};
pub use utl_map::UtlMap;
pub use utl_mem::UtlMem;
pub use utl_string::UtlString;
pub use utl_vec::UtlVec;
pub use var::{VarEntry, VarMap};
pub use vdf::Vdf;
pub use view::View;
pub use weapon::{WeaponInfo, WeaponKind};

//pub use panorama::{PanoramaEventRegistration, PanoramaUIEngine, UIEngine, UIPanel};

mod animation_layer;
mod animation_state;
mod atomic_mut;
mod beam;
mod client_mode;
mod console;
mod engine;
mod frame;
mod game_console;
mod globals;
mod hit_group;
mod input_system;
mod interfaces;
mod item_kind;
mod macros;
pub mod networked;
//mod panorama;
mod physics;
mod render;
mod sound;
mod steam;
mod surface;
mod utl_map;
mod utl_mem;
mod utl_string;
mod utl_vec;
mod var;
mod vdf;
mod view;
mod weapon;

pub mod app_system;
pub mod client;
pub mod entity;
pub mod ffi;
pub mod id;
pub mod input;
pub mod material;
pub mod model;
pub mod network;
pub mod player_model;
pub mod trace;

#[derive(Debug)]
pub struct Debug;
#[derive(Debug)]
pub struct Effects;
#[derive(Debug)]
pub struct Events;
#[derive(Debug)]
pub struct Filesystem;
#[derive(Debug)]
pub struct InputInternal;
#[derive(Debug)]
pub struct Kinds;
#[derive(Debug)]
pub struct Localize;
#[derive(Debug)]
pub struct Movement;
#[derive(Debug)]
pub struct Panel;
#[derive(Debug)]
pub struct Panorama;
#[derive(Debug)]
pub struct Physics;
#[derive(Debug)]
pub struct Prediction;
#[derive(Debug)]
pub struct Sound;
#[derive(Debug)]
pub struct VGui;
