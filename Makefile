clean:
	cargo clean || true
	mv target $$HOME/.local/share/Trash/files/ || true