/*
 * shl_data.rs
 * Defines a struct that holds data for the SHL instruction
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

/// Contextual data for the `SHL` instruction
pub struct ShlData {
    /// The register to shift
    vx: Register 
}

//implementation
impl ShlData {
    /// Constructs a new `ShlData` instance
    ///
    /// # Argument
    ///
    /// * `new_vx` - The register to shift left
    /// 
    /// # Panics
    ///
    /// This method will panic if `new_vx` refers to the `I` register.
    /// 
    /// # Returns
    ///
    /// A new `ShlData` instance with the given properties
    pub fn new(new_vx: Register) -> ShlData {
        //validate the register
        if new_vx == Register::I {
            panic!("Index register cannot be shifted");
        }

        //and return the instance
        return ShlData {
            vx: new_vx 
        }
    }
}

//CodeGen implementation
impl CodeGen for ShlData {
    /// Generates an opcode for the `SHL` instruction
    /// 
    /// # Returns
    ///
    /// The opcode generated from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0x800E;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vx.to_id() as u16) << 4;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the ShlData struct
    use super::*;

    //this test checks that the I register cannot be shifted
    #[test]
    #[should_panic]
    fn test_cannot_shift_index() {
        let _bd = ShlData::new(Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let shld = ShlData::new(Register::V1);
        assert_eq!(shld.gen_opcode(), 0x811E);
    }
}

//end of file
