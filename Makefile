SRC_LANGS := es zh

build:
	for lang in $(SRC_LANGS); do \
		$(MAKE) build-lang "LANG_CODE=$$lang"; \
	done
	# rsync -av public/ _site/
	cp -f public/index.html _site

build-lang:
	cd build-typst-docs \
	&& OUT_DIR='../_site/$(LANG_CODE)' \
		BASE_URL="$$BASE_URL/$(LANG_CODE)" \
		FIND_AND_REPLACE_MAP='../$(LANG_CODE)/i18n.yml' \
		cargo run \
		--config 'patch."https://github.com/typst/typst.git".typst.path="../typst-$(LANG_CODE)/crates/typst"' \
		--config 'patch."https://github.com/typst/typst.git".typst-docs.path="../typst-$(LANG_CODE)/crates/typst-docs"' \
		--config 'patch."https://github.com/typst/typst.git".typst-render.path="../typst-$(LANG_CODE)/crates/typst-render"'

diff:
	mkdir -p patches
	for lang in $(SRC_LANGS); do \
		git -C "typst-$$lang" diff > "patches/typst-$$lang.patch"; \
	done

apply:
	for lang in $(SRC_LANGS); do \
		git -C "typst-$$lang" apply "../patches/typst-$$lang.patch"; \
	done

version:
	for lang in $(SRC_LANGS); do \
		git -C "typst-$$lang" checkout 'v$(VERSION)'; \
	done
