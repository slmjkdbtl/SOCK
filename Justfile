# wengwengweng

run example="yo":
	cargo run --release --example {{example}}

doc crate:
	cargo doc -p {{crate}} --no-deps --open

loc:
	tokei

checkdep:
	cargo outdated --depth 1

test-windows example="yo":
	cargo build --release --target x86_64-pc-windows-gnu --example {{example}}

