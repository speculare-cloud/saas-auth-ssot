use crate::Pool;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;

/// GET /api/register
pub async fn handle_register(db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/register");

    Ok(HttpResponse::Ok().finish())
}
