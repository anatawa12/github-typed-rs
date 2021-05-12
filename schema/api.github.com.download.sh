
COMMIT="${1:-main}"
URL_BASE="https://github.com/github/rest-api-description/raw/$COMMIT/descriptions/api.github.com"

echo "$COMMIT" > api.github.com.commit.txt
curl -L "$URL_BASE/api.github.com.yaml" > api.github.com.yaml
curl -L "$URL_BASE/api.github.com.json" > api.github.com.json
