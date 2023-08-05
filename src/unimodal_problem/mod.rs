// The mod.rs file in a folder makes the folder behave as a library. This file
// defines what is publicly accessible to things that import this library, as
// well as what is privately available for files within this library to share.

mod problem;
pub use problem::{UnimodalProblem, UnimodalProblemBuilder};
