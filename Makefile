NAME := gomoku
BIN := Gomoku

.PHONY: all test fmt lint clean fclean re loc new_to_old setup_git_hooks flamegraph

# TODO: $(NAME) rule (think about relinks and cargo)

all:
	cargo build --release
	cp target/release/$(NAME) $(BIN)

clean:
	cargo clean
	rm -rf .venv
	rm -f perf.data*
	rm -f *.svg

fclean: clean
	rm -f $(BIN)

re: fclean all

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo check
	cargo clippy -- -D warnings

loc:
	@find src -name '*.rs' | sort | xargs wc -l

new_to_old:
	cp coeffs/coeffs_002ms_new.rs coeffs/coeffs_002ms_old.rs
	cp coeffs/coeffs_008ms_new.rs coeffs/coeffs_008ms_old.rs
	cp coeffs/coeffs_032ms_new.rs coeffs/coeffs_032ms_old.rs
# 	sed -e 's/pub fn new/pub fn old/g' < src/heuristics/new.rs > src/heuristics/old.rs
# 	sed -e 's/pub fn idabp_new/pub fn idabp_old/g' < src/bots/idabp_new.rs > src/bots/idabp_old.rs

setup_git_hooks:
	@rm -rf .git/hooks
	@ln -s ../git_hooks .git/hooks

# TODO: with rayon
flamegraph:
	@CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-Clink-arg=-Wl,--no-rosegment" \
		cargo flamegraph -- coeffistic coeffistic -g 20
