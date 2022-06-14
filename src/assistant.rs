use crate::error::TranslationError;

/// Fast Omniscient Robotic guiDe is a personal assistant on your hitchhike through the galaxy.
#[derive(Default, Debug)]
pub struct Ford {}

impl Ford {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&mut self, query: &str) -> Result<Option<String>, TranslationError> {
        todo!()
    }
}
