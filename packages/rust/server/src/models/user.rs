struct User {
    id: i32,
    uuid: String,
    username: String,
    email: String,
    password: String,
    name: String,
    created: String
}

impl User {
    fn create(email: String, password: String, phone: Option<String> ) -> User {
        todo!();
    }
    fn get_by_id(id: String) -> User {
        todo!();
    }
    fn get_by_uuid(uuid: String) -> User {
        todo!();
    }
    fn update(&self) {
        todo!();
    }
    fn delete(&self) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
}

