# wengwengweng

check:
	cargo c \
		--examples \
		--all-features \
		--tests

check-web:
	cargo c \
		--all-features \
		--target wasm32-unknown-unknown

run example:
	cargo run \
		--example {{example}}

build-web example:
	cargo build \
		--example {{example}} \
		--release \
		--target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/examples/{{example}}.wasm \
		--out-dir site/examples \
		--target web \
		--no-typescript

run-site:
	cd site; \
		now dev

deploy-site:
	cd site; \
		now --prod

test:
	cargo test \
		--all-features

test-web:
	cargo test \
		--all-features \
		--target wasm32-unknown-unknown

build:
	cargo build

doc crate:
	cargo doc \
		--no-deps \
		--open \
		-p {{crate}}

build-doc:
	rm -rf target/doc
	cargo rustdoc \
		--release \
		--all-features \
		-- \
		--extend-css misc/doc.css
	rm -rf site/doc
	cp -r target/doc site/
	cp site/doc/light.css site/doc/dark.css
	cp logo.png site/doc/rust-logo.png
	convert logo.png -resize 128x128 -filter point site/doc/favicon.ico

update:
	cargo update
	cargo outdated --root-deps-only

loc:
	loc

