# wengwengweng

run example="sprite":
	cargo run --example {{example}} --release

run-lua example="app":
	./bin/dirty examples/{{example}}.lua

install: build
	upx bin/dirty
	install -v bin/dirty $BIN

build:
	cargo build --release
	rm -rf bin/dirty
	cp target/release/dirty bin/dirty

build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

doc crate="dirty":
	cargo doc --no-deps --open -p {{crate}}

update:
	cargo update

bloat:
	cargo bloat --release --crates

loc:
	loc

checkdep:
	cargo outdated --root-deps-only

