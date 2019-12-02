/*
 * cls_data.rs
 * Defines a struct that holds data for the CLS instruction
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

/// Contextual data for the `CLS` instruction
pub struct ClsData {
    //no fields
}

//implementation
impl ClsData {
    /// Creates a new `ClsData` instance
    ///
    /// # Returns
    ///
    /// A new `ClsData` instance
    pub fn new() -> ClsData {
        return ClsData { };
    }
}

//CodeGen implementation
impl CodeGen for ClsData {
    /// Generates the opcode for the instruction
    /// 
    /// # Returns 
    ///
    /// The opcode for the `CLS` instruction
    fn gen_opcode(&self) -> u16 {
        return 0x00E0;
    }
}

//end of file
