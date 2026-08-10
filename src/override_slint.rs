use {
    super::File,
    std::cmp::Ordering
};

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.is_dir
            .cmp(&other.is_dir)
            .then(self.name.cmp(&other.name))
            .then(self.size.cmp(&other.size))
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for File {}