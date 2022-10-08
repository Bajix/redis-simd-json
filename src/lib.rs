use napi::bindgen_prelude::*;
use napi_derive::napi;
use redis::AsyncCommands;
use redis_swapplex::get_connection;
use serde_json::Value;

#[napi]
/// Get the value at a key and return the deserialized value
pub async fn get(key: String) -> Result<Option<Value>> {
  let mut conn = get_connection();

  let bytes: Option<Vec<u8>> = conn
    .get(key)
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  if let Some(mut bytes) = bytes {
    let value: Value = simd_json::serde::from_slice(bytes.as_mut_slice())
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

    Ok(Some(value))
  } else {
    Ok(None)
  }
}

#[napi]
/// Set the value at a key
pub async fn set(key: String, value: Value) -> Result<()> {
  let bytes: Vec<u8> =
    simd_json::to_vec(&value).map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

  let mut conn = get_connection();

  let _: () = conn
    .set(key, bytes.as_slice())
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  Ok(())
}
