pub mod naming {
    pub enum Naming {
        Uuid,
        Simple,
        Original(String),
        Custom,
    }
}