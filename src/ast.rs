#[derive(Debug)]
pub enum AST {
    Function { name: String, body: Box<AST> },

    Return { value: Box<AST> },
    If { cond: Box<AST>, then: Box<AST> },

    Integer { value: u32 },

    Add { lhs: Box<AST>, rhs: Box<AST> },
    Sub { lhs: Box<AST>, rhs: Box<AST> },
    Mul { lhs: Box<AST>, rhs: Box<AST> },
    Div { lhs: Box<AST>, rhs: Box<AST> },

    Equal { lhs: Box<AST>, rhs: Box<AST> },
    NotEqual { lhs: Box<AST>, rhs: Box<AST> },

    Lt { lhs: Box<AST>, rhs: Box<AST> },
    Lte { lhs: Box<AST>, rhs: Box<AST> },
    Gt { lhs: Box<AST>, rhs: Box<AST> },
    Gte { lhs: Box<AST>, rhs: Box<AST> },
}
