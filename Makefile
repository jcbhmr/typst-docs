LANGS := zh es

all: $(LANGS)
	mkdir -p _site public
	cp -r public/* _site/
	cp docs/index.html _site/index.html

.PHONY: $(LANGS)
$(LANGS):
	rm -rf _site/$@
	mkdir -p _site/$@
	DOC_LANG=$@ cargo run
	DOC_LANG=$@ deno run -A main2.ts

setup:
	curl -fsSL https://deno.land/install.sh | sh
