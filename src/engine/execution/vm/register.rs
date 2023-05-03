#![allow(dead_code)]

pub(crate) struct Register {
    pub(crate) r1: i64,
    pub(crate) r2: i64,
    pub(crate) r3: i64,
    pub(crate) r4: i64,
    pub(crate) r5: i64,
    pub(crate) r6: i64,
    pub(crate) r7: i64,
    pub(crate) r8: i64,
    pub(crate) r9: i64,
    pub(crate) r10: i64,
    pub(crate) r11: i64,
    pub(crate) r12: i64,
    pub(crate) r13: i64,
    pub(crate) r14: i64,
    pub(crate) r15: i64,
    pub(crate) r16: i64,
}

impl Register {
    pub(crate) fn new() -> Self {
        Self {
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            r16: 0,
        }
    }
}

#[allow(non_snake_case)]
pub(crate) mod RName {
    pub(crate) const R1: u8 = 0x01;
    pub(crate) const R2: u8 = 0x02;
    pub(crate) const R3: u8 = 0x03;
    pub(crate) const R4: u8 = 0x04;
    pub(crate) const R5: u8 = 0x05;
    pub(crate) const R6: u8 = 0x06;
    pub(crate) const R7: u8 = 0x07;
    pub(crate) const R8: u8 = 0x08;
    pub(crate) const R9: u8 = 0x09;
    pub(crate) const R10: u8 = 0x0A;
    pub(crate) const R11: u8 = 0x0B;
    pub(crate) const R12: u8 = 0x0C;
    pub(crate) const R13: u8 = 0x0D;
    pub(crate) const R14: u8 = 0x0E;
    pub(crate) const R15: u8 = 0x0F;
    pub(crate) const R16: u8 = 0x10;
}
