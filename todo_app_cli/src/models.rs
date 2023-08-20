pub use crate::schema::*;

use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaskDB {
    pub id: i32,
    pub name: String,
    pub state: String,
    pub importance: String,
    pub lifesphere: String,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub name: &'a str,
    pub state: &'a str,
    pub importance: &'a str,
    pub lifesphere: &'a str,
}
