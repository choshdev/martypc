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

    egui::instruction_history_viewer.rs

    Implements the instruction history viewer control.
    The control is a virtual window that will display the disassembly of
    the last X executed instructions.

*/
use crate::{token_listview::*, *};
use marty_core::syntax_token::*;

pub struct InstructionHistoryControl {
    pub address: String,
    pub row: usize,
    pub lastrow: usize,
    tlv: TokenListView,
}

impl InstructionHistoryControl {
    pub fn new() -> Self {
        Self {
            address: "cs:ip".to_string(),
            row: 0,
            lastrow: 0,
            tlv: TokenListView::new(),
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, events: &mut GuiEventQueue) {
        self.tlv.set_capacity(32);
        self.tlv.set_visible(32);

        let mut new_row = self.row;
        ui.horizontal(|ui| {
            self.tlv
                .draw(ui, events, &mut new_row, &mut |_scrolled_to, _sevents| {});
        });
    }

    pub fn set_content(&mut self, mem: Vec<Vec<SyntaxToken>>) {
        self.tlv.set_contents(mem, false);
    }

    #[allow(dead_code)]
    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    #[allow(dead_code)]
    pub fn get_address(&mut self) -> String {
        self.address.clone()
    }
}
