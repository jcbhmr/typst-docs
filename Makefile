LANGUAGE_TAGS := es zh

prebuild:
	for t in $(LANGUAGE_TAGS); do \
		export CARGO_TARGET_DIR="$$PWD/target" \
		&& out_json="$$PWD/$$t/typst-docs.json" \
		&& out_dir="$$PWD/public/$$t" \
		&& (cd "$$t/typst" && cargo run -p typst-docs --features=cli -- -o "$$out_dir" > "$$out_json"); \
	done

diff:
	for d in $$(git config --file .gitmodules --get-regexp path | awk '{ print $$2 }'); do \
		mkdir -p "$$(dirname "patches/$$d.patch")" \
		&& git -C "$$d" reset \
		&& git -C "$$d" add -AN \
		&& git -C "$$d" diff --binary $(GITDIFFFLAGS) > "patches/$$d.patch"; \
	done

apply:
	shopt -s globstar \
	&& for f in patches/**; do \
		if [ -d "$$f" ]; then continue; fi; \
		s="$${f#patches/}" \
		&& s="$${s%.patch}" \
		&& git -C "$$s" add -AN \
		&& git -C "$$s" reset --hard \
		&& git -C "$$s" apply $(GITAPPLYFLAGS) "$$PWD/$$f"; \
	done