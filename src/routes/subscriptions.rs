use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form,pool),
    fields(
        request_id=%Uuid::new_v4(),
        subscriber_email=%form.email,
        subscriber_name=%form.name
    ),
    ret
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool,&form).await
    {
        Ok(_)=>HttpResponse::Ok().finish(),
        Err(_)=>HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name="Saving new subscriber details in the database",
    skip(form,pool),
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name,
        subscriber_id=tracing::field::Empty
    ),
    ret,
    err
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    let subscriber_id=Uuid::new_v4();

    tracing::Span::current().record("subscriber_id",tracing::field::display(&subscriber_id));

    sqlx::query!(r#"INSERT INTO subscriptions (id,email,name,subscribed_at) VALUES ($1,$2,$3,$4)"#,
        subscriber_id,
        &form.email,
        &form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e|{
        e
    })?;
    Ok(())
}
