```sh
cargo run \
  --package build-typst-docs \
  --config 'patch."https://github.com/typst/typst.git".typst.path="typst-en/crates/typst"' \
  --config 'patch."https://github.com/typst/typst.git".typst-docs.path="typst-en/crates/typst-docs"' \
  --config 'patch."https://github.com/typst/typst.git".typst-render.path="typst-en/crates/typst-render"'
```