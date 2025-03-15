mod evm;
use evm::EVM;

fn main() {
//    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00, 0x50]; // PUSH1 1, PUSH1 2, ADD, STOP

// Testing POP (Removing Elements)
//    let bytecode = vec![0x60, 0x05, 0x60, 0x0A, 0x50, 0x00]; 
// // PUSH1 5, PUSH1 10, POP, STOP

//Testing DUP (Duplicating Values)
//let bytecode = vec![0x60, 0x03, 0x60, 0x07, 0x81, 0x00]; 
// // PUSH1 3, PUSH1 7, DUP2, STOP

//Testing SWAP (Swapping Values)
// let bytecode = vec![0x60, 0x04, 0x60, 0x08, 0x90, 0x00]; 
// // PUSH1 4, PUSH1 8, SWAP1, STOP

//Combining Multiple Stack Operations
 let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03, 0x81, 0x01, 0x50, 0x00]; 
// // PUSH1 1, PUSH1 2, PUSH1 3, DUP2, ADD, POP, STOP


    let mut evm = EVM::new(bytecode);
    evm.run();
}

