#[derive(Debug, PartialEq)]
pub enum OpKind {
    Eq,     // =
    Ne,     // <>
    Lt,     // <
    Le,     // <=
    Gt,     // >
    Ge,     // >=
}