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
pub struct Unresolved {
    resolution_type: i32,
    arg_integer: i32,
    resolution_string: String,
    memory_location: u16,
}

impl Unresolved {
    pub fn new(
        resolution_type: i32,
        arg_integer: i32,
        resolution_string: String,
        memory_location: u16,
    ) -> Self {
        Unresolved {
            resolution_type: resolution_type,
            arg_integer: arg_integer,
            resolution_string: resolution_string,
            memory_location: memory_location,
        }
    }

    pub fn resolution_type(&mut self, resolution_type: i32) -> &mut Unresolved {
        self.resolution_type = resolution_type;
        self
    }

    pub fn resolution_string(&mut self, resolution_string: String) -> &mut Unresolved {
        self.resolution_string = resolution_string;
        self
    }
    pub fn memory_location(&mut self, memory_location: u16) -> &mut Unresolved {
        self.memory_location = memory_location;
        self
    }
    pub fn arg_integer(&mut self, arg_integer: i32) -> &mut Unresolved {
        self.arg_integer = arg_integer;
        self
    }

    pub fn get_resolution_type(&mut self) -> i32 {
        self.resolution_type.clone()
    }
    pub fn get_arg_integer(&mut self) -> i32 {
        self.arg_integer.clone()
    }
    pub fn get_resolution_string(&mut self) -> String {
        self.resolution_string.clone()
    }
    pub fn get_memory_location(&mut self) -> u16 {
        self.memory_location.clone()
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct OperationalCode {
    address_location: u16, // the address requested, e.g. from the PC
    memory_location: u16,  // the output, e.g. what do do for each timer.
                           // the address location is always the 8-bit location from the PC with additional upper range 0-7 from the timer.bindings
}
impl OperationalCode {
    pub fn new(address_location: u16, memory_location: u16) -> Self {
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
    pub fn new(name: String, location: u16) -> Self {
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
pub struct Data {
    pub variable_name: String,
    pub memory_start_location: WordSize,
    pub memory_end_location: WordSize,
    // pub data_type: dtype, // Is this .ascii or byte, etc.
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: [WordSize; 5],
    pub args: [WordSize; 6],
}
