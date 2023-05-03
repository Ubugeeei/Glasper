pub(super) struct GarbageCollector {}

impl GarbageCollector {
    pub(super) fn new() -> Self {
        Self {}
    }

    pub(super) fn collect(&mut self) {
        self.mark();
        self.sweep();
        self.compact();
    }

    fn mark(&mut self) {
        todo!()
    }

    fn sweep(&mut self) {
        todo!()
    }

    fn compact(&mut self) {
        todo!()
    }
}
