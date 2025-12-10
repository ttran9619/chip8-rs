pub mod parser;

mod tests;

use super::types::{EightBitValue, FourBitValue, MemoryAddress, RegisterNumber, TwelveBitValue};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    // OpCode 0
    System {
        address: MemoryAddress,
    },
    ClearDisplay,
    ReturnFromSubroutine,

    // OpCode 1
    Jump {
        address: MemoryAddress,
    },

    // OpCode 2
    Call {
        address: MemoryAddress,
    },

    // OpCode 3
    SkipNextInstructionIfMatch {
        read_value_from: RegisterNumber,
        immediate: EightBitValue,
    },

    // OpCode 4
    SkipNextInstructionIfNotMatch {
        read_value_from: RegisterNumber,
        immediate: EightBitValue,
    },

    // OpCode 5
    SkipNextInstructionIfValuesMatch {
        lhs: RegisterNumber,
        rhs: RegisterNumber,
    },

    // OpCode 6
    LoadImmediateToRegister {
        immediate: EightBitValue,
        destination: RegisterNumber,
    },

    // OpCode 7
    AddImmediateToRegister {
        immediate: EightBitValue,
        destination: RegisterNumber,
    },

    // OpCode 8
    CopyRegisterValue {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    BitwiseOrRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    BitwiseAndRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    BitwiseXorRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    AddRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    SubtractDestinationFromSource {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    ShiftRightRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    SubtractSourceFromDestination {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    ShiftLeftRegisters {
        source: RegisterNumber,
        destination: RegisterNumber,
    },

    // OpCode 9
    SkipNextInstructionIfValuesDoNotMatch {
        lhs: RegisterNumber,
        rhs: RegisterNumber,
    },

    // OpCode A
    LoadToIRegister {
        immediate: TwelveBitValue,
    },

    // OpCode B
    JumpToSumOfV0ValueAndImmediate {
        immediate: TwelveBitValue,
    },

    // OpCode C
    LoadBitwiseAndOfRandomByteAndImmediate {
        destination: RegisterNumber,
        immediate: EightBitValue,
    },

    // OpCode D
    DrawSpritesFromMemory {
        read_x_axis_from: RegisterNumber,
        read_y_axis_from: RegisterNumber,
        bytes_to_read_from_i_register: FourBitValue,
    },

    // OpCode E
    SkipNextInstructionIfKeyIsPressed {
        read_key_number_from: RegisterNumber,
    },
    SkipNextInstructionIfKeyIsNotPressed {
        read_key_number_from: RegisterNumber,
    },

    // OpCode F
    LoadDelayTimerIntoRegister {
        destination: RegisterNumber,
    },
    AwaitKeyPressAndLoadIntoRegister {
        destination: RegisterNumber,
    },
    LoadIntoDelayTimer {
        source: RegisterNumber,
    },
    LoadIntoSoundTimer {
        source: RegisterNumber,
    },
    AddValueToIRegister {
        source: RegisterNumber,
    },
    LoadSpriteLocationForValueIntoIRegister {
        source: RegisterNumber,
    },
    LoadBinaryCodedDecimalValueIntoSequenceStartingAtIRegisterValue {
        source: RegisterNumber,
    },
    LoadValuesFromV0ToRegisterIntoSequenceStartingAtIRegisterValue {
        end: RegisterNumber,
    },
    LoadSequenceStartingAtIRegisterValueIntoV0ToRegister {
        end: RegisterNumber,
    },
}
