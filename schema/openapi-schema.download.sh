
COMMIT="${1:-main}"
URL="https://github.com/OAI/OpenAPI-Specification/raw/$COMMIT/schemas/v3.0/schema.json"

echo "$COMMIT" > openapi-schema.commit.txt
curl -L "$URL" > openapi-schema.json
cp openapi-schema.json openapi-schema.patched.json
patch openapi-schema.patched.json < openapi-schema.json.patch
diff -u openapi-schema.json openapi-schema.patched.json \
  | perl -pe 's/^((\+\+\+|---)[^\t]*).*$/$1/' \
  > openapi-schema.json.patch
