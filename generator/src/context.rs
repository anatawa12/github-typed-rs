use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use crate::api_tree::ApiTreeElement;
use crate::english::{as_singular, is_plural};
use crate::schema::*;

macro_rules! put_or_crash_with_path {
    ($map: expr, $key: expr, $value: expr, $path: expr) => {
        $map.insert($key, $value)
            .map(|_| panic!("duplicate element: {}", $path));
    };
}

pub(crate) struct Context {
    pub(crate) components: Components,
}

#[derive(Debug)]
pub(crate) enum FunctionParamType {
    String,
    Integer,
}

impl FunctionParamType {
    fn from(schema: &Schema) -> FunctionParamType {
        if let Some(ref type_) = schema.type_ {
            return FunctionParamType::from_str(type_)
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
    pub(crate) children: BTreeMap<String, Function>,
}

impl Context {
    pub(crate) fn transform<'a>(&'a self, elem: &'a ApiTreeElement) -> Function {
        self.transform_internal(
            elem,
            true,
            PathBuilder {
                parent: None,
                current: "THE_ROOT",
            },
        )
    }

    fn transform_internal<'a>(
        &'a self,
        elem: &'a ApiTreeElement,
        add_named_child: bool,
        path: PathBuilder,
    ) -> Function {
        let mut methods = BTreeMap::<String, Function>::new();

        if add_named_child {
            if let Some((child_name, value)) = elem.named_child.as_ref() {
                let new_path = path.child(child_name);
                put_or_crash_with_path!(
                    methods,
                    (*child_name).clone(),
                    self.build_plural_child(value, child_name, new_path, true),
                    new_path
                );
            }
        }

        for (fixed_name, child) in &elem.children {
            let named_child_added =
                self.try_add_named_child(&mut methods, &fixed_name, &child, path);
            if child.item.is_some() || !child.children.is_empty() {
                let new_path = path.child(fixed_name);
                put_or_crash_with_path!(
                    methods,
                    (*fixed_name).clone(),
                    self.transform_internal(child, !named_child_added, new_path),
                    new_path
                );
            }
        }

        Function {
            children: methods,
            params: vec![],
        }
    }

    fn try_add_named_child(
        &self,
        methods: &mut BTreeMap<String, Function>,
        fixed_name: &String,
        child: &Box<ApiTreeElement>,
        path: PathBuilder,
    ) -> bool {
        if let Some((child_name, value)) = child.named_child.as_ref() {
            if is_plural(&*fixed_name) {
                let name = as_singular(fixed_name);
                if self.try_add_named_child_with(
                    methods,
                    &name,
                    child_name,
                    value,
                    path,
                ) {
                    return true
                }
            }
            if child.item.is_none() && child.children.is_empty() {
                if self.try_add_named_child_with(
                    methods,
                    fixed_name,
                    child_name,
                    value,
                    path,
                ) {
                    return true
                }
            }
        }
        false
    }

    fn try_add_named_child_with(
        &self,
        methods: &mut BTreeMap<String, Function>,
        name: &String,
        param_name: &str,
        value: &Box<ApiTreeElement>,
        path: PathBuilder,
    ) -> bool {
        if !methods.contains_key(name) {
            let new_path = path.child(&name);
            put_or_crash_with_path!(
                methods,
                name.clone(),
                self.build_plural_child(value, param_name, new_path, true),
                new_path
            );
            return true;
        }
        false
    }

    fn build_plural_child(
        &self,
        mut child: &ApiTreeElement,
        param_name: &str,
        path: PathBuilder,
        with_children: bool,
    ) -> Function {
        let mut param_names = vec![param_name];
        if with_children {
            loop {
                if !child.children.is_empty() {
                    break;
                }
                if child.item.is_some() {
                    break;
                }
                if child.named_child.is_none() {
                    break;
                }
                let (child_name, value) = child.named_child.as_ref().unwrap();
                param_names.push(&child_name);
                child = value;
            }
        }
        let item = self.find_iterm_in(child);
        let params: Vec<_> = param_names
            .iter()
            .map(|name| self.get_path_param(item, name))
            .collect();
        let mut func = self.transform_internal(child, true, path);
        for x in params {
            func.params.push(FunctionParam {
                name: x.name.clone(),
                param_type: FunctionParamType::from(&self.resolve(x.schema.as_ref().unwrap())),
            })
        }
        func
    }

    fn find_iterm_in<'a>(&self, e: &'a ApiTreeElement) -> &'a PathItem {
        let mut finds = vec![e];
        loop {
            let cur = finds.remove(finds.len() - 1);
            if let Some(item) = &cur.item {
                return item;
            }
            if let Some((_, child)) = &cur.named_child {
                finds.push(child)
            }
            for x in cur.children.values() {
                finds.push(x)
            }
        }
    }
}

// utils
#[derive(Copy, Clone)]
struct PathBuilder<'s> {
    parent: Option<&'s PathBuilder<'s>>,
    current: &'s str,
}

impl<'s> PathBuilder<'s> {
    fn child(&'s self, name: &'s str) -> PathBuilder<'s> {
        PathBuilder {
            parent: Some(self),
            current: name,
        }
    }
}

impl<'s> Display for PathBuilder<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = Vec::<&PathBuilder>::new();
        {
            let mut cur = self;
            loop {
                list.push(cur);
                cur = if let Some(v) = cur.parent { v } else { break }
            }
        }
        let mut sep = "";
        for x in list.iter().rev() {
            f.write_str(sep)?;
            f.write_str(x.current)?;
            sep = ".";
        }
        Ok(())
    }
}

macro_rules! resolve_vec {
    ($self: expr, $expr: expr) => {
        $expr
            .as_ref()
            .unwrap_or_else(|| empty_vec_ref())
            .iter()
            .map(|x| $self.resolve(x))
    };
}

#[inline]
fn empty_vec_ref<T>() -> &'static Vec<T> {
    static EMPTY_VEC_REF: &'static Vec<std::convert::Infallible> = &std::vec::Vec::new();
    unsafe { &*(EMPTY_VEC_REF as *const _ as *const Vec<T>) }
}

impl Context {
    fn get_path_param<'a>(&'a self, item: &'a PathItem, name: &str) -> &'a Parameter {
        item.requests
            .values()
            .flat_map(|x| resolve_vec!(self, x.parameters))
            .chain(resolve_vec!(self, item.parameters))
            .filter(|x| x.in_ == "path" && x.name == name)
            .next()
            .unwrap_or_else(|| panic!("no path param for {} found", name))
    }
}

impl Context {
    fn resolve_internal<'a, T>(
        &'a self,
        map: Option<&'a ::std::collections::BTreeMap<String, MayRef<T>>>,
        prefix: &str,
        value: &'a MayRef<T>,
    ) -> &'a T {
        match value {
            MayRef::Ref(r) => {
                let map = map
                    .as_ref()
                    .unwrap_or_else(|| panic!("no map for reference {}", prefix));
                let path = &r.ref_;
                let path = path
                    .strip_prefix(prefix)
                    .unwrap_or_else(|| panic!("no prefix in {}", path));
                let value = map
                    .get(path)
                    .unwrap_or_else(|| panic!("no ref target found {}", &r.ref_));
                self.resolve_internal(Some(map), prefix, value)
            }
            MayRef::Value(v) => v,
        }
    }
}

// overload trick

trait RefResolver<T> {
    fn resolve<'a>(&'a self, value: &'a MayRef<T>) -> &'a T;
}

macro_rules! ref_resolver_impl {
    ($type: ident, $name: ident, $path: expr) => {
        impl RefResolver<$type> for Context {
            fn resolve<'a>(&'a self, value: &'a MayRef<$type>) -> &'a $type {
                self.resolve_internal(
                    self.components.$name.as_ref(),
                    concat!("#/components/", $path, "/"),
                    value,
                )
            }
        }
    };
}

ref_resolver_impl!(Callback, callbacks, "callbacks");
ref_resolver_impl!(Example, examples, "examples");
ref_resolver_impl!(Header, headers, "headers");
ref_resolver_impl!(Link, links, "links");
ref_resolver_impl!(Parameter, parameters, "parameters");
ref_resolver_impl!(RequestBody, request_bodies, "requestBodies");
ref_resolver_impl!(Response, responses, "responses");
ref_resolver_impl!(Schema, schemas, "schemas");
ref_resolver_impl!(SecurityScheme, security_schemes, "securitySchemes");
