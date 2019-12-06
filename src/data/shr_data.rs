/*
 * shr_data.rs
 * Defines a struct that holds data for the SHR instruction
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

/// Contextual data for the `SHR` instruction
pub struct ShrData {
    /// The register to shift
    vx: Register 
}

//implementation
impl ShrData {
    /// Constructs a new `ShrData` instance
    ///
    /// # Argument
    ///
    /// * `new_vx` - The register to shift right
    /// 
    /// # Panics
    ///
    /// This method will panic if `new_vx` refers to the `I` register.
    /// 
    /// # Returns
    ///
    /// A new `ShrData` instance with the given properties
    pub fn new(new_vx: Register) -> ShrData {
        //validate the register
        if new_vx == Register::I {
            panic!("Index register cannot be shifted");
        }

        //and return the instance
        return ShrData {
            vx: new_vx 
        }
    }
}

//CodeGen implementation
impl CodeGen for ShrData {
    /// Generates an opcode for the `SHR` instruction
    /// 
    /// # Returns
    ///
    /// The opcode generated from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0x8006;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vx.to_id() as u16) << 4;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the ShrData struct
    use super::*;

    //this test checks that the I register cannot be shifted
    #[test]
    #[should_panic]
    fn test_cannot_shift_index() {
        let _bd = ShrData::new(Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let shrd = ShrData::new(Register::V1);
        assert_eq!(shrd.gen_opcode(), 0x8116);
    }
}

//end of file
