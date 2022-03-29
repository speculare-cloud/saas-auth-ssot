use super::Specific;

use actix_web::{
    web::{self, ReqData},
    HttpResponse,
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
    db: web::Data<AuthPool>,
    info: web::Query<Specific>,
    sptk: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("Route PATCH /api/key/{{sptk}}");

    web::block(move || {
        // Check if it's ok to update the key (based on sptk result host_uuid == None)
        let api_key = ApiKey::get_entry(&db.pool.get()?, &sptk)?;

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
    db: web::Data<AuthPool>,
    inner_user: ReqData<InnerUser>,
    sptk: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("Route DELETE /api/key/{{sptk}}");

    let res = web::block(move || {
        let conn = &db.pool.get()?;
        // Check if the entry exists for that user
        let exists = ApiKey::entry_exists(conn, &inner_user.uuid, &sptk)?;

        if exists {
            Ok(ApiKey::delete_key(conn, &sptk)?)
        } else {
            Err(AppError {
                message: "Invalid SPTK token".to_owned(),
                error_type: AppErrorType::NotFound,
            })
        }
    })
    .await??;

    Ok(HttpResponse::Ok().body(res.to_string()))
}
