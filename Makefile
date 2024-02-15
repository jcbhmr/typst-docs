SRC_LOCALES := es zh
# TYPST_BASE_REF := v0.10.0
TYPST_BASE_REF := 79e37ccbac080212dc42e996d760664c75d1a56f

build:
	for locale in $(SRC_LOCALES); do \
		mkdir -p "_site/$$locale"; \
		(cd "create/typst-$$locale" && cargo run --package typst-docs -- -o "../../_site/$$locale"); \
	done
	mkdir -p _site
	cp -f public/index.html _site

diff:
	mkdir -p patches
	git -C crates/typst diff '$(TYPST_BASE_REF)' --binary > patches/typst.patch
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" diff '$(TYPST_BASE_REF)' --binary > "patches/typst-$$locale.patch"; \
	done

apply:
	git -C crates/typst reset --hard '$(TYPST_BASE_REF)'
	git -C crates/typst apply --allow-empty ../../patches/typst.patch
	git -C crates/typst add -A && git -C crates/typst commit --allow-empty -m 'Apply patch'
	for locale in $(SRC_LOCALES); do \
		git -C "crates/typst-$$locale" reset --hard '$(TYPST_BASE_REF)'; \
		git -C "crates/typst-$$locale" apply --allow-empty "../../patches/typst-$$locale.patch"; \
		git -C "crates/typst-$$locale" add -A && git -C "crates/typst-$$locale" commit --allow-empty -m 'Apply patch'; \
	done