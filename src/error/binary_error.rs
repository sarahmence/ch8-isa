/*
 * binary_error.rs
 * Defines a struct that holds error data from binary creation
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
use super::BinaryErrorType;

/// A possible error resulting from attempted binary creation
pub struct BinaryError {
    /// The error type 
    error_type: BinaryErrorType,

    /// The filename of the binary  
    binary_name: String,
}

//implementation
impl BinaryError {
    /// Constructs a new `BinaryError` instance
    ///
    /// # Arguments
    ///
    /// * `new_type` - The type of the error
    /// * `new_name` - The name of the binary
    /// 
    /// # Returns
    ///
    /// A new `BinaryError` instance with the given properties
    pub fn new(new_type: BinaryErrorType, new_name: &str) 
        -> BinaryError {
        return BinaryError {
            error_type: new_type,
            binary_name: String::from(new_name)
        };
    }

    /// Gets the error type
    /// 
    /// # Returns
    ///
    /// The error type
    pub fn get_type(&self) -> BinaryErrorType {
        return self.error_type;
    }

    /// Gets the binary name 
    ///
    /// # Returns
    ///
    /// The name of the binary that produced an error
    pub fn get_binary_name(&self) -> &str {
        return self.binary_name.as_str();
    }
}

//end of file
