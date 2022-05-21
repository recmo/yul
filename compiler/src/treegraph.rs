//! Convert SSA to Stack machine
//!
//! See <https://elc.yonsei.ac.kr/publications/TreegraphbIS-bytecode11.pdf>
//! See <https://elc.yonsei.ac.kr/publications/treegraphs.pdf>
//! See <http://users.ece.cmu.edu/~koopman/stack_compiler/stack_co.html>

// Less relevant:
// See <http://infolab.stanford.edu/~ullman/dragon/w06/lectures/inst-sched.pdf>

// TODO: Extend treegraph with partial order on operations so we can
// require `mstore`s to be kept in order, while allowing shuffling
// of `mload's between `mstore`s.
