## Ok json files

| File                      | Url                                                                                          |
| ------------------------- | -------------------------------------------------------------------------------------------- |
| emoji_ok.json             | /v1/emoji?offset=0&limit=50&api_key=xxx&random_id=xxx                                        |
| stickers_search_ok.json   | /v1/stickers/search?q=dogs&offset=0&limit=50&rating=pg-13&lang=enf&api_key=xxx&random_id=xxx |
| stickers_trending_ok.json | /v1/stickers/trending?offset=0&limit=50&rating=pg-13f&api_key=xxx&random_id=xxx              |
| text_trending_ok.json     | /v1/text/trending?offset=0&limit=50&rating=pg-13f&api_key=xxx&random_id=xxx                  |

## Err json files

| File                     | Status | Case                                |
| ------------------------ | ------ | ----------------------------------- |
| err_no_api_key.json      | 401    | /v1/stickers/trending               |
| err_invalid_api_key.json | 403    | /v1/stickers/trending?api_key=wrong |
