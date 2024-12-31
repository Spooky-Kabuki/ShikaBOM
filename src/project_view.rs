
pub enum ProjectSubState {
    Main,
    ProjectSubState,
    EditProjectName,
}
impl PartialEq for ProjectSubState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ProjectSubState::Main, ProjectSubState::Main) => true,
            _ => false,
        }
    }
}