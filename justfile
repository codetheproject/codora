# Codora Justfile 

serve-docs:
    miniserve docs/doc --index codora/index.html --port 8080

build-docs:
    cargo doc --no-deps --target-dir docs

build-serve-docs: build-docs serve-docs
