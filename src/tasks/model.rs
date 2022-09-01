use crate::db;
use crate::error_handler::CustomError;
use crate::schema::tasks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "tasks"]
pub struct Task {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tasks"]
pub struct Tasks {
    pub id: i32,
    pub subject : Option<String>,
    pub description : Option<String> ,
    pub status: Option<String>
}

impl Tasks {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let tasks = tasks::table.load::<Tasks>(&conn)?;
        Ok(tasks)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = tasks::table.filter(tasks::id.eq(id)).first(&conn)?;
        Ok(task)
    }

    pub fn create(task: Task) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = Task::from(task);
        let task = diesel::insert_into(tasks::table)
            .values(task)
            .get_result(&conn)?;
        Ok(task)
    }

    pub fn update(id: i32, task: Task) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let task = diesel::update(tasks::table)
            .filter(tasks::id.eq(id))
            .set(task)
            .get_result(&conn)?;
        Ok(task)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(tasks::table.filter(tasks::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Task {
    fn from(task: Task) -> Task {
        Task {
            subject: task.subject,
            description: task.description,
            status: task.status
        }
    }
}