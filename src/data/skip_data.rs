/*
 * skip_data.rs
 * Defines a struct that holds data for the SKIP instruction
 * Created on 12/2/2019
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
use super::SkipType;
use super::super::codegen::CodeGen;

/// Contextual data for the `SKIP` instruction
pub struct SkipData {
    /// The LHS side of the comparison
    vx: Register,

    /// The RHS of the comparison, if using a register
    vy: Option<Register>,

    /// The RHS of the comparison, if using a constant
    nn: Option<u8>,

    /// The skip type of the instruction
    skip_type: SkipType 
}

//implementation
impl SkipData {
    /// Constructs a new `SkipData` instance with a 
    /// register-to-register comparison
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The LHS of the comparison
    /// * `new_vy` - The RHS of the comparison
    /// * `new_type` - The skip type of the comparison
    ///
    /// # Returns
    ///
    /// A new `SkipData` instance with the given properties
    ///
    /// # Panics
    ///
    /// This method will panic if `new_type` is a key-related variant
    /// or if either register is the `I` register 
    pub fn with_register(new_vx: Register, new_vy: Register,
                         new_type: SkipType) -> SkipData {
        //verify the type
        if (new_type == SkipType::KeyUp) || 
            (new_type == SkipType::KeyDown) {
            panic!("Bad skip type: {:?}", new_type);
        }

        //verify the register
        if (new_vx == Register::I) || (new_vy == Register::I) {
            panic!("Index register is disallowed in skips");
        }

        //and return a new instance
        return SkipData::new(new_vx, Some(new_vy), None, new_type);
    }

    /// Constructs a new `SkipData` instance with a 
    /// register-to-constant comparison
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The LHS of the comparison
    /// * `new_nn` - The RHS of the comparison
    /// * `new_type` - The skip type of the comparison
    ///
    /// # Returns
    ///
    /// A new `SkipData` instance with the given properties
    ///
    /// # Panics
    ///
    /// This method will panic if `new_type` is a key-related variant
    /// or if `new_vx` is the `I` register 
    pub fn with_constant(new_vx: Register, new_nn: u8,
                         new_type: SkipType) -> SkipData {
        //verify the type
        if (new_type == SkipType::KeyUp) || 
            (new_type == SkipType::KeyDown) {
            panic!("Bad skip type: {:?}", new_type);
        }

        //verify the register
        if new_vx == Register::I {
            panic!("Index register is disallowed in skips");
        }

        //and return a new instance
        return SkipData::new(new_vx, None, Some(new_nn), new_type);
    }

    /// Constructs a new `SkipData` instance with a 
    /// key comparison
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The LHS of the comparison
    /// * `new_type` - The skip type of the comparison
    ///
    /// # Returns
    ///
    /// A new `SkipData` instance with the given properties
    ///
    /// # Panics
    ///
    /// This method will panic if `new_type` is not a key-related variant
    /// or if `new_vx` is the `I` register 
    pub fn with_key(new_vx: Register, 
                         new_type: SkipType) -> SkipData {
        //verify the type
        if (new_type != SkipType::KeyUp) && 
            (new_type != SkipType::KeyDown) {
            panic!("Bad skip type: {:?}", new_type);
        }

        //verify the register
        if new_vx == Register::I {
            panic!("Index register is disallowed in skips");
        }

        //and return a new instance
        return SkipData::new(new_vx, None, None, new_type);
    }


    /// Constructs a new `SkipData` instance
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The LHS of the comparison
    /// * `new_vy` - The RHS register
    /// * `new_nn` - The RHS constant
    /// * `new_type` - The skip type
    /// 
    /// # Returns 
    ///
    /// A new `SkipData` instance with the given properties
    fn new(new_vx: Register, new_vy: Option<Register>, 
            new_nn: Option<u8>, new_type: SkipType) -> SkipData {
        return SkipData {
            vx: new_vx,
            vy: new_vy,
            nn: new_nn,
            skip_type: new_type 
        };
    }
}

//CodeGen implementation
impl CodeGen for SkipData {
    /// Generates the opcode for the data
    /// 
    /// # Returns
    ///
    /// The numeric opcode for the data 
    fn gen_opcode(&self) -> u16 {
        //match the skip type
        match self.skip_type {
            SkipType::Equals => {
                //match whether the register field is not None
                match self.vy.clone() {
                    Some(y) => { //register comparison
                        let mut code = 0x5000;
                        code |= (self.vx.to_id() as u16) << 8;
                        code |= (y.to_id() as u16) << 4;
                        return code;
                    },
                    None => { //constant comparison
                        match self.nn {
                            Some(n) => {
                                let mut code = 0x3000;
                                code |= (self.vx.to_id() as u16) << 8;
                                code |= n as u16;
                                return code;
                            },
                            None => { //should never be reached
                                panic!("Invalid RHS of comparison");
                            }
                        }
                    }
                }
            },
            SkipType::NotEquals => {
                //match whether the register field is not None
                match self.vy.clone() {
                    Some(y) => { //register comparison
                        let mut code = 0x9000;
                        code |= (self.vx.to_id() as u16) << 8;
                        code |= (y.to_id() as u16) << 4;
                        return code;
                    },
                    None => { //constant comparison
                        match self.nn {
                            Some(n) => {
                                let mut code = 0x4000;
                                code |= (self.vx.to_id() as u16) << 8;
                                code |= n as u16;
                                return code;
                            },
                            None => { //should never be reached
                                panic!("Invalid RHS of comparison");
                            }
                        }
                    }
                }
            },
            SkipType::KeyDown => {
                let mut code = 0xE09E;
                code |= (self.vx.to_id() as u16) << 8;
                return code;
            },
            SkipType::KeyUp => {
                let mut code = 0xE0A1;
                code |= (self.vx.to_id() as u16) << 8;
                return code;
            }
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the SkipData struct
    use super::*;

    //this test checks that bad skip types panic
    #[test]
    #[should_panic]
    fn test_bad_skiptype_panic() {
        let _b1 = SkipData::with_register(Register::V0, Register::V1,
                                         SkipType::KeyDown);
        let _b2 = SkipData::with_register(Register::V0, Register::V1,
                                         SkipType::KeyUp);
        let _b3 = SkipData::with_constant(Register::V0, 0xFF,
                                         SkipType::KeyDown);
        let _b4 = SkipData::with_constant(Register::V0, 0xFF,
                                         SkipType::KeyUp);
        let _b5 = SkipData::with_key(Register::V0, SkipType::Equals);
        let _b6 = SkipData::with_key(Register::V0, SkipType::NotEquals);
    }

    //this test checks that using the index register panics
    #[test]
    #[should_panic]
    fn test_index_register_panic() {
        let _b1 = SkipData::with_register(Register::I, Register::V0,
                                          SkipType::Equals);
        let _b2 = SkipData::with_register(Register::V0, Register::I,
                                          SkipType::Equals);
        let _b3 = SkipData::with_constant(Register::I, 0xFF,
                                          SkipType::Equals);
        let _b4 = SkipData::with_key(Register::I, SkipType::KeyDown);
    }

    //this test checks code generation
    #[test]
    fn test_opcode_gen() {
        let sk1 = SkipData::with_constant(Register::V1, 0xFF,
                                          SkipType::Equals);
        assert_eq!(sk1.gen_opcode(), 0x31FF);
        let sk2 = SkipData::with_constant(Register::V1, 0xFF,
                                          SkipType::NotEquals);
        assert_eq!(sk2.gen_opcode(), 0x41FF);
        let sk3 = SkipData::with_register(Register::V1, Register::V2,
                                          SkipType::Equals);
        assert_eq!(sk3.gen_opcode(), 0x5120);
        let sk4 = SkipData::with_register(Register::V1, Register::V2,
                                          SkipType::NotEquals);
        assert_eq!(sk4.gen_opcode(), 0x9120);
        let sk5 = SkipData::with_key(Register::V1, SkipType::KeyDown);
        assert_eq!(sk5.gen_opcode(), 0xE19E);
        let sk6 = SkipData::with_key(Register::V1, SkipType::KeyUp);
        assert_eq!(sk6.gen_opcode(), 0xE1A1);
    }
}
