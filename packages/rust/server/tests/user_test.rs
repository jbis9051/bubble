use crate::helper::start_server;
use axum::http::StatusCode;
use bubble::models::confirmation::Confirmation;
use bubble::models::user::User;
use bubble::routes::user::{Confirm, CreateUser};

use bubble::models::session::Session;
use sqlx::{Executor, Row};

mod helper;

#[tokio::test]
async fn create_user() {
    let (db, client) = start_server().await;

    let created_user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "test".to_string(),
        password: "password".to_string(),
        phone: Some("18001239876".to_string()),
        name: "John Doe".to_string(),
    };

    let mut cleanup = cleanup!({
        pub confirmation_id: Option<i32>,
        pub session_id: Option<i32>,
        pub user_username: Option<String>,
    }, |db, resources| {
        if let Some(confirmation_id) = resources.confirmation_id {
            sqlx::query("DELETE FROM confirmation WHERE id = $1").bind(&confirmation_id).execute(&db).await.unwrap();
        }
        if let Some(session_id) = resources.session_id {
            sqlx::query("DELETE FROM session_token WHERE id = $1").bind(&session_id).execute(&db).await.unwrap();
        }
        if let Some(username) = resources.user_username {
            sqlx::query("DELETE FROM \"user\" WHERE username = $1").bind(&username).execute(&db).await.unwrap();
        }

    });
    let res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&created_user).unwrap())
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    cleanup.resources.user_username = Some(created_user.username.clone());

    //TODO implement from user
    let user = User::from_username(&db, &created_user.username)
        .await
        .unwrap();

    assert_eq!(user.username, created_user.username);
    assert_eq!(user.password, created_user.password);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);

    let confirmations = Confirmation::filter_user_id(&db, user.id).await.unwrap();
    assert_eq!(confirmations.len(), 1);

    let confirmation = &confirmations[0];
    cleanup.resources.confirmation_id = Some(confirmation.id);

    assert_eq!(confirmation.user_id, user.id);
    assert_eq!(confirmation.email, created_user.email);

    let confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: confirmation.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;
    cleanup.resources.confirmation_id = None;
    let session = &Session::filter_user_id(&db, user.id).await.unwrap()[0];
    cleanup.resources.session_id = Some(session.id);

    assert_eq!(confirm_res.status(), StatusCode::CREATED);

    let user = User::from_username(&db, &created_user.username)
        .await
        .unwrap();

    assert_eq!(user.username, created_user.username);
    assert_eq!(user.password, created_user.password);
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some(created_user.email));
    assert_eq!(user.phone, created_user.phone);
    assert_eq!(user.name, created_user.name);
}

#[tokio::test]
async fn create_multiple_user() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        pub confirmation_id: Vec<i32>,
        pub session_id: Vec<i32>,
        pub user_id: Vec<i32>,
    }, |db, resources| {
        for id in resources.confirmation_id {
            sqlx::query("DELETE FROM confirmation WHERE id = $1").bind(id).execute(&db).await.unwrap();
        }
        for id in resources.session_id {
            sqlx::query("DELETE FROM session_token WHERE id = $1").bind(id).execute(&db).await.unwrap();
        }
        for id in resources.user_id {
            sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(id).execute(&db).await.unwrap();
        }
    });

    let brian_in = CreateUser {
        email: "python@gmail.com".to_string(),
        username: "machine_learning_man".to_string(),
        password: "lots_of_abstraction".to_string(),
        phone: None,
        name: "Brian".to_string(),
    };
    let (brian, _brian_link) = helper::signup_user(&db, &client, &brian_in).await.unwrap();
    let brian_confirmation = &Confirmation::filter_user_id(&db, brian.id).await.unwrap()[0];
    cleanup.resources.user_id.push(brian.id);
    cleanup
        .resources
        .confirmation_id
        .push(brian_confirmation.id);

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.password, "lots_of_abstraction");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, None);
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");
    assert_eq!(brian_confirmation.user_id, brian.id);
    assert_eq!(brian_confirmation.email, "python@gmail.com");

    let timmy_in = CreateUser {
        email: "javascript@gmail.com".to_string(),
        username: "web_development_dude".to_string(),
        password: "html_rocks".to_string(),
        phone: Some("66260701534".to_string()),
        name: "Little Timmy III".to_string(),
    };
    let (timmy, _timmy_link) = helper::signup_user(&db, &client, &timmy_in).await.unwrap();
    let timmy_confirmation = &Confirmation::filter_user_id(&db, timmy.id).await.unwrap()[0];
    cleanup.resources.user_id.push(timmy.id);
    cleanup
        .resources
        .confirmation_id
        .push(timmy_confirmation.id);
    println!("Something is about to go wrong");

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.password, "html_rocks");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, None);
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
    assert_eq!(timmy_confirmation.user_id, timmy.id);
    assert_eq!(timmy_confirmation.email, "javascript@gmail.com");

    let brian_confirm_in = Confirm {
        link_id: brian_confirmation.link_id.to_string(),
    };
    let (brian, brian_token) = helper::signup_confirm_user(&db, &client, &brian_confirm_in, &brian)
        .await
        .unwrap();
    let brian_session = &Session::filter_user_id(&db, brian.id).await.unwrap()[0];
    cleanup.resources.confirmation_id.remove(0);
    cleanup.resources.session_id.push(brian_session.id);

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.password, "lots_of_abstraction");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, Some("python@gmail.com".to_string()));
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");
    assert_eq!(brian_session.user_id, brian.id);
    assert_eq!(brian_session.token, brian_token);

    let bill_in = CreateUser {
        email: "rust@gmail.com".to_string(),
        username: "big_programmer_pro".to_string(),
        password: "cool_crustacean".to_string(),
        phone: Some("18004321234".to_string()),
        name: "bill".to_string(),
    };
    let (bill, _bill_link) = helper::signup_user(&db, &client, &bill_in).await.unwrap();
    let bill_confirmation = &Confirmation::filter_user_id(&db, bill.id).await.unwrap()[0];
    cleanup.resources.user_id.push(bill.id);
    cleanup.resources.confirmation_id.push(bill_confirmation.id);

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.password, "cool_crustacean");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, None);
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "bill");
    assert_eq!(bill_confirmation.user_id, bill.id);
    assert_eq!(bill_confirmation.email, "rust@gmail.com");

    let bill_confirm_in = Confirm {
        link_id: bill_confirmation.link_id.to_string(),
    };
    let (bill, bill_token) = helper::signup_confirm_user(&db, &client, &bill_confirm_in, &bill)
        .await
        .unwrap();
    let bill_session = &Session::filter_user_id(&db, bill.id).await.unwrap()[0];
    cleanup.resources.confirmation_id.remove(1);
    cleanup.resources.session_id.push(bill_session.id);

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.password, "cool_crustacean");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, Some("rust@gmail.com".to_string()));
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "bill");
    assert_eq!(bill_session.user_id, bill.id);
    assert_eq!(bill_session.token, bill_token);

    let timmy_confirm_in = Confirm {
        link_id: timmy_confirmation.link_id.to_string(),
    };
    let (timmy, timmy_token) = helper::signup_confirm_user(&db, &client, &timmy_confirm_in, &timmy)
        .await
        .unwrap();
    let timmy_session = &Session::filter_user_id(&db, timmy.id).await.unwrap()[0];
    cleanup.resources.confirmation_id.remove(0);
    cleanup.resources.session_id.push(timmy_session.id);

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.password, "html_rocks");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, Some("javascript@gmail.com".to_string()));
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
    assert_eq!(timmy_session.user_id, timmy.id);
    assert_eq!(timmy_session.token, timmy_token);
}

#[tokio::test]
async fn test_signin_signout() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        pub session_id: Option<i32>,
        pub user_id: Option<i32>,
    }, |db, resources| {
        if let Some(id) = resources.session_id {
            sqlx::query("DELETE FROM session_token WHERE id = $1").bind(id).execute(&db).await.unwrap();
        }
        if let Some(id) = resources.user_id {
            sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(id).execute(&db).await.unwrap();
        }
    });
    let user = CreateUser {
        email: "test@gmail.com".to_string(),
        username: "testusername".to_string(),
        password: "testpassword".to_string(),
        phone: None,
        name: "testname".to_string(),
    };

    let (token, user) = helper::initialize_user(&db, &client, &user).await.unwrap();
    cleanup.resources.user_id = Some(user.id);
    let session = Session::from_token(&db, token).await.unwrap();
    cleanup.resources.session_id = Some(session.id);

    assert_eq!(user.username, "testusername");
    assert_eq!(user.password, "testpassword");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, None);
    assert_eq!(user.name, "testname");
    assert_eq!(session.user_id, user.id);
    assert_eq!(session.token, token);

    helper::signout_user(&db, &client, &session).await.unwrap();
    cleanup.resources.session_id = None;

    let sessions = Session::filter_user_id(&db, user.id).await.unwrap();
    assert_eq!(sessions.len(), 0);

    let token = helper::signin_user(&db, &client, &user).await.unwrap();
    let session = Session::from_token(&db, token).await.unwrap();
    cleanup.resources.session_id = Some(session.id);
    assert_eq!(session.token, token);
    assert_eq!(session.user_id, user.id);
}
