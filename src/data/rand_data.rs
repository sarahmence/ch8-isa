/*
 * rand_data.rs
 * Defines a struct that holds data for the RAND instruction
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

/// Contextual data for the `RAND` instruction
pub struct RandData {
    /// The register to store the random value in
    vx: Register,

    /// The value to AND the generated value with
    nn: u8
}

//implementation
impl RandData {
    /// Constructs a new `RandData` instance
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The register to store the random value in
    /// * `new_nn` - The value to AND the generated value with
    /// 
    /// # Panics
    ///
    /// This method will panic if `new_vx` refers to the `I` register.
    /// 
    /// # Returns
    ///
    /// A new `RandData` instance with the given properties
    pub fn new(new_vx: Register, new_nn: u8) -> RandData {
        //validate the register
        if new_vx == Register::I {
            panic!("Index register cannot be randomized");
        }

        //and return an instance
        return RandData {
            vx: new_vx,
            nn: new_nn 
        };
    }
}

//CodeGen implementation
impl CodeGen for RandData {
    /// Generates an opcode for the `RAND` instruction 
    /// 
    /// # Returns
    ///
    /// The opcode generated from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0xC000;
        code |= (self.vx.to_id() as u16) << 8;
        code |= self.nn as u16;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the RandData struct
    use super::*;

    //this test checks that the I register cannot be randomized
    #[test]
    #[should_panic]
    fn test_cannot_randomize_index() {
        let _bd = RandData::new(Register::I, 0xFF);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let randd = RandData::new(Register::V1, 0xFC);
        assert_eq!(randd.gen_opcode(), 0xC1FC);
    }
}

//end of file
