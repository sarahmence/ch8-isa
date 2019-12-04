/*
 * or_data.rs
 * Defines a struct that holds data for the OR instruction
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

/// Contextual data for the `OR` instruction
pub struct OrData {
    /// The first (destination) operand
    vx: Register,

    /// The second operand
    vy: Register 
}

//implementation
impl OrData {
    /// Constructs a new `OrData` instance
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The first (destination) operand
    /// * `new_vy` - The second operand
    ///
    /// # Panics
    ///
    /// This method will panic if either register is the `I` register
    /// 
    /// # Returns
    ///
    /// A new `OrData` instance with the given properties
    pub fn new(new_vx: Register, new_vy: Register) -> OrData {
        //validate the registers
        if (new_vx == Register::I) || (new_vy == Register::I) {
            panic!("Index register cannot be ORed!");
        }

        //and return an instance
        return OrData {
            vx: new_vx,
            vy: new_vy 
        };
    }
}

//CodeGen implementation
impl CodeGen for OrData {
    /// Generates an opcode for the `OR` instruction
    /// 
    /// # Returns
    ///
    /// An opcode for the `OR` instruction from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0x8001;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vy.to_id() as u16) << 4;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the OrData struct
    use super::*;

    //this test checks that the I register cannot be used
    #[test]
    #[should_panic]
    fn test_cannot_or_index() {
        let _d1 = OrData::new(Register::I, Register::V0);
        let _d2 = OrData::new(Register::V0, Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let ord = OrData::new(Register::V1, Register::V2);
        assert_eq!(ord.gen_opcode(), 0x8121);
    }
}

//end of file
