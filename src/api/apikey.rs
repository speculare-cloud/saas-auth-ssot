use super::Specific;

use actix_web::{
    web::{self, ReqData},
    HttpRequest, HttpResponse,
};
use sproot::{
    errors::{AppError, AppErrorType},
    models::{ApiKey, ApiKeyDTOUpdate, AuthPool, InnerUser},
};

/// POST /api/key
///
/// Create a new ApiKey for the currently logged user (inner_user).
/// The resulting ApiKey is returned back via Json.
/// We'll also do the check for the quota of the user here,
/// depending on his plan, we'll allow him to create (or not)
/// a new ApiKey.
pub async fn post_apikey(
    _request: HttpRequest,
    _db: web::Data<AuthPool>,
    _inner_user: ReqData<InnerUser>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/key");
    todo!()
}

/// PATCH /api/key
///
/// This route update the host_uuid of the ApiKey entry
/// with key == sptk if the host_uuid was previously None.
/// The host_uuid is took from the Specific query params (?uuid=)
pub async fn update_apikey(
    request: HttpRequest,
    db: web::Data<AuthPool>,
    info: web::Query<Specific>,
) -> Result<HttpResponse, AppError> {
    info!("Route PATCH /api/key");

    // Get the SPTK header, error if not found (400)
    let sptk = match request.headers().get("SPTK") {
        Some(sptk) => sptk.to_owned(),
        None => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    web::block(move || {
        // Check if it's ok to update the key (based on sptk result host_uuid == None)
        let api_key = ApiKey::get_entry(&db.pool.get()?, sptk.to_str().unwrap())?;

        // If the host_uuid is none, we update the value with the current host_uuid from header
        // Otherwise it's an error as it's not authorized
        if api_key.host_uuid.is_none() {
            ApiKeyDTOUpdate {
                host_uuid: Some(info.uuid.to_owned()),
                ..Default::default()
            }
            .update(&db.pool.get()?, api_key.id)?;
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

/// DELETE /api/key
///
/// Delete an ApiKey with the key == SPTK.
/// Check if the ApiKey matching the SPTK is owned by the
/// the currently logged user (inner_user).
pub async fn delete_apikey(
    request: HttpRequest,
    _db: web::Data<AuthPool>,
    _inner_user: ReqData<InnerUser>,
) -> Result<HttpResponse, AppError> {
    info!("Route DELETE /api/key");

    // Get the SPTK header, error if not found (400)
    let _sptk = match request.headers().get("SPTK") {
        Some(sptk) => sptk.to_owned(),
        None => {
            return Ok(HttpResponse::BadRequest().finish());
        }
    };

    todo!()
}
