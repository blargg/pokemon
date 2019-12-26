use crate::moves::*;
use serde_json::{Value};

pub fn tm_array(json: &Value) -> Option<Vec<TM>> {
    if let Value::Array(arr) = json {
        arr.iter().map(|v| {
            let num = v.as_u64()?;
            Some(TM::new(num))
        })
        .collect()
    } else {
        None
    }
}

pub fn tr_array(json: &Value) -> Option<Vec<TR>> {
    if let Value::Array(arr) = json {
        arr.iter().map(|v| {
            let num = v.as_u64()?;
            Some(TR::new(num))
        })
        .collect()
    } else {
        None
    }
}

pub fn read_lvl_moves(json: &Value) -> Option<Vec<(u64, String)>> {
    if let Value::Array(arr) = json {
        arr
            .iter()
            .map(|v| read_lvl_pair(v))
            .collect::<Option<Vec<_>>>()
    } else {
        None
    }
}

pub fn read_lvl_pair(json: &Value) -> Option<(u64, String)> {
    if let Value::Array(arr) = json {
        let level = arr[0].as_u64()?;
        let name = if let Value::String(s) = &arr[1] {
            Some(s.to_string())
        } else {
            None
        }?;

        Some((level, name))
    } else {
        None
    }
}

pub fn read_items(json: &Value) -> Option<Vec<(u64, String)>> {
    if let Value::Array(arr) = json {
        arr
            .iter()
            .map(|v| read_pair(v))
            .collect::<Option<Vec<_>>>()
    } else {
        None
    }
}

fn read_pair(json: &Value) -> Option<(u64, String)> {
    if let Value::Array(arr) = json {
        let prob = arr[1].as_u64()?;
        let item = if let Value::String(s) = &arr[0] {
            Some(s.to_string())
        } else {
            None
        }?;
        Some((prob, item))
    } else {
        None
    }
}

pub fn str_vec<S: ToString>(json: &Value, err_msg: S) -> Result<Vec<String>, String> {
    let arr = array(json, err_msg.to_string())?;
    arr
        .iter()
        .map(|v| string(v, err_msg.to_string()))
        .collect::<Result<Vec<String>, String>>()
}

pub fn parse_dex(json: &Value) -> Result<Option<u32>, String> {
    if let Value::String(dex_string) = json {
        if dex_string == "foreign" {
            Ok(None)
        } else {
            let num = dex_string.parse::<u32>()
                .map_err(|_| "galar_dex".to_string())?;
            Ok(Some(num))
        }
    } else {
        err("galar_dex")
    }
}

fn array<S: ToString>(json: &Value, error_msg: S) -> Result<Vec<Value>, String> {
    if let Value::Array(res) = json {
        Ok(res.to_vec())
    } else {
        Err(error_msg.to_string())
    }
}

pub fn string<S: ToString>(json: &Value, error_msg: S) -> Result<String, String> {
    if let Value::String(res) = json {
        Ok(res.to_string())
    } else {
        Err(error_msg.to_string())
    }
}

pub fn i64_json<S: ToString>(json: &Value, error_msg: S) -> Result<i64, String> {
    if let Value::Number(num) = json {
        num.as_i64().ok_or(error_msg.to_string())
    } else {
        Err(error_msg.to_string())
    }
}

pub fn u64_json(json: &Value) -> Option<u64> {
    if let Value::Number(num) = json {
        num.as_u64()
    } else {
        None
    }
}

pub fn f64_json(json: &Value) -> Option<f64> {
    if let Value::Number(num) = json {
        num.as_f64()
    } else {
        None
    }
}

fn err<A, S: ToString>(s: S) -> Result<A, String> {
    Err(s.to_string())
}
