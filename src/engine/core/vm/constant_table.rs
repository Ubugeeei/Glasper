type Constant = String;

pub(crate) struct ConstantTable {
    table: Vec<Constant>,
}

impl ConstantTable {
    pub(crate) fn new() -> Self {
        ConstantTable { table: vec![] }
    }

    pub(crate) fn add(&mut self, constant: Constant) -> u32 {
        let index = self.table.len();
        self.table.push(constant);
        index as u32
    }

    pub(crate) fn get(&self, index: u32) -> &Constant {
        &self.table[index as usize]
    }

    pub(crate) fn get_mut(&mut self, index: u32) -> &mut Constant {
        &mut self.table[index as usize]
    }
}
