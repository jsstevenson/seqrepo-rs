use crate::error::Error;
use crate::{AliasOrSeqId, Interface, SeqRepo};
use chrono::{DateTime, Utc};
use std::collections::HashSet;

pub struct SequenceMetadata {
    pub added: DateTime<Utc>,
    pub aliases: Vec<String>,
    pub alphabet: String,
    pub length: i32,
}

pub trait DataProxy {
    fn get_sequence(
        &self,
        identifier: &AliasOrSeqId,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<String, Error>;

    fn get_metadata(&self, identifier: &AliasOrSeqId) -> Result<SequenceMetadata, Error>;

    // todo cached
    fn translate_sequence_identifier(
        &self,
        identifier: &AliasOrSeqId,
        namespace: Option<&str>,
    ) -> Result<Vec<String>, Error> {
        let metadata = match self.get_metadata(identifier) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Err(err);
            }
        };
        let unique_aliases = metadata.aliases.into_iter().collect::<HashSet<_>>();
        if let Some(namespace_value) = namespace {
            let nsd = format!("{}:", namespace_value);
            unique_aliases
                .into_iter()
                .filter(|alias| alias.starts_with(&nsd))
                .collect()
        } else {
            Ok(unique_aliases.into_iter().collect::<Vec<_>>())
        }
    }
}

pub struct LocalDataProxy {
    pub sr: SeqRepo,
}

impl DataProxy for LocalDataProxy {
    fn get_sequence(
        &self,
        identifier: &AliasOrSeqId,
        start: Option<usize>,
        end: Option<usize>,
    ) -> Result<String, Error> {
        self.sr.fetch_sequence_part(identifier, start, end)
    }

    // fn get_metadata(&self, identifier: &AliasOrSeqId) -> Result<SequenceMetadata, Error> {
    //     let sequence_id = match identifier {
    //         AliasOrSeqId::Alias { value, namespace } => {
    //
    //
    //     }
    /*
     *         ns, a = coerce_namespace(identifier).split(":", 2)
    r = list(self.sr.aliases.find_aliases(namespace=ns, alias=a))
    if len(r) == 0:
        raise KeyError(identifier)
    seq_id = r[0]["seq_id"]
    seqinfo = self.sr.sequences.fetch_seqinfo(seq_id)
    aliases = self.sr.aliases.find_aliases(seq_id=seq_id)
    md = {
        "length": seqinfo["len"],
        "alphabet": seqinfo["alpha"],
        "added": _isoformat(seqinfo["added"]),
        "aliases": [f"{a['namespace']}:{a['alias']}" for a in aliases],
    }
    return md
    */
    // }
}
