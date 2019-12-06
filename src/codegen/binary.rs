/*
 * binary.rs
 * Defines a struct that represents a Chip-8 binary
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

//crate import
extern crate byteorder;

//usage statements
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use super::Instruction;
use super::CodeGen;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use super::super::error::{BinaryError, BinaryErrorType};

/// A Chip-8 binary
pub struct Binary {
    /// The raw bytes that make up
    /// the binary
    data: Vec<u8>,

    /// The path to the binary file
    name: String,

    /// The length of the binary (in bytes)
    length: u16 
}

//implementation
impl Binary {
    /// Constructs a new `Binary` instance
    ///
    /// # Argument
    ///
    /// * `new_name` - The name of the binary file 
    ///
    /// # Returns
    ///
    /// A new `Binary` instance, wrapped in a `Result`
    pub fn new(new_name: &str) -> Result<Binary, BinaryError> {
        //check to see if the name exists
        if Path::new(new_name).exists() {
            return Err(BinaryError::new(BinaryErrorType::FileExists,
                                        new_name));
        } else {
            return Ok(Binary {
                data: Vec::new(),
                name: String::from(new_name),
                length: 0
            });
        }
    }

    /// Gets the length of the binary
    /// 
    /// # Returns
    ///
    /// The length of the binary (in bytes)
    pub fn len(&self) -> u16 {
        return self.length;
    }

    /// Adds an instruction to the binary
    /// 
    /// # Argument
    ///
    /// * `instr` - The `Instruction` to add
    ///
    /// # Returns
    ///
    /// `Ok` if the instruction was added successfully,
    /// `Err<BinaryError>` if the operation fails
    pub fn add_instruction(&mut self, instr: &Instruction)
        -> Result<(), BinaryError> {
        //add the instruction and return the result
        return self.add_word(instr.gen_opcode());
    }

    /// Adds a 16-bit word to the binary 
    /// 
    /// # Argument
    ///
    /// * `word` - The word to add
    ///
    /// # Returns
    ///
    /// `Ok` if the word was written successfully,
    /// `Err<BinaryError>` if the operation fails
    pub fn add_word(&mut self, word: u16) -> Result<(), BinaryError> {
        //attempt to add the word
        let res = self.data.write_u16::<BigEndian>(word);

        //and determine whether an error occurred
        if res.is_err() {
            return Err(BinaryError::new(BinaryErrorType::DataError,
                                        self.name.as_str()));
        } else {
            self.length += 2;
            return Ok(());
        }
    }

    /// Adds a byte to the binary
    /// 
    /// # Argument
    ///
    /// * `byte` - The byte to add
    /// 
    /// # Returns
    ///
    /// `Ok` if the byte was written successfully,
    /// `Err<BinaryError>` if the operation fails
    pub fn add_byte(&mut self, byte: u8) -> Result<(), BinaryError> {
        //attempt to write the byte
        let res = self.data.write_u8(byte);

        //and determine whether an error occurred
        if res.is_err() {
            return Err(BinaryError::new(BinaryErrorType::DataError,
                                        self.name.as_str()));
        } else {
            self.length += 1;
            return Ok(());
        }
    }

    /// Writes the entire binary to a file
    /// 
    /// # Returns
    ///
    /// A `Result` that on a success contains the number of bytes
    /// written, and that on a failure contains a `BinaryError` object
    ///
    /// # Panics
    ///
    /// This method will panic if the binary file handle 
    /// fails to be created successfully. 
    pub fn write_to_file(&mut self) -> Result<usize, BinaryError> {
        //ensure that the binary has an even number of bytes
        if (self.length % 2) != 0 {
            self.add_byte(0x00).unwrap();
        }

        //create the file
        let mut file = match File::create(self.name.as_str()) {
            Err(why) => panic!("Couldn't create {}: {:?}",
                               self.name.as_str(), why),
            Ok(file) => file 
        };

        //write the buffer to the file
        let res = file.write(self.data.as_slice());

        //and determine whether an error occurred
        if res.is_ok() {
            let size = res.unwrap();
            return Ok(size);
        } else {
            return Err(BinaryError::new(BinaryErrorType::FileError,
                                        self.name.as_str()));
        }
    }
}

