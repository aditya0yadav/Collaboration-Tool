use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


use dotenv::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;
use warp::{Filter, Rejection, Reply};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Environment variable not set: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Argon2 error: {0}")]
    Argon2(String),
    #[error("User already exists")]
    UserExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
}

impl warp::reject::Reject for ApiError {}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(err: argon2::password_hash::Error) -> Self {
        ApiError::Argon2(err.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    username: String,
    password: String,
    image: Option<Vec<u8>>, // Optional image field as a byte vector
}

#[derive(Debug, Deserialize, Serialize)]
struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    token_type: String,
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "Authorization"])
        .max_age(3600);

    let register_route = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_register);

    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_login);

    let refresh_route = warp::path("refresh")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_refresh);

    // let image_route = warp::path("user_image")
    //     .and(warp::get())
    //     .and(warp::query::<HashMap<String, String>>())
    //     .and_then(handle_get_image);

    let routes = register_route
        .or(login_route)
        .or(refresh_route)
        // .or(image_route)
        .with(cors)
        .with(warp::log("warp::server"));

    println!("Server starting on http://127.0.0.1:3031");
    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
}

async fn handle_register(user: User) -> Result<impl Reply, Rejection> {
        match register_user(&user).await {
        Ok(tokens) => Ok(warp::reply::json(&tokens)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn handle_login(creds: LoginCredentials) -> Result<impl Reply, Rejection> {
    match login_user(&creds).await {
        Ok(tokens) => Ok(warp::reply::json(&tokens)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn handle_refresh(refresh_req: RefreshRequest) -> Result<impl Reply, Rejection> {
    match refresh_token(&refresh_req.refresh_token).await {
        Ok(tokens) => Ok(warp::reply::json(&tokens)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn register_user(user: &User) -> Result<TokenResponse, ApiError> {
    let conn = Connection::open("users.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;

    if conn.query_row(
        "SELECT 1 FROM users WHERE username = ?1",
        params![&user.username],
        |_| Ok(1)
    ).optional()?.is_some() {
        return Err(ApiError::UserExists);
    }

    let hashed_password = hash_password(&user.password)?;

    conn.execute(
        "INSERT INTO users (name, email, username, password) VALUES (?1, ?2, ?3, ?4)",
        params![&user.name, &user.email, &user.username, &hashed_password],
    )?;

    generate_tokens(&user.username)
}

async fn login_user(creds: &LoginCredentials) -> Result<TokenResponse, ApiError> {
    let conn = Connection::open("users.db")?;

    let result: Option<(String, String)> = conn
        .query_row(
            "SELECT username, password FROM users WHERE username = ?1",
            params![&creds.username],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;

    if let Some((username, hashed_password)) = result {
        if verify_password(&creds.password, &hashed_password)? {
            return generate_tokens(&username);
        }
    }

    Err(ApiError::InvalidCredentials)
}

async fn refresh_token(refresh_token: &str) -> Result<TokenResponse, ApiError> {
    let claims = validate_jwt(refresh_token)?;
    
    if claims.token_type != "refresh" {
        return Err(ApiError::InvalidToken);
    }

    generate_tokens(&claims.sub)
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    // Parameters for Argon2 hashing (default values)
    let params = argon2::Params::default();

    // Hash the password with the generated salt and default parameters
    let hashed_password = argon2.hash_password(password.as_bytes(), None, params.clone(), &salt)?;

    // Convert the hashed password to a String
    Ok(hashed_password.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

fn generate_tokens(username: &str) -> Result<TokenResponse, ApiError> {
    let access_token = create_token(username, "access", 15)?; // 15 minutes expiration
    let refresh_token = create_token(username, "refresh", 60 * 24 * 7)?; // 1 week expiration

    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}


fn create_token(username: &str, token_type: &str, expiration_minutes: i64) -> Result<String, ApiError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(expiration_minutes))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
        token_type: token_type.to_string(),
    };

    let secret = env::var("JWT_SECRET")?;
    let key = EncodingKey::from_secret(secret.as_bytes());
    Ok(encode(&Header::default(), &claims, &key)?)
}

fn validate_jwt(token: &str) -> Result<Claims, ApiError> {
    let secret = env::var("JWT_SECRET")?;
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = false; // We'll handle expiration check manually
    let token_data = decode::<Claims>(token, &key, &validation)?;
    
    let current_time = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp < current_time {
        return Err(ApiError::InvalidToken);
    }

    Ok(token_data.claims)
}
