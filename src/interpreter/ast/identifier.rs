pub type Id = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub offset: Option<usize>,
}

impl Identifier {
    pub fn fresh(name: String, offset: Option<usize>) -> Self {
        Identifier { name, offset }
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.offset {
            Some(x) => write!(f, "{}[{x}]", self.name),
            None => write!(f, "{}[-]", self.name),
        };
        Ok(())
    }
}

impl Identifier {
    pub fn name(&self) -> &Id {
        &self.name
    }

    pub fn offset(&self) -> Option<usize> {
        self.offset
    }
}
