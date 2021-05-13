#[allow(legacy_derive_helpers)]
#[allow(dead_code)]
mod schema;

fn main() {
    println!("Hello, world!");
    let _: schema::OpenApiSchema = serde_json::from_str("{}").unwrap();
}
