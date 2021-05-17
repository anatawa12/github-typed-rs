use std::fs::File;
use std::io::stdout;
use std::io::BufReader;

use crate::api_tree::{dump_tree, parse_path, ApiTreeElement};
use crate::schema::OpenApiSchema;

mod api_tree;
mod context;
mod english;
#[allow(legacy_derive_helpers)]
#[allow(dead_code)]
mod schema;
mod type_struct;

fn main() {
    let json = File::open("schema/api.github.com.patched.json").expect("openapi-schema not found");
    let json = BufReader::new(json);

    let schema: OpenApiSchema = serde_json::from_reader(json).unwrap();

    let mut root = ApiTreeElement::default();
    for (path, item) in schema.paths {
        parse_path(&path, item.clone(), &mut root);
    }
    root.children.remove("scim");

    dump_tree(&root);

    let mut ctx = context::Context::new(schema.components.unwrap());

    ctx.transform(&root).dump_to(stdout(), "Github").unwrap();
    for (s, t) in ctx.types {
        t.dump_to(stdout(), &s).unwrap();
    }
}
