NAME    := gomoku
SYMLINK := Gomoku
CARGO   := cargo
PROFILE := release

ifeq ($(PROFILE),release)
	CARGO_FLAGS := --release
else
	CARGO_FLAGS :=
endif

.PHONY: all build run test fmt lint doc clean fclean re loc

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)
	ln -sf target/release/$(NAME) $(SYMLINK)

run:
	$(CARGO) run $(CARGO_FLAGS)

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

lint:
	$(CARGO) clippy -- -D warnings

doc:
	$(CARGO) doc --no-deps --open

loc:
	@find src -name '*.rs' | sort | xargs wc -l

clean:
	$(CARGO) clean

fclean: clean
	rm -f $(SYMLINK)

re: fclean all
