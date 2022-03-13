use crate::Pool;

use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};
use sproot::models::Alerts;

/// GET /api/callback
/// Exchange the code from the callback for a CookieSession
pub async fn exchange_callback(
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/callback");

	Ok(HttpResponse::Ok().finish())
}