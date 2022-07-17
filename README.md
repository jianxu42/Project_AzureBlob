# Blob Service

Example of Azure Blob Service

Run with `cargo run`. This starts a server at http://localhost:3000 with one route:

- `POST /api/putblob` - you can upload a file here

Example upload request:

```bash
curl --location --request POST 'http://localhost:3000/api/uploadfile' \
--header 'Content-Type: multipart/form-data' \
--form 'file=@/home/somewhere/picture.png'
```