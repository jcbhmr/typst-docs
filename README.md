<p>
  <b>You're probably looking for <a href="https://typst.community/typst-docs/">typst.community/typst-docs</a></b>
</p>

## Development

### Add a new language

1. Run `git submodule add https://github.com/typst/typst.git $LANGUAGE_TAG` where `$LANGUAGE_TAG` is your new BCP-47 language tag to add a new patched submodule instance of Typst
2. Update [`run.js`](run.js) and [`[route].paths.ts`](./[route].paths.ts) `languageTags` list to add your new language tag
3. Create a new file in `.vitepress/config/` with the file name `${languageTag}.ts` where `${languageTag}` is your new language tagg
4. Import and add that new language configuration to `.vitepress/config/index.ts`
5. Run `npm run diff` to save any changes you make to the submodule that you added in step 1

### Edit the documentation content

You should run `npm run apply` with the submodule tree initialized and present (`git submodule update` if you didn't clone with `--recursive`) before doing any documentation editing. This `npm run apply` will use `git apply` to apply the patch files from `patches/*.patch` to the corresponding submodule. Do this before editing so that you're editing the patched version of the submodule. Then after you've finished editing the submodule use `npm run diff` to save the diff of all the changes made to the submodule backc into `patches/*.patch`. Then commit the `patches/*.patch` files.

You can run `npm run generate` with the Rust toolchain installed to compile & build the `typst-docs` crate CLI which spits out the JSON docs structure into `pages.*.json` for each known and tracked language variant. This must be done manually and does take quite a while to compile. After finishing the generation **add and commit the generated file** so other users in the future can edit the VitePress site without compiling `typst-docs` themselves.

**TL;DR:**

1. Clone the repository and `npm install`
3. `npm run apply` to apply `patches/*.patch` work from previous authors
4. Edit the submodule contents
5. `npm run generate` to generate the `pages.*.json`
6. `npm run dev` to make sure it works
7. Commit and push!

**🙌 We need you! ❤️** Translations and localization can always be improved. If you spot any errors or want to add more translations, please do!

### Edit the VitePress site

Someone should already have been nice enough to generate & commit a `pages.*.json` for a particular language. `[route].paths.ts` will generate all the per-page JSON data that is then routed to a specific component in `[route].md`. You can find these components in `.vitepress/theme/components/`. There's a different component for each `kind` of page body listed in the `pages.*.json`.

**🙌 We need you! ❤️** Some of the page types need some love to expose more of the JSON-provided information in pretty HTML. Particularily `Func.vue`.
