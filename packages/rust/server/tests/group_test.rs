use crate::helper::{start_server, TempDatabase};
use axum::http::StatusCode;
use axum_test_helper::TestClient;
use std::borrow::Borrow;

use bubble::models::group::{Group, Role};
use bubble::routes::group::{GroupInfo, GroupName, NameChange, UserID};

use sqlx::Row;

use bubble::routes::user::CreateUser;

use uuid::Uuid;

mod helper;

async fn create_user_struct() -> CreateUser {
    CreateUser {
        email: "rina@gmail.com".to_string(),
        username: "Rina Sawayama".to_string(),
        password: "thishell".to_string(),
        phone: None,
        name: "chosenfamily".to_string(),
    }
}
async fn create_second_user_struct() -> CreateUser {
    CreateUser {
        email: "bj@gmail.com".to_string(),
        username: "Billy Joel".to_string(),
        password: "shesbeenlivingina".to_string(),
        phone: None,
        name: "uptownworld".to_string(),
    }
}

async fn create_third_user_struct() -> CreateUser {
    CreateUser {
        email: "am@gmail.com".to_string(),
        username: "Artic Monkeys".to_string(),
        password: "doiwannaknow".to_string(),
        phone: None,
        name: "arabella".to_string(),
    }
}

async fn add_user_helper(db: &TempDatabase, client: &TestClient) -> (String, Uuid) {
    let first_user = create_user_struct().await;
    let (token_admin, _) = helper::initialize_user(db.pool(), client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), client, group_name_in, bearer)
        .await
        .unwrap();
    let group_info: GroupInfo = res.json().await;
    (group_info.uuid, token_admin)
}

#[tokio::test]
async fn create_group() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;
    let (token, test_user) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();
    let status = res.status();
    let group_info: GroupInfo = res.json().await;

    assert_eq!(status, StatusCode::CREATED);

    let group_uuid = group_info.uuid;
    let group = Group::from_uuid(db.pool(), &Uuid::parse_str(&group_uuid).unwrap())
        .await
        .expect("No group exists in database.");

    assert_eq!(group.group_name, "test_group_1");

    let group_name = group_info.name;

    assert_eq!(group_name, "test_group_1");

    let role_id = group.role(db.pool(), test_user.id).await.unwrap();
    assert_eq!(role_id, Role::Admin);
}

#[tokio::test]
async fn negative_create_group_no_json() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;
    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let _res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token);
    let res = client
        .post("/group")
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn negative_create_group_no_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_name_in_duplicate = "test_group_1".to_owned();
    let res = client
        .post("/group")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: group_name_in_duplicate,
            })
            .unwrap(),
        )
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn negative_create_group_not_authorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let bearer = format!("Bearer {}", Uuid::new_v4());
    let res = client
        .post("/group")
        .header("Content-Type", "application/json")
        .body(
            serde_json::to_string(&GroupName {
                name: "test_group".to_string(),
            })
            .unwrap(),
        )
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn negative_create_group_not_json() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;
    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token);
    let group_name_in_duplicate = "test_group_1".to_owned();
    let res = client
        .post("/group")
        .header("Content-Type", "application/json")
        .body(group_name_in_duplicate)
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn read_group() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;

    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
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
}

#[tokio::test]
async fn negative_read_group_not_authorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;

    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;

    let token_bad = Uuid::new_v4();
    let read_route = format!("/group/{}", group_uuid);
    let bearer = format!("Bearer {}", token_bad);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn negative_read_group_not_found() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;

    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let group_uuid = Uuid::new_v4();
    let read_route = format!("/group/{}", group_uuid);
    let bearer = format!("Bearer {}", token);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn negative_read_group_incorrect_route() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;

    let (token, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let group_uuid = "914150185";
    let read_route = format!("/group/{}", group_uuid);
    let bearer = format!("Bearer {}", token);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn add_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;
    let (token_admin, creator) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;

    let first_user = create_second_user_struct().await;
    let (_, billy_joel) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let first_user = create_third_user_struct().await;
    let (_, kanye_west) = helper::initialize_user(db.pool(), &client, &first_user)
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
    let new_group = Group::from_uuid(db.pool(), &Uuid::parse_str(group_uuid_ref).unwrap())
        .await
        .unwrap();

    let billy_joel_role = new_group.role(db.pool(), billy_joel.id).await.unwrap();
    let kanye_west_role = new_group.role(db.pool(), kanye_west.id).await.unwrap();
    assert_eq!(billy_joel_role, Role::Member);
    assert_eq!(kanye_west_role, Role::Member);

    let bearer = format!("Bearer {}", token_admin);
    let read_route = format!("/group/{}/members", group_uuid);
    let res = client
        .get(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);
    let user_ids: UserID = res.json().await;

    assert_eq!(*user_ids.users.get(0).unwrap(), creator.uuid.to_string());
    assert_eq!(*user_ids.users.get(1).unwrap(), billy_joel.uuid.to_string());
    assert_eq!(*user_ids.users.get(2).unwrap(), kanye_west.uuid.to_string());
}

#[tokio::test]
async fn negative_add_user_not_authorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let second_user = create_second_user_struct().await;
    let (_, test_user) = helper::initialize_user(db.pool(), &client, &second_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![test_user.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bad_token = Uuid::new_v4();
    let bearer = format!("Bearer {}", bad_token);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn negative_add_user_non_admin() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let first_user = create_second_user_struct().await;
    let (bad_token, billy_joel) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![billy_joel.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let _ = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    let first_user = create_third_user_struct().await;
    let (_, kanye_west) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();
    let user_ids: Vec<String> = vec![kanye_west.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", bad_token);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn negative_add_user_violate_unique_constraints() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let first_user = create_second_user_struct().await;
    let (_, billy_joel) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let first_user = create_third_user_struct().await;
    let (_, kanye_west) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![billy_joel.uuid.to_string(), kanye_west.uuid.to_string()];

    let read_route = format!("/group/{}/new_users", group_uuid.0);

    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let user_ids: Vec<String> = vec![billy_joel.uuid.to_string(), kanye_west.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn negative_add_user_users_do_not_exist() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let user_ids: Vec<String> = vec![Uuid::new_v4().to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_user() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let first_user = create_second_user_struct().await;
    let (_, dolly_parton) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let first_user = create_third_user_struct().await;
    let (_, artic_monkeys) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![
        dolly_parton.uuid.to_string(),
        artic_monkeys.uuid.to_string(),
    ];

    let read_route = format!("/group/{}/new_users", group_uuid.0);

    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;

    let group_uuid_ref: &str = &*group_uuid.0;
    let new_group = Group::from_uuid(db.pool(), &Uuid::parse_str(group_uuid_ref).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let user_ids: Vec<String> = vec![dolly_parton.uuid.to_string()];

    let read_route = format!("/group/{}/delete_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .delete(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let remaining_user_group_row =
        helper::get_user_group(db.pool(), new_group.id, artic_monkeys.id)
            .await
            .unwrap();
    let artic_monkeys_role: i32 = remaining_user_group_row.get("role_id");
    assert_eq!(artic_monkeys_role, Role::Member as i32);

    let remaining_user_group_status =
        match helper::get_user_group(db.pool(), new_group.id, artic_monkeys.id).await {
            Ok(_row) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

    assert_eq!(remaining_user_group_status, StatusCode::OK);

    let deleted_user_error =
        match helper::get_user_group(db.pool(), new_group.id, dolly_parton.id).await {
            Ok(_row) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
    assert_eq!(deleted_user_error, StatusCode::INTERNAL_SERVER_ERROR);

    let user_ids: Vec<String> = vec![artic_monkeys.uuid.to_string()];

    let read_route = format!("/group/{}/delete_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .delete(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let deleted_user_error =
        match helper::get_user_group(db.pool(), new_group.id, artic_monkeys.id).await {
            Ok(_row) => StatusCode::OK,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
    assert_eq!(deleted_user_error, StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn negative_delete_user_delete_admin() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let first_user = create_user_struct().await;
    let (token_admin, creator) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();
    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "test_group_1".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();
    let user_ids: Vec<String> = vec![creator.uuid.to_string()];
    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;
    let read_route = format!("/group/{}/delete_users", group_uuid);
    let bearer = format!("Bearer {}", token_admin);
    let res = client
        .delete(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn negative_delete_user_not_authorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;
    let test_user = create_second_user_struct().await;
    let (_, the_police) = helper::initialize_user(db.pool(), &client, &test_user)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![the_police.uuid.to_string()];
    let read_route = format!("/group/{}/delete_users", group_uuid.0);
    let unauthorized_user = Uuid::new_v4();
    let bearer = format!("Bearer {}", unauthorized_user);
    let res = client
        .delete(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn negative_delete_user_to_delete_nonexisitant() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;

    let user_ids: Vec<String> = vec![Uuid::new_v4().to_string()];
    let read_route = format!("/group/{}/delete_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .delete(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn change_name() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let first_user = create_user_struct().await;
    let (token_admin, _) = helper::initialize_user(db.pool(), &client, &first_user)
        .await
        .unwrap();

    let bearer = format!("Bearer {}", token_admin);
    let group_name_in = "GroupNameOne".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
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
    let changed_group = Group::from_uuid(db.pool(), &Uuid::parse_str(&*group_uuid).unwrap())
        .await
        .unwrap();

    let group_name = changed_group.group_name;
    assert_eq!(group_name, "GroupNameTwo");
}
#[tokio::test]
async fn negative_change_name_unauthorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let read_route = format!("/group/{}/name", group_uuid.0);
    let unauthorized_user = Uuid::new_v4();
    let bearer = format!("Bearer {}", unauthorized_user);
    let name_to_change: String = "GroupNameThree".to_string();
    let res = client
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
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn negative_change_name_by_non_member() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let test_user = create_second_user_struct().await;
    let (token_test_user, _thepolice) = helper::initialize_user(db.pool(), &client, &test_user)
        .await
        .unwrap();

    let read_route = format!("/group/{}/name", group_uuid.0);
    let bearer = format!("Bearer {}", token_test_user);
    let name_to_change: String = "GroupNameThree".to_string();
    let res = client
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
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn negative_change_name_by_non_admin() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;

    let group_uuid = add_user_helper(&db, &client).await;

    let test_user = create_second_user_struct().await;
    let (token_test_user, thepolice) = helper::initialize_user(db.pool(), &client, &test_user)
        .await
        .unwrap();
    let user_ids: Vec<String> = vec![thepolice.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let bearer = format!("Bearer {}", token_test_user);
    let name_to_change: String = "GroupNameFour".to_string();
    let read_route = format!("/group/{}/name", group_uuid.0);
    let res = client
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
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn delete() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;
    let bearer = format!("Bearer {}", group_uuid.1);
    let read_route = format!("/group/{}", group_uuid.0);
    let res = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let delete_group =
        match Group::from_uuid(db.pool(), &Uuid::parse_str(&group_uuid.0).unwrap()).await {
            Ok(_group) => StatusCode::OK,
            Err(_) => StatusCode::BAD_REQUEST,
        };
    assert_eq!(delete_group, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn negative_delete_unauthorized() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;
    let bearer = format!("Bearer {}", group_uuid.1);
    let group_name_in = "test_group_2".to_owned();
    let res = helper::create_group(db.pool(), &client, group_name_in, bearer)
        .await
        .unwrap();
    assert_eq!(StatusCode::CREATED, res.status());

    let group_info: GroupInfo = res.json().await;
    let group_uuid = group_info.uuid;
    let bearer = format!("Bearer {}", Uuid::new_v4());
    let read_route = format!("/group/{}", group_uuid);
    let res = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn negative_delete_non_admin() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;
    let cyndi = create_second_user_struct().await;
    let (token_non_admin, cyndi) = helper::initialize_user(db.pool(), &client, &cyndi)
        .await
        .unwrap();

    let user_ids: Vec<String> = vec![cyndi.uuid.to_string()];
    let read_route = format!("/group/{}/new_users", group_uuid.0);
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .post(read_route.borrow())
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&UserID { users: user_ids }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::OK);

    let bearer = format!("Bearer {}", token_non_admin);
    let read_route = format!("/group/{}", group_uuid.0);
    let res = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn negative_delete_group_does_not_exist() {
    let db = TempDatabase::new().await;
    let client = start_server(db.pool().clone()).await;
    let group_uuid = add_user_helper(&db, &client).await;
    let bearer = format!("Bearer {}", group_uuid.1);
    let read_route = format!("/group/{}", group_uuid.0);
    let _ = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    let bearer = format!("Bearer {}", group_uuid.1);
    let res = client
        .delete(read_route.borrow())
        .header("Authorization", bearer)
        .send()
        .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
