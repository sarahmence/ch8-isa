/*
 * draw_data.rs
 * Defines a struct that holds data for the DRAW instruction
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

/// Contextual data for the `DRAW` instruction
pub struct DrawData {
    /// The register that contains the x-coordinate of the sprite
    vx: Register,

    /// The register that contains the y-coordinate of the sprite
    vy: Register,

    /// The height of the sprite
    h: u8 
}

//implementation
impl DrawData {
    /// Constructs a new `DrawData` instance 
    ///
    /// # Arguments
    ///
    /// * `new_vx` - The register containing the x-coordinate
    /// * `new_vy` - The register containing the y-coordinate
    /// * `new_h`  - The height of the sprite to draw 
    ///
    /// # Panics
    ///
    /// This method will panic if `new_vx` or `new_vy` refers to
    /// the `I` register.
    /// 
    /// # Returns
    ///
    /// A new `DrawData` instance with the given properties
    pub fn new(new_vx: Register, new_vy: Register, new_h: u8) 
        -> DrawData {
        //validate the registers
        if (new_vx == Register::I) || (new_vy == Register::I) {
            panic!("Cannot use index register as coordinates");
        }

        //mask the height
        let new_h = new_h & 0x0F;

        //and return the instance
        return DrawData {
            vx: new_vx,
            vy: new_vy,
            h: new_h 
        }
    }
}

//CodeGen implementation
impl CodeGen for DrawData {
    /// Generates an opcode for the `DRAW` instruction
    /// 
    /// # Returns
    ///
    /// The opcode generated from the data
    fn gen_opcode(&self) -> u16 {
        let mut code = 0xD000;
        code |= (self.vx.to_id() as u16) << 8;
        code |= (self.vy.to_id() as u16) << 4;
        code |= self.h as u16;
        return code;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the DrawData struct
    use super::*;

    //this test checks that the I register cannot be used as coordinates
    #[test]
    #[should_panic]
    fn test_index_cannot_be_used_as_coordinate() {
        let _b1 = DrawData::new(Register::I, Register::V0, 0xF);
        let _b2 = DrawData::new(Register::V0, Register::I, 0xF);
    }

    //this test checks that the height field is masked
    #[test]
    fn test_height_is_masked() {
        let drawd = DrawData::new(Register::V1, Register::V2, 0xFB);
        assert_eq!(drawd.h, 0xB);
    }

    //this test checks opcode generation
    #[test]
    fn test_opcode_gen() {
        let drawd = DrawData::new(Register::V1, Register::V2, 0x5);
        assert_eq!(drawd.gen_opcode(), 0xD125);
    }
}

//end of file
