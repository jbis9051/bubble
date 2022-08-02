use crate::helper::start_server;
use axum::http::StatusCode;
use bubble::models::user::User;
use bubble::routes::user::{Confirm, Confirmation, CreateUser};

use sqlx::{Executor, Row};

mod helper;

#[tokio::test]
async fn create_user() {
    let (db, client) = start_server().await;

    let _clean = cleanup!(|db| {
        db.execute("DELETE FROM \"confirmation\"").await.unwrap();
        db.execute("DELETE FROM \"session_token\"").await.unwrap();
        db.execute("DELETE FROM \"user\"").await.unwrap();
    });

    let res = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateUser {
                email: "test@gmail.com".to_string(),
                username: "test".to_string(),
                password: "password".to_string(),
                phone: Some("18001239876".to_string()),
                name: "John Doe".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(res.status(), StatusCode::CREATED);

    //TEMP
    let user_uuid = User::get_uuid_by_username(&db, "test").await.unwrap();
    let user = User::get_by_uuid(&db, user_uuid)
        .await
        .expect("user doesn't exist");

    assert_eq!(user.username, "test");
    assert_eq!(user.password, "password");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, None);
    assert_eq!(user.phone, Some("18001239876".to_string()));
    assert_eq!(user.name, "John Doe");

    let row = sqlx::query("SELECT * FROM confirmation WHERE id = 1;")
        .fetch_one(&db)
        .await
        .unwrap();

    let conf = Confirmation {
        id: row.get("id"),
        user_id: row.get("user_id"),
        link_id: row.get("link_id"),
        email: row.get("email"),
        created: row.get("created"),
    };

    assert_eq!(conf.user_id, user.id);
    assert_eq!(conf.email, "test@gmail.com");

    let confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: conf.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(confirm_res.status(), StatusCode::CREATED);

    let user = User::get_by_uuid(&db, user_uuid).await.unwrap();

    assert_eq!(user.username, "test");
    assert_eq!(user.password, "password");
    assert_eq!(user.profile_picture, None);
    assert_eq!(user.email, Some("test@gmail.com".to_string()));
    assert_eq!(user.phone, Some("18001239876".to_string()));
    assert_eq!(user.name, "John Doe");
}

#[tokio::test]
async fn create_multiple_user() {
    let (db, client) = start_server().await;

    let res1 = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateUser {
                email: "python@gmail.com".to_string(),
                username: "machine_learning_man".to_string(),
                password: "lots_of_abstraction".to_string(),
                phone: None,
                name: "Brian".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    let res2 = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateUser {
                email: "javascript@gmail.com".to_string(),
                username: "web_development_dude".to_string(),
                password: "html_rocks".to_string(),
                phone: Some("66260701534".to_string()),
                name: "Little Timmy III".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(res1.status(), StatusCode::CREATED);
    assert_eq!(res2.status(), StatusCode::CREATED);
    //TEMP
    let brian_uuid = User::get_uuid_by_username(&db, "machine_learning_man")
        .await
        .unwrap();
    let brian = User::get_by_uuid(&db, brian_uuid).await.unwrap();
    let brian_row = sqlx::query("SELECT * FROM confirmation WHERE user_id = $1;")
        .bind(brian.id)
        .fetch_one(&db)
        .await
        .unwrap();
    let brian_conf = Confirmation {
        id: brian_row.get("id"),
        user_id: brian_row.get("user_id"),
        link_id: brian_row.get("link_id"),
        email: brian_row.get("email"),
        created: brian_row.get("created"),
    };

    //TEMP
    let timmy_uuid = User::get_uuid_by_username(&db, "web_development_dude")
        .await
        .unwrap();
    let timmy = User::get_by_uuid(&db, timmy_uuid)
        .await
        .expect("user doesn't exist");
    let timmy_row = sqlx::query("SELECT * FROM confirmation WHERE user_id = $1;")
        .bind(timmy.id)
        .fetch_one(&db)
        .await
        .unwrap();
    let timmy_conf = Confirmation {
        id: timmy_row.get("id"),
        user_id: timmy_row.get("user_id"),
        link_id: timmy_row.get("link_id"),
        email: timmy_row.get("email"),
        created: timmy_row.get("created"),
    };

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.password, "lots_of_abstraction");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, None);
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");
    assert_eq!(brian_conf.user_id, brian.id);
    assert_eq!(brian_conf.email, "python@gmail.com");

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.password, "html_rocks");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, None);
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
    assert_eq!(timmy_conf.user_id, timmy.id);
    assert_eq!(timmy_conf.email, "javascript@gmail.com");

    let brian_confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: brian_conf.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(brian_confirm_res.status(), StatusCode::CREATED);

    let brian = User::get_by_uuid(&db, brian_uuid).await.unwrap();

    assert_eq!(brian.username, "machine_learning_man");
    assert_eq!(brian.password, "lots_of_abstraction");
    assert_eq!(brian.profile_picture, None);
    assert_eq!(brian.email, Some("python@gmail.com".to_string()));
    assert_eq!(brian.phone, None);
    assert_eq!(brian.name, "Brian");

    let res3 = client
        .post("/user/signup")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&CreateUser {
                email: "rust@gmail.com".to_string(),
                username: "big_programmer_pro".to_string(),
                password: "cool_crustacean".to_string(),
                phone: Some("18004321234".to_string()),
                name: "Bill Gates".to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(res3.status(), StatusCode::CREATED);

    //TEMP
    let bill_uuid = User::get_uuid_by_username(&db, "big_programmer_pro")
        .await
        .unwrap();
    let bill = User::get_by_uuid(&db, bill_uuid)
        .await
        .expect("user doesn't exist");
    let bill_row = sqlx::query("SELECT * FROM confirmation WHERE user_id = $1;")
        .bind(bill.id)
        .fetch_one(&db)
        .await
        .unwrap();
    let bill_conf = Confirmation {
        id: bill_row.get("id"),
        user_id: bill_row.get("user_id"),
        link_id: bill_row.get("link_id"),
        email: bill_row.get("email"),
        created: bill_row.get("created"),
    };

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.password, "cool_crustacean");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, None);
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "Bill Gates");
    assert_eq!(bill_conf.user_id, bill.id);
    assert_eq!(bill_conf.email, "rust@gmail.com");

    let bill_confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: bill_conf.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(bill_confirm_res.status(), StatusCode::CREATED);

    let bill = User::get_by_uuid(&db, bill_uuid).await.unwrap();

    assert_eq!(bill.username, "big_programmer_pro");
    assert_eq!(bill.password, "cool_crustacean");
    assert_eq!(bill.profile_picture, None);
    assert_eq!(bill.email, Some("rust@gmail.com".to_string()));
    assert_eq!(bill.phone, Some("18004321234".to_string()));
    assert_eq!(bill.name, "Bill Gates");

    let timmy_confirm_res = client
        .post("/user/signup-confirm")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&Confirm {
                link_id: timmy_conf.link_id.to_string(),
            })
            .unwrap(),
        )
        .send()
        .await;

    assert_eq!(timmy_confirm_res.status(), StatusCode::CREATED);

    let timmy = User::get_by_uuid(&db, timmy_uuid).await.unwrap();

    assert_eq!(timmy.username, "web_development_dude");
    assert_eq!(timmy.password, "html_rocks");
    assert_eq!(timmy.profile_picture, None);
    assert_eq!(timmy.email, Some("javascript@gmail.com".to_string()));
    assert_eq!(timmy.phone, Some("66260701534".to_string()));
    assert_eq!(timmy.name, "Little Timmy III");
}
