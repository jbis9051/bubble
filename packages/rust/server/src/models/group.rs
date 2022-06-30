
// Based on up.sql
struct Group{
    id: &str,
    uuid: &str,
    group_name: &str,
    created: &str,
    members: Vec<&str>
}

// CRUD functions
impl Group{
    fn create(&self, name: &str){
        todo!();
    }
    fn read(&self){
        todo!();
    }
    fn read_users(&self){
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