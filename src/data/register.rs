/*
 * register.rs
 * Enumerates register IDs
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

/// The ID of a Chip-8 register
#[derive(Debug)]
pub enum Register {
    //general purpose registers
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    //index register
    I 
}

//implementation
impl Register {
    /// Converts a `Register` to its numeric ID
    ///
    /// # Returns 
    ///
    /// The ID of a given `Register`
    pub fn to_id(&self) -> u8 {
        return match *self {
            Register::I => 0x10,
            Register::V0 => 0x0,
            Register::V1 => 0x1,
            Register::V2 => 0x2,
            Register::V3 => 0x3,
            Register::V4 => 0x4,
            Register::V5 => 0x5,
            Register::V6 => 0x6,
            Register::V7 => 0x7,
            Register::V8 => 0x8,
            Register::V9 => 0x9,
            Register::VA => 0xA,
            Register::VB => 0xB,
            Register::VC => 0xC,
            Register::VD => 0xD,
            Register::VE => 0xE,
            Register::VF => 0xF,
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the Register enum
    use super::*;

    //this test checks converting
    //a Register to its ID
    #[test]
    fn test_register_to_id() {
        assert_eq!(Register::I.to_id(), 0x10);
        assert_eq!(Register::V0.to_id(), 0x0);
        assert_eq!(Register::V1.to_id(), 0x1);
        assert_eq!(Register::V2.to_id(), 0x2);
        assert_eq!(Register::V3.to_id(), 0x3);
        assert_eq!(Register::V4.to_id(), 0x4);
        assert_eq!(Register::V5.to_id(), 0x5);
        assert_eq!(Register::V6.to_id(), 0x6);
        assert_eq!(Register::V7.to_id(), 0x7);
        assert_eq!(Register::V8.to_id(), 0x8);
        assert_eq!(Register::V9.to_id(), 0x9);
        assert_eq!(Register::VA.to_id(), 0xA);
        assert_eq!(Register::VB.to_id(), 0xB);
        assert_eq!(Register::VC.to_id(), 0xC);
        assert_eq!(Register::VD.to_id(), 0xD);
        assert_eq!(Register::VE.to_id(), 0xE);
        assert_eq!(Register::VF.to_id(), 0xF);
    }
}

//end of file
