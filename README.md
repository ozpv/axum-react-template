# Axum and react template
nothing special, just serves a vite/react app from axum using tower-http

has a couple extra features like tracing and gzip compression but that's all

## to run
```
cd frontend
npm run build
cd ../backend
cargo r
```

## info for backend 
DIST_DIR="\<dir-here\>"
SITE_ADDR="\<full-addr-here\>"

defaults:
`DIST_DIR="${CARGO_MANIFEST_DIR}/../frontend/dist"`
`SITE_ADDR="127.0.0.1:3000"`
