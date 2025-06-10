pub type Id = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub offset: usize,
}

impl Identifier {
    pub fn fresh(name: String, distance: usize) -> Self {
        Identifier {
            name,
            offset: distance,
        }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.name, self.offset);
        Ok(())
    }
}

impl Identifier {
    pub fn name(&self) -> &Id {
        &self.name
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
