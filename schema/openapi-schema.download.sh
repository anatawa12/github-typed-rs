
COMMIT="${1:-main}"
URL="https://github.com/OAI/OpenAPI-Specification/raw/$COMMIT/schemas/v3.0/schema.json"

echo "$COMMIT" > openapi-schema.commit.txt
curl -L "$URL" > openapi-schema.json
