#[derive(Clone)]
pub struct ServerPath {
    buff: Vec<String>,
}

impl ServerPath {
    pub fn new() -> Self {
        Self { buff: Vec::new() }
    }

    /// Push a single directory.
    pub fn push(&mut self, path: &str) {
        self.buff.push(path.to_owned());
    }

    /// Pop a singe directory.
    pub fn pop(&mut self) -> bool {
        self.buff.pop().is_some()
    }

    pub fn with(mut self, element: &str) -> Self {
        self.buff.push(element.to_owned());
        self
    }
}

impl ToString for ServerPath {
    fn to_string(&self) -> String {
        let mut s = String::from("/");
        for element in &self.buff {
            s.push_str(element);
            s.push('/');
        }
        s
    }
}

impl Into<String> for ServerPath {
    fn into(self) -> String {
        self.to_string()
    }
}