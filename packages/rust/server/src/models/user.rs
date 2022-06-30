struct User{
    id: String,
    uuid: String,
    username: String,
    email: String,
    password: String,
    name: String,
    created: String
}

impl User {
    fn create(&self, email: String, password: String, phone: Option<String> ) {
        todo!()
    }
    fn get_by_id(id: String) {
        todo!()
    }
    fn get_by_uuid(uuid: String) {
        todo!()
    }
    fn delete(&self) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
}

