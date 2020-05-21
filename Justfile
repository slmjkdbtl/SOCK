# wengwengweng

check:
	cargo c

run example="raw":
	cargo run \
		--example {{example}}

build-web example="raw":
	cargo build \
		--example {{example}} \
		--release \
		--target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/examples/{{example}}.wasm \
		--out-dir target/wasm32-unknown-unknown/release/examples/ \
		--target web \
		--no-typescript

run-web:
	miniserve . \
		--index examples/web/index.html

run-site:
	cd site; \
		now dev

deploy-site:
	cd site; \
		now --prod

# TODO
site-examples:

test:
	cargo test --tests

build:
	cargo build

doc crate="dirty":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update
	cargo outdated --root-deps-only

loc:
	loc

