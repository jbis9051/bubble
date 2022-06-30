
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
    fn create(&self, name: &str){
        todo!();
    }
    fn read(&self){
        todo!();
    }
    fn add_users(&self, new_users: Vec<&str>){
        todo!();
    }
    fn delete_users(&self, users_to_delete: Vec<&str>){
        todo!();
    }
    fn change_name(&self, name: &str){
        todo!();
    }
    fn delete(&self){
        todo!();
    }
}