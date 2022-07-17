# Blob Service

Example of Azure Blob Service

Run with `cargo run`. This starts a server at http://localhost:3000 with one route:

- `POST /api/putblob` - you can upload a file here

Example upload request:

```bash
curl --location --request POST 'https://*.azurewebsites.net/api/putblob' \
--header 'x-ms-blob-type: BlockBlob' \
--header 'x-ms-blob-account: jianxu20220330' \
--header 'x-ms-blob-sv: ?sv=2021-06-08&ss=bfqt&srt=sco&sp=rwdlacupitfx&se=2022-01-06T15:57:23Z&st=2022-12-09T07:57:23Z&spr=https&sig=*---*' \
--header 'x-ms-blob-container: file' \
--form '=@"/C:/Users/Test/Downloads/test.txt"'
```

- `GET /api/getblob` - you can upload a file here
