use std::collections::BTreeMap;
use std::fmt;
use std::io;

use crate::schema::Schema;

#[derive(Debug)]
pub(crate) enum FunctionParamType {
    String,
    Integer,
}

impl fmt::Display for FunctionParamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionParamType::String => f.write_str("string"),
            FunctionParamType::Integer => f.write_str("integer"),
        }
    }
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

impl Type {
    pub(crate) fn dump_to<W: io::Write>(&self, mut out: W, type_name: &str) -> io::Result<()> {
        writeln!(out, "impl {} {{", type_name)?;
        for (name, func) in &self.methods {
            write!(out, "    fn {}(", name)?;
            for p in &func.params {
                write!(out, "{}: {}, ", &p.name, p.param_type)?;
            }
            writeln!(out, ") -> {};", func.returns)?;
        }
        writeln!(out, "}}")?;
        Ok(())
    }
}
