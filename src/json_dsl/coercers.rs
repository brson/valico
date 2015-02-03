use rustc_serialize::json::{self, ToJson};

use mutable_json::MutableJson;
use super::errors;

#[allow(dead_code)]
#[derive(Copy)]
pub enum PrimitiveType {
    String, 
    I64, 
    U64,
    F64,
    Boolean,
    Null,
    Array,
    Object,
    // Reserved for future use in Rustless
    File
}

pub trait Coercer: Send + Sync {
    fn get_primitive_type(&self) -> PrimitiveType;
    fn coerce(&self, &mut json::Json, &str) -> super::DslResult<Option<json::Json>>;
}

#[derive(Copy)]
pub struct StringCoercer;

impl Coercer for StringCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::String }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_string() {
            Ok(None)
        } else if val.is_number() {
            Ok(Some(val.to_string().to_json()))
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce value to string".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct I64Coercer;

impl Coercer for I64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::I64 }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_i64() {
            return Ok(None)
        } else if val.is_u64() {
            let val = val.as_u64().unwrap();
            return Ok(Some((val as i64).to_json()));
        } else if val.is_f64() {
            let val = val.as_f64().unwrap();
            return Ok(Some((val as i64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<i64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(vec![
                    Box::new(errors::WrongType {
                        path: path.to_string(),
                        detail: "Can't coerce string value to i64".to_string()
                    })
                ])
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object value to i64".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct U64Coercer;

impl Coercer for U64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::U64 }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_u64() {
            return Ok(None)
        } else if val.is_i64() {
            let val = val.as_i64().unwrap();
            return Ok(Some((val as u64).to_json()));
        } else if val.is_f64() {
            let val = val.as_f64().unwrap();
            return Ok(Some((val as u64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<u64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(vec![
                    Box::new(errors::WrongType {
                        path: path.to_string(),
                        detail: "Can't coerce string value to u64".to_string()
                    })
                ])
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object value to u64".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct F64Coercer;

impl Coercer for F64Coercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::F64 }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_f64() {
            return Ok(None)
        } else if val.is_i64() {
            let val = val.as_i64().unwrap();
            return Ok(Some((val as f64).to_json()));
        } else if val.is_u64() {
            let val = val.as_u64().unwrap();
            return Ok(Some((val as f64).to_json()));
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            let converted: Option<f64> = val.parse();
            match converted {
                Some(num) => Ok(Some(num.to_json())),
                None => Err(vec![
                    Box::new(errors::WrongType {
                        path: path.to_string(),
                        detail: "Can't coerce string value to f64".to_string()
                    })
                ])
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object value to f64".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct BooleanCoercer;

impl Coercer for BooleanCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Boolean }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_boolean() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "true" {
                Ok(Some(true.to_json()))
            } else if val == "false" {
                Ok(Some(false.to_json()))
            } else {
                Err(vec![
                    Box::new(errors::WrongType {
                        path: path.to_string(),
                        detail: "Can't coerce this string value to boolean. Correct values are 'true' and 'false'".to_string()
                    })
                ])
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object to boolean".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct NullCoercer;

impl Coercer for NullCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Null }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_null() {
            Ok(None)
        } else if val.is_string() {
            let val = val.as_string().unwrap();
            if val == "" {
                Ok(Some(json::Json::Null))
            } else {
                Err(vec![
                    Box::new(errors::WrongType {
                        path: path.to_string(),
                        detail: "Can't coerce this string value to null. Correct value is only empty string".to_string()
                    })
                ])
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object to null".to_string()
                })
            ])
        }
    }
}

pub struct ArrayCoercer {
    sub_coercer: Option<Box<Coercer + Send + Sync>>
}

impl ArrayCoercer {
    pub fn new() -> ArrayCoercer {
        ArrayCoercer {
            sub_coercer: None
        }
    }

    pub fn of_type(sub_coercer: Box<Coercer + Send + Sync>) -> ArrayCoercer {
        ArrayCoercer {
            sub_coercer: Some(sub_coercer)
        }
    }
}

impl Coercer for ArrayCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Array }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_array() {
            let array = val.as_array_mut().unwrap();
            if self.sub_coercer.is_some() {
                let sub_coercer = self.sub_coercer.as_ref().unwrap();
                let mut errors = vec![];
                for i in range(0, array.len()) {
                    let item_path = [path, i.to_string().as_slice()].connect("/");
                    match sub_coercer.coerce(&mut array[i], item_path.as_slice()) {
                        Ok(Some(value)) => {
                            array.remove(i);
                            array.insert(i, value);
                        },
                        Ok(None) => (),
                        Err(mut err) => {
                            errors.append(&mut err)
                        }
                    }
                }

                if errors.len() == 0 {
                    Ok(None)
                } else {
                    Err(errors)
                }
            } else {
                Ok(None)
            }
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce object to array".to_string()
                })
            ])
        }
    }
}

#[derive(Copy)]
pub struct ObjectCoercer;

impl Coercer for ObjectCoercer {
    fn get_primitive_type(&self) -> PrimitiveType { PrimitiveType::Object }
    fn coerce(&self, val: &mut json::Json, path: &str) -> super::DslResult<Option<json::Json>> {
        if val.is_object() {
            Ok(None)    
        } else {
            Err(vec![
                Box::new(errors::WrongType {
                    path: path.to_string(),
                    detail: "Can't coerce non-object value to the object type".to_string()
                })
            ])
        }
    }
}