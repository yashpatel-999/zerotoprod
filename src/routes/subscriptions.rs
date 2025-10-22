use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscriber_id = Uuid::new_v4();
    tracing::info!("request_id {} - Adding {} {} as a new subscriber.",subscriber_id,form.email,form.name);
    tracing::info!("request_id {} - Saving new subscriber details in the database",subscriber_id);
    let result = sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(subscriber_id)
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved",subscriber_id);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Subscription successful",
                "subscriber": {
                    "id": subscriber_id.to_string(),
                    "name": form.name,
                    "email": form.email
                }
            }))
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}",subscriber_id,e);

            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to save subscription"
            }))
        }
    }
}
