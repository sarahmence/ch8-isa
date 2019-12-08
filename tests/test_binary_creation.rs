/*
 * test_binary_creation.rs
 * Defines integration tests for generating Chip-8 binaries
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

extern crate ch8_isa;
extern crate byteorder;
use byteorder::{ReadBytesExt, BigEndian};
use ch8_isa::*;
use std::fs;
use std::mem::drop;

//this test checks creating a binary from Chip-8 instructions
#[test]
fn test_binary_creation() {
    //create the Binary
    let mut rom = codegen::Binary::new("test_tmp.c8").unwrap();

    //create instructions 
    let i1 = codegen::Instruction::CLS;
    let i2 = codegen::Instruction::CALL(data::CallData::new(0x0FC4));
    let i3 = codegen::Instruction::DRAW(data::DrawData::new(
                                        data::Register::V1,
                                        data::Register::V2,
                                        0x5));

    //write them to the binary
    rom.add_instruction(&i1).unwrap();
    rom.add_instruction(&i2).unwrap();
    rom.add_instruction(&i3).unwrap();

    //write a word
    rom.add_word(0xFC00).unwrap();

    //write a single byte
    rom.add_byte(0xAB).unwrap();

    //write the binary to a file
    let sz = rom.write_to_file().unwrap();

    //verify that the correct number of bytes was written
    assert_eq!(sz, 10);

    //open the rom file
    let mut file = fs::File::open("test_tmp.c8").unwrap();

    //read the 5 words and validate them
    let mut word = file.read_u16::<BigEndian>().unwrap();
    assert_eq!(word, 0x00E0);
    word = file.read_u16::<BigEndian>().unwrap();
    assert_eq!(word, 0x2FC4);
    word = file.read_u16::<BigEndian>().unwrap();
    assert_eq!(word, 0xD125);
    word = file.read_u16::<BigEndian>().unwrap();
    assert_eq!(word, 0xFC00);
    word = file.read_u16::<BigEndian>().unwrap();
    assert_eq!(word, 0xAB00);

    //close the file
    drop(file);

    //and delete the file
    fs::remove_file("test_tmp.c8").unwrap();
}


