/*
 * sub_data.rs
 * Defines a struct that holds data for the SUB instruction
 * Created on 12/5/2019
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

/// Contextual data for the `SUB` instruction
pub struct SubData {
    /// The destination register
    vx: Register,

    /// The register to be subtracted from `vx`
    vy: Register 
}

//implementation
impl SubData {
    /// Constructs a new `SubData` instance
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The destination register
    /// * `new_vy` - The register to subtract from `new_vx`
    ///
    /// # Panics
    ///
    /// This method will panic if either `Register` argument 
    /// refers the the `I` register.
    ///
    /// # Returns
    ///
    /// A new `SubData` instance with the given properties
    pub fn new(new_vx: Register, new_vy: Register) -> SubData {
        //check the registers
        if (new_vx == Register::I) || (new_vy == Register::I) {
            panic!("Subtraction is not defined for the index register");
        }

        //and return an instance
        return SubData {
            vx: new_vx,
            vy: new_vy 
        };
    }
}

//CodeGen implementation
impl CodeGen for SubData {
    /// Generates an opcode for the `SUB` instruction
    /// 
    /// # Returns
    ///
    /// An opcode for the `SUB` instruction from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0x8005;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vy.to_id() as u16) << 4;
        return code;
    }
}

//tests
#[cfg(test)]
mod tests {
    //import the SubData struct
    use super::*;

    //this test ensures that the I register cannot be used in subtraction
    #[test]
    #[should_panic]
    fn test_cannot_subtract_index() {
        let _b1 = SubData::new(Register::I, Register::V0);
        let _b2 = SubData::new(Register::V0, Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let subd = SubData::new(Register::V1, Register::V2);
        assert_eq!(subd.gen_opcode(), 0x8125);
    }
}

//end of file
