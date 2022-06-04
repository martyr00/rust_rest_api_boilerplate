use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::database::connect_to_db::MongoDB;
use crate::r#const::{
    ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_WEAK_LOGIN, ERROR_WEAK_PASSWORD,
    ERROR_WRONG_REQUEST,
};
use crate::routes::routes_models::registration_request::RegistrationRequest;
use crate::routes::{valid_password_and_login, GetIsValidLoginAndPassword};

#[post(
    "/registration",
    format = "json",
    data = "<maybe_registration_request>"
)]
pub async fn registration(
    database: &State<MongoDB>,
    maybe_registration_request: Option<Json<RegistrationRequest>>,
) -> Result<Status, (Status, &'static str)> {
    match maybe_registration_request {
        None => Err(ERROR_WRONG_REQUEST),
        Some(registration_request) => {
            match valid_password_and_login(
                &registration_request.login,
                &registration_request.password,
            ) {
                GetIsValidLoginAndPassword::Ok => {
                    match database
                        .find_user_by_login(registration_request.login.clone())
                        .await
                    {
                        Ok(Some(_)) => Err(ERROR_ALREADY_REGISTERED),
                        Ok(None) => {
                            match database.registration(registration_request).await {
                                Ok(true) => Ok(Status::Ok), //todo response TOKEN
                                Ok(false) => Err(ERROR_WEAK_PASSWORD),
                                Err(_) => Err(ERROR_WEAK_PASSWORD),
                            }
                        }
                        Err(_) => Err(ERROR_UNKNOWN),
                    }
                }
                GetIsValidLoginAndPassword::BadLogin => Err(ERROR_WEAK_LOGIN),
                GetIsValidLoginAndPassword::BadPassword => Err(ERROR_WEAK_PASSWORD),
            }
        }
    }
}