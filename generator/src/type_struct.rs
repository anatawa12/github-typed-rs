use crate::schema::Schema;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) enum FunctionParamType {
    String,
    Integer,
}

impl FunctionParamType {
    pub(crate) fn from(schema: &Schema) -> FunctionParamType {
        if let Some(ref type_) = schema.type_ {
            return FunctionParamType::from_str(type_);
        } else if let Some(_) = schema.one_of {
            return FunctionParamType::String;
        }
        panic!("invalid type: {:?}", schema);
    }

    fn from_str(name: &str) -> FunctionParamType {
        match name {
            "string" => FunctionParamType::String,
            "integer" => FunctionParamType::Integer,
            _ => panic!("invalid type: {}", name),
        }
    }
}

#[derive(Debug)]
pub(crate) struct FunctionParam {
    pub(crate) name: String,
    pub(crate) param_type: FunctionParamType,
}

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) params: Vec<FunctionParam>,
    pub(crate) returns: String,
}

#[derive(Debug)]
pub(crate) struct Type {
    pub(crate) methods: BTreeMap<String, Function>,
}
