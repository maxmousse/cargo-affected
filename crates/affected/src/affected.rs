use workspace::Workspace;

use crate::result::Result;

pub struct Affected {
    workspace: Workspace,
}

impl Affected {
    pub fn new() -> Result<Self> {
        let workspace = Workspace::new()?;

        Ok(Affected { workspace })
    }
}
