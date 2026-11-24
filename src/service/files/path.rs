#[derive(Clone)]
pub struct ServerPath {
    buff: Vec<String>,
}

impl ServerPath {
    pub fn new(inner: Vec<String>) -> Self {
        Self { buff: inner }
    }

    /// Push an element to end
    pub fn push(&mut self, path: &str) {
        self.buff.push(path.to_owned());
    }

    /// Remove the last element
    pub fn pop(&mut self) -> Option<String> {
        self.buff.pop()
    }

    pub fn with(mut self, element: &str) -> Self {
        self.buff.push(element.to_owned());
        self
    }

    pub fn count(&self) -> usize {
        self.buff.len()
    }
}

impl From<String> for ServerPath {
    fn from(value: String) -> Self {
        Self { buff: value.split(std::path::MAIN_SEPARATOR).map(|s| s.to_owned()).collect() }
    }
}

impl Default for ServerPath {
    fn default() -> Self {
        Self { buff: Vec::new() }
    }
}

impl ToString for ServerPath {
    fn to_string(&self) -> String {
        let cap: usize = self.buff.iter().map(|s| s.len()).sum();

        if cap == 0 {
            return String::from('/')
        }

        let mut s = String::with_capacity(cap + self.count() + 1);
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

impl Into<Vec<String>> for ServerPath {
    fn into(self) -> Vec<String> {
        self.buff
    }
}

// From 3.x requires
pub fn dirpath_is_correct(target: &str) -> bool {
    !target.is_empty() && !target.contains("..")
}

// From 3.x requires
pub fn filepath_is_correct(target: &str) -> bool {
    !target.is_empty() && !target.contains('/') && target.len() < 75
}