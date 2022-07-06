use std::time::{Duration, SystemTime};
// Based on up.sql
pub struct Group{
    //ids and uuids are mutually unique
    id: i32,
    uuid: String,
    pub group_name: String,
    pub created: String,
    pub members: Vec<i32>
}

// CRUD functions
impl Group{
    pub fn create(name: String) -> Group {
       let group = Group {
           id: 2,
           uuid: "".to_string(),
           group_name: String::from(name),
           created: SystemTime::now().to_string(),
           members: vec![]
       };
        group
    }
    pub fn read(&mut self){
        todo!();
    }
    pub fn add_users(&mut self, mut new_users: &[i32]){
        *self.members.append(&mut new_users);
    }
    pub fn delete_users(&mut self, users_to_delete: Vec<i32>){
        for i in users_to_delete{
            if &self.members.contains(&i){
                &self.members.retain(|x| *x != &i);
            }
        }
    }
    pub fn change_name(&self, name: String){
        &self.group_name = &name;
    }
    pub fn delete_group(&self){
        todo!();
    }
}