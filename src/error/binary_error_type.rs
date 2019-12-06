/*
 * binary_error_type.rs
 * Enumerates types of binary creation errors
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

//usage statement
use std::fmt;

/// Types of binary creation errors
#[derive(Clone)]
pub enum BinaryErrorType {
    /// The desired filename exists already
    FileExists,

    /// Error writing data to the binary
    DataError,

    /// Error writing the binary to a file
    FileError
}

//Debug implementation
impl fmt::Debug for BinaryErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryErrorType::DataError => {
                write!(f, "Error writing data into binary")
            },
            BinaryErrorType::FileError => {
                write!(f, "Error writing binary to disk")
            },
            BinaryErrorType::FileExists => {
                write!(f, "A binary already exists with the given name")
            }
        }
    }
}

//Display implementation
impl fmt::Display for BinaryErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryErrorType::DataError => {
                write!(f, "Error writing data into binary")
            },
            BinaryErrorType::FileError => {
                write!(f, "Error writing binary to disk")
            },
            BinaryErrorType::FileExists => {
                write!(f, "A binary already exists with the given name")
            }
        }
    }
}


//end of file
