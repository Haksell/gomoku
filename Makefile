NAME := gomoku
SYMLINK := Gomoku

.PHONY: all test fmt lint clean fclean re loc new_to_old setup_git_hooks flamegraph

# TODO: $(NAME) rule (think about relinks and cargo)

all:
	cargo build --release
	ln -sf target/release/$(NAME) $(SYMLINK)

clean:
	cargo clean
	rm -f perf.data*
	rm -f *.svg

fclean: clean
	rm -f $(SYMLINK)

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
	sed -e 's/pub fn new/pub fn old/g' < src/heuristics/new.rs > src/heuristics/old.rs
	sed -e 's/pub fn idabp_new/pub fn idabp_old/g' < src/bots/idabp_new.rs > src/bots/idabp_old.rs

setup_git_hooks:
	@rm -rf .git/hooks
	@ln -s ../git_hooks .git/hooks

dev_install:
	cargo install flamegraph
	cargo install hyperfine

# TODO: with rayon
flamegraph:
	@CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS="-Clink-arg=-Wl,--no-rosegment" \
		cargo flamegraph -- new new -g 12

hyperfine:
	hyperfine "target/release/gomoku new new -g 4" --show-output