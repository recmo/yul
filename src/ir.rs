use crate::evm::Opcode;

// TODO: Compile ast into ir
// * Global value numbering
// *

struct IR {
    blocks:    Vec<Block>,
    functions: Vec<Function>,
}

struct Block {
    instructions: Vec<Instruction>,
    branch:       Branch,
}

struct Instruction {
    operation: Operation,
    inputs:    Vec<usize>,
    outputs:   Vec<usize>,
}

enum Operation {
    Opcode(Opcode),

    // datasize, dataoffset
    UnresolvedImmediate(String),
    Call(usize),
}

struct Function {
    // TODO
}

enum Branch {
    Return,
    Jump(usize),
    Jumpi(usize),
    Final,
}

impl Operation {
    fn num_inputs() -> usize {
        todo!()
    }

    fn num_outputs() -> usize {
        todo!()
    }
}
