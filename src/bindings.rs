/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl std::fmt::Debug for root::AkMIDIEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AkMIDIEvent").finish()
    }
}

/// Invalid game object (may also mean all game objects)
pub const AK_INVALID_GAME_OBJECT: u64 = u64::MAX;

/// Invalid audio object ID
pub const AK_INVALID_AUDIO_OBJECT_ID: u64 = u64::MAX;
