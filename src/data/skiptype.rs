/*
 * skiptype.rs
 * Enumerates condition types for the SKIP instruction
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

/// Types of `SKIP` conditions
#[derive(Debug, PartialEq)]
pub enum SKIPType {
    /// Test equality between two registers
    /// or a register and a constant value
    Equals,
    
    /// Test inequality between two registers
    /// or a register and a constant value
    NotEquals,

    /// Test whether a given key is pressed
    KeyDown,

    /// Test whether a given key is released
    KeyUp
}

//end of file
