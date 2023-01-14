/// Genre
#[derive(Debug)]
pub struct Genre {
    // primo genere
    first: String,
    // secondo genere
    second: Option<String>,
}

impl Genre {
    pub fn new(first: String, second: Option<String>) -> Self {
        Self { first, second }
    }

    pub fn first(&self) -> &String {
        &self.first
    }

    pub fn second(&self) -> &Option<String> {
        &self.second
    }
}
