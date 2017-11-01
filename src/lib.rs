/*
Copyright 2017 Florent D'halluin

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

//! MooMooT is a modular sound synthesis system.
//!
//! The fundamental principle is that MooMooT follows a tree structure for sound synthesis
//! where leaves are indiviual "unit" synthesiser that gets mixed and applied effects
//! down the tree. Each effect or unit synthesiser's parameters can get changed in real
//! time via an internal parameter bus system.

extern crate uuid;
extern crate jack;

mod base;
mod traits;
#[macro_use]
mod params;
pub mod synth;
pub mod efx;
mod tree;

pub use base::MooMoot;
pub use traits::SoundSample;
