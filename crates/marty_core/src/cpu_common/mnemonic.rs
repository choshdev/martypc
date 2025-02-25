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

    cpu_common::mnemonic.rs

    Defines mnemonic enum.

*/

use std::fmt;

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Mnemonic {
    Invalid,
    NoOpcode,
    Group,
    Extension,
    Prefix,
    NOP,
    AAA,
    AAD,
    AAM,
    AAS,
    ADC,
    ADD,
    AND,
    CALL,
    CALLF,
    CBW,
    CLC,
    CLD,
    CLI,
    CMC,
    CMP,
    CMPSB,
    CMPSW,
    CWD,
    DAA,
    DAS,
    DEC,
    DIV,
    ESC,
    WAIT,
    HLT,
    IDIV,
    IMUL,
    IN,
    INC,
    INT,
    INT3,
    INTO,
    IRET,
    JB,
    JBE,
    JCXZ,
    JL,
    JLE,
    JMP,
    JMPF,
    JNB,
    JNBE,
    JNL,
    JNLE,
    JNO,
    JNP,
    JNS,
    JNZ,
    JO,
    JP,
    JS,
    JZ,
    LAHF,
    LDS,
    LEA,
    LES,
    LOCK,
    LODSB,
    LODSW,
    LOOP,
    LOOPNE,
    LOOPE,
    MOV,
    MOVSB,
    MOVSW,
    MUL,
    NEG,
    NOT,
    OR,
    OUT,
    POP,
    POPF,
    PUSH,
    PUSHF,
    RCL,
    RCR,
    REP,
    REPNE,
    REPE,
    RETF,
    RETN,
    ROL,
    ROR,
    SAHF,
    SALC,
    SAR,
    SBB,
    SCASB,
    SCASW,
    SETMO,
    SETMOC,
    SHL,
    SHR,
    STC,
    STD,
    STI,
    STOSB,
    STOSW,
    SUB,
    TEST,
    XCHG,
    XLAT,
    XOR,
    // 186 Instructions
    PUSHA,
    POPA,
    BOUND,
    INSB,
    INSW,
    OUTSB,
    OUTSW,
    ENTER,
    LEAVE,
    // V20 Instructions
    UNDEF,
    FPO2,
    TEST1,
    CLR1,
    SET1,
    NOT1,
    ADD4S,
    SUB4S,
    CMP4S,
    ROL4,
    ROR4,
    BINS,
    BEXT,
    BRKEM,
}

impl Default for Mnemonic {
    fn default() -> Self {
        Mnemonic::Invalid
    }
}

// TODO: Is this any faster than just using derive Debug?
pub(crate) fn mnemonic_to_str(op: Mnemonic) -> &'static str {
    match op {
        Mnemonic::NOP => "NOP",
        Mnemonic::AAA => "AAA",
        Mnemonic::AAD => "AAD",
        Mnemonic::AAM => "AAM",
        Mnemonic::AAS => "AAS",
        Mnemonic::ADC => "ADC",
        Mnemonic::ADD => "ADD",
        Mnemonic::AND => "AND",
        Mnemonic::CALL => "CALL",
        Mnemonic::CALLF => "CALLF",
        Mnemonic::CBW => "CBW",
        Mnemonic::CLC => "CLC",
        Mnemonic::CLD => "CLD",
        Mnemonic::CLI => "CLI",
        Mnemonic::CMC => "CMC",
        Mnemonic::CMP => "CMP",
        Mnemonic::CMPSB => "CMPSB",
        Mnemonic::CMPSW => "CMPSW",
        Mnemonic::CWD => "CWD",
        Mnemonic::DAA => "DAA",
        Mnemonic::DAS => "DAS",
        Mnemonic::DEC => "DEC",
        Mnemonic::DIV => "DIV",
        Mnemonic::ESC => "ESC",
        Mnemonic::WAIT => "WAIT",
        Mnemonic::HLT => "HLT",
        Mnemonic::IDIV => "IDIV",
        Mnemonic::IMUL => "IMUL",
        Mnemonic::IN => "IN",
        Mnemonic::INC => "INC",
        Mnemonic::INT => "INT",
        Mnemonic::INT3 => "INT3",
        Mnemonic::INTO => "INTO",
        Mnemonic::IRET => "IRET",
        Mnemonic::JB => "JB",
        Mnemonic::JBE => "JBE",
        Mnemonic::JCXZ => "JCXZ",
        Mnemonic::JL => "JL",
        Mnemonic::JLE => "JLE",
        Mnemonic::JMP => "JMP",
        Mnemonic::JMPF => "JMPF",
        Mnemonic::JNB => "JNB",
        Mnemonic::JNBE => "JNBE",
        Mnemonic::JNL => "JNL",
        Mnemonic::JNLE => "JNLE",
        Mnemonic::JNO => "JNO",
        Mnemonic::JNP => "JNP",
        Mnemonic::JNS => "JNS",
        Mnemonic::JNZ => "JNZ",
        Mnemonic::JO => "JO",
        Mnemonic::JP => "JP",
        Mnemonic::JS => "JS",
        Mnemonic::JZ => "JZ",
        Mnemonic::LAHF => "LAHF",
        Mnemonic::LDS => "LDS",
        Mnemonic::LEA => "LEA",
        Mnemonic::LES => "LES",
        Mnemonic::LOCK => "LOCK",
        Mnemonic::LODSB => "LODSB",
        Mnemonic::LODSW => "LODSW",
        Mnemonic::LOOP => "LOOP",
        Mnemonic::LOOPNE => "LOOPNE",
        Mnemonic::LOOPE => "LOOPE",
        Mnemonic::MOV => "MOV",
        Mnemonic::MOVSB => "MOVSB",
        Mnemonic::MOVSW => "MOVSW",
        Mnemonic::MUL => "MUL",
        Mnemonic::NEG => "NEG",
        Mnemonic::NOT => "NOT",
        Mnemonic::OR => "OR",
        Mnemonic::OUT => "OUT",
        Mnemonic::POP => "POP",
        Mnemonic::POPF => "POPF",
        Mnemonic::PUSH => "PUSH",
        Mnemonic::PUSHF => "PUSHF",
        Mnemonic::RCL => "RCL",
        Mnemonic::RCR => "RCR",
        Mnemonic::REP => "REP",
        Mnemonic::REPNE => "REPNE",
        Mnemonic::REPE => "REPE",
        Mnemonic::RETF => "RETF",
        Mnemonic::RETN => "RETN",
        Mnemonic::ROL => "ROL",
        Mnemonic::ROR => "ROR",
        Mnemonic::SAHF => "SAHF",
        Mnemonic::SALC => "SALC",
        Mnemonic::SAR => "SAR",
        Mnemonic::SBB => "SBB",
        Mnemonic::SCASB => "SCASB",
        Mnemonic::SCASW => "SCASW",
        Mnemonic::SETMO => "SETMO",
        Mnemonic::SETMOC => "SETMOC",
        Mnemonic::SHL => "SHL",
        Mnemonic::SHR => "SHR",
        Mnemonic::STC => "STC",
        Mnemonic::STD => "STD",
        Mnemonic::STI => "STI",
        Mnemonic::STOSB => "STOSB",
        Mnemonic::STOSW => "STOSW",
        Mnemonic::SUB => "SUB",
        Mnemonic::TEST => "TEST",
        Mnemonic::XCHG => "XCHG",
        Mnemonic::XLAT => "XLAT",
        Mnemonic::XOR => "XOR",
        // 186 Instructions
        Mnemonic::PUSHA => "PUSHA",
        Mnemonic::POPA => "POPA",
        Mnemonic::BOUND => "BOUND",
        Mnemonic::INSB => "INSB",
        Mnemonic::INSW => "INSW",
        Mnemonic::OUTSB => "OUTSB",
        Mnemonic::OUTSW => "OUTSW",
        Mnemonic::ENTER => "ENTER",
        Mnemonic::LEAVE => "LEAVE",
        // V20 Instructions
        Mnemonic::UNDEF => "UNDEF",
        Mnemonic::FPO2 => "FPO2",
        Mnemonic::TEST1 => "TEST1",
        Mnemonic::CLR1 => "CLR1",
        Mnemonic::SET1 => "SET1",
        Mnemonic::NOT1 => "NOT1",
        Mnemonic::ADD4S => "ADD4S",
        Mnemonic::SUB4S => "SUB4S",
        Mnemonic::CMP4S => "CMP4S",
        Mnemonic::ROL4 => "ROL4",
        Mnemonic::ROR4 => "ROR4",
        Mnemonic::BINS => "BINS",
        Mnemonic::BEXT => "BEXT",
        Mnemonic::BRKEM => "BRKEM",
        _ => "INVALID",
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", mnemonic_to_str(*self))
    }
}
