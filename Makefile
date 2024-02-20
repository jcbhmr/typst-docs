LANGUAGE_TAGS := es zh

build:
	rm -rf _site && cp -r public _site
	for t in $(LANGUAGE_TAGS); do \
		CARGO_TARGET_DIR="$$PWD/target" make -C "$$t" build-docs; \
		mv "$$t/public" "_site/$$t"
	done

setup:
	for t in $(LANGUAGE_TAGS); do make -C "$$t" setup; done

diff:
	mkdir -p patches
	for t in $(LANGUAGE_TAGS); do \
		git -C "$$t" reset \
		&& git -C "$$t" add -AN \
		&& git -C "$$t" diff --binary $(GITDIFFFLAGS) > "patches/$$t.patch"; \
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