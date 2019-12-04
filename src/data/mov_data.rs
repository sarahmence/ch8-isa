/*
 * mov_data.rs
 * Defines a struct that holds data for the MOV instruction
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

/// Contextual data for the `MOV` instruction
pub struct MovData {
    /// The destination register 
    dest: Register,

    /// The source register
    src_reg: Option<Register>,

    /// The source constant
    src_cst: Option<u16>
}

//implementation
impl MovData {
    /// Constructs a new `MovData` instance
    /// with a source register
    /// 
    /// # Arguments
    ///
    /// * `new_dest` - The destination register
    /// * `new_src` - The source register
    /// 
    /// # Panics
    ///
    /// This method will panic if `new_dest` or `new_src`
    /// refers to the `I` register 
    ///
    /// # Returns
    ///
    /// A new `MovData` instance with the given properties
    pub fn with_register(new_dest: Register, new_src: Register)
        -> MovData {
        //validate the registers
        if new_dest == Register::I {
            panic!("Index register must be assigned from a constant");
        }
        if new_src == Register::I {
            panic!("Index register cannot be assigned from");
        }

        //and call the new method
        return MovData::new(new_dest, Some(new_src), None);
    }

    /// Constructs a new `MovData` instance with a source constant
    ///
    /// # Arguments
    ///
    /// * `new_dest` - The destination register
    /// * `new_src` - The source constant
    ///
    /// # Returns
    ///
    /// A new `MovData` instance with the given properties
    pub fn with_constant(new_dest: Register, new_src: u16) -> MovData {
        //mask the source value
        let new_src = match new_dest {
            Register::I => new_src & 0x0FFF,
            _ => new_src & 0x00FF
        };

        //call the new method
        return MovData::new(new_dest, None, Some(new_src));
    }

    /// Constructs a new `MovData` instance
    ///
    /// # Arguments
    ///
    /// * `new_dest` - The destination register
    /// * `new_src_reg` - The source register
    /// * `new_src_cst` - The source constant
    ///
    /// # Returns
    ///
    /// A new `MovData` instance with the given properties
    fn new(new_dest: Register, new_src_reg: Option<Register>,
           new_src_cst: Option<u16>) -> MovData {
        return MovData {
            dest: new_dest,
            src_reg: new_src_reg,
            src_cst: new_src_cst 
        };
    }
}

//CodeGen implementation
impl CodeGen for MovData {
    /// Generates an opcode from the data
    /// 
    /// # Returns
    ///
    /// An opcode generated from the data
    fn gen_opcode(&self) -> u16 {
        //handle destination objects
        match self.dest {
            Register::I => {
                match self.src_cst.clone() {
                    Some(cst) => {
                        let mut code = 0xA000;
                        code |= cst;
                        return code;
                    },
                    None => {
                        panic!("Invalid source value");
                    }
                };
            },
            _ => { //normal register
                //handle different source values
                match self.src_reg.clone() {
                    Some(reg) => { //source is a register
                        let mut code = 0x8000;
                        code |= (self.dest.to_id() as u16) << 8;
                        code |= (reg.to_id() as u16) << 4;
                        return code;
                    },
                    None => { //source is a constant
                        match self.src_cst {
                            Some(cst) => {
                                let mut code = 0x6000;
                                code |= (self.dest.to_id() as u16) << 8;
                                code |= cst;
                                return code;
                            },
                            None => { //should not be reached
                                panic!("Invalid RHS of MOV instruction");
                            }
                        };
                    }
                };
            }
        };
    }
}

//tests
#[cfg(test)]
mod tests {
    //import the MovData structure
    use super::*;

    //this test checks that constant values are masked
    #[test]
    fn test_constants_are_masked() {
        let d1 = MovData::with_constant(Register::I, 0xFFFF);
        assert_eq!(d1.src_cst.unwrap(), 0x0FFF);
        let d2 = MovData::with_constant(Register::V0, 0xFFFF);
        assert_eq!(d2.src_cst.unwrap(), 0x00FF);
    }

    //this test checks that the I register cannot be assigned from
    #[test]
    #[should_panic]
    fn test_cannot_assign_from_index() {
        let _b1 = MovData::with_register(Register::V0, Register::I);
    }

    //this test checks that the I register cannot have a register
    //assigned to it
    #[test]
    #[should_panic]
    fn test_cannot_assign_register_to_index() {
        let _b1 = MovData::with_register(Register::I, Register::V0);
    }

    //this test checks generating opcodes
    #[test]
    fn test_opcode_gen() {
        let m1 = MovData::with_register(Register::V1, Register::V2);
        assert_eq!(m1.gen_opcode(), 0x8120);
        let m2 = MovData::with_constant(Register::V1, 0xFC);
        assert_eq!(m2.gen_opcode(), 0x61FC);
        let m3 = MovData::with_constant(Register::I, 0x0FC1);
        assert_eq!(m3.gen_opcode(), 0xAFC1);
    }
}

//end of file
