use std::sync::Arc;

use meilidb_core::DocumentId;

use crate::database::raw_index::InnerRawIndex;
use super::Error;

#[derive(Clone)]
pub struct DocsWordsIndex(pub(crate) InnerRawIndex);

impl DocsWordsIndex {
    pub fn doc_words(&self, id: DocumentId) -> Result<Option<fst::Set>, Error> {
        let key = id.0.to_be_bytes();
        match self.0.get_pinned(key)? {
            Some(bytes) => {
                let len = bytes.len();
                let value = Arc::from(bytes.as_ref());
                let fst = fst::raw::Fst::from_shared_bytes(value, 0, len)?;
                Ok(Some(fst::Set::from(fst)))
            },
            None => Ok(None)
        }
    }

    pub fn set_doc_words(&self, id: DocumentId, words: &fst::Set) -> Result<(), Error> {
        let key = id.0.to_be_bytes();
        self.0.set(key, words.as_fst().as_bytes())?;
        Ok(())
    }

    pub fn del_doc_words(&self, id: DocumentId) -> Result<(), Error> {
        let key = id.0.to_be_bytes();
        self.0.delete(key)?;
        Ok(())
    }
}
