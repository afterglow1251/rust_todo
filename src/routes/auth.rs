use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use mongodb::bson::{doc, from_document, Document};
use mongodb::{Client, Collection};
use crate::models::User;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: Option<String>,
}

#[post("/signup")]
async fn signup(
    client: web::Data<Client>,
    request: web::Json<SignupRequest>,
) -> impl Responder {
    let collection: Collection<User> = client.database("Rust_Todo_App").collection("users");

    // Перевірка наявності користувача з таким самим email
    let filter = doc! { "email": &request.email };
    let existing_user = collection.find_one(filter).await;

    match existing_user {
        Ok(Some(_)) => {
            // Користувач з таким email вже існує
            return HttpResponse::BadRequest().json(SignupResponse {
                message: "User with this email already exists".to_string(),
                user_id: None,
            });
        },
        Err(_) => return HttpResponse::InternalServerError().json("Error checking user"),
        _ => {} // Продовжуємо, якщо користувач не знайдений
    }

    let new_user = User {
        email: request.email.clone(),
        password: request.password.clone(),
    };

    let insert_result = collection.insert_one(new_user.clone()).await;

    match insert_result {
        Ok(inserted) => {
            let user_id = inserted.inserted_id.as_object_id().unwrap().to_string(); // Отримуємо ObjectId як рядок
            HttpResponse::Ok().json(SignupResponse {
                message: "User created successfully".to_string(),
                user_id: Some(user_id),
            })
        },
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: Option<String>,
}

#[post("/login")]
async fn login(
    client: web::Data<Client>,
    request: web::Json<LoginRequest>,
) -> impl Responder {
    let collection: Collection<Document> = client.database("Rust_Todo_App").collection("users");

    // Пошук користувача по email
    let filter = doc! { "email": &request.email };
    let user_doc = collection.find_one(filter).await;

    match user_doc {
        Ok(Some(doc)) => {
            let user: User = from_document(doc.clone()).expect("Failed to convert BSON to User");
            if user.password == request.password {
                let user_id = doc.get_object_id("_id").unwrap().to_string();
                HttpResponse::Ok().json(LoginResponse {
                    message: "Login successful".to_string(),
                    user_id: Some(user_id),
                })
            } else {
                HttpResponse::Unauthorized().json(LoginResponse {
                    message: "Invalid password".to_string(),
                    user_id: None,
                })
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(LoginResponse {
            message: "User not found".to_string(),
            user_id: None,
        }),
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
            message: "Login failed".to_string(),
            user_id: None,
        }),
    }
}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().json("Logout successful")
}
