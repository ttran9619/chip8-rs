pub fn parse_instruction(input: &[u8; 2]) -> Option<Instruction> {
    let (_, result) = instruction(input).ok()?;
    Some(result)
}

use crate::emulator::instruction::Instruction;
use crate::emulator::types::{
    EightBitValue, FourBitValue, MemoryAddress, RegisterNumber, TwelveBitValue,
};

use nom::{
    bits,
    branch::alt,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

pub fn instruction(input: &[u8; 2]) -> IResult<&[u8], Instruction> {
    let opcode_parsers = alt((
        opcode_0, opcode_1, opcode_2, opcode_3, opcode_4, opcode_5, opcode_6, opcode_7, opcode_8,
        opcode_9, opcode_a, opcode_b, opcode_c, opcode_d, opcode_e, opcode_f,
    ));

    bits::bits(opcode_parsers)(input)
}

fn opcode_0(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let clear_display = |input| {
        let (input, _) = bits::complete::tag(0x0E0, 12usize)(input)?;
        Ok((input, Instruction::ClearDisplay))
    };
    let return_from_subroutine = |input| {
        let (input, _) = bits::complete::tag(0x0EE, 12usize)(input)?;
        Ok((input, Instruction::ReturnFromSubroutine))
    };
    let system = |input| {
        let (input, address) = memory_address(input)?;
        Ok((input, Instruction::System { address }))
    };

    let sub_opcodes = alt((clear_display, return_from_subroutine, system));
    preceded(match_opcode::<0>, sub_opcodes)(input)
}
fn opcode_1(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let (input, address) = preceded(match_opcode::<1>, memory_address)(input)?;
    Ok((input, Instruction::Jump { address }))
}
fn opcode_2(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let (input, address) = preceded(match_opcode::<2>, memory_address)(input)?;
    Ok((input, Instruction::Call { address }))
}
fn opcode_3(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, eight_bit_value);
    let (input, (read_value_from, immediate)) = preceded(match_opcode::<3>, data)(input)?;

    Ok((
        input,
        Instruction::SkipNextInstructionIfMatch {
            read_value_from,
            immediate,
        },
    ))
}
fn opcode_4(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, eight_bit_value);
    let (input, (read_value_from, immediate)) = preceded(match_opcode::<4>, data)(input)?;

    Ok((
        input,
        Instruction::SkipNextInstructionIfNotMatch {
            read_value_from,
            immediate,
        },
    ))
}
fn opcode_5(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, register_number);
    let (input, (lhs, rhs)) = delimited(match_opcode::<5>, data, match_nibble::<0>)(input)?;

    Ok((
        input,
        Instruction::SkipNextInstructionIfValuesMatch { lhs, rhs },
    ))
}
fn opcode_6(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, eight_bit_value);
    let (input, (destination, immediate)) = preceded(match_opcode::<6>, data)(input)?;

    Ok((
        input,
        Instruction::LoadImmediateToRegister {
            immediate,
            destination,
        },
    ))
}
fn opcode_7(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, eight_bit_value);
    let (input, (destination, immediate)) = preceded(match_opcode::<7>, data)(input)?;

    Ok((
        input,
        Instruction::AddImmediateToRegister {
            immediate,
            destination,
        },
    ))
}
fn opcode_8(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    fn sub_opcode<const VALUE: i32>(
        input: (&[u8], usize),
    ) -> IResult<(&[u8], usize), (RegisterNumber, RegisterNumber)> {
        let data = pair(register_number, register_number);
        terminated(data, match_nibble::<VALUE>)(input)
    }
    let copy = |input| {
        let (input, (destination, source)) = sub_opcode::<0>(input)?;
        Ok((
            input,
            Instruction::CopyRegisterValue {
                source,
                destination,
            },
        ))
    };
    let bitwise_or = |input| {
        let (input, (destination, source)) = sub_opcode::<1>(input)?;
        Ok((
            input,
            Instruction::BitwiseOrRegisters {
                source,
                destination,
            },
        ))
    };
    let bitwise_and = |input| {
        let (input, (destination, source)) = sub_opcode::<2>(input)?;
        Ok((
            input,
            Instruction::BitwiseAndRegisters {
                source,
                destination,
            },
        ))
    };
    let bitwise_xor = |input| {
        let (input, (destination, source)) = sub_opcode::<3>(input)?;
        Ok((
            input,
            Instruction::BitwiseXorRegisters {
                source,
                destination,
            },
        ))
    };
    let add = |input| {
        let (input, (destination, source)) = sub_opcode::<4>(input)?;
        Ok((
            input,
            Instruction::AddRegisters {
                source,
                destination,
            },
        ))
    };
    let subtract_destination_from_source = |input| {
        let (input, (destination, source)) = sub_opcode::<5>(input)?;
        Ok((
            input,
            Instruction::SubtractDestinationFromSource {
                source,
                destination,
            },
        ))
    };
    let shift_right = |input| {
        let (input, (destination, source)) = sub_opcode::<6>(input)?;
        Ok((
            input,
            Instruction::ShiftRightRegisters {
                source,
                destination,
            },
        ))
    };
    let subtract_source_from_destination = |input| {
        let (input, (destination, source)) = sub_opcode::<7>(input)?;
        Ok((
            input,
            Instruction::SubtractSourceFromDestination {
                source,
                destination,
            },
        ))
    };
    let shift_left = |input| {
        let (input, (destination, source)) = sub_opcode::<0xE>(input)?;
        Ok((
            input,
            Instruction::ShiftLeftRegisters {
                source,
                destination,
            },
        ))
    };

    let sub_opcodes = alt((
        copy,
        bitwise_or,
        bitwise_and,
        bitwise_xor,
        add,
        subtract_destination_from_source,
        shift_right,
        subtract_source_from_destination,
        shift_left,
    ));
    preceded(match_opcode::<8>, sub_opcodes)(input)
}
fn opcode_9(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, register_number);
    let (input, (lhs, rhs)) = delimited(match_opcode::<9>, data, match_nibble::<0>)(input)?;

    Ok((
        input,
        Instruction::SkipNextInstructionIfValuesDoNotMatch { lhs, rhs },
    ))
}
fn opcode_a(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let (input, immediate) = preceded(match_opcode::<0xa>, twelve_bit_value)(input)?;
    Ok((input, Instruction::LoadToIRegister { immediate }))
}
fn opcode_b(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let (input, immediate) = preceded(match_opcode::<0xb>, twelve_bit_value)(input)?;
    Ok((
        input,
        Instruction::JumpToSumOfV0ValueAndImmediate { immediate },
    ))
}
fn opcode_c(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = pair(register_number, eight_bit_value);
    let (input, (destination, immediate)) = preceded(match_opcode::<0xc>, data)(input)?;
    Ok((
        input,
        Instruction::LoadBitwiseAndOfRandomByteAndImmediate {
            destination,
            immediate,
        },
    ))
}
fn opcode_d(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let data = tuple((register_number, register_number, four_bit_value));
    let (input, (read_x_axis_from, read_y_axis_from, bytes_to_read_from_i_register)) =
        preceded(match_opcode::<0xd>, data)(input)?;
    Ok((
        input,
        Instruction::DrawSpritesFromMemory {
            read_x_axis_from,
            read_y_axis_from,
            bytes_to_read_from_i_register,
        },
    ))
}
fn opcode_e(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let skip_key_pressed = |input| {
        let (input, read_key_number_from) = register_terminated_by_byte::<0x9e>(input)?;
        Ok((
            input,
            Instruction::SkipNextInstructionIfKeyIsPressed {
                read_key_number_from,
            },
        ))
    };
    let skip_key_not_pressed = |input| {
        let (input, read_key_number_from) = register_terminated_by_byte::<0xa1>(input)?;
        Ok((
            input,
            Instruction::SkipNextInstructionIfKeyIsNotPressed {
                read_key_number_from,
            },
        ))
    };

    let sub_opcodes = alt((skip_key_pressed, skip_key_not_pressed));
    preceded(match_opcode::<0xe>, sub_opcodes)(input)
}
fn opcode_f(input: (&[u8], usize)) -> IResult<(&[u8], usize), Instruction> {
    let load_delay_timer_into_register = |input| {
        let (input, destination) = register_terminated_by_byte::<0x07>(input)?;
        Ok((
            input,
            Instruction::LoadDelayTimerIntoRegister { destination },
        ))
    };
    let await_key_press_and_load_into_register = |input| {
        let (input, destination) = register_terminated_by_byte::<0x0a>(input)?;
        Ok((
            input,
            Instruction::AwaitKeyPressAndLoadIntoRegister { destination },
        ))
    };
    let load_into_delay_timer = |input| {
        let (input, source) = register_terminated_by_byte::<0x15>(input)?;
        Ok((input, Instruction::LoadIntoDelayTimer { source }))
    };
    let load_into_sound_timer = |input| {
        let (input, source) = register_terminated_by_byte::<0x18>(input)?;
        Ok((input, Instruction::LoadIntoSoundTimer { source }))
    };
    let add_value_to_i = |input| {
        let (input, source) = register_terminated_by_byte::<0x1e>(input)?;
        Ok((input, Instruction::AddValueToIRegister { source }))
    };
    let load_sprite = |input| {
        let (input, source) = register_terminated_by_byte::<0x29>(input)?;
        Ok((
            input,
            Instruction::LoadSpriteLocationForValueIntoIRegister { source },
        ))
    };
    let load_bcd = |input| {
        let (input, source) = register_terminated_by_byte::<0x33>(input)?;
        Ok((
            input,
            Instruction::LoadBinaryCodedDecimalValueIntoSequenceStartingAtIRegisterValue { source },
        ))
    };
    let store_to_memory = |input| {
        let (input, end) = register_terminated_by_byte::<0x55>(input)?;
        Ok((
            input,
            Instruction::LoadValuesFromV0ToRegisterIntoSequenceStartingAtIRegisterValue { end },
        ))
    };
    let load_from_memory = |input| {
        let (input, end) = register_terminated_by_byte::<0x65>(input)?;
        Ok((
            input,
            Instruction::LoadSequenceStartingAtIRegisterValueIntoV0ToRegister { end },
        ))
    };

    let sub_opcodes = alt((
        load_delay_timer_into_register,
        await_key_press_and_load_into_register,
        load_into_delay_timer,
        load_into_sound_timer,
        add_value_to_i,
        load_sprite,
        load_bcd,
        store_to_memory,
        load_from_memory,
    ));
    preceded(match_opcode::<0xf>, sub_opcodes)(input)
}

// Utilities
fn register_terminated_by_byte<const VALUE: i32>(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), RegisterNumber> {
    terminated(register_number, bits::complete::tag(VALUE, 8usize))(input)
}

fn match_nibble<const VALUE: i32>(input: (&[u8], usize)) -> IResult<(&[u8], usize), i32> {
    bits::complete::tag(VALUE, 4usize)(input)
}

fn match_opcode<const OPCODE: i32>(input: (&[u8], usize)) -> IResult<(&[u8], usize), i32> {
    match_nibble::<OPCODE>(input)
}

fn register_number(input: (&[u8], usize)) -> IResult<(&[u8], usize), RegisterNumber> {
    let (input, value) = four_bit_value(input)?;
    Ok((input, RegisterNumber(value)))
}

fn memory_address(input: (&[u8], usize)) -> IResult<(&[u8], usize), MemoryAddress> {
    let (input, value) = twelve_bit_value(input)?;
    Ok((input, MemoryAddress(value)))
}

fn four_bit_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), FourBitValue> {
    let (input, raw_value) = bits::complete::take(4usize)(input)?;
    Ok((input, FourBitValue(raw_value)))
}

fn eight_bit_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), EightBitValue> {
    let (input, raw_value) = bits::complete::take(8usize)(input)?;
    Ok((input, EightBitValue(raw_value)))
}

fn twelve_bit_value(input: (&[u8], usize)) -> IResult<(&[u8], usize), TwelveBitValue> {
    let (input, raw_value) = bits::complete::take(12usize)(input)?;
    Ok((input, TwelveBitValue(raw_value)))
}
