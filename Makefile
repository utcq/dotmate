all:
	clear
	cargo build

test:
	clear
	cargo build
	if [ $$? -eq 0 ]; then \
			clear; \
			./target/debug/dotmate $(ARGS); \
	else \
			echo "Build failed!"; \
	fi
