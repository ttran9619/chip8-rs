#[cfg(test)]
mod tests {
    use crate::emulator::instruction::*;
    use crate::emulator::instruction::parser::*;


    use nom_test_helpers::prelude::*;

    // OpCode 0
    #[test]
    fn clear_display() {
        // Act
        let parsed = instruction(b"\x00\xE0");

        // Verify
        let expected_value = Instruction::ClearDisplay;
        assert_finished_and_eq!(parsed, expected_value);
    }

    #[test]
    fn system_instruction() {
        // Act
        let parsed = instruction(b"\x01\x23");

        // Verify
        let expected_value = Instruction::System {
            address: MemoryAddress(TwelveBitValue(0x123)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn return_from_subroutine() {
        // Act
        let parsed = instruction(b"\x00\xEE");

        // Verify
        let expected_value = Instruction::ReturnFromSubroutine;
        assert_finished_and_eq!(parsed, expected_value);
    }

    // OpCode 1
    #[test]
    fn jump() {
        // Act
        let parsed = instruction(b"\x11\x23");

        // Verify
        let expected_value = Instruction::Jump {
            address: MemoryAddress(TwelveBitValue(0x123)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 2
    #[test]
    fn call() {
        // Act
        let parsed = instruction(b"\x21\x23");

        // Verify
        let expected_value = Instruction::Call {
            address: MemoryAddress(TwelveBitValue(0x123)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 3
    #[test]
    fn skip_next_instruction_if_match() {
        // Act
        let parsed = instruction(b"\x31\x23");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfMatch {
            read_value_from: RegisterNumber(FourBitValue(0x1)),
            immediate: EightBitValue(0x23),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 4
    #[test]
    fn skip_next_instruction_if_not_match() {
        // Act
        let parsed = instruction(b"\x41\x23");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfNotMatch {
            read_value_from: RegisterNumber(FourBitValue(0x1)),
            immediate: EightBitValue(0x23),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 5
    #[test]
    fn skip_next_instruction_if_values_match() {
        // Act
        let parsed = instruction(b"\x51\x20");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfValuesMatch {
            lhs: RegisterNumber(FourBitValue(0x1)),
            rhs: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 6
    #[test]
    fn load_immediate_to_register() {
        // Act
        let parsed = instruction(b"\x61\x23");

        // Verify
        let expected_value = Instruction::LoadImmediateToRegister {
            destination: RegisterNumber(FourBitValue(0x1)),
            immediate: EightBitValue(0x23),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 7
    #[test]
    fn add_immediate_to_register() {
        // Act
        let parsed = instruction(b"\x71\x23");

        // Verify
        let expected_value = Instruction::AddImmediateToRegister {
            destination: RegisterNumber(FourBitValue(0x1)),
            immediate: EightBitValue(0x23),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 8
    #[test]
    fn copy_values() {
        // Act
        let parsed = instruction(b"\x81\x20");

        // Verify
        let expected_value = Instruction::CopyRegisterValue {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn bitwise_or_values() {
        // Act
        let parsed = instruction(b"\x81\x21");

        // Verify
        let expected_value = Instruction::BitwiseOrRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn bitwise_and_values() {
        // Act
        let parsed = instruction(b"\x81\x22");

        // Verify
        let expected_value = Instruction::BitwiseAndRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn bitwise_values() {
        // Act
        let parsed = instruction(b"\x81\x23");

        // Verify
        let expected_value = Instruction::BitwiseXorRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn add_values() {
        // Act
        let parsed = instruction(b"\x81\x24");

        // Verify
        let expected_value = Instruction::AddRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn subtract_destination_from_source_values() {
        // Act
        let parsed = instruction(b"\x81\x25");

        // Verify
        let expected_value = Instruction::SubtractDestinationFromSource {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn shift_right_values() {
        // Act
        let parsed = instruction(b"\x81\x26");

        // Verify
        let expected_value = Instruction::ShiftRightRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn subtract_source_from_destination_values() {
        // Act
        let parsed = instruction(b"\x81\x27");

        // Verify
        let expected_value = Instruction::SubtractSourceFromDestination {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn shift_left_values() {
        // Act
        let parsed = instruction(b"\x81\x2e");

        // Verify
        let expected_value = Instruction::ShiftLeftRegisters {
            destination: RegisterNumber(FourBitValue(0x1)),
            source: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode 9
    #[test]
    fn skip_next_instruction_if_values_do_not_match() {
        // Act
        let parsed = instruction(b"\x91\x20");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfValuesDoNotMatch {
            lhs: RegisterNumber(FourBitValue(0x1)),
            rhs: RegisterNumber(FourBitValue(0x2)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode A
    #[test]
    fn load_to_i_register() {
        // Act
        let parsed = instruction(b"\xa1\x23");

        // Verify
        let expected_value = Instruction::LoadToIRegister {
            immediate: TwelveBitValue(0x123),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode B
    #[test]
    fn jump_to_sum_of_v0_value_and_immediate() {
        // Act
        let parsed = instruction(b"\xb1\x23");

        // Verify
        let expected_value = Instruction::JumpToSumOfV0ValueAndImmediate {
            immediate: TwelveBitValue(0x123),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode C
    #[test]
    fn load_sum_of_random_byte_and_immediate() {
        // Act
        let parsed = instruction(b"\xc1\x23");

        // Verify
        let expected_value = Instruction::LoadBitwiseAndOfRandomByteAndImmediate {
            destination: RegisterNumber(FourBitValue(0x1)),
            immediate: EightBitValue(0x23),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode D
    #[test]
    fn draw_sprites_from_memory() {
        // Act
        let parsed = instruction(b"\xd1\x23");

        // Verify
        let expected_value = Instruction::DrawSpritesFromMemory {
            read_x_axis_from: RegisterNumber(FourBitValue(0x1)),
            read_y_axis_from: RegisterNumber(FourBitValue(0x2)),
            bytes_to_read_from_i_register: FourBitValue(0x3),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode E
    #[test]
    fn skip_next_instruction_if_key_is_pressed() {
        // Act
        let parsed = instruction(b"\xe1\x9e");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfKeyIsPressed {
            read_key_number_from: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn skip_next_instruction_if_key_is_not_pressed() {
        // Act
        let parsed = instruction(b"\xe1\xa1");

        // Verify
        let expected_value = Instruction::SkipNextInstructionIfKeyIsNotPressed {
            read_key_number_from: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    // OpCode F
    #[test]
    fn load_delay_timer_into_register() {
        // Act
        let parsed = instruction(b"\xf1\x07");

        // Verify
        let expected_value = Instruction::LoadDelayTimerIntoRegister {
            destination: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }
    #[test]
    fn await_key_press_and_load_into_register() {
        // Act
        let parsed = instruction(b"\xf1\x0a");

        // Verify
        let expected_value = Instruction::AwaitKeyPressAndLoadIntoRegister {
            destination: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }
    #[test]
    fn load_into_delay_timer() {
        // Act
        let parsed = instruction(b"\xf1\x15");

        // Verify
        let expected_value = Instruction::LoadIntoDelayTimer {
            source: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }
    #[test]
    fn load_into_sound_timer() {
        // Act
        let parsed = instruction(b"\xf1\x18");

        // Verify
        let expected_value = Instruction::LoadIntoSoundTimer {
            source: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn add_value_to_i_register() {
        // Act
        let parsed = instruction(b"\xf1\x1e");

        // Verify
        let expected_value = Instruction::AddValueToIRegister {
            source: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn load_sprite_location_for_value_into_i_register() {
        // Act
        let parsed = instruction(b"\xf1\x29");

        // Verify
        let expected_value = Instruction::LoadSpriteLocationForValueIntoIRegister {
            source: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn load_bcd_into_register_sequence() {
        // Act
        let parsed = instruction(b"\xf1\x33");

        // Verify
        let expected_value =
            Instruction::LoadBinaryCodedDecimalValueIntoSequenceStartingAtIRegisterValue {
                source: RegisterNumber(FourBitValue(0x1)),
            };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn store_sequence_into_memory() {
        // Act
        let parsed = instruction(b"\xf1\x55");

        // Verify
        let expected_value =
            Instruction::LoadValuesFromV0ToRegisterIntoSequenceStartingAtIRegisterValue {
                end: RegisterNumber(FourBitValue(0x1)),
            };

        assert_finished_and_eq!(parsed, expected_value)
    }

    #[test]
    fn load_sequence_from_memory() {
        // Act
        let parsed = instruction(b"\xf1\x65");

        // Verify
        let expected_value = Instruction::LoadSequenceStartingAtIRegisterValueIntoV0ToRegister {
            end: RegisterNumber(FourBitValue(0x1)),
        };

        assert_finished_and_eq!(parsed, expected_value)
    }
}
