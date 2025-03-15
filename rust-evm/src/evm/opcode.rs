use primitive_types::U256;

#[derive(Debug)]
pub enum Opcode {
    // STACK RELATED OPCODES
    PUSH1(U256),     // PUSH1 to push32
    ADD,
    STOP,                           
    POP,               
    DUP(usize),    // DUP1 to DUP16
    SWAP(usize),    // SWAP1 to SWAP16
    SUB, 
    MUL,
    DIV,
    MOD,
    LT,
    GT,
    EQ
    // MEMORY RELATED OPCODES

}

impl Opcode {
    pub fn gas_cost(&self) -> U256 {
        match self {
            //STACK RELATED
            Opcode::PUSH1(_) => U256::from(3),  // PUSH1 costs 3 gas
            Opcode::ADD => U256::from(3),       // ADD costs 3 gas
            Opcode::POP => U256::from(2),       // POP costs 2 gas
            Opcode::DUP(_) => U256::from(3),    // DUP1-16 cost 3 gas each
            Opcode::SWAP(_) => U256::from(3),   // SWAP1-16 cost 3 gas each
            Opcode::STOP => U256::from(0),      // STOP has no gas cost
            Opcode::SUB => U256::from(3),
            Opcode::MUL => U256::from(5),
            Opcode::DIV => U256::from(5),
            Opcode::MOD => U256::from(5),
            Opcode::LT => U256::from(3),
            Opcode::GT => U256::from(3),
            Opcode::EQ => U256::from(3),

        }
    }
}

//disassemble: It’s converting raw EVM bytecode into a vector of Opcode enums that you can later execute.
pub fn decode_bytecode(bytecode: &[u8]) -> Vec<Opcode> {
    let mut ops = Vec::new();
    let mut i = 0;

    while i < bytecode.len() {
        match bytecode[i] {
            0x60 => {
                i += 1;
                let value = U256::from(bytecode[i]);
                ops.push(Opcode::PUSH1(value));
            }
            0x50 => ops.push(Opcode::POP), // POP
            0x80..=0x8F => ops.push(Opcode::DUP((bytecode[i] - 0x80 + 1) as usize)), // DUP1 to DUP16
            0x90..=0x9F => ops.push(Opcode::SWAP((bytecode[i] - 0x90 + 1) as usize)), // SWAP1 to SWAP16
            0x01 => ops.push(Opcode::ADD),
            0x00 => ops.push(Opcode::STOP),

            0x03 => ops.push(Opcode::SUB),
            0x02 => ops.push(Opcode::MUL),
            0x04 => ops.push(Opcode::DIV),
            0x06 => ops.push(Opcode::MOD),
            0x10 => ops.push(Opcode::LT),
            0x11 => ops.push(Opcode::GT),
            0x14 => ops.push(Opcode::EQ),
            _ => panic!("Unknown opcode"),
        }
        i += 1;
    }

    ops
}
