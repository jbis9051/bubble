use std::time::{Duration, SystemTime};
// Based on up.sql
struct Group{
    id: String,
    uuid: String,
    group_name: String,
    created: String,
    members: Vec<String>
}

// CRUD functions
impl Group{
    fn create(&self, name: String){
       let group = Group {
           id: "".to_string(),
           uuid: "".to_string(),
           group_name: String::from(name),
           created: SystemTime::now().to_string(),
           members: vec![]
       };
    }
    fn read(&self){
        todo!();
    }
    fn add_users(&mut self, mut new_users: Vec<String>){
        self.members.append(&mut new_users);
    }
    fn delete_users(&self, users_to_delete: Vec<String>){
        todo!();
    }
    fn change_name(&self, name: String){
        todo!();
    }
    fn delete_group(&self){
        todo!();
    }
}