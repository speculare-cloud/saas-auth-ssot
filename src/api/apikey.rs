use crate::api::{get_header_value, get_user_session};

use super::Specific;

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use rand::{thread_rng, Rng};
use sproot::{
    apierrors::ApiError,
    models::{ApiKey, ApiKeyDTO, AuthPool},
};

/// GET /api/key
pub async fn get_apikey(
    session: Session,
    request: HttpRequest,
    db: web::Data<AuthPool>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/key");

    let user_uuid = get_user_session(&session)?;

    match get_header_value(&request, "SPTK") {
        // If the header is defined, we get the specific key for the current user
        Ok(sptk) => {
            let data = web::block(move || {
                ApiKey::get_key_owned(&mut db.pool.get()?, &user_uuid, sptk.to_str().unwrap())
            })
            .await??;

            Ok(HttpResponse::Ok().json(data))
        }
        // Otherwise we get all the keys for that user
        Err(_) => {
            let data =
                web::block(move || ApiKey::get_keys(&mut db.pool.get()?, &user_uuid)).await??;

            Ok(HttpResponse::Ok().json(data))
        }
    }
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
) -> Result<HttpResponse, ApiError> {
    info!("Route PATCH /api/key");

    let sptk = get_header_value(&request, "SPTK")?;

    web::block(move || {
        // Get the key which have the key == sptk
        let api_key = ApiKey::get_entry(&mut db.pool.get()?, sptk.to_str().unwrap())?;

        // If the host_uuid of that key is none, we update the value with the
        // current host_uuid from Specific otherwise it's an error as the user
        // try to update a Key that doesn't belong to him.
        if api_key.host_uuid.is_none() {
            ApiKeyDTO {
                host_uuid: Some(info.uuid.to_owned()),
                ..Default::default()
            }
            .update(&mut db.pool.get()?, api_key.id)?;
            Ok(())
        } else {
            Err(ApiError::AuthorizationError(None))
        }
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// POST /api/key
///
/// Create a new ApiKey for the currently logged user (inner_user).
/// The resulting ApiKey is returned back via Json.
/// We'll also do the check for the quota of the user here,
/// depending on his plan, we'll allow him to create (or not)
/// a new ApiKey.
pub async fn post_apikey(
    session: Session,
    db: web::Data<AuthPool>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/key");

    let user_uuid = get_user_session(&session)?;

    // TODO - Add check that the user can in fact create
    //        the key (based on his plan subscriptions)

    // Insert/get the inserted key
    let data = web::block(move || {
        let item: ApiKeyDTO = ApiKeyDTO {
            key: Some(
                thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect(),
            ),
            host_uuid: None,
            customer_id: Some(user_uuid),
            berta: Some("B1".to_owned()),
        };

        item.ginsert(&mut db.pool.get()?)
    })
    .await??;

    Ok(HttpResponse::Ok().json(data))
}

/// DELETE /api/key
///
/// Delete an ApiKey with the key == SPTK.
/// Check if the ApiKey matching the SPTK is owned by the
/// the currently logged user (inner_user).
pub async fn delete_apikey(
    session: Session,
    request: HttpRequest,
    db: web::Data<AuthPool>,
) -> Result<HttpResponse, ApiError> {
    info!("Route DELETE /api/key");

    let sptk = get_header_value(&request, "SPTK")?;
    let user_uuid = get_user_session(&session)?;

    let res = web::block(move || {
        let conn = &mut db.pool.get()?;
        let sptk = sptk.to_str().unwrap();

        // Check if the entry exists for that user
        let exists = ApiKey::own_key(conn, &user_uuid, sptk)?;

        if exists {
            Ok(ApiKey::delete_key(conn, sptk)?)
        } else {
            Err(ApiError::AuthorizationError(None))
        }
    })
    .await??;

    // Return the number of row affected (1 if went well, 0 otherwise)
    Ok(HttpResponse::Ok().body(res.to_string()))
}
