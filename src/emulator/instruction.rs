use super::types::{EightBitValue, FourBitValue, MemoryAddress, RegisterNumber, TwelveBitValue};
pub enum Instruction {
    System {
        address: MemoryAddress,
    },
    ClearDisplay,
    ReturnFromSubroutine,
    Jump {
        address: MemoryAddress,
    },
    Call {
        address: MemoryAddress,
    },
    SkipNextInstructionIfMatch {
        read_value_from: RegisterNumber,
        immediate: EightBitValue,
    },
    SkipNextInstructionIfNotMatch {
        read_value_from: RegisterNumber,
        immediate: EightBitValue,
    },
    SkipNextInstructionIfValuesMatch {
        lhs: RegisterNumber,
        rhs: RegisterNumber,
    },
    LoadImmediateToRegister {
        immediate: EightBitValue,
        destination: RegisterNumber,
    },
    AddImmediateToRegister {
        immediate: EightBitValue,
        destination: RegisterNumber,
    },
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
        // Some implementations ignore source and perform shift using destination
        destination: RegisterNumber,
    },
    SubtractSourceFromDestination {
        source: RegisterNumber,
        destination: RegisterNumber,
    },
    ShiftLeftRegisters {
        source: RegisterNumber,
        // Some implementations ignore source and perform shift using destination
        destination: RegisterNumber,
    },
    SkipNextInstructionIfValuesDoNotMatch {
        lhs: RegisterNumber,
        rhs: RegisterNumber,
    },
    LoadToIRegister {
        immediate: TwelveBitValue,
    },
    JumpToSumOfV0ValueAndImmediate {
        immediate: TwelveBitValue,
    },
    LoadSumOfRandomByteAndImmediate {
        destination: RegisterNumber,
        immediate: EightBitValue,
    },
    DrawSpritesFromMemory {
        read_x_axis_from: RegisterNumber,
        read_y_axis_from: RegisterNumber,
        bytes_to_read_from_i_register: FourBitValue,
    },
    SkipNextInstructionIfKeyIsPressed {
        read_key_number_from: RegisterNumber,
    },
    SkipNextInstructionIfKeyIsNotPressed {
        read_key_number_from: RegisterNumber,
    },
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
