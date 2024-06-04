clean:
	cargo clean || true
	rm -rf target || true
	mv target $$HOME/.local/share/Trash/files/ || true