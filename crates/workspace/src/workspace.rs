use std::collections::{HashMap, HashSet};

use cargo_metadata::{MetadataCommand, Package, PackageId};

use crate::result::Result;

type Packages = HashMap<PackageId, Package>;
type Graph = HashMap<PackageId, HashSet<PackageId>>;

/// Describe a project workspace
pub struct Workspace {
    packages: Packages,
    graph: Graph,
}

impl Workspace {
    pub fn new() -> Result<Self> {
        let metadata = MetadataCommand::new().exec()?;

        // Get workspace pakages ids
        let package_ids: HashSet<PackageId> = metadata
            .workspace_packages()
            .into_iter()
            .map(|p| p.id.clone())
            .collect();

        // Build workspace packages hashmap
        let packages = metadata
            .packages
            .iter()
            .filter(|p| package_ids.contains(&p.id))
            .fold(HashMap::new(), |mut acc, p| {
                acc.insert(p.id.clone(), p.clone());
                acc
            });

        // Build the workspace package dependency graph
        let graph: HashMap<PackageId, HashSet<PackageId>> = metadata
            .resolve
            .as_ref()
            // Todo: handle None properly
            .unwrap()
            .nodes
            .iter()
            .fold(HashMap::new(), |mut acc, p| {
                if package_ids.contains(&p.id) {
                    acc.insert(
                        p.id.clone(),
                        p.deps
                            .iter()
                            .filter_map(|p| package_ids.get(&p.pkg).cloned())
                            .collect(),
                    );
                }
                acc
            });

        Ok(Self { packages, graph })
    }

    pub fn list_packages(&self) -> Vec<String> {
        self.packages.values().map(|p| p.name.clone()).collect()
    }
}
