use actix_web::{HttpResponse, Responder, get, post, web};
use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: i64,
    email: String,
}
#[derive(Deserialize, Debug)]
pub struct UserInput {
    email: String,
    password: String,
}
#[get("/users")]
pub async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT id,email FROM users")
        .fetch_all(&**pool)
        .await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Lỗi khi truy vấn users: {}", e);
            actix_web::HttpResponse::InternalServerError().body("Lỗi khi lấy danh sách users")
        }
    }
}
#[post("/users")]
pub async fn create_user(pool: web::Data<MySqlPool>, user: web::Json<UserInput>) -> impl Responder {
    println!("Data server gửi lên {:?}", &user);
    if user.email.is_empty() || !user.email.contains('@') {
        return HttpResponse::BadRequest().body("Email không hợp lệ");
    }
    if user.password.len() < 6 {
        return HttpResponse::BadRequest().body("Password phải dài ít nhất 6 ký tự");
    }
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Lỗi khi mã hóa mật khẩu: {}", e);
            return HttpResponse::InternalServerError().body("Lỗi khi tạo user");
        }
    };
    let result = sqlx::query!(
        "INSERT INTO users(email, password) VALUES(?, ?)",
        user.email,
        hashed_password
    )
    .execute(&**pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                HttpResponse::Created().body("User created successfully")
            } else {
                HttpResponse::InternalServerError().body("Failed to create user")
            }
        }
        Err(e) => {
            eprintln!("Lỗi khi tạo user: {}", e);
            HttpResponse::Conflict().body("Email đã tồn tại hoặc lỗi khi tạo user")
        }
    }
}
