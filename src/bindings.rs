/*
 * Copyright (c) 2022 Contributors to the Rrise project
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const AK_INVALID_GAME_OBJECT: root::AkAudioObjectID = u64::MAX;
pub const AK_INVALID_AUDIO_OBJECT_ID: root::AkAudioObjectID = u64::MAX;
