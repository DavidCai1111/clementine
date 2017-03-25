#[derive(Debug)]
pub enum PersistType {
    Memory,
    File(String),
}
