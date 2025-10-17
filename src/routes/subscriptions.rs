use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    
    let subscriber_id = Uuid::new_v4();
    let result = sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"
    )
    .bind(subscriber_id)
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;
    
    match result {
        Ok(_) => {
            println!("Successfully saved subscriber: {}", form.email);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Subscription successful",
                "subscriber": {
                    "id": subscriber_id.to_string(),
                    "name": form.name,
                    "email": form.email
                }
            }))
        },
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to save subscription"
            }))
        }
    }
}

