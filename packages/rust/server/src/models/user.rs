struct User{
    id: &str,
    uuid: &str,
    name: &str,
    created: &str
}

pub fn signup(email: &str, password: &str, phone: Option<&str>) -> Option<User> {
    todo!();
    // generate id, uuid, created
    // do sql things - create routes row, confirmation row
    // send email with verification link_id

    // somehow implement a timeout for signup
}

pub fn signin(email: &str, password: &str) -> Option<(&str, &str)> {
    todo!();
    // do sql things with email and password
}

pub fn confirm(link_id: &str) {
    todo!();
    // confirm routes
    // update routes email
    // delete confirmation row
}

pub fn forgot(email: &str) {
    todo!();
    // if email exists in db
    // create forgot_password row using new email
    // send email with forgot_id to new email
}

pub fn forgot_confirm(forgot_id: &str) {
    todo!();
    // check if forgot_id exists
    // update password in routes table
    // delete forgot_password row
}

impl User {
    fn get(&self) {
        self.id
    }
    fn signout(&self, token: &str) {
        todo!();
        // delete session_token row
    }
    fn change_email(&self, new_email: &str) {
        todo!();
        // create confirmation row
        // send email with link_id to new email
    }
    fn delete(&self) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
    fn uppity() {

    }
}

