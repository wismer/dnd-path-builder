use diesel::Queryable;

#[derive(Queryable)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct Path {
    pub id: i32,
    pub name: String,
    pub description: String,
}
