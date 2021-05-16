use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use crate::api_tree::ApiTreeElement;
use crate::english::{as_singular, is_plural, pascalize};
use crate::schema::*;
use crate::type_struct::*;

macro_rules! put_or_crash_with_path {
    ($map: expr, $key: expr, $value: expr, $path: expr $(,)?) => {
        $map.insert($key, $value)
            .map(|_| panic!("duplicate element: {}", $path));
    };
}

pub(crate) struct Context {
    pub(crate) components: Components,
    pub(crate) types: BTreeMap<String, Type>,
}

impl Context {
    pub(crate) fn new(components: Components) -> Self {
        Self {
            components,
            types: BTreeMap::new(),
        }
    }

    pub(crate) fn transform<'a>(&'a mut self, elem: &'a ApiTreeElement) -> Type {
        self.transform_internal(
            elem,
            true,
            PathBuilder {
                parent: None,
                current: "THE_ROOT",
            },
            "",
        )
    }

    fn transform_internal<'a>(
        &'a mut self,
        elem: &'a ApiTreeElement,
        add_named_child: bool,
        path: PathBuilder,
        type_name: &str,
    ) -> Type {
        let mut methods = BTreeMap::<String, Function>::new();

        if add_named_child {
            if let Some((child_name, value)) = elem.named_child.as_ref() {
                self.add_named_child_with(
                    &mut methods,
                    child_name,
                    child_name,
                    value,
                    path,
                    type_name,
                );
            }
        }

        for (fixed_name, child) in &elem.children {
            let named_child_added =
                self.try_add_named_child(&mut methods, &fixed_name, &child, path, type_name);
            if child.item.is_some() || !child.children.is_empty() {
                let new_path = path.child(fixed_name);
                let new_type_name = format!("{}{}", type_name, pascalize(&fixed_name));
                let type_ =
                    self.transform_internal(child, !named_child_added, new_path, &new_type_name);
                put_or_crash_with_path!(self.types, new_type_name.clone(), type_, &new_type_name,);
                let func = Function {
                    params: vec![],
                    returns: new_type_name,
                };
                put_or_crash_with_path!(methods, (*fixed_name).clone(), func, new_path);
            }
        }

        Type { methods }
    }

    fn try_add_named_child(
        &mut self,
        methods: &mut BTreeMap<String, Function>,
        fixed_name: &String,
        child: &Box<ApiTreeElement>,
        path: PathBuilder,
        type_name: &str,
    ) -> bool {
        if let Some((child_name, value)) = child.named_child.as_ref() {
            if is_plural(&*fixed_name) {
                let name = as_singular(fixed_name);
                if self.try_add_named_child_with(methods, &name, child_name, value, path, type_name)
                {
                    return true;
                }
            }
            if child.item.is_none() && child.children.is_empty() {
                if self.try_add_named_child_with(
                    methods, fixed_name, child_name, value, path, type_name,
                ) {
                    return true;
                }
            }
        }
        false
    }

    fn add_named_child_with(
        &mut self,
        methods: &mut BTreeMap<String, Function>,
        name: &String,
        param_name: &str,
        value: &Box<ApiTreeElement>,
        path: PathBuilder,
        type_name: &str,
    ) {
        let new_path = path.child(&name);
        let new_type_name = format!("{}{}", type_name, pascalize(name));
        put_or_crash_with_path!(
            methods,
            name.clone(),
            self.build_plural_child(value, param_name, new_path, true, new_type_name),
            new_path
        );
    }

    fn try_add_named_child_with(
        &mut self,
        methods: &mut BTreeMap<String, Function>,
        name: &String,
        param_name: &str,
        value: &Box<ApiTreeElement>,
        path: PathBuilder,
        type_name: &str,
    ) -> bool {
        if !methods.contains_key(name) {
            self.add_named_child_with(methods, name, param_name, value, path, type_name);
            return true;
        }
        false
    }

    fn build_plural_child(
        &mut self,
        mut child: &ApiTreeElement,
        param_name: &str,
        path: PathBuilder,
        with_children: bool,
        new_type_name: String,
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
            .map(|x| FunctionParam {
                name: x.name.clone(),
                param_type: FunctionParamType::from(&self.resolve(x.schema.as_ref().unwrap())),
            })
            .collect();
        let type_ = self.transform_internal(child, true, path, &new_type_name);
        put_or_crash_with_path!(self.types, new_type_name.clone(), type_, &new_type_name);
        Function {
            params,
            returns: new_type_name.clone(),
        }
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
