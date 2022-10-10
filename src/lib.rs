//!
//! Optimized Redis GET / MGET / SET / MSET commands utilizing SIMD JSON with NodeJS [bindings](https://www.npmjs.com/package/redis-simd-json)

use napi::bindgen_prelude::*;
use napi_derive::napi;
use redis::AsyncCommands;
use redis_swapplex::get_connection;
use serde_json::Value;

#[napi]
/// Get the deserialized value of a key
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
/// Get the deserialized values of a set of keys
pub async fn mget(keys: Vec<String>) -> Result<Vec<Option<Value>>> {
  let mut conn = get_connection();

  let data: Vec<Option<Vec<u8>>> = redis::cmd("MGET")
    .arg(&keys)
    .query_async(&mut conn)
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  data
    .into_iter()
    .map(|bytes| {
      if let Some(mut bytes) = bytes {
        let value = simd_json::serde::from_slice(bytes.as_mut_slice())
          .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

        Ok(Some(value))
      } else {
        Ok(None)
      }
    })
    .collect::<Result<Vec<Option<Value>>>>()
}

#[napi]
/// Serialize and set the value at a key
pub async fn set(key: String, value: Option<Value>) -> Result<()> {
  let value = match value {
    Some(value) => Some(
      simd_json::to_vec(&value)
        .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?,
    ),
    None => None,
  };

  let mut conn = get_connection();

  let _: () = conn
    .set(key, value)
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  Ok(())
}

#[napi]
/// Serialize and set the value at a key if the current value hasn't changed. Returns count of modified keys. Requires that [redis-cas](https://github.com/Bajix/redis-cas) is loaded on Redis
pub async fn compare_and_swap(key: String, current: Value, value: Option<Value>) -> Result<i64> {
  let current = simd_json::to_vec(&current)
    .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

  let value = match value {
    Some(value) => simd_json::to_vec(&value)
      .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?,

    None => vec![],
  };

  let mut conn = get_connection();

  let n_modified: i64 = redis::cmd("CAS")
    .arg(&[key.as_bytes(), &current, &value])
    .query_async(&mut conn)
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  Ok(n_modified)
}

#[napi]
/// Serialize and set the values of multiple keys
pub async fn mset(data: Vec<(String, Option<Value>)>) -> Result<()> {
  let data = data
    .into_iter()
    .map(|(key, value)| match value {
      Some(value) => {
        let bytes: Vec<u8> = simd_json::to_vec(&value)
          .map_err(|err| Error::new(Status::GenericFailure, err.to_string()))?;

        Ok((key, Some(bytes)))
      }
      None => Ok((key, None)),
    })
    .collect::<Result<Vec<(String, Option<Vec<u8>>)>>>()?;

  let mut conn = get_connection();

  let _: () = redis::cmd("MSET")
    .arg(&data[..])
    .query_async(&mut conn)
    .await
    .map_err(|err| Error::new(Status::GenericFailure, format!("{:?}", err)))?;

  Ok(())
}
