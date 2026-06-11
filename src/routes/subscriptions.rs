use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use sqlx::types::chrono;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>,pool: web::Data<PgPool>) -> HttpResponse {
    // Execute the insertion query
    let request_id = Uuid::new_v4();
    let request_span=tracing::info_span!("Adding a new subscriber.", %request_id, subscriber_email=%form.email, %subscriber_name=%form.name);
    let _request_span_guard=request_span.enter();
    tracing::info!("request_id {} - Adding {} {} as a new subscriber", request_id, form.email, form.name);
    tracing::info!("Saving new subscriber details in the database");
    match sqlx::query!(r#"INSERT INTO subscriptions (id, email, name, subscribed_at)VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(pool.get_ref()) // Use get_ref to get a reference to the PgPool
        .await
    {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details saved have been database", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}