pub mod ast;
pub mod lexer;
pub mod vm;

pub mod prelude {
    pub use crate::ast::*;
    pub use crate::lexer::*;
    pub use crate::vm::*;
}
