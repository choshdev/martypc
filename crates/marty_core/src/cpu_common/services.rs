/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2025 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

    ---------------------------------------------------------------------------

    cpu_common::services.rs

    Implements common CPU debug services like instruction history and
    disassembly recording.

*/

use std::{collections::BTreeMap, path::PathBuf};

use crate::cpu_common::Instruction;

pub enum ListingEntry {
    InstructionEntry {
        cs: u16,
        ip: u16,
        jump_target: bool,
        call_target: bool,
        code_bytes: Vec<u8>,
        i: Instruction,
    },
}

pub struct CPUDebugServices {
    pub listing_filename: Option<PathBuf>,
    pub listing: BTreeMap<(u16, u16), ListingEntry>,
}

impl Default for CPUDebugServices {
    fn default() -> Self {
        CPUDebugServices {
            listing_filename: None,
            listing: BTreeMap::new(),
        }
    }
}

impl CPUDebugServices {
    pub fn new(listing_path: Option<PathBuf>) -> Self {
        CPUDebugServices {
            listing_filename: listing_path,
            ..CPUDebugServices::default()
        }
    }

    pub fn clear(&mut self) {
        self.listing.clear();
    }

    pub fn start_listing_recording(&mut self) {
        self.clear();
    }

    pub fn stop_listing_recording(&mut self) {}

    pub fn add_instruction(
        &mut self,
        cs: u16,
        ip: u16,
        jump_target: bool,
        call_target: bool,
        code_bytes: Vec<u8>,
        i: Instruction,
    ) {
        self.listing.insert(
            (cs, ip),
            ListingEntry::InstructionEntry {
                cs,
                ip,
                jump_target,
                call_target,
                code_bytes,
                i,
            },
        );
    }
}
