#[derive(Debug)]
pub enum Object {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
