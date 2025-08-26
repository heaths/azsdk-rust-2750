# reqwest/system-proxy

The `reqwest/system-proxy` feature uses the `system-configuration` crate for macOS and `windows-registry` crate for Windows.
`reqwest` will seemingly always work for standard `HTTP_PROXY`, `HTTPS_PROXY`, and variants, so the only way to tell without
running on macOS or Windows is to look for those dependencies in `Cargo.lock`.
