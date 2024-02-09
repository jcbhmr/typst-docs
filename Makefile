LANGS := es zh

build:
	rm -rf _site
	mkdir _site
	for lang in $(LANGS); do \
		cd build-typst-docs; \
		cargo run \
			--config "patch.'https://github.com/typst/typst.git'.typst-cli.path='../typst-$$lang/crates/typst-cli'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-docs.path='../typst-$$lang/crates/typst-docs'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-ide.path='../typst-$$lang/crates/typst-ide'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-macros.path='../typst-$$lang/crates/typst-macros'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-pdf.path='../typst-$$lang/crates/typst-pdf'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-render.path='../typst-$$lang/crates/typst-render'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-svg.path='../typst-$$lang/crates/typst-svg'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-syntax.path='../typst-$$lang/crates/typst-syntax'" \
			--config "patch.'https://github.com/typst/typst.git'.typst-timing.path='../typst-$$lang/crates/typst-timing'" \
			--config "patch.'https://github.com/typst/typst.git'.typst.path='../typst-$$lang/crates/typst'"; \
		cd -; \
	done
	cp -Rf public/* _site