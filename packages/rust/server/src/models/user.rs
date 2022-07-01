struct User {
    id: i32,
    uuid: String,
    username: String,
    email: String,
    password: String,
    phone: Option<String>,
    name: String,
    created: String
}

impl User {
    pub fn create(username: String, email: String, password: String, phone: Option<String>, name: String) -> Option<User> {
        let t_id: i32 = 0;
        let t_uuid: String = "0".to_string();
        let t_created: String = "0".to_string();
        let user = User {
            id: t_id,
            uuid: t_uuid,
            username,
            email,
            password,
            phone,
            name,
            created: t_created,
        };
        user
    }

    fn get_by_id(id: String) -> Option<User> {
        todo!();
    }
    fn get_by_uuid(uuid: String) -> Option<User> {
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

