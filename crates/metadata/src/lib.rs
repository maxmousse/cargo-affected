use cargo_metadata::{Metadata, MetadataCommand};

pub mod result;

use result::Result;

pub fn get_metadata() -> Result<Metadata> {
    let cmd = MetadataCommand::new();

    let metadata = cmd.exec()?;

    Ok(metadata)
}
