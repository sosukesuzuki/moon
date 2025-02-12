use moon_lang::SupportedLanguage;
use moon_project::ProjectID;
use moon_task::TargetID;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub enum ActionNode {
    InstallDeps(SupportedLanguage),
    RunTarget(TargetID),
    SetupToolchain,
    SyncProject(SupportedLanguage, ProjectID),
}

impl ActionNode {
    pub fn label(&self) -> String {
        match self {
            ActionNode::InstallDeps(lang) => format!("Install{}Deps", lang),
            ActionNode::RunTarget(id) => format!("RunTarget({})", id),
            ActionNode::SetupToolchain => "SetupToolchain".into(),
            ActionNode::SyncProject(lang, id) => format!("Sync{}Project({})", lang, id),
        }
    }
}

impl PartialEq for ActionNode {
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
    }
}

impl Hash for ActionNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state);
    }
}
