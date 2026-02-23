run:
	RUST_LOG=info,german_td=debug,wgpu_hal=off cargo run -p german_td_game

edit:
	RUST_LOG=info,german_td=debug,wgpu_hal=off cargo run -p german_td_editor

ci-check:
	cargo fmt --check -v && cargo clippy -v
