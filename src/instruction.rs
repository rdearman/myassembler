// Instruction Manager
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub type WordSize = u16;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct OperationalCode {
    address_location: u16,
    memory_location: u16,
}
impl OperationalCode {
    pub fn new(
        address_location: u16,
        memory_location: u16,

    ) -> Self {
        OperationalCode {
            address_location: address_location,
            memory_location: memory_location,
        }
    }

    pub fn name(&mut self, address_location: u16) -> &mut OperationalCode {
        self.address_location = address_location;
        self
    }
    pub fn location(&mut self, memory_location: u16) -> &mut OperationalCode {
        self.memory_location = memory_location;
        self
    }
    pub fn get_address_location(&mut self) -> u16 {
        self.address_location.clone()
    }
    pub fn get_memory_location(&mut self) -> u16 {
        self.memory_location.clone()
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Label {
    name: String,
    location: u16,
}

impl Label {
    pub fn new(
        name: String,
        location: u16,

    ) -> Self {
        Label {
            name: name.to_string(),
            location: location,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Label {
        self.name = name;
        self
    }
    pub fn location(&mut self, location: u16) -> &mut Label {
        self.location = location;
        self
    }
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
    pub fn get_location(&mut self) -> u16 {
        self.location.clone()
    }
}
































#[derive(Debug)]
pub struct Data 
{
    pub variable_name: String,
    pub memory_start_location: WordSize,
    pub memory_end_location: WordSize,
    // pub data_type: dtype, // Is this .ascii or byte, etc. 
}

#[derive(Debug)]
pub struct Instruction
{
    pub opcode: [WordSize; 5],
    pub args: [WordSize; 6],
}
