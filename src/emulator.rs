pub mod platform;

mod instruction;
mod tests;
mod types;

use self::{
    instruction::Instruction,
    types::{EightBitValue, MemoryAddress, RegisterNumber},
};

use platform::*;
pub struct Emulator<PLATFORM: Platform> {
    // Internal State
    program_counter: u16,
    stack_pointer: usize,
    stack: Stack,

    // Program Accessible
    i_register: u16,
    v_registers: RegisterBank,
    sound_timer: u8,
    delay_timer: u8,

    // Memory
    memory: Memory,

    // Platform support
    platform: PLATFORM,
}

const MEMORY_SIZE: usize = 4096;
type Memory = [u8; MEMORY_SIZE];

const REGISTER_BANK_SIZE: usize = 16;
type RegisterBank = [u8; REGISTER_BANK_SIZE];

const STACK_SIZE: usize = 16;
type Stack = [u16; STACK_SIZE];

const DATA_START_ADDRESS: u16 = 0x200;

impl<PLATFORM: Platform> Emulator<PLATFORM> {
    pub fn new(platform: PLATFORM) -> Self {
        let program_counter = DATA_START_ADDRESS;
        let stack_pointer = 0;
        let i_register = 0;
        let sound_timer = 0;
        let delay_timer = 0;
        let memory: Memory = [0; MEMORY_SIZE];
        let v_registers: RegisterBank = [0; REGISTER_BANK_SIZE];
        let stack: Stack = [0; STACK_SIZE];

        Emulator {
            program_counter,
            stack_pointer,
            i_register,
            sound_timer,
            delay_timer,
            platform,
            v_registers,
            memory,
            stack,
        }
    }

    async fn load_into_memory(&mut self, data: &[u8]) {
        let destination = &mut self.memory[0..data.len()];
        // let mut destination = memory[DATA_START_ADDRESS as usize; data.len()];
        destination.copy_from_slice(&data);
    }

    async fn start_program(&mut self) {
        use futures::select;
        use futures::FutureExt;
        let instruction_frequency = core::time::Duration::from_secs(1).div_f32(500f32);
        let mut instruction_interval = async_io::Timer::interval(instruction_frequency).fuse();

        let timer_frequency = core::time::Duration::from_secs(1).div_f32(60f32);
        let mut timer_interval = async_io::Timer::interval(timer_frequency).fuse();

        select! {
            _ = timer_interval => self.handle_timers().await,
            _ = instruction_interval => self.run_instruction_loop().await,
        }
    }

    async fn handle_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.delay_timer -= 1;
        }

        self.handler_buzzer_state().await;
    }

    async fn run_instruction_loop(&mut self) {
        // Fetch
        let pc = self.program_counter as usize;

        self.increment_program_counter();

        let instruction_bytes: &[u8; 2] = &self.memory[pc..pc + 2].try_into().unwrap();
        // Decode
        let instruction = instruction::parser::parse_instruction(instruction_bytes).unwrap();

        // Execute
        self.execute_instruction(instruction).await;
    }

    async fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::System { address } | Instruction::Jump { address } => {
                self.set_program_counter(address.into());
            }
            Instruction::ClearDisplay => self.platform.clear_display().await,
            Instruction::ReturnFromSubroutine => {
                self.set_program_counter(self.stack[self.stack_pointer]);
                self.stack_pointer -= 1;
            }
            Instruction::Call { address } => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter;
                self.set_program_counter(address.into());
            }
            Instruction::SkipNextInstructionIfMatch {
                read_value_from,
                immediate,
            } => {
                let register_value = self.read_v_register(read_value_from);
                let immediate_value = immediate;

                if register_value == immediate_value {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipNextInstructionIfNotMatch {
                read_value_from,
                immediate,
            } => {
                let register_value = self.read_v_register(read_value_from);
                let immediate_value = immediate;

                if register_value != immediate_value {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipNextInstructionIfValuesMatch { lhs, rhs } => {
                let lhs_value = self.read_v_register(lhs);
                let rhs_value = self.read_v_register(rhs);

                if lhs_value == rhs_value {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipNextInstructionIfValuesDoNotMatch { lhs, rhs } => {
                let lhs_value = self.read_v_register(lhs);
                let rhs_value = self.read_v_register(rhs);

                if lhs_value != rhs_value {
                    self.increment_program_counter();
                }
            }
            Instruction::LoadImmediateToRegister {
                immediate,
                destination,
            } => {
                self.set_v_register(destination, immediate);
            }
            Instruction::AddImmediateToRegister {
                immediate,
                destination,
            } => {
                let value = self.read_v_register(destination) + immediate;

                self.set_v_register(destination, value);
            }
            Instruction::CopyRegisterValue {
                source,
                destination,
            } => {
                let value = self.read_v_register(source);

                self.set_v_register(destination, value);
            }
            Instruction::BitwiseOrRegisters {
                source,
                destination,
            } => {
                let source_value = self.read_v_register(source);
                let destination_value = self.read_v_register(source);

                self.set_v_register(destination, destination_value | source_value);
            }
            Instruction::BitwiseAndRegisters {
                source,
                destination,
            } => {
                let source_value = self.read_v_register(source);
                let destination_value = self.read_v_register(source);

                self.set_v_register(destination, destination_value & source_value);
            }
            Instruction::BitwiseXorRegisters {
                source,
                destination,
            } => {
                let source_value = self.read_v_register(source);
                let destination_value = self.read_v_register(source);

                self.set_v_register(destination, destination_value ^ source_value);
            }
            Instruction::AddRegisters {
                source,
                destination,
            } => {
                let source_value: u8 = self.read_v_register(source).into();
                let destination_value: u8 = self.read_v_register(destination).into();

                let (value, carry) = source_value.overflowing_add(destination_value);

                self.set_v_register(destination, value.into());
                self.set_carry_in_vf_register(carry);
            }
            Instruction::SubtractDestinationFromSource {
                source,
                destination,
            } => {
                let source_value = self.read_v_register(source);
                let destination_value = self.read_v_register(source);

                self.set_v_register(destination, source_value - destination_value);
            }
            Instruction::SubtractSourceFromDestination {
                source,
                destination,
            } => {
                let source_value = self.read_v_register(source);
                let destination_value = self.read_v_register(source);

                self.set_v_register(destination, destination_value - source_value);
            }
            Instruction::ShiftRightRegisters {
                source,
                destination,
            } => {
                let source_value: u8 = self.read_v_register(source).into();
                let destination_value: u8 = self.read_v_register(destination).into();

                let (value, carry) = source_value.overflowing_shr(destination_value.into());

                self.set_v_register(destination, value.into());
                self.set_carry_in_vf_register(carry);
            }
            Instruction::ShiftLeftRegisters {
                source,
                destination,
            } => {
                let source_value: u8 = self.read_v_register(source).into();
                let destination_value: u8 = self.read_v_register(destination).into();

                let (value, carry) = source_value.overflowing_shl(destination_value.into());

                self.set_v_register(destination, value.into());
                self.set_carry_in_vf_register(carry);
            }
            Instruction::LoadToIRegister { immediate } => {
                self.i_register = immediate.into();
            }
            Instruction::JumpToSumOfV0ValueAndImmediate { immediate } => {
                let register_value: u8 = self.read_v_register(RegisterNumber::zero()).into();
                let immediate_value: u16 = immediate.into();

                self.set_program_counter(immediate_value + register_value as u16);
            }
            Instruction::SkipNextInstructionIfKeyIsPressed {
                read_key_number_from,
            } => {
                let register_value: u8 = self.read_v_register(read_key_number_from).into();
                let expected_key_number = KeypadNumber(register_value.into());

                if let KeyState::On = self.platform.read_keypress_state(expected_key_number).await {
                    self.increment_program_counter();
                }
            }
            Instruction::SkipNextInstructionIfKeyIsNotPressed {
                read_key_number_from,
            } => {
                let register_value: u8 = self.read_v_register(read_key_number_from).into();
                let expected_key_number = KeypadNumber(register_value.into());

                if let KeyState::Off = self.platform.read_keypress_state(expected_key_number).await
                {
                    self.increment_program_counter();
                }
            }
            Instruction::LoadIntoDelayTimer { source } => {
                let register_value: u8 = self.read_v_register(source).into();
                self.delay_timer = register_value;
            }
            Instruction::LoadIntoSoundTimer { source } => {
                let register_value: u8 = self.read_v_register(source).into();
                self.sound_timer = register_value;
                self.handler_buzzer_state().await;
            }
            Instruction::AddValueToIRegister { source } => {
                let register_value: u8 = self.read_v_register(source).into();
                self.i_register += register_value as u16;
            }
            Instruction::LoadSpriteLocationForValueIntoIRegister { source } => {
                let register_value: u8 = self.read_v_register(source).into();
                if let Some(address) = self.find_address_of_value_in_memory(register_value) {
                    self.i_register = address.into();
                }
            }
            Instruction::LoadValuesFromV0ToRegisterIntoSequenceStartingAtIRegisterValue { end } => {
                let register_slice = Self::get_register_slice_up_to(&self.v_registers, end);

                let memory_start_address = MemoryAddress(self.i_register.into());
                let bytes_to_read: u8 = end.0.into();
                let memory_slice = Self::get_mut_memory_slice(
                    &mut self.memory,
                    memory_start_address,
                    bytes_to_read,
                );

                memory_slice.copy_from_slice(register_slice);
            }
            Instruction::LoadSequenceStartingAtIRegisterValueIntoV0ToRegister { end } => {
                let memory_start_address = MemoryAddress(self.i_register.into());
                let bytes_to_read: u8 = end.0.into();
                let memory_slice =
                    Self::get_memory_slice(&mut self.memory, memory_start_address, bytes_to_read);

                let register_slice = Self::get_mut_register_slice_up_to(&mut self.v_registers, end);
                register_slice.copy_from_slice(memory_slice);
            }
            Instruction::LoadBitwiseAndOfRandomByteAndImmediate {
                destination,
                immediate,
            } => {
                let immediate_value: u8 = immediate.into();
                let random_value: u8 = rand::random();
                let result = immediate_value & random_value;

                self.set_v_register(destination, result.into());
            }
            Instruction::LoadDelayTimerIntoRegister { destination } => {
                let value: EightBitValue = self.delay_timer.into();
                self.set_v_register(destination, value);
            }
            Instruction::AwaitKeyPressAndLoadIntoRegister { destination } => {
                let key_state = self.platform.block_for_any_keypress().await;
                // TODO Timers should still tick, use async or decrement PC and loop again

                match key_state {
                    KeyState::On => self.set_v_register(destination, 1.into()),
                    KeyState::Off => self.set_v_register(destination, 0.into()),
                }
            }
            Instruction::LoadBinaryCodedDecimalValueIntoSequenceStartingAtIRegisterValue {
                source,
            } => {
                // Generate digits. Order will be smallest to highest digit
                let mut digits = vec![];
                let mut bcd_value: u8 = self.read_v_register(source).into();
                while bcd_value != 0 {
                    let digit = bcd_value % 10;
                    digits.push(digit);
                    bcd_value /= 10;
                }

                // Reverse to store in memory
                digits = digits.into_iter().rev().collect();

                let memory_start_address = MemoryAddress(self.i_register.into());
                let bytes_to_read = digits.len() as u8;
                let memory_slice = Self::get_mut_memory_slice(
                    &mut self.memory,
                    memory_start_address,
                    bytes_to_read,
                );

                memory_slice.copy_from_slice(digits.as_slice());
            }
            Instruction::DrawSpritesFromMemory {
                read_x_axis_from,
                read_y_axis_from,
                bytes_to_read_from_i_register,
            } => {
                let starting_x_value: u8 = self.read_v_register(read_x_axis_from).into();
                let starting_y_value: u8 = self.read_v_register(read_y_axis_from).into();

                // Wrap Starting position
                let display_width = self.platform.get_display_width().await;
                let display_height = self.platform.get_display_width().await;
                let starting_x_value = starting_x_value % display_width;
                let starting_y_value = starting_y_value % display_height;

                // Set carry
                self.v_registers[0xf] = 0;

                // Fetch sprites
                let memory_start_address = MemoryAddress(self.i_register.into());
                let bytes_to_read: u8 = bytes_to_read_from_i_register.into();
                let sprites =
                    Self::get_memory_slice(&self.memory, memory_start_address, bytes_to_read);

                use bitvec::prelude::*;

                // Render
                let mut row = starting_y_value;
                for sprite in sprites {
                    let bool_to_pixel_state = |pixel_set| {
                        if pixel_set {
                            PixelState::On
                        } else {
                            PixelState::Off
                        }
                    };
                    let pixel_iter = sprite
                        .view_bits::<Msb0>()
                        .iter()
                        .by_vals()
                        .map(bool_to_pixel_state);

                    let mut column = starting_x_value;
                    for pixel_state in pixel_iter {
                        let pixel = Pixel {
                            column: starting_x_value,
                            row: starting_y_value,
                        };
                        let current_state = self.platform.get_pixel(pixel).await;

                        if pixel_state == current_state {
                            self.platform.set_pixel(pixel, PixelState::Off).await;
                            self.v_registers[0xf] = 1;
                        } else {
                            self.platform.set_pixel(pixel, PixelState::On).await;
                        }
                        // Needed?
                        column += 1;
                        if column >= display_width {
                            break;
                        }
                    }

                    row += 1;

                    // Needed?
                    if row >= display_height {
                        break;
                    }
                }
            }
        };
    }

    fn get_register_slice_up_to(registers: &RegisterBank, end: RegisterNumber) -> &[u8] {
        let register_end_index: usize = end.into();
        &registers[0..register_end_index]
    }
    fn get_mut_register_slice_up_to(
        registers: &mut RegisterBank,
        end: RegisterNumber,
    ) -> &mut [u8] {
        let register_end_index: usize = end.into();
        &mut registers[0..register_end_index]
    }

    fn get_memory_slice(memory: &Memory, start_address: MemoryAddress, bytes: u8) -> &[u8] {
        let memory_start_address: usize = start_address.into();
        let memory_end_address = memory_start_address + bytes as usize;
        &memory[memory_start_address..memory_end_address]
    }
    fn get_mut_memory_slice(
        memory: &mut Memory,
        start_address: MemoryAddress,
        bytes: u8,
    ) -> &mut [u8] {
        let memory_start_address: usize = start_address.into();

        let memory_end_address = memory_start_address + bytes as usize;
        &mut memory[memory_start_address..memory_end_address]
    }

    fn find_address_of_value_in_memory(&self, value: u8) -> Option<MemoryAddress> {
        self.memory
            .iter()
            .position(|&memory_value| memory_value == value)
            .and_then(|opt| Some(MemoryAddress((opt as u16).into())))
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    fn set_carry_in_vf_register(&mut self, carry_set: bool) {
        self.v_registers[0xf] = carry_set.into();
    }

    fn read_v_register(&self, number: RegisterNumber) -> EightBitValue {
        let index: usize = number.into();
        self.v_registers[index].into()
    }

    fn set_v_register(&mut self, number: RegisterNumber, value: EightBitValue) {
        let index: usize = number.into();
        let raw_value: u8 = value.into();
        self.v_registers[index] = raw_value;
    }

    async fn handler_buzzer_state(&mut self) {
        if self.sound_timer > 0 {
            self.platform.set_buzzer(BuzzerState::On).await;
        } else {
            self.platform.set_buzzer(BuzzerState::Off).await;
        }
    }
}
