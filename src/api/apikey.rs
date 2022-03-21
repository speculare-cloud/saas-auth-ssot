use crate::Pool;

use actix_web::{web, HttpRequest, HttpResponse};
use sproot::{
    errors::{AppError, AppErrorType},
    models::{ApiKey, ApiKeyDTOUpdate},
};

/// PATCH /api/key
pub async fn update_apikey(
    request: HttpRequest,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    info!("Route PATCH /api/key");

    // Get the SPTK header and the SP-UUID, error if not found (400)
    let sptk = match request.headers().get("SPTK") {
        Some(sptk) => sptk.to_owned(),
        None => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };
    let host_uuid = match request.headers().get("SP-UUID") {
        Some(host_uuid) => host_uuid.to_owned(),
        None => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    web::block(move || {
        // Check if it's ok to update the key (based on sptk result host_uuid == None)
        let api_key = ApiKey::get_entry(&db.get()?, sptk.to_str().unwrap())?;

        // If the host_uuid is none, we update the value with the current host_uuid from header
        // Otherwise it's an error as it's not authorized
        if api_key.host_uuid.is_none() {
            ApiKeyDTOUpdate {
                host_uuid: Some(host_uuid.to_str().unwrap().to_owned()),
                ..Default::default()
            }
            .update(&db.get()?, api_key.id)?;
            Ok(())
        } else {
            Err(AppError {
                message: "Invalid JWT token, access denied".to_owned(),
                error_type: AppErrorType::InvalidToken,
            })
        }
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}
