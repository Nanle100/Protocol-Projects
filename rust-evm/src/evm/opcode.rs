use primitive_types::U256;

#[derive(Debug)]
pub enum Opcode {
    // STACK RELATED OPCODES
    PUSH1(U256),     //place items on stack starting from byte 1 to byte 32
    ADD,
    STOP,                           
    //adding more opcodes
    POP,               // remove items from stack
    DUP(usize),    // DUP1 to DUP16
    SWAP(usize),    // SWAP1 to SWAP16
}

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
            _ => panic!("Unknown opcode"),
        }
        i += 1;
    }

    ops
}
