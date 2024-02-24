build:
	./build.rs

setup:
	rustup toolchain install nightly
	if ! command -v zola; then \
		command -v cargo-binstall || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash; \
	 	cargo binstall -y --git https://github.com/getzola/zola.git zola; \
	done

diff:
	for d in $$(git config --file .gitmodules --get-regexp path | awk '{ print $$2 }'); do \
		mkdir -p "$$(dirname "patches/$$d.patch")" \
		&& git -C "$$d" reset \
		&& git -C "$$d" add -AN \
		&& git -C "$$d" diff --binary $(GITDIFFFLAGS) > "patches/$$d.patch"; \
	done

apply:
	# TODO: Support spaces in file names
	for f in $$(find patches -type f); do \
		if [ -d "$$f" ]; then continue; fi; \
		s="$${f#patches/}" \
		&& s="$${s%.patch}" \
		&& git -C "$$s" add -AN \
		&& git -C "$$s" reset --hard \
		&& git -C "$$s" apply $(GITAPPLYFLAGS) "$$PWD/$$f"; \
	done