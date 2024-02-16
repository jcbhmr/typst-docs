SRC_LOCALES := es zh

build:
	for locale in $(SRC_LOCALES); do \
		mkdir -p "_site/$$locale"; \
		(cd "crates/typst-$$locale" && cargo run --package typst-docs -- -o "../../_site/$$locale" $(TYPSTDOCSFLAGS)); \
	done
	mkdir -p _site
	cp -f public/index.html _site

diff:
	mkdir -p patches
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset; \
		git -C "crates/typst-$$locale" add -AN; \
		git -C "crates/typst-$$locale" diff --binary $(GITDIFFFLAGS) > "patches/typst-$$locale.patch"; \
	done

apply:
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" apply $(GITAPPLYFLAGS) "../../patches/typst-$$locale.patch"; \
	done

checkout:
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" checkout '$(PATHSPEC)'; \
	done
