NAME    := gomoku
SYMLINK := Gomoku
CARGO   := cargo

.PHONY: all build test fmt lint clean fclean re loc setup_git_hooks

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)
	ln -sf target/release/$(NAME) $(SYMLINK)

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy -- -D warnings

clean:
	$(CARGO) clean

fclean: clean
	rm -f $(SYMLINK)

re: fclean all

loc:
	@find src -name '*.rs' | sort | xargs wc -l

new_to_old:
	sed -e 's/pub fn new/pub fn old/g' < src/heuristics/new.rs > src/heuristics/old.rs
	sed -e 's/pub fn idabp_new/pub fn idabp_old/g' < src/bots/idabp_new.rs > src/bots/idabp_old.rs

setup_git_hooks:
	@rm -rf .git/hooks
	@ln -s ../git_hooks .git/hooks
