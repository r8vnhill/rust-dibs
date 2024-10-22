pub struct EvenNumbers {
    current: u32,
}

impl EvenNumbers {
    pub fn new() -> Self {
        EvenNumbers { current: 0 }
    }
}

impl Iterator for EvenNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current;
        self.current = self.current + 2;
        Some(next)
    }
}
