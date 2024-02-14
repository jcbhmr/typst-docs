```sh
cargo run \
  --package build-typst-docs \
  --config 'patch."https://github.com/typst/typst.git".typst.path="my-typst/crates/typst"' \
  --config 'patch."https://github.com/typst/typst.git".typst-docs.path="my-typst/crates/typst-docs"'
```