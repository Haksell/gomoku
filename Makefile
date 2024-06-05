clean:
	cargo clean || true
	rm -rf target || true
	mv target $$HOME/.local/share/Trash/files/ || true

loc:
	@find src -name '*.rs' | sort | xargs wc -l