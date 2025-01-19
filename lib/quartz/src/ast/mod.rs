#[derive(Debug)]
pub struct QuartzModule {
    pub global_imports: Vec<Import>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Import {
    pub path: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub is_global: bool,
    pub imports: Vec<Import>,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Create {
        name: String,
        type_: Type,
        initializer: Initializer,
    },
    Pipeline {
        value: Expression,
        operations: Vec<Operation>,
    },
    When {
        condition: Expression,
        cases: Vec<(Pattern, Block)>,
    },
    ForEach {
        variable: String,
        collection: Expression,
        body: Block,
    },
}
