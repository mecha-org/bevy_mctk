#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Moon = 0xe900,
    Bulb = 0xe901,
}

impl Icon {
    /// Converts the icon to its UTF-8 string representation.
    pub fn to_string(&self) -> String {
        let code_point = *self as u32;

        match std::char::from_u32(code_point) {
            Some(c) => String::from(c),
            None => String::new(),
        }
    }
}

impl Into<String> for Icon {
    fn into(self) -> String {
        self.to_string()
    }
}
