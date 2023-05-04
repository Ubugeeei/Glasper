#![allow(dead_code)]

pub(crate) struct Register {
    pub(crate) r0: usize,
    pub(crate) r1: usize,
    pub(crate) r2: usize,
    pub(crate) r3: usize,
    pub(crate) r4: usize,
    pub(crate) r5: usize,
    pub(crate) r6: usize,
    pub(crate) r7: usize,
    pub(crate) r8: usize,
    pub(crate) r9: usize,
    pub(crate) r10: usize,
    pub(crate) r11: usize,
    pub(crate) r12: usize,
    pub(crate) r13: usize,
    pub(crate) r14: usize,
    pub(crate) r15: usize,
}

impl Register {
    pub(crate) fn new() -> Self {
        Self {
            r0: 0,
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
        }
    }
}
