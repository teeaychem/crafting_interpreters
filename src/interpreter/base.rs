pub struct Base<'s> {
    pub stdio: Box<dyn std::io::Write + 's>,
}

impl<'s> Base<'s> {
    pub fn update_stdio<W: std::io::Write + 's>(&mut self, destination: &'s mut W) {
        self.stdio = Box::new(destination)
    }
}

impl Default for Base<'_> {
    fn default() -> Self {
        Base {
            stdio: Box::new(std::io::stdout()),
        }
    }
}
