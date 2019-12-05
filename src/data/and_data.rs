/*
 * and_data.rs
 * Defines a struct that holds data for the AND instruction
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

/// Contextual data for the `AND` instruction
pub struct AndData {
    /// The first (destination) operand
    vx: Register,

    /// The second operand
    vy: Register 
}

//implementation
impl AndData {
    /// Constructs a new `AndData` instance
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
    /// A new `AndData` instance with the given properties
    pub fn new(new_vx: Register, new_vy: Register) -> AndData {
        //validate the registers
        if (new_vx == Register::I) || (new_vy == Register::I) {
            panic!("Index register cannot be ANDed!");
        }

        //and return an instance
        return AndData {
            vx: new_vx,
            vy: new_vy 
        };
    }
}

//CodeGen implementation
impl CodeGen for AndData {
    /// Generates an opcode for the `AND` instruction
    /// 
    /// # Returns
    ///
    /// An opcode for the `AND` instruction from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0x8002;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vy.to_id() as u16) << 4;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the AndData struct
    use super::*;

    //this test checks that the I register cannot be used
    #[test]
    #[should_panic]
    fn test_cannot_and_index() {
        let _d1 = AndData::new(Register::I, Register::V0);
        let _d2 = AndData::new(Register::V0, Register::I);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let andd = AndData::new(Register::V1, Register::V2);
        assert_eq!(andd.gen_opcode(), 0x8122);
    }
}

//end of file
