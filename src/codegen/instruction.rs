/*
 * instruction.rs
 * Enumerates instructions for the Chip-8 ISA
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
use super::super::data;
use super::CodeGen;

/// A Chip-8 instruction.
/// All instructions that
/// execute a calculation 
/// store the result in their
/// first argument. 
pub enum Instruction {
    /// Clears the screen
    CLS,

    /// Returns from a subroutine
    RET,

    /// Unconditionally jumps to an address
    JMP(data::JmpData),

    /// Calls a subroutine
    CALL(data::CallData),

    /// Skips the next instruction if
    /// a `SkipType` condition is met
    SKIP(data::SkipData),

    /// Assigns values to registers
    /// or moves one register's value
    /// into another
    MOV(data::MovData),

    /// Adds two registers, or
    /// a register and a value. 
    /// When both operands are
    /// registers, `VF` is set to `1` 
    /// when the result
    /// exceeds the size of the
    /// destination register, and `0`
    /// otherwise. 
    ADD(data::AddData),

    /// Bitwise ORs two registers
    OR(data::OrData),

    /// Bitwise ANDs two registers
    AND(data::AndData),

    /// Bitwise XORs two registers
    XOR(data::XorData),

    /// Subtracts one register
    /// from another, with the
    /// second register subtracted
    /// from the first. `VF` is
    /// set to `0` when there's
    /// a borrow, and `1` if
    /// there isn't. 
    SUB(data::SubData),

    /// Stores the least significant
    /// bit of a register in `VF`
    /// and shifts that register
    /// to the right 1 bit
    SHR(data::ShrData),

    /// Subtracts one register
    /// from another, with the 
    /// first register subtracted
    /// from the second. `VF` is 
    /// set to `0` when there's
    /// a borrow, and `1` when 
    /// there isn't. 
    SUBN(data::SubnData),

    /// Stores the most significant
    /// bit of a register in `VF`
    /// and shifts that register
    /// to the left one bit
    SHL(data::ShlData),

    /// Unconditionally jumps to
    /// an address defined by
    /// a constant plus the
    /// `V0` register
    JPC(data::JpcData),

    /// Stores a random 8-bit 
    /// integer bitwise ANDed
    /// with a given constant 
    /// in a register
    RAND(data::RandData),

    /// Draws a sprite at
    /// coordinates defined
    /// by two registers with
    /// a width of 8 pixels
    /// and a given height. The
    /// sprite is read as bits
    /// from memory starting at 
    /// address `I`. `VF` is set
    /// to `1` if any pixels are
    /// flipped from on to off, and
    /// to `0` if not. 
    DRAW(data::DrawData),

    /// Sets a register to the 
    /// current value of the
    /// delay timer
    GDL(data::GdlData),

    /// Stops execution until a key is
    /// pressed, then stores that key
    /// in a register
    KEY(data::KeyData),

    /// Sets the delay timer to
    /// the value of a register
    SDL(data::SdlData),

    /// Sets the sound timer to
    /// the value of a register
    SND(data::SndData),

    /// Puts the memory location
    /// of the hex character 
    /// corresponding to the value
    /// of a register into `I`
    SCH(data::SchData),

    /// Stores the binary-coded decimal
    /// representation of a register
    /// in memory location `I`, with
    /// the hundreds digit in the lowest
    /// byte. 
    BCD(data::BcdData),

    /// Dumps the contents of all registers
    /// up to and including a specified register
    /// into memory starting at location `I`
    RDP(data::RdpData),

    /// Loads all registers up to and including
    /// a specified register from data starting
    /// at location `I`
    RLD(data::RldData)
}

//CodeGen implementation
impl CodeGen for Instruction {
    /// Generates the opcode for
    /// the `Instruction`
    ///
    /// # Returns
    ///
    /// The opcode corresponding to
    /// the instruction and its
    /// data fields
    fn gen_opcode(&self) -> u16 {
        return match *self {
            Instruction::CLS => 0x00E0,
            Instruction::RET => 0x00EE,
            Instruction::JMP(ref data) => data.gen_opcode(),
            Instruction::CALL(ref data) => data.gen_opcode(),
            Instruction::SKIP(ref data) => data.gen_opcode(),
            Instruction::MOV(ref data) => data.gen_opcode(),
            Instruction::ADD(ref data) => data.gen_opcode(),
            Instruction::OR(ref data) => data.gen_opcode(),
            Instruction::AND(ref data) => data.gen_opcode(),
            Instruction::XOR(ref data) => data.gen_opcode(),
            Instruction::SUB(ref data) => data.gen_opcode(),
            Instruction::SHR(ref data) => data.gen_opcode(),
            Instruction::SUBN(ref data) => data.gen_opcode(),
            Instruction::SHL(ref data) => data.gen_opcode(),
            Instruction::JPC(ref data) => data.gen_opcode(),
            Instruction::RAND(ref data) => data.gen_opcode(),
            Instruction::DRAW(ref data) => data.gen_opcode(),
            Instruction::GDL(ref data) => data.gen_opcode(),
            Instruction::KEY(ref data) => data.gen_opcode(),
            Instruction::SDL(ref data) => data.gen_opcode(),
            Instruction::SND(ref data) => data.gen_opcode(),
            Instruction::SCH(ref data) => data.gen_opcode(),
            Instruction::BCD(ref data) => data.gen_opcode(),
            Instruction::RDP(ref data) => data.gen_opcode(),
            Instruction::RLD(ref data) => data.gen_opcode()
        };
    }
}

//unit tests
#[cfg(test)]
mod tests {
    //import the Instruction enum
    use super::*;

    //this test checks generation 
    //of the CLS opcode
    #[test]
    fn test_cls_opcode_gen() {
        let instr = Instruction::CLS;
        assert_eq!(instr.gen_opcode(), 0x00E0);
    }

    //this test checks generation
    //of the RET opcode
    #[test]
    fn test_ret_opcode_gen() {
        let instr = Instruction::RET;
        assert_eq!(instr.gen_opcode(), 0x00EE);
    }
}

//end of file
