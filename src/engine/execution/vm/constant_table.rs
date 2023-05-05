type Constant = String;

pub(super) struct ConstantTable {
    table: Vec<Constant>,
}

impl ConstantTable {
    pub(super) fn new() -> Self {
        ConstantTable { table: vec![] }
    }

    pub(super) fn add(&mut self, constant: Constant) -> u32 {
        let index = self.table.len();
        self.table.push(constant);
        index as u32
    }

    pub(super) fn get(&self, index: u32) -> &Constant {
        &self.table[index as usize]
    }

    pub(super) fn get_mut(&mut self, index: u32) -> &mut Constant {
        &mut self.table[index as usize]
    }
}
