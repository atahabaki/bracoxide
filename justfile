alias cr := commit_ready
alias d := gen_doc

gen_doc:
	cargo doc --open

commit_ready:
	cargo fmt
	cargo test
	cargo clippy
	cargo check
