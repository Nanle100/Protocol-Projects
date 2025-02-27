mod evm;
use evm::EVM;

fn main() {
    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00]; // PUSH1 1, PUSH1 2, ADD, STOP
    let mut evm = EVM::new(bytecode);
    evm.run();
}

