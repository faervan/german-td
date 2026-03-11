run:
	RUST_LOG=info,german_td=debug,wgpu_hal=off,cranelift_jit=warn cargo run -p german_td_game

edit:
	RUST_LOG=info,german_td=debug,wgpu_hal=off,cranelift_jit=warn cargo run -p german_td_editor

ci-check:
	cargo +nightly fmt -- --config error_on_line_overflow=true --check && cargo clippy -v
