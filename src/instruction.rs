// Instruction Manager

#[derive(Debug)]
pub struct Label
{
    pub label_name: String,
    pub memory_location: u8,
}

impl Label {
    pub fn new(label_name: &str, memory_location: u8, ) -> Label
    {
        Label {
            label_name: label_name.to_string(),
            memory_location: memory_location,
        }
    }

    pub fn label_name(&mut self, label_name: String) -> &mut Label {
        self.label_name = label_name;
        self
    }
    pub fn memory_location(&mut self, memory_location: u8) -> &mut Label {
        self.memory_location = memory_location;
        self
    }

}

pub struct Data {
    pub variable_name: String,
    pub memory_start_location: u8,
    pub memory_end_location: u8,
    // pub data_type: dtype, // Is this .ascii or byte, etc. 
}

pub struct Instruction {
    pub instruction_location: u8,
    pub opcode: u8,
    pub argc: usize,
    // pub arguments: [Argument; 3],
}