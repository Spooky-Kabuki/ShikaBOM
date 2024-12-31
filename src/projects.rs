use crate::db;
pub struct Project {
    //Nothing in this struct can be null, so no optional types needed.
    pub name: String,
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: "".to_string(),
        }
    }
}

pub fn fetch_all_projects() -> Vec<Project> {
    let mut project_list = Vec::new();

    let query = "SELECT * FROM projects";
    let mut client = db::postgres_init();
    let rows = client.query(query, &[]).unwrap();
    for row in rows {
        let mut project = Project::new();
        project.name = row.try_get("project_name").unwrap_or("".to_string());
        project_list.push(project);
    }
    return project_list;
}

pub fn create_new_project(project: Project) {
    let query = "INSERT INTO projects (project_name) VALUES ($1)";
    let mut client = db::postgres_init();
    client.execute(query, &[&project.name]).unwrap();
}