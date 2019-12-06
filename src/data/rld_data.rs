/*
 * rld_data.rs
 * Defines a struct that holds data for the RLD instruction
 * Created on 12/6/2019
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

/// Contextual data for the `RLD` instruction 
pub struct RldData {
    /// The end register of the loaded sequence
    vx: Register 
}

//implementation
impl RldData {
    /// Constructs a new `RldData` instance
    ///
    /// # Argument
    ///
    /// * `new_vx` - The end register of the loaded sequence
    /// 
    /// # Panics
    ///
    /// This method will panic if `new_vx` refers to the `I` register.
    /// 
    /// # Returns 
    ///
    /// A new `RldData` instance with the given properties
    pub fn new(new_vx: Register) -> RldData {
        //validate the register
        if new_vx == Register::I {
            panic!("Cannot load index register from memory");
        }

        //and return the instance
        return RldData {
            vx: new_vx 
        };
    }
}

//CodeGen implementation
impl CodeGen for RldData {
    /// Generates an opcode for the `RLD` instruction from the data 
    /// 
    /// # Returns
    ///
    /// The generated opcode
    fn gen_opcode(&self) -> u16 {
        return 0xF065 | ((self.vx.to_id() as u16) << 8);
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the RldData struct
    use super::*;

    //this test checks that the I register cannot be used
    #[test]
    #[should_panic]
    fn test_cannot_load_index() {
        let _b = RldData::new(Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let rldd = RldData::new(Register::V1);
        assert_eq!(rldd.gen_opcode(), 0xF165);
    }
}

//end of file
