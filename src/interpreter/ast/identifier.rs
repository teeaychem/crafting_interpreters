#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub distance: usize,
}

impl Identifier {
    pub fn fresh(name: String, distance: usize) -> Self {
        Identifier { name, distance }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.name, self.distance);
        Ok(())
    }
}
