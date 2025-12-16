pub use workspace::{
    DiscoveryOptions, Editability, MemberDiscovery, ProjectDiscovery, ProjectWorkspace,
    RequiresPythonSources, VirtualProject, Workspace, WorkspaceCache, WorkspaceConfig,
    WorkspaceConfigError, WorkspaceError, WorkspaceMember, find_workspace_config,
};

pub mod dependency_groups;
pub mod pyproject;
pub mod pyproject_mut;
mod workspace;
