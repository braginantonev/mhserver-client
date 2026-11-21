#[derive(Clone, Copy)]
pub struct Size(usize);

impl Size {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn bytes(&self) -> usize {
        self.0
    }

    /// Return eye candy size. Example: `12 mb`, `1 gb` etc
    pub fn to_string_candy(&self) -> String {
        const EXTENSIONS: [&str; 5] = ["b", "kb", "mb", "gb", "tb"];
        let mut size = self.0 as f32;
        let mut i = 0;
        while size > 1024.0 && i < EXTENSIONS.len() - 1 {
            size = size / 1024.0;
            i += 1;
        }
        format!("{:.2} {}", size as f32, EXTENSIONS[i])
    }
}

impl Into<String> for Size {
    fn into(self) -> String {
        self.to_string_candy()
    }
}

impl Into<usize> for Size {
    fn into(self) -> usize {
        self.0
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::zero()
    }
}
