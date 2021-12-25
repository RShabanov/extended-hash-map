#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpKind {
    Eq, // =
    Ne, // <>
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=
}
