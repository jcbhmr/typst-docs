LANGS := zh es

all: $(LANGS)
	mkdir -p _site public
	cp -r public/* _site/
	cp docs/index.html _site/index.html

.PHONY: $(LANGS)
$(LANGS):
	rm -rf _site/$@
	mkdir -p _site/$@
	DOC_LANG=$@ DOCS_DIR="$$PWD/$@" IMAGE_PREFIX="/assets/" cargo run --manifest-path main1/Cargo.toml
	DOC_LANG=$@ deno run -A main2.ts

setup:
	curl -fsSL https://deno.land/install.sh | sh
