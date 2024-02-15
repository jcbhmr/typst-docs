SRC_LOCALES := es zh

build:
	for locale in $(SRC_LOCALES); do \
		mkdir -p "_site/$$locale"; \
		(cd "create/typst-$$locale" && cargo run --package typst-docs -- -o "../../_site/$$locale"); \
	done
	mkdir -p _site
	cp -f public/index.html _site

diff:
	mkdir -p patches
	git -C crates/typst add -A
	git -C crates/typst diff --staged --binary > patches/typst.patch
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" add -A; \
		git -C "crates/typst-$$locale" diff --staged --binary > "patches/typst-$$locale.patch"; \
	done

apply:
	git -C crates/typst reset --hard
	git -C crates/typst apply ../../patches/typst.patch
	git -C crates/typst add -A
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" apply "../../patches/typst-$$locale.patch"; \
		git -C "crates/typst-$$locale" add -A; \
	done

version:
	git -C crates/typst reset --hard
	git -C crates/typst checkout 'v$(VERSION)'
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" checkout 'v$(VERSION)'; \
	done
