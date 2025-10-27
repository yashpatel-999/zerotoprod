use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::routes::domain::{SubscriberName, NewSubscriber};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id=%Uuid::new_v4(),
        subscriber_email=tracing::field::Empty,
        subscriber_name=tracing::field::Empty
    ),
    ret
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    tracing::Span::current()
        .record("subscriber_email", tracing::field::display(&form.email))
        .record("subscriber_name", tracing::field::display(&form.name));

    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: name
    };
    
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name="Saving new subscriber details in the database",
    skip(new_subscriber, pool),
    fields(
        subscriber_email=tracing::field::Empty,
        subscriber_name=tracing::field::Empty,
        subscriber_id=tracing::field::Empty
    ),
    ret,
    err
)]
pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
    let subscriber_id = Uuid::new_v4();

    tracing::Span::current()
        .record("subscriber_id", tracing::field::display(&subscriber_id))
        .record("subscriber_email", tracing::field::display(&new_subscriber.email))
        .record("subscriber_name", tracing::field::display(new_subscriber.name.as_ref()));

    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        subscriber_id,
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    
    Ok(())
}