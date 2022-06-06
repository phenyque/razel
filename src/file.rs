use crate::cache::BlobDigest;
use crate::{ArenaId, CommandId};
use std::path::PathBuf;

pub struct File {
    pub id: FileId,
    /// argument from original command line
    pub arg: String,
    /// path to be used for exec
    pub path: PathBuf,
    /// files without creating_command are input files (data or executable) which must exist before running any commands
    pub creating_command: Option<CommandId>,
    pub digest: Option<BlobDigest>,
}

pub type FileId = ArenaId<File>;
