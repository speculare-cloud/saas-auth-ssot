use crate::Pool;

use actix_web::{web, HttpResponse};
use sproot::errors::AppError;

/// GET /api/login
pub async fn handle_login(db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/login");

    Ok(HttpResponse::Ok().finish())
}
