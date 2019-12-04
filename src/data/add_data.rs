/*
 * add_data.rs
 * Defines a struct that holds data for the ADD instruction
 * Created on 12/4/2019
 * Created by Andrew Davis
 *
 * Copyright (C) 2019  Andrew Davis
 *
 * This program is free software: you can redistribute it and/or modify   
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//usage statements
use super::Register;
use super::super::codegen::CodeGen;

/// Contextual data for the `ADD` instruction
pub struct AddData {
    /// The first addend (also the destination register)
    first: Register,

    /// The second register addend
    second_reg: Option<Register>,

    /// The second constant addend
    second_cst: Option<u16>
}

//implementation
impl AddData {
    /// Constructs a new `AddData` instance with a register addend
    ///
    /// # Arguments
    ///
    /// * `new_first` - The first addend (also the destination)
    /// * `new_second` - The second addend
    ///
    /// # Panics
    ///
    /// This method will panic if `new_second` refers to the `I` register.
    ///
    /// # Returns
    ///
    /// A new `AddData` instance with the given properties
    pub fn with_register(new_first: Register, new_second: Register)
        -> AddData {
        //validate the second register
        if new_second == Register::I {
            panic!("Cannot add the index register to a register");
        }

        //and call the new method
        return AddData::new(new_first, Some(new_second), None);
    }

    /// Constructs a new `AddData` instance with a constant addend
    ///
    /// # Arguments
    ///
    /// * `new_first` - The first addend (also the destination)
    /// * `new_second` - The second addend
    ///
    /// # Panics
    ///
    /// This method will panic if `new_first` refers to the `I` register.
    ///
    /// # Returns
    ///
    /// A new `AddData` instance with the given properties
    pub fn with_constant(new_first: Register, new_second: u16)
        -> AddData {
        //check the register
        if new_first == Register::I {
            panic!("Cannot add constant to index register");
        }

        //mask the second addend
        let new_second = new_second & 0x00FF;

        //and call the new method
        return AddData::new(new_first, None, Some(new_second));
    }

    /// Constructs a new `AddData` instance
    ///
    /// # Arguments
    ///
    /// * `new_first` - The first addend (also the destination)
    /// * `new_second_reg` - The second register addend
    /// * `new_second_cst` - The second constant addend
    ///
    /// # Returns
    ///
    /// A new `AddData` instance with the given properties
    fn new(new_first: Register, new_second_reg: Option<Register>,
           new_second_cst: Option<u16>) -> AddData {
        return AddData {
            first: new_first,
            second_reg: new_second_reg,
            second_cst: new_second_cst 
        };
    }
}

//CodeGen implementation
impl CodeGen for AddData {
    /// Generates an opcode for the `ADD` instruction
    /// 
    /// # Returns
    ///
    /// An opcode for the `ADD` instruction with the given data
    fn gen_opcode(&self) -> u16 {
        //match the first addend
        match self.first {
            Register::I => { //index register
                //handle different second addends
                match self.second_reg.clone() {
                    Some(reg) => {  
                        let mut code = 0xF01E;
                        code |= (reg.to_id() as u16) << 8;
                        return code;
                    },
                    None => { //should not be reached
                        panic!("Invalid RHS of add statement");
                    }
                };
            },
            _ => { //normal register
                //handle different second addends
                match self.second_reg.clone() {
                    Some(reg) => { //VX += VY
                        let mut code = 0x8004;
                        code |= (self.first.to_id() as u16) << 8;
                        code |= (reg.to_id() as u16) << 4;
                        return code;
                    },
                    None => { //VX += NN
                        match self.second_cst {
                            Some(cst) => {
                                let mut code = 0x7000;
                                code |= (self.first.to_id() as u16) << 8;
                                code |= cst;
                                return code;
                            },
                            None => { //should never be reached
                                panic!("Invalid second addend");
                            }
                        };
                    }
                };
            }
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the AddData struct
    use super::*;

    //this test checks that constants are masked
    #[test]
    fn test_constants_are_masked() {
        let d = AddData::with_constant(Register::V0, 0xFFFB);
        assert_eq!(d.second_cst.unwrap(), 0xFB);
    }

    //this test ensures that the index register cannot be added
    //to a normal register
    #[test]
    #[should_panic]
    fn test_cannot_add_index_to_normal() {
        let _d = AddData::with_register(Register::V0, Register::I);
    }

    //this test ensures that a constant cannot be directly added
    //to the index register
    #[test]
    #[should_panic]
    fn test_cannot_add_constant_to_index() {
        let _d = AddData::with_constant(Register::I, 0x0FFF);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let d1 = AddData::with_register(Register::V1, Register::V2);
        assert_eq!(d1.gen_opcode(), 0x8124);
        let d2 = AddData::with_register(Register::I, Register::V1);
        assert_eq!(d2.gen_opcode(), 0xF11E);
        let d3 = AddData::with_constant(Register::V1, 0xFC);
        assert_eq!(d3.gen_opcode(), 0x71FC);
    }
}

//end of file
