use actix_web::{dev::{ServiceRequest, ServiceResponse}, body::MessageBody, Error};
use actix_web_lab::middleware::Next;

use crate::structures::{Session, Base};

pub async fn authorization(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get("Authorization");
    if let Some(session) = auth.map(|c| c.to_str().unwrap()) {
        Session::find_one("token = $1", vec![session])
            .await
            .map_err(|_| actix_web::error::ErrorUnauthorized("Unauthorized."))?;
        let res = next.call(req).await?;
        return Ok(res);
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized."))
}