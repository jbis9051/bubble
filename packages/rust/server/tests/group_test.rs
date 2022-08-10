use crate::helper::start_server;
use axum::http::StatusCode;
use std::borrow::Borrow;

use bubble::models::group::{Group, Role};
use bubble::routes::group::{GroupInfo, NameChange, UserID};

use sqlx::Executor;

use sqlx::Row;

use bubble::routes::user::CreateUser;
use uuid::Uuid;

mod helper;

#[tokio::test]
async fn create_group() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<i32>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
     }, |db, resources| {
         if let Some((group_id, user_id)) = resources.user_group_id {
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;")
            .bind(&user_id)
            .bind(&group_id)
            .execute(&db)
            .await
            .unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE id = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_user = CreateUser {
        email: "rina@gmail.com".to_string(),
        username: "Rina Sawayama".to_string(),
        password: "thishell".to_string(),
        phone: None,
        name: "chosenfamily".to_string(),
    };
    let (token, test_user) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();
    let status = res.status();
    let group_info: GroupInfo = res.json().await;

    assert_eq!(status, StatusCode::CREATED);

    let group_uuid = group_info.uuid;
    let group_name = group_info.name;

    assert_eq!(group_name, "test_group_1");

    let group = Group::from_uuid(&db, Uuid::parse_str(&group_uuid).unwrap())
        .await
        .expect("No group exists in database.");
    assert_eq!(group.group_name, "test_group_1");

    let role_id = group.role(&db, test_user.id).await.unwrap();
    assert_eq!(role_id, Role::Admin);

    let user_group: (i32, i32) = (test_user.id, group.id);

    cleanup.resources.user_group_id = Some(user_group);
    cleanup.resources.group_id = Some(group.id);
    cleanup.resources.session_token = Some(token);
    cleanup.resources.user_id = Some(test_user.id);
}

#[tokio::test]
async fn read_group() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<i32>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
     }, |db, resources| {
         if let Some((group_id, user_id)) = resources.user_group_id {
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;")
            .bind(&user_id)
            .bind(&group_id)
            .execute(&db)
            .await
            .unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE id = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_user = CreateUser {
        email: "kissmemore@gmail.com".to_string(),
        username: "Madonna".to_string(),
        password: "material".to_string(),
        phone: None,
        name: "girl".to_string(),
    };

    let (token, test_user) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_name = group_info.name;
    let group_uuid = group_info.uuid;

    let read_route = format!("/group/{}", group_uuid);

    let bearer = format!("Bearer {}", token);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    let read_group: GroupInfo = res.json().await;

    assert_eq!(read_group.name, group_name);
    assert_eq!(read_group.uuid, group_uuid);

    let group_uuid = Uuid::parse_str(&group_uuid).unwrap();
    let temp_group = Group::from_uuid(&db, group_uuid).await.unwrap();
    let user_group: (i32, i32) = (temp_group.id, test_user.id);

    cleanup.resources.session_token = Some(token);
    cleanup.resources.group_id = Some(temp_group.id);
    cleanup.resources.user_id = Some(test_user.id);
    cleanup.resources.user_group_id = Some(user_group);
}

//route get_users is also tested to verify add_users
#[tokio::test]
async fn add_user() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<Vec<(i32, i32)>>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<Uuid>,
        pub session_token: Option<Vec<Uuid>>,
        pub user_id: Option<Vec<i32>>
     }, |db, resources| {
         if let Some(user_group_ids) = resources.user_group_id {
            for i in user_group_ids {
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;")
            .bind(&i.0)
            .bind(&i.1)
            .execute(&db)
            .await
            .unwrap();
                }
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE uuid = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
            for i in session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&i).execute(&db).await.unwrap();
        }
            }
        if let Some(user_id) = resources.user_id {
            for i in user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&i).execute(&db).await.unwrap();
        }
            }
    });

    let first_user = CreateUser {
        email: "everythigngoeson@gmail.com".to_string(),
        username: "Porter Robinson".to_string(),
        password: "musician".to_string(),
        phone: None,
        name: "passthesettingsun".to_string(),
    };
    let (token_admin, creator) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;

    let first_user = CreateUser {
        email: "bj@gmail.com".to_string(),
        username: "Billy Joel".to_string(),
        password: "shesbeenlivingina".to_string(),
        phone: None,
        name: "uptownworld".to_string(),
    };
    let (token_user_1, billy_joel) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();

    let first_user = CreateUser {
        email: "kw@gmail.com".to_string(),
        username: "Kanye West".to_string(),
        password: "ifigetlockedup".to_string(),
        phone: None,
        name: "iwontfinishthesent".to_string(),
    };
    let (token_user_2, kanye_west) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![billy_joel.uuid.to_string(), kanye_west.uuid.to_string()];

    let read_route = format!("/group/{}/new_users", group_uuid);

    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let group_uuid_ref: &str = &*group_uuid;
    let new_group = Group::from_uuid(&db, Uuid::parse_str(group_uuid_ref).unwrap())
        .await
        .unwrap();
    let billy_joel_role = new_group.role(&db, billy_joel.id).await.unwrap();
    let kanye_west_role = new_group.role(&db, kanye_west.id).await.unwrap();
    assert_eq!(billy_joel_role, Role::Child);
    assert_eq!(kanye_west_role, Role::Child);

    //TESTING GET_USERS, A SEPERATE TEST WOULD BE VERY WET
    let bearer = format!("Bearer {}", token_admin);
    let read_route = format!("/group/{}/members", group_uuid);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let user_ids: UserID = res.json().await;
    assert_eq!(*user_ids.users.get(0).unwrap(), creator.id.to_string());
    assert_eq!(*user_ids.users.get(1).unwrap(), billy_joel.id.to_string());
    assert_eq!(*user_ids.users.get(2).unwrap(), kanye_west.id.to_string());

    let tokens: Vec<Uuid> = vec![token_admin, token_user_1, token_user_2];
    let users: Vec<i32> = vec![creator.id, billy_joel.id, kanye_west.id];
    let user_groups: Vec<(i32, i32)> = vec![
        (creator.id, new_group.id),
        (billy_joel.id, new_group.id),
        (kanye_west.id, new_group.id),
    ];

    let group_uuid_ref: &str = &*group_uuid;

    cleanup.resources.user_group_id = Some(user_groups);
    cleanup.resources.group_id = Some(Uuid::parse_str(group_uuid_ref).unwrap());
    cleanup.resources.session_token = Some(tokens);
    cleanup.resources.user_id = Some(users);
}

#[tokio::test]
async fn delete_user() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<Vec<(i32, i32)>>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<Uuid>,
        pub session_token: Option<Vec<Uuid>>,
        pub user_id: Option<Vec<i32>>
    }, |db, resources| {
        if let Some(user_group_ids) = resources.user_group_id {
            for i in user_group_ids {
                sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;").bind(&i.0).bind(&i.1).execute(&db).await.unwrap();
            }
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE uuid = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
            for i in session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&i).execute(&db).await.unwrap();
            }
        }
        if let Some(user_id) = resources.user_id {
            for i in user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&i).execute(&db).await.unwrap();
            }
        }
    });

    let first_user = CreateUser {
        email: "dancinginthedark@gmail.com".to_string(),
        username: "Bruce Springsteen".to_string(),
        password: "sitaroundgettingolder".to_string(),
        phone: None,
        name: "thejokesonme".to_string(),
    };
    let (token_admin, creator) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;

    let first_user = CreateUser {
        email: "dp@gmail.com".to_string(),
        username: "Dolly Parton".to_string(),
        password: "clearbluemorning".to_string(),
        phone: None,
        name: "ninetofive".to_string(),
    };
    let (token_user_1, dolly_parton) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();

    let first_user = CreateUser {
        email: "am@gmail.com".to_string(),
        username: "Artic Monkeys".to_string(),
        password: "doiwannaknow".to_string(),
        phone: None,
        name: "arabella".to_string(),
    };
    let (token_user_2, artic_monkeys) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![
        dolly_parton.uuid.to_string(),
        artic_monkeys.uuid.to_string(),
    ];

    let read_route = format!("/group/{}/new_users", group_uuid);

    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user_ids: Vec<String> = vec![dolly_parton.uuid.to_string()];

    let read_route = format!("/group/{}/delete_users", group_uuid);
    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let group_uuid_ref: &str = &*group_uuid;
    let new_group = Group::from_uuid(&db, Uuid::parse_str(group_uuid_ref).unwrap())
        .await
        .unwrap();

    let remaining_user_group_row = helper::get_user_group(&db, new_group.id, artic_monkeys.id)
        .await
        .unwrap();
    let artic_monkeys_role: i32 = remaining_user_group_row.get("role_id");
    assert_eq!(artic_monkeys_role, Role::Child as i32);

    let remaining_user_group_status =
        match helper::get_user_group(&db, new_group.id, artic_monkeys.id).await {
            Ok(_row) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

    assert_eq!(remaining_user_group_status, StatusCode::OK);

    let deleted_user_error = match helper::get_user_group(&db, new_group.id, dolly_parton.id).await
    {
        Ok(_row) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    assert_eq!(deleted_user_error, StatusCode::INTERNAL_SERVER_ERROR);

    let user_ids: Vec<String> = vec![artic_monkeys.uuid.to_string()];

    let read_route = format!("/group/{}/delete_users", group_uuid);
    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let deleted_user_error = match helper::get_user_group(&db, new_group.id, artic_monkeys.id).await
    {
        Ok(_row) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    assert_eq!(deleted_user_error, StatusCode::INTERNAL_SERVER_ERROR);

    let user_ids: Vec<String> = vec![creator.uuid.to_string()];

    let read_route = format!("/group/{}/delete_users", group_uuid);
    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let tokens: Vec<Uuid> = vec![token_admin, token_user_1, token_user_2];

    let users: Vec<i32> = vec![creator.id, dolly_parton.id, artic_monkeys.id];
    let user_groups: Vec<(i32, i32)> = vec![
        (creator.id, new_group.id),
        (dolly_parton.id, new_group.id),
        (artic_monkeys.id, new_group.id),
    ];

    let group_uuid_ref: &str = &*group_uuid;

    cleanup.resources.user_group_id = Some(user_groups);
    cleanup.resources.group_id = Some(Uuid::parse_str(group_uuid_ref).unwrap());
    cleanup.resources.session_token = Some(tokens);
    cleanup.resources.user_id = Some(users);
}

#[tokio::test]
async fn change_name() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<Uuid>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
    }, |db, resources| {
        if let Some(user_group_ids) = resources.user_group_id {
                sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;").bind(&user_group_ids.0).bind(&user_group_ids.1).execute(&db).await.unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE uuid = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_user = CreateUser {
        email: "nmh@gmail.com".to_string(),
        username: "Neutral Milk Hotel".to_string(),
        password: "nmh".to_string(),
        phone: None,
        name: "Neutral Milk Hotel".to_string(),
    };
    let (token_admin, creator) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "GroupNameOne".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_name = group_info.name;
    let group_uuid = group_info.uuid;

    assert_eq!(group_name, "GroupNameOne");

    let read_route = format!("/group/{}/name", group_uuid);

    let bearer = format!("Bearer {}", token_admin);
    let name_to_change: String = "GroupNameTwo".to_string();
    let res_change = client
        .patch(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&NameChange {
                name: name_to_change,
            })
            .unwrap(),
        )
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res_change.status(), StatusCode::OK);
    let changed_group = Group::from_uuid(&db, Uuid::parse_str(&*group_uuid).unwrap())
        .await
        .unwrap();
    let group_name = changed_group.group_name;
    assert_eq!(group_name, "GroupNameTwo");

    let user_group: (i32, i32) = (changed_group.id, creator.id);
    let group_uuid_ref: &str = &*group_uuid;

    cleanup.resources.user_group_id = Some(user_group);
    cleanup.resources.group_id = Some(Uuid::parse_str(group_uuid_ref).unwrap());
    cleanup.resources.session_token = Some(token_admin);
    cleanup.resources.user_id = Some(creator.id);
}

#[tokio::test]
async fn delete() {
    let (db, client) = start_server().await;

    let mut cleanup = cleanup!({
        //user_group_id accepts the group_id of entry to be deleted
        pub user_group_id: Option<(i32, i32)>,
        pub location_group_id: Option<i32>,
        pub group_id: Option<i32>,
        pub session_token: Option<Uuid>,
        pub user_id: Option<i32>
     }, |db, resources| {
         if let Some((group_id, user_id)) = resources.user_group_id {
            sqlx::query("DELETE FROM user_group WHERE user_id = $1 AND group_id = $2;")
            .bind(&user_id)
            .bind(&group_id)
            .execute(&db)
            .await
            .unwrap();
        }
        if let Some(location_group_id) = resources.location_group_id {
                sqlx::query("DELETE FROM location_group WHERE id = $1").bind(&location_group_id).execute(&db).await.unwrap();
        }
        if let Some(group_id) = resources.group_id {
                sqlx::query("DELETE FROM \"group\" WHERE id = $1").bind(&group_id).execute(&db).await.unwrap();
        }
        if let Some(session_token) = resources.session_token {
                sqlx::query("DELETE FROM session_token WHERE token = $1").bind(&session_token).execute(&db).await.unwrap();
        }
        if let Some(user_id) = resources.user_id {
                sqlx::query("DELETE FROM \"user\" WHERE id = $1").bind(&user_id).execute(&db).await.unwrap();
        }
    });

    let first_user = CreateUser {
        email: "backinthesummerof69@gmail.com".to_string(),
        username: "Bryan Adams".to_string(),
        password: "testy".to_string(),
        phone: None,
        name: "testname".to_string(),
    };
    let (token_admin, creator) = helper::initialize_user(&db, &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(&db, &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;
    let bearer = format!("Bearer {}", token_admin);
    let read_route = format!("/group/{}", group_uuid);
    let res = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let delete_group = match Group::from_uuid(&db, Uuid::parse_str(&group_uuid).unwrap()).await {
        Ok(_group) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    };
    assert_eq!(delete_group, StatusCode::BAD_REQUEST);
    cleanup.resources.session_token = Some(token_admin);
    cleanup.resources.user_id = Some(creator.id);
}
