use postgres::Row;
use tracing::{error, info};
use crate::db;
use crate::parts::Part;

pub struct Project {
    //Nothing in this struct can be null, so no optional types needed.
    pub name: String,
    pub parts: Vec<ProjectPart>
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: "".to_string(),
            parts: vec![]
        }
    }
}

pub struct ProjectPart {
    //Values specific to project part info
    pub partnumber: String,
    pub designators: String,
    pub qty: i32,
    //Values from the part info, not specific to the project part
    pub part_info: Part
}

pub fn fetch_project_list() -> Vec<Project> {
    let mut project_list = Vec::new();

    let query = "SELECT * FROM projects";
    let mut client = db::postgres_init();
    let rows = client.query(query, &[]).unwrap();
    for row in rows {
        let mut project = Project::new();
        project.name = row.try_get("project_name").unwrap_or("".to_string());
        project_list.push(project);
    }
    project_list
}

pub fn fetch_project_details(project: &mut Project) {
    let query = "SELECT * FROM project_components WHERE project_name = $1;";
    let mut client = db::postgres_init();
    let rows = client.query(query, &[&project.name]).unwrap();
    project.parts.clear();
    for row in rows {
        project.parts.push(project_part_from_row(row));
    }
}

pub fn create_new_project_name(name: String) {
    let query = "INSERT INTO projects (project_name) VALUES ($1)";
    let mut client = db::postgres_init();
    client.execute(query, &[&name]).unwrap();
}

fn project_part_from_row(row: Row) -> ProjectPart {
    let pn = row.try_get("partnumber").unwrap_or("".to_string());
    let new_part = ProjectPart {
        partnumber: pn.clone(),
        qty: row.try_get("total_qty").unwrap_or(0),
        designators: row.try_get("designators").unwrap_or("".to_string()),
        part_info: Part::new_from_pn(&*pn)
    };
    new_part
}

pub fn fetch_pns_not_in_project(project: &Project) -> Vec<String> {
    let query = "SELECT p.partnumber
                        FROM parts p
                        WHERE p.partnumber NOT IN (
                            SELECT pc.partnumber
                            FROM project_components pc
                            WHERE pc.project_name = $1
                        );";
    let mut client = db::postgres_init();
    let row_result = client.query(query, &[&project.name]);
    let rows = row_result.unwrap_or_else(|e| {
        error!("Error fetching data: {:?}", e);
        Vec::new()
    });
    let mut ret_vec = vec![];
    for row in rows {
        match row.try_get("partnumber") {
            Ok(val) => ret_vec.push(val),
            Err(_) => ()
        }
    }
    ret_vec
}

pub fn add_pn_to_project(project: &Project, ppart: &ProjectPart) {
    let query = "insert into project_components (project_name, partnumber, designators, qty)
        values ($1, $2, $3, $4)";
    let mut client = db::postgres_init();
    client.execute(query, &[&project.name, &ppart.partnumber, &ppart.designators, &ppart.qty]).unwrap();
}