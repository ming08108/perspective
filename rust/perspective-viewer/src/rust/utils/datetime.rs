////////////////////////////////////////////////////////////////////////////////
//
// Copyright (c) 2018, the Perspective Authors.
//
// This file is part of the Perspective library, distributed under the terms
// of the Apache License 2.0.  The full license can be found in the LICENSE
// file.

use chrono::{Local, NaiveDateTime, TimeZone};
use wasm_bindgen::prelude::*;

use crate::utils::*;

fn input_value_format(x: &str) -> Result<&str, JsValue> {
    match x.len() {
        23 => Ok("%Y-%m-%dT%H:%M:%S%.3f"),
        19 => Ok("%Y-%m-%dT%H:%M:%S"),
        16 => Ok("%Y-%m-%dT%H:%M"),
        _ => Err(format!("Unknown format {}", x).into()),
    }
}

pub fn posix_to_utc_str(x: f64) -> ApiResult<String> {
    if x > 0_f64 {
        let dt = Local.timestamp_millis(x as i64);
        Ok(dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
    } else {
        Err(format!("Unknown timestamp {}", x).into())
    }
}

pub fn str_to_utc_posix(val: &str) -> Result<f64, ApiError> {
    let posix = NaiveDateTime::parse_from_str(val, input_value_format(val)?)?;
    let dt = Local.from_local_datetime(&posix).unwrap();
    Ok(dt.timestamp_millis() as f64)
}
