
COMMIT="${1:-main}"
URL_BASE="https://github.com/github/rest-api-description/raw/$COMMIT/descriptions/api.github.com"

echo "$COMMIT" > api.github.com.commit.txt
curl -L "$URL_BASE/api.github.com.json" > api.github.com.json
patch api.github.com.patched.json < api.github.com.json.patch
diff -u api.github.com.json api.github.com.patched.json \
  | perl -pe 's/^((\+\+\+|---)[^\t]*).*$/$1/' \
  > api.github.com.json.patch
