use crate::js_interface::FrontendInstance;
use crate::Error;

// create a database with sqlite
// set database to update global db with user entry
// identity = key
// make a key and store in db
// send an api route to create a user from uuid create account db
// not updating global var

impl FrontendInstance {
    pub async fn register(
        _username: String,
        _password: String,
        _name: String,
        _email: String,
    ) -> Result<(), Error> {
        todo!()
    }

    pub async fn login(_username: String, _password: String) -> Result<(), Error> {
        todo!()
    }

    pub async fn logout() -> Result<(), Error> {
        todo!()
    }

    pub async fn forgot(_email: String) -> Result<(), Error> {
        todo!()
    }
}
/*
pub async fn register(
    username: String,
    password: String,
    name: String,
    email: String,
) -> Result<(), Error> {
    //create key pair and store as signing key for client
    let mut csprng = OsRng {};
    let account_key = Keypair::generate(&mut csprng);

    // api request
    let _path = "/v1/user/register";
    // let url = account_url(path).await;
    let url = "".to_string();
    let created_user = CreateUser {
        email: email.clone(),
        username: username.clone(),
        password: password.clone(),
        name: name.clone(),
        identity: Base64(account_key.secret.to_bytes().to_vec()),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&created_user).unwrap())
        .send()
        .await?
        .json::<RegisteredClientsResponse>()
        .await?;

    let user_uuid = response.uuid;
    let client_uuid = Uuid::new_v4().to_string();


    // create an account db based on up in migrations
    let account_db = SqlitePoolOptions::new()
        .connect("sqlite:accounts.db")
        .await?;
    sqlx::migrate!("./migrations/account")
        .run(&account_db)
        .await?;

    let mut temp = account_db.begin().await?;

    //assign new account_db to global account data var, clone when writing to it? or use a mutex
    let bearer = AccountKv::get(&account_db, "bearer").await?;
    let _domain = AccountKv::get(&account_db, "domain").await?;

    if let Some(_bearer) = bearer {
        let _write = crate::GLOBAL_ACCOUNT_DATA.write().await;
        /*   *write = Some(GlobalAccountData {
                    bearer: RwLock::new(bearer),
                    domain: domain.unwrap_or_default(),
                    database: account_db.clone(),
                });
                drop(write);
        */

        // I'm lost here fuck
    }

    // update account db
    sqlx::query("INSERT INTO user (uuid, name, identity, updated_date) VALUES ($1, $2, $3, $4);")
        .bind(user_uuid)
        .bind(&name)
        .bind(&username)
        .bind(chrono::Utc::now().timestamp())
        .execute(&account_db)
        .await?;

    let user_id = sqlx::query("SELECT id FROM user WHERE uuid = $1")
        .bind(user_uuid)
        .fetch_one(&account_db)
        .await
        .unwrap()
        .get::<i32, _>("id");

    sqlx::query(
        "INSERT INTO client (uuid, user_id, signing_key, validated_date) VALUES ($1, $2, $3, $4)",
    )
        .bind(&client_uuid)
        .bind(user_id)
        .bind(&account_key.public.to_bytes().to_vec())
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut temp)
        .await?;

    temp.commit().await?;

    Ok(())
}

pub async fn login(username: String, _password: String) -> Result<(), Error> {
    // let db = crate::GLOBAL_DATABASE.get().unwrap();
    let account_db = SqlitePoolOptions::new()
        .connect("sqlite:accounts.db")
        .await?;
    let _user_id = sqlx::query("SELECT FROM user WHERE identity = $1")
        .bind(&username)
        .fetch_one(db)
        .await?;
    let _real_password = sqlx::query("SELECT password FROM user WHERE identity = $1")
        .bind(&username)
        .fetch_one(db)
        .await?;
    /*
    ???????????? Im sure theres mroe to do but idk
     */
    Ok(())
}

pub async fn logout() -> Result<(), Error> {
    let account_db = SqlitePoolOptions::new()
        .connect("sqlite:accounts.db")
        .await?;

    AccountKv::delete(&account_db, "current_account").await?;
    Ok(())
}

pub async fn forgot(_email: String) -> Result<(), Error> {
    let account_db = SqlitePoolOptions::new()
        .connect("sqlite:accounts.db")
        .await?;

    AccountKv::set(&account_db, "forgot_email", &_email).await?;

    let _response = reqwest::get("accounts/user/forgot")
        .await?
        .text()
        .await?;

    Ok(())
}
*/
