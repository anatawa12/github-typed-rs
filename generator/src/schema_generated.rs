// THIS IS GENERATED BY build.rs FROM openapi-schema.json
// WITH schemafy AND rustfmt. DO NOT EDIT BY HAND.
#[serde(rename = "APIKeySecurityScheme")]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ApikeySecurityScheme {
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub in_: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AuthorizationCodeOAuthFlow {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: Option<::std::collections::BTreeMap<String, String>>,
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
}
pub type Callback = ::std::collections::BTreeMap<String, PathItem>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ClientCredentialsFlow {
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: Option<::std::collections::BTreeMap<String, String>>,
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Components {
    pub callbacks: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub examples: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub headers: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub links: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub parameters: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "requestBodies")]
    pub request_bodies: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub responses: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub schemas: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Contact {
    pub email: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Discriminator {
    pub mapping: Option<::std::collections::BTreeMap<String, String>>,
    #[serde(rename = "propertyName")]
    pub property_name: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Encoding {
    #[serde(rename = "allowReserved")]
    pub allow_reserved: Option<bool>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub explode: Option<bool>,
    pub headers: Option<::std::collections::BTreeMap<String, Header>>,
    pub style: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Example {
    pub description: Option<String>,
    #[serde(rename = "externalValue")]
    pub external_value: Option<String>,
    pub summary: Option<String>,
    pub value: Option<serde_json::Value>,
}
#[doc = " Example and examples are mutually exclusive"]
pub type ExampleXORExamples = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}
#[serde(rename = "HTTPSecurityScheme")]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct HttpsecurityScheme {
    #[serde(rename = "bearerFormat")]
    pub bearer_format: Option<String>,
    pub description: Option<String>,
    pub scheme: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Header {}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ImplicitOAuthFlow {
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: ::std::collections::BTreeMap<String, String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Info {
    pub contact: Option<Contact>,
    pub description: Option<String>,
    pub license: Option<License>,
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    pub title: String,
    pub version: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Link {
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    #[serde(rename = "operationRef")]
    pub operation_ref: Option<String>,
    pub parameters: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<serde_json::Value>,
    pub server: Option<Server>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct MediaType {}
#[serde(rename = "OAuth2SecurityScheme")]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Oauth2SecurityScheme {
    pub description: Option<String>,
    pub flows: OauthFlows,
    #[serde(rename = "type")]
    pub type_: String,
}
#[serde(rename = "OAuthFlows")]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct OauthFlows {
    #[serde(rename = "authorizationCode")]
    pub authorization_code: Option<AuthorizationCodeOAuthFlow>,
    #[serde(rename = "clientCredentials")]
    pub client_credentials: Option<ClientCredentialsFlow>,
    pub implicit: Option<ImplicitOAuthFlow>,
    pub password: Option<PasswordOAuthFlow>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OpenIdConnectSecurityScheme {
    pub description: Option<String>,
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Operation {
    pub callbacks: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub deprecated: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<serde_json::Value>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<serde_json::Value>,
    pub responses: Responses,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<Server>>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Parameter {}
#[doc = " Parameter location"]
pub type ParameterLocation = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct PasswordOAuthFlow {
    #[serde(rename = "refreshUrl")]
    pub refresh_url: Option<String>,
    pub scopes: Option<::std::collections::BTreeMap<String, String>>,
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PathItem {
    #[serde(rename = "$ref")]
    pub _ref: Option<String>,
    pub description: Option<String>,
    pub parameters: Option<Vec<serde_json::Value>>,
    pub servers: Option<Vec<Server>>,
    pub summary: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Paths {}
pub type Reference = ::std::collections::BTreeMap<String, serde_json::Value>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub content: ::std::collections::BTreeMap<String, MediaType>,
    pub description: Option<String>,
    pub required: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Response {
    pub content: Option<::std::collections::BTreeMap<String, MediaType>>,
    pub description: String,
    pub headers: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    pub links: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Responses {
    pub default: Option<serde_json::Value>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<serde_json::Value>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<serde_json::Value>>,
    #[serde(rename = "anyOf")]
    pub any_of: Option<Vec<serde_json::Value>>,
    pub default: Option<serde_json::Value>,
    pub deprecated: Option<bool>,
    pub description: Option<String>,
    pub discriminator: Option<Discriminator>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<serde_json::Value>>,
    pub example: Option<serde_json::Value>,
    #[serde(rename = "exclusiveMaximum")]
    pub exclusive_maximum: Option<bool>,
    #[serde(rename = "exclusiveMinimum")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    pub format: Option<String>,
    pub items: Option<serde_json::Value>,
    #[serde(rename = "maxItems")]
    pub max_items: Option<i64>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxProperties")]
    pub max_properties: Option<i64>,
    pub maximum: Option<f64>,
    #[serde(rename = "minItems")]
    pub min_items: Option<i64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minProperties")]
    pub min_properties: Option<i64>,
    pub minimum: Option<f64>,
    #[serde(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    pub not: Option<serde_json::Value>,
    pub nullable: Option<bool>,
    #[serde(rename = "oneOf")]
    pub one_of: Option<Vec<serde_json::Value>>,
    pub pattern: Option<String>,
    pub properties: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,
    pub required: Option<Vec<String>>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(rename = "writeOnly")]
    pub write_only: Option<bool>,
    pub xml: Option<Xml>,
}
#[doc = " Schema and content are mutually exclusive, at least one is required"]
pub type SchemaXORContent = serde_json::Value;
pub type SecurityRequirement = ::std::collections::BTreeMap<String, Vec<String>>;
pub type SecurityScheme = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Server {
    pub description: Option<String>,
    pub url: String,
    pub variables: Option<::std::collections::BTreeMap<String, ServerVariable>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ServerVariable {
    pub default: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub description: Option<String>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    pub name: String,
}
#[serde(rename = "XML")]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Xml {
    pub attribute: Option<bool>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub wrapped: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct OpenApiSchema {
    pub components: Option<Components>,
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    pub info: Info,
    pub openapi: String,
    pub paths: Paths,
    pub security: Option<Vec<SecurityRequirement>>,
    pub servers: Option<Vec<Server>>,
    pub tags: Option<Vec<Tag>>,
}
