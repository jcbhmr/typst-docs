SRC_LOCALES := es zh

build:
	for locale in $(SRC_LOCALES); do \
		(cd "$$locale" && cargo run --package typst-docs -- -o ); \
	done
	mkdir -p _site
	cp -f public/index.html _site

diff:
	mkdir -p patches
	git -C crates/typst add -AN
	git -C crates/typst diff > patches/typst.patch
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" add -AN; \
		git -C "crates/typst-$$locale" diff > "patches/typst-$$locale.patch"; \
	done

apply:
	git -C crates/typst reset --hard
	git -C crates/typst apply ../../patches/typst.patch
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" apply "../../patches/typst-$$locale.patch"; \
	done

version:
	git -C crates/typst reset --hard
	git -C crates/typst checkout 'v$(VERSION)'
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard; \
		git -C "crates/typst-$$locale" checkout 'v$(VERSION)'; \
	done
