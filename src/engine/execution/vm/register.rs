#![allow(dead_code)]

pub(crate) struct Register {
    pub(crate) r0: i64,
    pub(crate) r1: i64,
    pub(crate) r2: i64,
    pub(crate) r3: i64,
    pub(crate) r4: i64,
    pub(crate) r5: i64,
    pub(crate) r6: i64,
    pub(crate) r7: i64,
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
        }
    }
}
