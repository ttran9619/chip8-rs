mod instruction;
mod types;

struct Emulator {
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
}

type Memory = [u8; 4096];
type RegisterBank = [u8; 16];
type Stack = [u16; 16];