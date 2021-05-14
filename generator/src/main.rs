use std::fs::File;
use std::io::BufReader;
use crate::schema::OpenApiSchema;
use crate::api_tree::{dump_tree, parse_path, ApiTreeElement};

#[allow(legacy_derive_helpers)]
#[allow(dead_code)]
mod schema;
mod api_tree;

fn main() {
    let json = File::open("schema/api.github.com.patched.json")
        .expect("openapi-schema not found");
    let json = BufReader::new(json);

    let schema: OpenApiSchema = serde_json::from_reader(json).unwrap();

    let mut root = ApiTreeElement::default();
    for (path, item) in schema.paths {
        parse_path(&path, item.clone(), &mut root);
    }

    dump_tree(&root);
}
