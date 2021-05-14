use std::cmp::Ordering;
use std::collections::BTreeMap;

pub use debugging::dump_tree;

use crate::schema::PathItem;

#[derive(Default, Clone)]
pub struct ApiTreeElement {
    item: Option<PathItem>,
    children: BTreeMap<PathElementName, Box<ApiTreeElement>>,
}

#[derive(Eq, PartialEq, Clone)]
pub enum PathElementName {
    Fixed(String),
    Named(String),
}

impl Ord for PathElementName {
    fn cmp(&self, other: &Self) -> Ordering {
        use PathElementName::*;
        match (self, other) {
            (Fixed(self_name), Fixed(other_name)) => self_name.cmp(other_name),
            (Named(self_name), Named(other_name)) => self_name.cmp(other_name),

            (Named(_), Fixed(_)) => Ordering::Less,
            (Fixed(_), Named(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for PathElementName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ToString for PathElementName {
    fn to_string(&self) -> String {
        match self {
            PathElementName::Fixed(name) => format!("{}", name),
            PathElementName::Named(name) => format!("{{{}}}", name),
        }
    }
}

pub fn parse_path(path: &str, item: PathItem, mut parent: &mut ApiTreeElement) {
    if path == "/" {
        parent.item = Some(item);
        return;
    }
    let mut iter = path.split("/");
    iter.next();
    let mut prev = match iter.next() {
        None => return,
        Some(v) => v,
    };

    loop {
        let cur = match iter.next() {
            None => break,
            Some(v) => v,
        };
        let name = parse_path_element(prev);

        if !parent.children.contains_key(&name) {
            parent
                .children
                .insert(name.clone(), Box::new(ApiTreeElement::default()));
        }
        parent = parent.children.get_mut(&name).unwrap();

        prev = cur;
    }

    let name = parse_path_element(prev);

    if !parent.children.contains_key(&name) {
        parent
            .children
            .insert(name.clone(), Box::new(ApiTreeElement::default()));
    }
    parent.children.get_mut(&name).unwrap().as_mut().item = Some(item);
}

fn parse_path_element(path_element_in: &str) -> PathElementName {
    let mut path_element = path_element_in;
    let is_named = path_element.starts_with("{");
    if is_named {
        path_element = path_element
            .strip_prefix("{")
            .and_then(|x| x.strip_suffix("}"))
            .unwrap()
    }
    if path_element.contains("{") || path_element.contains("}") {
        panic!(
            "invalid path component: {{ or }} found : {}",
            path_element_in
        );
    }
    return if is_named {
        PathElementName::Named(path_element.into())
    } else {
        PathElementName::Fixed(path_element.into())
    };
}

mod debugging {
    use super::*;

    pub fn dump_tree(tree: &ApiTreeElement) {
        dump_tree_internal(
            (
                &PathElementName::Fixed(String::new()),
                &Box::new((*tree).clone()),
            ),
            &(String::new(), String::new()),
        );
    }

    fn dump_tree_internal<'a>(
        current: (&'a PathElementName, &'a Box<ApiTreeElement>),
        (indent_name, indent_element): &(String, String),
    ) {
        print!("{}", indent_name);
        println!("/{}", current.0.to_string());

        // run methods
        if let Some(item) = &current.1.item {
            let header_indent = &format!("{} :-", indent_element);
            let header_indent_end = &if current.1.children.is_empty() {
                format!("{} `-", indent_element)
            } else {
                format!("{} +-", indent_element)
            };
            for_each_with_is_end(item.requests.iter(), |(method, operation), is_end| {
                let indent = if is_end {
                    header_indent_end
                } else {
                    header_indent
                };
                println!("{}{}: {}", indent, method, operation.description
                    .as_ref().and_then(|x| x.lines().next()).unwrap_or("no desc"));
            });
        }

        // run children
        let tree = &current.1.children;
        let child_indent = (
            format!("{} +-", indent_element),
            format!("{} | ", indent_element),
        );
        let child_indent_end = (
            format!("{} `-", indent_element),
            format!("{}   ", indent_element),
        );
        for_each_with_is_end(tree.iter(), |cur, is_end| {
            dump_tree_internal(
                cur,
                if is_end {
                    &child_indent_end
                } else {
                    &child_indent
                },
            );
        });
    }

    // util
    #[inline]
    fn for_each_with_is_end<I: Iterator, F: Fn(<I as Iterator>::Item, bool) -> ()>(
        mut iter: I,
        func: F,
    ) -> () {
        if let Some(mut prev) = iter.next() {
            loop {
                if let Some(cur) = iter.next() {
                    func(prev, false);
                    prev = cur;
                } else {
                    break;
                };
            }
            func(prev, true);
        }
    }
}
