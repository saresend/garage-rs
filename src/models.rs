

use schema::KeyVal;

#[derive(Queryable)]
pub struct DataEntry {
    pub id: i32,
    pub key: String,
    pub value: String,
}


#[derive(Insertable)]
#[table_name = "KeyVal"]
pub struct NewEntry {
    pub key: String,
    pub value: String,
}
