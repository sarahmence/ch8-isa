/*
 * call_data.rs
 * Defines a struct that holds data for the CALL instruction
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

//usage statement
use super::super::codegen::CodeGen;

/// Contextual data for the `CALL` instruction
pub struct CallData {
    /// The address of the subroutine to call 
    addr: u16
}

//struct implementation
impl CallData {
    /// Creates a new `CallData` instance
    ///
    /// # Argument
    ///
    /// * `new_addr` - The address of the subroutine to call 
    /// 
    /// # Returns
    ///
    /// A new `CallData` instance with the given address
    pub fn new(new_addr: u16) -> CallData {
        //mask the new address
        let mask_addr = new_addr & 0x0FFF;

        //and return a new instance
        return CallData {
            addr: mask_addr
        };
    }

    /// Gets the address to jump to 
    ///
    /// # Returns
    ///
    /// The address value of the data 
    pub fn get_addr(&self) -> u16 {
        return self.addr;
    }
}

//CodeGen implementation
impl CodeGen for CallData {
    /// Generates the opcode for the data
    /// 
    /// # Returns
    ///
    /// The numeric opcode for the data
    fn gen_opcode(&self) -> u16 {
        return 0x2000 | self.addr;
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the CallData struct
    use super::*;

    //this test checks that address
    //values are masked when a new
    //instance is created
    #[test]
    fn test_address_is_masked() {
        let data = CallData::new(0xFFFF);
        assert_eq!(data.addr, 0x0FFF);
    }

    //this test checks proper opcode generation
    #[test]
    fn test_opcode_gen() {
        let data = CallData::new(0x0CCC);
        assert_eq!(data.gen_opcode(), 0x2CCC);
    }
}

//end of file
