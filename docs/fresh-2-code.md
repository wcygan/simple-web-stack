Directory structure:
└── denoland-fresh/
    ├── README.md
    ├── _typos.toml
    ├── CODE_OF_CONDUCT.md
    ├── deno.json
    ├── deno.lock
    ├── LICENSE
    ├── versions.json
    ├── docs/
    │   ├── toc.ts
    │   ├── canary/
    │   │   ├── examples/
    │   │   │   └── migration-guide.md
    │   │   └── the-canary-version/
    │   │       └── index.md
    │   └── latest/
    │       ├── concepts/
    │       │   ├── ahead-of-time-builds.md
    │       │   ├── app-wrapper.md
    │       │   ├── architecture.md
    │       │   ├── data-fetching.md
    │       │   ├── deployment.md
    │       │   ├── error-pages.md
    │       │   ├── forms.md
    │       │   ├── index.md
    │       │   ├── islands.md
    │       │   ├── layouts.md
    │       │   ├── middleware.md
    │       │   ├── partials.md
    │       │   ├── plugins.md
    │       │   ├── routes.md
    │       │   ├── routing.md
    │       │   ├── server-components.md
    │       │   ├── server-configuration.md
    │       │   ├── static-files.md
    │       │   └── updating.md
    │       ├── examples/
    │       │   ├── active-links.md
    │       │   ├── authentication-with-supabase.md
    │       │   ├── changing-the-src-dir.md
    │       │   ├── client-side-components-and-libraries.md
    │       │   ├── creating-a-crud-api.md
    │       │   ├── dealing-with-cors.md
    │       │   ├── handling-complex-routes.md
    │       │   ├── index.md
    │       │   ├── init-the-server.md
    │       │   ├── migrating-to-tailwind.md
    │       │   ├── modifying-the-head.md
    │       │   ├── rendering-markdown.md
    │       │   ├── rendering-raw-html.md
    │       │   ├── setting-the-language.md
    │       │   ├── sharing-state-between-islands.md
    │       │   ├── using-csp.md
    │       │   ├── using-fresh-canary-version.md
    │       │   ├── using-twind-v1.md
    │       │   └── writing-tests.md
    │       ├── getting-started/
    │       │   ├── adding-interactivity.md
    │       │   ├── create-a-project.md
    │       │   ├── create-a-route.md
    │       │   ├── custom-handlers.md
    │       │   ├── deploy-to-production.md
    │       │   ├── dynamic-routes.md
    │       │   ├── form-submissions.md
    │       │   ├── index.md
    │       │   └── running-locally.md
    │       ├── integrations/
    │       │   └── index.md
    │       └── introduction/
    │           └── index.md
    ├── examples/
    │   ├── README.md
    │   ├── deno.json
    │   ├── LICENSE
    │   └── src/
    │       ├── app1.tsx
    │       ├── app2.tsx
    │       ├── island.tsx
    │       └── shared.tsx
    ├── init/
    │   ├── README.md
    │   ├── deno.json
    │   └── src/
    │       ├── init.ts
    │       ├── init_test.ts
    │       ├── mod.ts
    │       └── assets/
    ├── plugin-tailwindcss/
    │   ├── README.md
    │   ├── deno.json
    │   └── src/
    │       ├── compiler.ts
    │       ├── mod.ts
    │       └── types.ts
    ├── src/
    │   ├── app.ts
    │   ├── app_test.tsx
    │   ├── build_cache.ts
    │   ├── build_cache_test.ts
    │   ├── compat.ts
    │   ├── config.ts
    │   ├── config_test.ts
    │   ├── constants.ts
    │   ├── context.ts
    │   ├── context_test.tsx
    │   ├── define.ts
    │   ├── define_test.ts
    │   ├── error.ts
    │   ├── error_test.ts
    │   ├── finish_setup.tsx
    │   ├── fs.ts
    │   ├── handlers.ts
    │   ├── mod.ts
    │   ├── otel.ts
    │   ├── router.ts
    │   ├── router_test.ts
    │   ├── test_utils.ts
    │   ├── types.ts
    │   ├── utils.ts
    │   ├── utils_test.ts
    │   ├── dev/
    │   │   ├── builder.ts
    │   │   ├── builder_test.ts
    │   │   ├── dev_build_cache.ts
    │   │   ├── dev_build_cache_test.ts
    │   │   ├── esbuild.ts
    │   │   ├── file_transformer.ts
    │   │   ├── file_transformer_test.ts
    │   │   ├── mod.ts
    │   │   ├── update_check.ts
    │   │   ├── update_check_test.ts
    │   │   └── middlewares/
    │   │       ├── automatic_workspace_folders.ts
    │   │       ├── live_reload.ts
    │   │       └── error_overlay/
    │   │           ├── code_frame.tsx
    │   │           ├── middleware.tsx
    │   │           ├── middleware_test.tsx
    │   │           └── overlay.tsx
    │   ├── jsonify/
    │   │   ├── constants.ts
    │   │   ├── custom_test.ts
    │   │   ├── parse.ts
    │   │   ├── round_trip_test.ts
    │   │   ├── stringify.ts
    │   │   ├── stringify_test.ts
    │   │   └── __snapshots__/
    │   │       └── round_trip_test.ts.snap
    │   ├── middlewares/
    │   │   ├── mod.ts
    │   │   ├── mod_test.ts
    │   │   ├── static_files.ts
    │   │   ├── static_files_test.ts
    │   │   ├── trailing_slashes.ts
    │   │   └── trailing_slashes_test.ts
    │   ├── plugins/
    │   │   └── fs_routes/
    │   │       ├── mod.ts
    │   │       ├── mod_test.tsx
    │   │       ├── render_middleware.ts
    │   │       └── render_middleware_test.tsx
    │   ├── runtime/
    │   │   ├── build_id.ts
    │   │   ├── shared.ts
    │   │   ├── shared_internal.tsx
    │   │   ├── client/
    │   │   │   ├── dev.ts
    │   │   │   ├── mod.tsx
    │   │   │   ├── partials.ts
    │   │   │   ├── polyfills.ts
    │   │   │   ├── preact_hooks_client.ts
    │   │   │   └── reviver.ts
    │   │   └── server/
    │   │       └── preact_hooks.tsx
    │   └── server/
    │       └── tailwind_aot_error_page.tsx
    ├── tests/
    │   ├── active_links_test.tsx
    │   ├── islands_test.tsx
    │   ├── lorem_ipsum.txt
    │   ├── partials_test.tsx
    │   ├── precompile_test.ts
    │   ├── test_utils.tsx
    │   ├── fixture_island_groups/
    │   │   └── routes/
    │   │       ├── index.tsx
    │   │       └── foo/
    │   │           ├── index.tsx
    │   │           └── (_islands)/
    │   │               └── Foo.tsx
    │   ├── fixture_precompile/
    │   │   ├── invalid/
    │   │   │   ├── deno.json
    │   │   │   ├── dev.ts
    │   │   │   └── main.tsx
    │   │   └── valid/
    │   │       ├── deno.json
    │   │       └── main.tsx
    │   ├── fixture_update_check/
    │   │   └── mod.ts
    │   └── fixtures_islands/
    │       ├── Counter.tsx
    │       ├── CounterWithSlots.tsx
    │       ├── data.json
    │       ├── EscapeIsland.tsx
    │       ├── FnIsland.tsx
    │       ├── FragmentIsland.tsx
    │       ├── FreshAttrs.tsx
    │       ├── IslandInIsland.tsx
    │       ├── JsonIsland.tsx
    │       ├── JsxChildrenIsland.tsx
    │       ├── JsxConditional.tsx
    │       ├── JsxIsland.tsx
    │       ├── Multiple.tsx
    │       ├── NodeProcess.tsx
    │       ├── NullIsland.tsx
    │       ├── OptOutPartialLink.tsx
    │       ├── PartialInIsland.tsx
    │       ├── PassThrough.tsx
    │       └── SelfCounter.tsx
    ├── tools/
    │   ├── check_docs.ts
    │   └── release.ts
    ├── update/
    │   ├── README.md
    │   ├── deno.json
    │   └── src/
    │       ├── mod.ts
    │       ├── update.ts
    │       ├── update_test.ts
    │       └── utils.ts
    ├── www/
    │   ├── README.md
    │   ├── deno.json
    │   ├── dev.ts
    │   ├── main.ts
    │   ├── main_test.ts
    │   ├── tailwind.config.ts
    │   ├── telemetry.ts
    │   ├── components/
    │   │   ├── CodeBlock.tsx
    │   │   ├── CodeWindow.tsx
    │   │   ├── DocsSidebar.tsx
    │   │   ├── FancyLink.tsx
    │   │   ├── FeatureIcons.tsx
    │   │   ├── Footer.tsx
    │   │   ├── Header.tsx
    │   │   ├── Icons.tsx
    │   │   ├── NavigationBar.tsx
    │   │   ├── PageSection.tsx
    │   │   ├── Projects.tsx
    │   │   ├── SideBySide.tsx
    │   │   ├── WaveTank.ts
    │   │   └── homepage/
    │   │       ├── CodeExampleBox.tsx
    │   │       ├── CTA.tsx
    │   │       ├── DemoBox.tsx
    │   │       ├── DenoSection.tsx
    │   │       ├── ExampleArrow.tsx
    │   │       ├── FormsSection.tsx
    │   │       ├── Hero.tsx
    │   │       ├── IslandsSection.tsx
    │   │       ├── PartialsSection.tsx
    │   │       ├── RecipeDemo.tsx
    │   │       ├── RenderingSection.tsx
    │   │       ├── SectionHeading.tsx
    │   │       ├── Simple.tsx
    │   │       └── SocialProof.tsx
    │   ├── data/
    │   │   ├── docs.ts
    │   │   └── showcase.json
    │   ├── islands/
    │   │   ├── CopyArea.tsx
    │   │   ├── Counter.tsx
    │   │   ├── FormSubmitDemo.tsx
    │   │   ├── LemonBottom.tsx
    │   │   ├── LemonDrop.tsx
    │   │   ├── LemonTop.tsx
    │   │   ├── SearchButton.tsx
    │   │   ├── TableOfContents.tsx
    │   │   ├── ThemeToggle.tsx
    │   │   └── VersionSelect.tsx
    │   ├── routes/
    │   │   ├── _app.tsx
    │   │   ├── _error.tsx
    │   │   ├── _middleware.ts
    │   │   ├── index.tsx
    │   │   ├── raw.ts
    │   │   ├── showcase-bak.tsx
    │   │   ├── showcase.tsx
    │   │   ├── thanks.tsx
    │   │   ├── update.tsx
    │   │   ├── docs/
    │   │   │   ├── [...slug].tsx
    │   │   │   ├── _layout.tsx
    │   │   │   ├── _middleware.ts
    │   │   │   └── index.tsx
    │   │   └── recipes/
    │   │       ├── _layout.tsx
    │   │       ├── _middleware.ts
    │   │       ├── lemon-honey-tea.tsx
    │   │       ├── lemonade.tsx
    │   │       └── lemondrop.tsx
    │   ├── static/
    │   │   ├── docsearch.css
    │   │   ├── google40caa9e535ae39e9.html
    │   │   ├── markdown.css
    │   │   ├── og-image.webp
    │   │   ├── prism.css
    │   │   ├── styles.css
    │   │   ├── docs/
    │   │   ├── fonts/
    │   │   │   ├── FixelVariable.woff2
    │   │   │   └── FixelVariableItalic.woff2
    │   │   ├── gallery/
    │   │   │   └── hero-bg.webp
    │   │   ├── illustration/
    │   │   ├── logos/
    │   │   └── showcase/
    │   │       └── deco.webp
    │   └── utils/
    │       ├── markdown.ts
    │       ├── screenshot.ts
    │       └── state.ts
    └── .github/
        ├── CONTRIBUTING.md
        ├── dependabot.yml
        └── workflows/
            ├── ci.yml
            ├── deploy.yml
            ├── post_publish.yml
            └── publish.yml


Files Content:

(Files content cropped to 300k characters, download full ingest to see more)
================================================
FILE: README.md
================================================
[Documentation](#-documentation) | [Getting started](#-getting-started) |
[API Reference](https://deno.land/x/fresh?doc)

# fresh

<img align="right" src="https://fresh.deno.dev/logo.svg" height="150px" alt="The Fresh logo: a sliced lemon dripping with juice">

**Fresh** is a next generation web framework, built for speed, reliability, and
simplicity.

Some stand-out features:

- Just-in-time rendering on the edge.
- Island based client hydration for maximum interactivity.
- Zero runtime overhead: no JS is shipped to the client by default.
- No configuration necessary.
- TypeScript support out of the box.
- File-system routing à la Next.js.

## 📖 Documentation

The [documentation](https://fresh.deno.dev/docs/introduction) is available on
[fresh.deno.dev](https://fresh.deno.dev/).

## 🚀 Getting started

Install the latest [Deno CLI](https://deno.com/) version.

You can scaffold a new project by running the Fresh init script. To scaffold a
project run the following:

```sh
deno run -A -r https://fresh.deno.dev
```

Then navigate to the newly created project folder:

```
cd deno-fresh-demo
```

From within your project folder, start the development server using the
`deno task` command:

```
deno task start
```

Now open http://localhost:8000 in your browser to view the page. You make
changes to the project source code and see them reflected in your browser.

To deploy the project to the live internet, you can use
[Deno Deploy](https://deno.com/deploy):

1. Push your project to GitHub.
2. [Create a Deno Deploy project](https://dash.deno.com/new).
3. [Link](https://docs.deno.com/deploy/manual/#deploy-your-project) the Deno
   Deploy project to the **`main.ts`** file in the root of the created
   repository.
4. The project will be deployed to a public $project.deno.dev subdomain.

For a more in-depth getting started guide, visit the
[Getting Started](https://fresh.deno.dev/docs/getting-started) page in the Fresh
docs.

## Contributing

We appreciate your help! To contribute, please read our
[contributing guideline](./.github/CONTRIBUTING.md).

## Adding your project to the showcase

If you feel that your project would be helpful to other Fresh users, please
consider putting your project on the
[showcase](https://fresh.deno.dev/showcase). However, websites that are just for
promotional purposes may not be listed.

To take a screenshot, run the following command.

```sh
deno task screenshot [url] [your-app-name]
```

Then add your site to
[showcase.json](https://github.com/denoland/fresh/blob/main/www/data/showcase.json),
preferably with source code on GitHub, but not required.

## Badges

![Made with Fresh](./www/static/fresh-badge.svg)

```md
[![Made with Fresh](https://fresh.deno.dev/fresh-badge.svg)](https://fresh.deno.dev)
```

```html
<a href="https://fresh.deno.dev">
  <img
    width="197"
    height="37"
    src="https://fresh.deno.dev/fresh-badge.svg"
    alt="Made with Fresh"
  />
</a>
```

![Made with Fresh(dark)](./www/static/fresh-badge-dark.svg)

```md
[![Made with Fresh](https://fresh.deno.dev/fresh-badge-dark.svg)](https://fresh.deno.dev)
```

```html
<a href="https://fresh.deno.dev">
  <img
    width="197"
    height="37"
    src="https://fresh.deno.dev/fresh-badge-dark.svg"
    alt="Made with Fresh"
  />
</a>
```

## Hashtags

Use the following hashtags in your social media posts that reference Fresh and
as Topics in the About section of your GitHub repos that contain Fresh code. It
will assure maximum visibility for your posts and code, and promote Fresh
development ecosystem visibility.

- #denofresh
- #deno

Github repo Topics will not include the hash symbol.



================================================
FILE: _typos.toml
================================================
[files]
extend-exclude = [
  "tests/fixture_partials/routes/scroll_restoration/index.tsx",
  "www/static/fonts/FixelVariable.woff2",
  "www/static/fonts/FixelVariableItalic.woff2",
  "tests/lorem_ipsum.txt",
]

[default]
extend-ignore-identifiers-re = ["Fixel"]



================================================
FILE: CODE_OF_CONDUCT.md
================================================
# Contributor Covenant Code of Conduct

## Our Pledge

We as members, contributors, and leaders pledge to make participation in our
community a harassment-free experience for everyone, regardless of age, body
size, visible or invisible disability, ethnicity, sex characteristics, gender
identity and expression, level of experience, education, socio-economic status,
nationality, personal appearance, race, religion, or sexual identity and
orientation.

We pledge to act and interact in ways that contribute to an open, welcoming,
diverse, inclusive, and healthy community.

## Our Standards

Examples of behavior that contributes to a positive environment for our
community include:

- Demonstrating empathy and kindness toward other people
- Being respectful of differing opinions, viewpoints, and experiences
- Giving and gracefully accepting constructive feedback
- Accepting responsibility and apologizing to those affected by our mistakes,
  and learning from the experience
- Focusing on what is best not just for us as individuals, but for the overall
  community

Examples of unacceptable behavior include:

- The use of sexualized language or imagery, and sexual attention or advances of
  any kind
- Trolling, insulting or derogatory comments, and personal or political attacks
- Public or private harassment
- Publishing others' private information, such as a physical or email address,
  without their explicit permission
- Other conduct which could reasonably be considered inappropriate in a
  professional setting

## Enforcement Responsibilities

Community leaders are responsible for clarifying and enforcing our standards of
acceptable behavior and will take appropriate and fair corrective action in
response to any behavior that they deem inappropriate, threatening, offensive,
or harmful.

Community leaders have the right and responsibility to remove, edit, or reject
comments, commits, code, wiki edits, issues, and other contributions that are
not aligned to this Code of Conduct, and will communicate reasons for moderation
decisions when appropriate.

## Scope

This Code of Conduct applies within all community spaces, and also applies when
an individual is officially representing the community in public spaces.
Examples of representing our community include using an official e-mail address,
posting via an official social media account, or acting as an appointed
representative at an online or offline event.

## Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the community leaders responsible for enforcement at hello@lcas.dev.
All complaints will be reviewed and investigated promptly and fairly.

All community leaders are obligated to respect the privacy and security of the
reporter of any incident.

## Enforcement Guidelines

Community leaders will follow these Community Impact Guidelines in determining
the consequences for any action they deem in violation of this Code of Conduct:

### 1. Correction

**Community Impact**: Use of inappropriate language or other behavior deemed
unprofessional or unwelcome in the community.

**Consequence**: A private, written warning from community leaders, providing
clarity around the nature of the violation and an explanation of why the
behavior was inappropriate. A public apology may be requested.

### 2. Warning

**Community Impact**: A violation through a single incident or series of
actions.

**Consequence**: A warning with consequences for continued behavior. No
interaction with the people involved, including unsolicited interaction with
those enforcing the Code of Conduct, for a specified period of time. This
includes avoiding interactions in community spaces as well as external channels
like social media. Violating these terms may lead to a temporary or permanent
ban.

### 3. Temporary Ban

**Community Impact**: A serious violation of community standards, including
sustained inappropriate behavior.

**Consequence**: A temporary ban from any sort of interaction or public
communication with the community for a specified period of time. No public or
private interaction with the people involved, including unsolicited interaction
with those enforcing the Code of Conduct, is allowed during this period.
Violating these terms may lead to a permanent ban.

### 4. Permanent Ban

**Community Impact**: Demonstrating a pattern of violation of community
standards, including sustained inappropriate behavior, harassment of an
individual, or aggression toward or disparagement of classes of individuals.

**Consequence**: A permanent ban from any sort of public interaction within the
community.

## Attribution

This Code of Conduct is adapted from the [Contributor Covenant][homepage],
version 2.0, available at
https://www.contributor-covenant.org/version/2/0/code_of_conduct.html.

Community Impact Guidelines were inspired by
[Mozilla's code of conduct
enforcement ladder](https://github.com/mozilla/diversity).

[homepage]: https://www.contributor-covenant.org

For answers to common questions about this code of conduct, see the FAQ at
https://www.contributor-covenant.org/faq. Translations are available at
https://www.contributor-covenant.org/translations.



================================================
FILE: deno.json
================================================
{
  "workspace": [
    "./examples",
    "./init",
    "./plugin-tailwindcss",
    "./update",
    "./www"
  ],
  "name": "@fresh/core",
  "version": "2.0.0-alpha.34",
  "license": "MIT",
  "exports": {
    ".": "./src/mod.ts",
    "./runtime": "./src/runtime/shared.ts",
    "./dev": "./src/dev/mod.ts",
    "./compat": "./src/compat.ts"
  },
  "tasks": {
    "test": "deno test -A --parallel",
    "fixture": "deno run -A --watch=static/,routes/ tests/fixture/dev.ts",
    "www": "deno task --cwd=www start",
    "build-www": "deno task --cwd=www build",
    "screenshot": "deno run -A www/utils/screenshot.ts",
    "check:types": "deno check --allow-import",
    "check:docs": "deno run -A tools/check_docs.ts",
    "ok": "deno fmt --check && deno lint && deno task check:types && deno task test",
    "test:www": "deno test -A www/main_test.*",
    "release": "deno run -A tools/release.ts"
  },
  "exclude": ["**/_fresh/*", "**/tmp/*", "*/tests_OLD/**"],
  "publish": {
    "include": [
      "src/**",
      "deno.json",
      "README.md",
      "LICENSE",
      "www/static/fresh-badge.svg",
      "www/static/fresh-badge-dark.svg",
      "*.todo"
    ],
    "exclude": ["**/*_test.*", "src/__OLD/**", "*.todo"]
  },
  "imports": {
    "@deno/doc": "jsr:@deno/doc@^0.172.0",
    "@std/cli": "jsr:@std/cli@^1.0.17",
    "@std/collections": "jsr:@std/collections@^1.0.11",
    "@std/http": "jsr:@std/http@^1.0.15",
    "@std/uuid": "jsr:@std/uuid@^1.0.7",
    "fresh": "jsr:@fresh/core@^2.0.0-alpha.29",
    "preact": "npm:preact@^10.26.6",
    "preact-render-to-string": "npm:preact-render-to-string@^6.5.11",
    "$ga4": "https://raw.githubusercontent.com/denoland/ga4/main/mod.ts",
    "@luca/esbuild-deno-loader": "jsr:@luca/esbuild-deno-loader@^0.11.0",
    "@opentelemetry/api": "npm:@opentelemetry/api@^1.9.0",
    "@preact/signals": "npm:@preact/signals@^2.0.4",
    "esbuild": "npm:esbuild@0.25.4",
    "esbuild-wasm": "npm:esbuild-wasm@0.25.4",
    "@std/crypto": "jsr:@std/crypto@1",
    "@std/datetime": "jsr:@std/datetime@^0.225.2",
    "@std/encoding": "jsr:@std/encoding@1",
    "@std/fmt": "jsr:@std/fmt@^1.0.7",
    "@std/fs": "jsr:@std/fs@1",
    "@std/html": "jsr:@std/html@1",
    "@std/jsonc": "jsr:@std/jsonc@1",
    "@std/media-types": "jsr:@std/media-types@1",
    "@std/path": "jsr:@std/path@1",
    "@std/semver": "jsr:@std/semver@1",
    "@std/streams": "jsr:@std/streams@1",

    "@astral/astral": "jsr:@astral/astral@^0.5.3",
    "@marvinh-test/fresh-island": "jsr:@marvinh-test/fresh-island@^0.0.1",
    "linkedom": "npm:linkedom@^0.18.10",
    "@std/async": "jsr:@std/async@^1.0.13",
    "@std/expect": "jsr:@std/expect@^1.0.16",
    "@std/testing": "jsr:@std/testing@^1.0.12",

    "autoprefixer": "npm:autoprefixer@10.4.17",
    "cssnano": "npm:cssnano@6.0.3",
    "postcss": "npm:postcss@8.4.35",
    "tailwindcss": "npm:tailwindcss@^3.4.1",

    "ts-morph": "npm:ts-morph@^25.0.1",

    "@fresh/plugin-tailwind": "jsr:@fresh/plugin-tailwind@^0.0.1-alpha.7",
    "@std/front-matter": "jsr:@std/front-matter@^1.0.5",
    "fresh/compat": "./src/compat/mod.ts",
    "fresh/dev": "./src/dev/mod.ts",
    "fresh/runtime": "./src/runtime/shared.ts",
    "github-slugger": "npm:github-slugger@^2.0.0",
    "marked": "npm:marked@^15.0.11",
    "marked-mangle": "npm:marked-mangle@^1.1.9",
    "prismjs": "npm:prismjs@^1.29.0"
  },
  "compilerOptions": {
    "lib": ["dom", "dom.asynciterable", "deno.ns", "deno.unstable"],
    "jsx": "precompile",
    "jsxImportSource": "preact",
    "jsxPrecompileSkipElements": ["a", "img", "source", "body", "html", "head"]
  },
  "lint": {
    "rules": {
      "tags": ["recommended", "fresh", "jsr", "jsx", "react"],
      "exclude": ["no-window"],
      "include": ["no-console"]
    }
  }
}



================================================
FILE: deno.lock
================================================
{
  "version": "5",
  "specifiers": {
    "jsr:@deno/cache-dir@0.14": "0.14.0",
    "jsr:@deno/doc@0.172": "0.172.0",
    "jsr:@deno/graph@0.86": "0.86.9",
    "jsr:@deno/graph@~0.82.3": "0.82.3",
    "jsr:@deno/otel@*": "0.0.2",
    "jsr:@fresh/core@^2.0.0-alpha.29": "2.0.0-alpha.34",
    "jsr:@fresh/plugin-tailwind@^0.0.1-alpha.7": "0.0.1-alpha.7",
    "jsr:@luca/esbuild-deno-loader@0.11": "0.11.1",
    "jsr:@marvinh-test/fresh-island@*": "0.0.1",
    "jsr:@marvinh-test/fresh-island@^0.0.1": "0.0.1",
    "jsr:@std/assert@0.221": "0.221.0",
    "jsr:@std/async@^1.0.13": "1.0.13",
    "jsr:@std/bytes@^1.0.2": "1.0.6",
    "jsr:@std/bytes@^1.0.5": "1.0.6",
    "jsr:@std/cli@^1.0.17": "1.0.17",
    "jsr:@std/collections@^1.0.11": "1.1.0",
    "jsr:@std/crypto@1": "1.0.5",
    "jsr:@std/crypto@^1.0.4": "1.0.5",
    "jsr:@std/datetime@~0.225.2": "0.225.4",
    "jsr:@std/encoding@1": "1.0.10",
    "jsr:@std/encoding@^1.0.10": "1.0.10",
    "jsr:@std/encoding@^1.0.5": "1.0.10",
    "jsr:@std/fmt@1": "1.0.8",
    "jsr:@std/fmt@^1.0.3": "1.0.8",
    "jsr:@std/fmt@^1.0.7": "1.0.8",
    "jsr:@std/fmt@^1.0.8": "1.0.8",
    "jsr:@std/front-matter@^1.0.5": "1.0.9",
    "jsr:@std/fs@1": "1.0.17",
    "jsr:@std/fs@^1.0.16": "1.0.17",
    "jsr:@std/fs@^1.0.6": "1.0.17",
    "jsr:@std/html@1": "1.0.4",
    "jsr:@std/html@^1.0.4": "1.0.4",
    "jsr:@std/http@^1.0.15": "1.0.16",
    "jsr:@std/io@0.225": "0.225.2",
    "jsr:@std/json@^1.0.2": "1.0.2",
    "jsr:@std/jsonc@1": "1.0.2",
    "jsr:@std/media-types@1": "1.1.0",
    "jsr:@std/media-types@^1.1.0": "1.1.0",
    "jsr:@std/net@^1.0.4": "1.0.4",
    "jsr:@std/path@0.221": "0.221.0",
    "jsr:@std/path@1": "1.0.9",
    "jsr:@std/path@^1.0.6": "1.0.9",
    "jsr:@std/path@^1.0.8": "1.0.9",
    "jsr:@std/path@^1.0.9": "1.0.9",
    "jsr:@std/semver@1": "1.0.5",
    "jsr:@std/streams@1": "1.0.9",
    "jsr:@std/streams@^1.0.9": "1.0.9",
    "jsr:@std/toml@^1.0.3": "1.0.5",
    "jsr:@std/uuid@^1.0.7": "1.0.7",
    "jsr:@std/yaml@^1.0.5": "1.0.6",
    "npm:@opentelemetry/api@1": "1.9.0",
    "npm:@opentelemetry/api@^1.9.0": "1.9.0",
    "npm:@opentelemetry/sdk-trace-base@1": "1.30.1_@opentelemetry+api@1.9.0",
    "npm:@preact/signals@^1.2.3": "1.3.2_preact@10.26.6",
    "npm:@preact/signals@^2.0.4": "2.0.4_preact@10.26.6",
    "npm:@types/node@*": "22.15.15",
    "npm:autoprefixer@10.4.17": "10.4.17_postcss@8.4.35",
    "npm:cssnano@6.0.3": "6.0.3_postcss@8.4.35",
    "npm:esbuild-wasm@0.23.1": "0.23.1",
    "npm:esbuild-wasm@0.25.4": "0.25.4",
    "npm:esbuild@0.23.1": "0.23.1",
    "npm:esbuild@0.25.4": "0.25.4",
    "npm:github-slugger@2": "2.0.0",
    "npm:linkedom@~0.18.10": "0.18.10",
    "npm:marked-mangle@^1.1.9": "1.1.10_marked@15.0.12",
    "npm:marked@^15.0.11": "15.0.12",
    "npm:postcss@8.4.35": "8.4.35",
    "npm:preact-render-to-string@^6.5.11": "6.5.13_preact@10.26.6",
    "npm:preact@^10.22.0": "10.26.6",
    "npm:preact@^10.26.6": "10.26.6",
    "npm:prismjs@^1.29.0": "1.30.0",
    "npm:tailwindcss@^3.4.1": "3.4.17_postcss@8.5.3",
    "npm:ts-morph@^25.0.1": "25.0.1"
  },
  "jsr": {
    "@deno/cache-dir@0.14.0": {
      "integrity": "729f0b68e7fc96443c09c2c544b830ca70897bdd5168598446d752f7a4c731ad",
      "dependencies": [
        "jsr:@deno/graph@0.86",
        "jsr:@std/fmt@^1.0.3",
        "jsr:@std/fs@^1.0.6",
        "jsr:@std/io",
        "jsr:@std/path@^1.0.8"
      ]
    },
    "@deno/doc@0.172.0": {
      "integrity": "72a68ed533576a06feb930a84784ad9ba6d83ca9d581fc734d498c58e32b7cf5",
      "dependencies": [
        "jsr:@deno/cache-dir",
        "jsr:@deno/graph@~0.82.3"
      ]
    },
    "@deno/graph@0.82.3": {
      "integrity": "5c1fe944368172a9c87588ac81b82eb027ca78002a57521567e6264be322637e"
    },
    "@deno/graph@0.86.9": {
      "integrity": "c4f353a695bcc5246c099602977dabc6534eacea9999a35a8cb24e807192e6a1"
    },
    "@deno/otel@0.0.2": {
      "integrity": "4ef61b7eb1c4063f8224d66fc43f25e428a566d2e18785d0dc67bb70a318f0ff",
      "dependencies": [
        "npm:@opentelemetry/api@1",
        "npm:@opentelemetry/sdk-trace-base"
      ]
    },
    "@fresh/core@2.0.0-alpha.34": {
      "integrity": "e177fc69b049b04128de87d243bd7de76582417d80d8d12dc19dd6786f196efa",
      "dependencies": [
        "jsr:@luca/esbuild-deno-loader",
        "jsr:@std/crypto@1",
        "jsr:@std/datetime",
        "jsr:@std/encoding@1",
        "jsr:@std/fmt@1",
        "jsr:@std/fs@1",
        "jsr:@std/html@1",
        "jsr:@std/http",
        "jsr:@std/jsonc",
        "jsr:@std/media-types@1",
        "jsr:@std/path@1",
        "jsr:@std/semver",
        "npm:@opentelemetry/api@^1.9.0",
        "npm:@preact/signals@^2.0.4",
        "npm:esbuild-wasm@0.23.1",
        "npm:esbuild@0.23.1",
        "npm:preact-render-to-string",
        "npm:preact@^10.26.6"
      ]
    },
    "@fresh/plugin-tailwind@0.0.1-alpha.7": {
      "integrity": "b940991bdb76f0995dc58b25183f1001d72c4020e049d384ad3fb751556aa2a9",
      "dependencies": [
        "jsr:@std/path@0.221",
        "npm:autoprefixer",
        "npm:cssnano",
        "npm:postcss",
        "npm:tailwindcss"
      ]
    },
    "@luca/esbuild-deno-loader@0.11.1": {
      "integrity": "dc020d16d75b591f679f6b9288b10f38bdb4f24345edb2f5732affa1d9885267",
      "dependencies": [
        "jsr:@std/bytes@^1.0.2",
        "jsr:@std/encoding@^1.0.5",
        "jsr:@std/path@^1.0.6"
      ]
    },
    "@marvinh-test/fresh-island@0.0.1": {
      "integrity": "890f2595e60b1aaeaa8d73c6ad2c1247d4c5b895387df230f7f3b2a4da29b585",
      "dependencies": [
        "npm:@preact/signals@^1.2.3",
        "npm:preact@^10.22.0",
        "npm:preact@^10.26.6"
      ]
    },
    "@std/assert@0.221.0": {
      "integrity": "a5f1aa6e7909dbea271754fd4ab3f4e687aeff4873b4cef9a320af813adb489a"
    },
    "@std/async@1.0.13": {
      "integrity": "1d76ca5d324aef249908f7f7fe0d39aaf53198e5420604a59ab5c035adc97c96"
    },
    "@std/bytes@1.0.6": {
      "integrity": "f6ac6adbd8ccd99314045f5703e23af0a68d7f7e58364b47d2c7f408aeb5820a"
    },
    "@std/cli@1.0.17": {
      "integrity": "e15b9abe629e17be90cc6216327f03a29eae613365f1353837fa749aad29ce7b"
    },
    "@std/collections@1.1.0": {
      "integrity": "2ee8761c84c3d203f7a4ecd376f9ca88a0c559817a4a54c9150f28c0b948027c"
    },
    "@std/crypto@1.0.5": {
      "integrity": "0dcfbb319fe0bba1bd3af904ceb4f948cde1b92979ec1614528380ed308a3b40"
    },
    "@std/datetime@0.225.4": {
      "integrity": "682bc21738b941a4ed1465be6da01704e8010a3a6d9b615de9458202b84e00ec"
    },
    "@std/encoding@1.0.10": {
      "integrity": "8783c6384a2d13abd5e9e87a7ae0520a30e9f56aeeaa3bdf910a3eaaf5c811a1"
    },
    "@std/fmt@1.0.8": {
      "integrity": "71e1fc498787e4434d213647a6e43e794af4fd393ef8f52062246e06f7e372b7"
    },
    "@std/front-matter@1.0.9": {
      "integrity": "ee6201d06674cbef137dda2252f62477450b48249e7d8d9ab57a30f85ff6f051",
      "dependencies": [
        "jsr:@std/toml",
        "jsr:@std/yaml"
      ]
    },
    "@std/fs@1.0.17": {
      "integrity": "1c00c632677c1158988ef7a004cb16137f870aafdb8163b9dce86ec652f3952b",
      "dependencies": [
        "jsr:@std/path@^1.0.9"
      ]
    },
    "@std/html@1.0.4": {
      "integrity": "eff3497c08164e6ada49b7f81a28b5108087033823153d065e3f89467dd3d50e"
    },
    "@std/http@1.0.16": {
      "integrity": "80c8d08c4bfcf615b89978dcefb84f7e880087cf3b6b901703936f3592a06933",
      "dependencies": [
        "jsr:@std/cli",
        "jsr:@std/encoding@^1.0.10",
        "jsr:@std/fmt@^1.0.8",
        "jsr:@std/html@^1.0.4",
        "jsr:@std/media-types@^1.1.0",
        "jsr:@std/net",
        "jsr:@std/path@^1.0.9",
        "jsr:@std/streams@^1.0.9"
      ]
    },
    "@std/io@0.225.2": {
      "integrity": "3c740cd4ee4c082e6cfc86458f47e2ab7cb353dc6234d5e9b1f91a2de5f4d6c7",
      "dependencies": [
        "jsr:@std/bytes@^1.0.5"
      ]
    },
    "@std/json@1.0.2": {
      "integrity": "d9e5497801c15fb679f55a2c01c7794ad7a5dfda4dd1bebab5e409cb5e0d34d4"
    },
    "@std/jsonc@1.0.2": {
      "integrity": "909605dae3af22bd75b1cbda8d64a32cf1fd2cf6efa3f9e224aba6d22c0f44c7",
      "dependencies": [
        "jsr:@std/json"
      ]
    },
    "@std/media-types@1.1.0": {
      "integrity": "c9d093f0c05c3512932b330e3cc1fe1d627b301db33a4c2c2185c02471d6eaa4"
    },
    "@std/net@1.0.4": {
      "integrity": "2f403b455ebbccf83d8a027d29c5a9e3a2452fea39bb2da7f2c04af09c8bc852"
    },
    "@std/path@0.221.0": {
      "integrity": "0a36f6b17314ef653a3a1649740cc8db51b25a133ecfe838f20b79a56ebe0095",
      "dependencies": [
        "jsr:@std/assert"
      ]
    },
    "@std/path@1.0.9": {
      "integrity": "260a49f11edd3db93dd38350bf9cd1b4d1366afa98e81b86167b4e3dd750129e"
    },
    "@std/semver@1.0.5": {
      "integrity": "529f79e83705714c105ad0ba55bec0f9da0f24d2f726b6cc1c15e505cc2c0624"
    },
    "@std/streams@1.0.9": {
      "integrity": "a9d26b1988cdd7aa7b1f4b51e1c36c1557f3f252880fa6cc5b9f37078b1a5035",
      "dependencies": [
        "jsr:@std/bytes@^1.0.5"
      ]
    },
    "@std/toml@1.0.5": {
      "integrity": "08061156e9c5716443a144b6e40a8668738b8b424ad99ab0b6fdf1b6ea4da806",
      "dependencies": [
        "jsr:@std/collections"
      ]
    },
    "@std/uuid@1.0.7": {
      "integrity": "6885db5cd60794049d1661b5cf06b1e1ed65b2affd054ec8b06da7d2efd421ca",
      "dependencies": [
        "jsr:@std/bytes@^1.0.5",
        "jsr:@std/crypto@^1.0.4"
      ]
    },
    "@std/yaml@1.0.6": {
      "integrity": "c9a5a914e1d51c46756cb10e356710035cfa905e713c90d3b711413fd3aead27"
    }
  },
  "npm": {
    "@alloc/quick-lru@5.2.0": {
      "integrity": "sha512-UrcABB+4bUrFABwbluTIBErXwvbsU/V7TZWfmbgJfbkwiBuziS9gxdODUyuiecfdGQ85jglMW6juS3+z5TsKLw=="
    },
    "@esbuild/aix-ppc64@0.23.1": {
      "integrity": "sha512-6VhYk1diRqrhBAqpJEdjASR/+WVRtfjpqKuNw11cLiaWpAT/Uu+nokB+UJnevzy/P9C/ty6AOe0dwueMrGh/iQ==",
      "os": ["aix"],
      "cpu": ["ppc64"]
    },
    "@esbuild/aix-ppc64@0.25.4": {
      "integrity": "sha512-1VCICWypeQKhVbE9oW/sJaAmjLxhVqacdkvPLEjwlttjfwENRSClS8EjBz0KzRyFSCPDIkuXW34Je/vk7zdB7Q==",
      "os": ["aix"],
      "cpu": ["ppc64"]
    },
    "@esbuild/android-arm64@0.23.1": {
      "integrity": "sha512-xw50ipykXcLstLeWH7WRdQuysJqejuAGPd30vd1i5zSyKK3WE+ijzHmLKxdiCMtH1pHz78rOg0BKSYOSB/2Khw==",
      "os": ["android"],
      "cpu": ["arm64"]
    },
    "@esbuild/android-arm64@0.25.4": {
      "integrity": "sha512-bBy69pgfhMGtCnwpC/x5QhfxAz/cBgQ9enbtwjf6V9lnPI/hMyT9iWpR1arm0l3kttTr4L0KSLpKmLp/ilKS9A==",
      "os": ["android"],
      "cpu": ["arm64"]
    },
    "@esbuild/android-arm@0.23.1": {
      "integrity": "sha512-uz6/tEy2IFm9RYOyvKl88zdzZfwEfKZmnX9Cj1BHjeSGNuGLuMD1kR8y5bteYmwqKm1tj8m4cb/aKEorr6fHWQ==",
      "os": ["android"],
      "cpu": ["arm"]
    },
    "@esbuild/android-arm@0.25.4": {
      "integrity": "sha512-QNdQEps7DfFwE3hXiU4BZeOV68HHzYwGd0Nthhd3uCkkEKK7/R6MTgM0P7H7FAs5pU/DIWsviMmEGxEoxIZ+ZQ==",
      "os": ["android"],
      "cpu": ["arm"]
    },
    "@esbuild/android-x64@0.23.1": {
      "integrity": "sha512-nlN9B69St9BwUoB+jkyU090bru8L0NA3yFvAd7k8dNsVH8bi9a8cUAUSEcEEgTp2z3dbEDGJGfP6VUnkQnlReg==",
      "os": ["android"],
      "cpu": ["x64"]
    },
    "@esbuild/android-x64@0.25.4": {
      "integrity": "sha512-TVhdVtQIFuVpIIR282btcGC2oGQoSfZfmBdTip2anCaVYcqWlZXGcdcKIUklfX2wj0JklNYgz39OBqh2cqXvcQ==",
      "os": ["android"],
      "cpu": ["x64"]
    },
    "@esbuild/darwin-arm64@0.23.1": {
      "integrity": "sha512-YsS2e3Wtgnw7Wq53XXBLcV6JhRsEq8hkfg91ESVadIrzr9wO6jJDMZnCQbHm1Guc5t/CdDiFSSfWP58FNuvT3Q==",
      "os": ["darwin"],
      "cpu": ["arm64"]
    },
    "@esbuild/darwin-arm64@0.25.4": {
      "integrity": "sha512-Y1giCfM4nlHDWEfSckMzeWNdQS31BQGs9/rouw6Ub91tkK79aIMTH3q9xHvzH8d0wDru5Ci0kWB8b3up/nl16g==",
      "os": ["darwin"],
      "cpu": ["arm64"]
    },
    "@esbuild/darwin-x64@0.23.1": {
      "integrity": "sha512-aClqdgTDVPSEGgoCS8QDG37Gu8yc9lTHNAQlsztQ6ENetKEO//b8y31MMu2ZaPbn4kVsIABzVLXYLhCGekGDqw==",
      "os": ["darwin"],
      "cpu": ["x64"]
    },
    "@esbuild/darwin-x64@0.25.4": {
      "integrity": "sha512-CJsry8ZGM5VFVeyUYB3cdKpd/H69PYez4eJh1W/t38vzutdjEjtP7hB6eLKBoOdxcAlCtEYHzQ/PJ/oU9I4u0A==",
      "os": ["darwin"],
      "cpu": ["x64"]
    },
    "@esbuild/freebsd-arm64@0.23.1": {
      "integrity": "sha512-h1k6yS8/pN/NHlMl5+v4XPfikhJulk4G+tKGFIOwURBSFzE8bixw1ebjluLOjfwtLqY0kewfjLSrO6tN2MgIhA==",
      "os": ["freebsd"],
      "cpu": ["arm64"]
    },
    "@esbuild/freebsd-arm64@0.25.4": {
      "integrity": "sha512-yYq+39NlTRzU2XmoPW4l5Ifpl9fqSk0nAJYM/V/WUGPEFfek1epLHJIkTQM6bBs1swApjO5nWgvr843g6TjxuQ==",
      "os": ["freebsd"],
      "cpu": ["arm64"]
    },
    "@esbuild/freebsd-x64@0.23.1": {
      "integrity": "sha512-lK1eJeyk1ZX8UklqFd/3A60UuZ/6UVfGT2LuGo3Wp4/z7eRTRYY+0xOu2kpClP+vMTi9wKOfXi2vjUpO1Ro76g==",
      "os": ["freebsd"],
      "cpu": ["x64"]
    },
    "@esbuild/freebsd-x64@0.25.4": {
      "integrity": "sha512-0FgvOJ6UUMflsHSPLzdfDnnBBVoCDtBTVyn/MrWloUNvq/5SFmh13l3dvgRPkDihRxb77Y17MbqbCAa2strMQQ==",
      "os": ["freebsd"],
      "cpu": ["x64"]
    },
    "@esbuild/linux-arm64@0.23.1": {
      "integrity": "sha512-/93bf2yxencYDnItMYV/v116zff6UyTjo4EtEQjUBeGiVpMmffDNUyD9UN2zV+V3LRV3/on4xdZ26NKzn6754g==",
      "os": ["linux"],
      "cpu": ["arm64"]
    },
    "@esbuild/linux-arm64@0.25.4": {
      "integrity": "sha512-+89UsQTfXdmjIvZS6nUnOOLoXnkUTB9hR5QAeLrQdzOSWZvNSAXAtcRDHWtqAUtAmv7ZM1WPOOeSxDzzzMogiQ==",
      "os": ["linux"],
      "cpu": ["arm64"]
    },
    "@esbuild/linux-arm@0.23.1": {
      "integrity": "sha512-CXXkzgn+dXAPs3WBwE+Kvnrf4WECwBdfjfeYHpMeVxWE0EceB6vhWGShs6wi0IYEqMSIzdOF1XjQ/Mkm5d7ZdQ==",
      "os": ["linux"],
      "cpu": ["arm"]
    },
    "@esbuild/linux-arm@0.25.4": {
      "integrity": "sha512-kro4c0P85GMfFYqW4TWOpvmF8rFShbWGnrLqlzp4X1TNWjRY3JMYUfDCtOxPKOIY8B0WC8HN51hGP4I4hz4AaQ==",
      "os": ["linux"],
      "cpu": ["arm"]
    },
    "@esbuild/linux-ia32@0.23.1": {
      "integrity": "sha512-VTN4EuOHwXEkXzX5nTvVY4s7E/Krz7COC8xkftbbKRYAl96vPiUssGkeMELQMOnLOJ8k3BY1+ZY52tttZnHcXQ==",
      "os": ["linux"],
      "cpu": ["ia32"]
    },
    "@esbuild/linux-ia32@0.25.4": {
      "integrity": "sha512-yTEjoapy8UP3rv8dB0ip3AfMpRbyhSN3+hY8mo/i4QXFeDxmiYbEKp3ZRjBKcOP862Ua4b1PDfwlvbuwY7hIGQ==",
      "os": ["linux"],
      "cpu": ["ia32"]
    },
    "@esbuild/linux-loong64@0.23.1": {
      "integrity": "sha512-Vx09LzEoBa5zDnieH8LSMRToj7ir/Jeq0Gu6qJ/1GcBq9GkfoEAoXvLiW1U9J1qE/Y/Oyaq33w5p2ZWrNNHNEw==",
      "os": ["linux"],
      "cpu": ["loong64"]
    },
    "@esbuild/linux-loong64@0.25.4": {
      "integrity": "sha512-NeqqYkrcGzFwi6CGRGNMOjWGGSYOpqwCjS9fvaUlX5s3zwOtn1qwg1s2iE2svBe4Q/YOG1q6875lcAoQK/F4VA==",
      "os": ["linux"],
      "cpu": ["loong64"]
    },
    "@esbuild/linux-mips64el@0.23.1": {
      "integrity": "sha512-nrFzzMQ7W4WRLNUOU5dlWAqa6yVeI0P78WKGUo7lg2HShq/yx+UYkeNSE0SSfSure0SqgnsxPvmAUu/vu0E+3Q==",
      "os": ["linux"],
      "cpu": ["mips64el"]
    },
    "@esbuild/linux-mips64el@0.25.4": {
      "integrity": "sha512-IcvTlF9dtLrfL/M8WgNI/qJYBENP3ekgsHbYUIzEzq5XJzzVEV/fXY9WFPfEEXmu3ck2qJP8LG/p3Q8f7Zc2Xg==",
      "os": ["linux"],
      "cpu": ["mips64el"]
    },
    "@esbuild/linux-ppc64@0.23.1": {
      "integrity": "sha512-dKN8fgVqd0vUIjxuJI6P/9SSSe/mB9rvA98CSH2sJnlZ/OCZWO1DJvxj8jvKTfYUdGfcq2dDxoKaC6bHuTlgcw==",
      "os": ["linux"],
      "cpu": ["ppc64"]
    },
    "@esbuild/linux-ppc64@0.25.4": {
      "integrity": "sha512-HOy0aLTJTVtoTeGZh4HSXaO6M95qu4k5lJcH4gxv56iaycfz1S8GO/5Jh6X4Y1YiI0h7cRyLi+HixMR+88swag==",
      "os": ["linux"],
      "cpu": ["ppc64"]
    },
    "@esbuild/linux-riscv64@0.23.1": {
      "integrity": "sha512-5AV4Pzp80fhHL83JM6LoA6pTQVWgB1HovMBsLQ9OZWLDqVY8MVobBXNSmAJi//Csh6tcY7e7Lny2Hg1tElMjIA==",
      "os": ["linux"],
      "cpu": ["riscv64"]
    },
    "@esbuild/linux-riscv64@0.25.4": {
      "integrity": "sha512-i8JUDAufpz9jOzo4yIShCTcXzS07vEgWzyX3NH2G7LEFVgrLEhjwL3ajFE4fZI3I4ZgiM7JH3GQ7ReObROvSUA==",
      "os": ["linux"],
      "cpu": ["riscv64"]
    },
    "@esbuild/linux-s390x@0.23.1": {
      "integrity": "sha512-9ygs73tuFCe6f6m/Tb+9LtYxWR4c9yg7zjt2cYkjDbDpV/xVn+68cQxMXCjUpYwEkze2RcU/rMnfIXNRFmSoDw==",
      "os": ["linux"],
      "cpu": ["s390x"]
    },
    "@esbuild/linux-s390x@0.25.4": {
      "integrity": "sha512-jFnu+6UbLlzIjPQpWCNh5QtrcNfMLjgIavnwPQAfoGx4q17ocOU9MsQ2QVvFxwQoWpZT8DvTLooTvmOQXkO51g==",
      "os": ["linux"],
      "cpu": ["s390x"]
    },
    "@esbuild/linux-x64@0.23.1": {
      "integrity": "sha512-EV6+ovTsEXCPAp58g2dD68LxoP/wK5pRvgy0J/HxPGB009omFPv3Yet0HiaqvrIrgPTBuC6wCH1LTOY91EO5hQ==",
      "os": ["linux"],
      "cpu": ["x64"]
    },
    "@esbuild/linux-x64@0.25.4": {
      "integrity": "sha512-6e0cvXwzOnVWJHq+mskP8DNSrKBr1bULBvnFLpc1KY+d+irZSgZ02TGse5FsafKS5jg2e4pbvK6TPXaF/A6+CA==",
      "os": ["linux"],
      "cpu": ["x64"]
    },
    "@esbuild/netbsd-arm64@0.25.4": {
      "integrity": "sha512-vUnkBYxZW4hL/ie91hSqaSNjulOnYXE1VSLusnvHg2u3jewJBz3YzB9+oCw8DABeVqZGg94t9tyZFoHma8gWZQ==",
      "os": ["netbsd"],
      "cpu": ["arm64"]
    },
    "@esbuild/netbsd-x64@0.23.1": {
      "integrity": "sha512-aevEkCNu7KlPRpYLjwmdcuNz6bDFiE7Z8XC4CPqExjTvrHugh28QzUXVOZtiYghciKUacNktqxdpymplil1beA==",
      "os": ["netbsd"],
      "cpu": ["x64"]
    },
    "@esbuild/netbsd-x64@0.25.4": {
      "integrity": "sha512-XAg8pIQn5CzhOB8odIcAm42QsOfa98SBeKUdo4xa8OvX8LbMZqEtgeWE9P/Wxt7MlG2QqvjGths+nq48TrUiKw==",
      "os": ["netbsd"],
      "cpu": ["x64"]
    },
    "@esbuild/openbsd-arm64@0.23.1": {
      "integrity": "sha512-3x37szhLexNA4bXhLrCC/LImN/YtWis6WXr1VESlfVtVeoFJBRINPJ3f0a/6LV8zpikqoUg4hyXw0sFBt5Cr+Q==",
      "os": ["openbsd"],
      "cpu": ["arm64"]
    },
    "@esbuild/openbsd-arm64@0.25.4": {
      "integrity": "sha512-Ct2WcFEANlFDtp1nVAXSNBPDxyU+j7+tId//iHXU2f/lN5AmO4zLyhDcpR5Cz1r08mVxzt3Jpyt4PmXQ1O6+7A==",
      "os": ["openbsd"],
      "cpu": ["arm64"]
    },
    "@esbuild/openbsd-x64@0.23.1": {
      "integrity": "sha512-aY2gMmKmPhxfU+0EdnN+XNtGbjfQgwZj43k8G3fyrDM/UdZww6xrWxmDkuz2eCZchqVeABjV5BpildOrUbBTqA==",
      "os": ["openbsd"],
      "cpu": ["x64"]
    },
    "@esbuild/openbsd-x64@0.25.4": {
      "integrity": "sha512-xAGGhyOQ9Otm1Xu8NT1ifGLnA6M3sJxZ6ixylb+vIUVzvvd6GOALpwQrYrtlPouMqd/vSbgehz6HaVk4+7Afhw==",
      "os": ["openbsd"],
      "cpu": ["x64"]
    },
    "@esbuild/sunos-x64@0.23.1": {
      "integrity": "sha512-RBRT2gqEl0IKQABT4XTj78tpk9v7ehp+mazn2HbUeZl1YMdaGAQqhapjGTCe7uw7y0frDi4gS0uHzhvpFuI1sA==",
      "os": ["sunos"],
      "cpu": ["x64"]
    },
    "@esbuild/sunos-x64@0.25.4": {
      "integrity": "sha512-Mw+tzy4pp6wZEK0+Lwr76pWLjrtjmJyUB23tHKqEDP74R3q95luY/bXqXZeYl4NYlvwOqoRKlInQialgCKy67Q==",
      "os": ["sunos"],
      "cpu": ["x64"]
    },
    "@esbuild/win32-arm64@0.23.1": {
      "integrity": "sha512-4O+gPR5rEBe2FpKOVyiJ7wNDPA8nGzDuJ6gN4okSA1gEOYZ67N8JPk58tkWtdtPeLz7lBnY6I5L3jdsr3S+A6A==",
      "os": ["win32"],
      "cpu": ["arm64"]
    },
    "@esbuild/win32-arm64@0.25.4": {
      "integrity": "sha512-AVUP428VQTSddguz9dO9ngb+E5aScyg7nOeJDrF1HPYu555gmza3bDGMPhmVXL8svDSoqPCsCPjb265yG/kLKQ==",
      "os": ["win32"],
      "cpu": ["arm64"]
    },
    "@esbuild/win32-ia32@0.23.1": {
      "integrity": "sha512-BcaL0Vn6QwCwre3Y717nVHZbAa4UBEigzFm6VdsVdT/MbZ38xoj1X9HPkZhbmaBGUD1W8vxAfffbDe8bA6AKnQ==",
      "os": ["win32"],
      "cpu": ["ia32"]
    },
    "@esbuild/win32-ia32@0.25.4": {
      "integrity": "sha512-i1sW+1i+oWvQzSgfRcxxG2k4I9n3O9NRqy8U+uugaT2Dy7kLO9Y7wI72haOahxceMX8hZAzgGou1FhndRldxRg==",
      "os": ["win32"],
      "cpu": ["ia32"]
    },
    "@esbuild/win32-x64@0.23.1": {
      "integrity": "sha512-BHpFFeslkWrXWyUPnbKm+xYYVYruCinGcftSBaa8zoF9hZO4BcSCFUvHVTtzpIY6YzUnYtuEhZ+C9iEXjxnasg==",
      "os": ["win32"],
      "cpu": ["x64"]
    },
    "@esbuild/win32-x64@0.25.4": {
      "integrity": "sha512-nOT2vZNw6hJ+z43oP1SPea/G/6AbN6X+bGNhNuq8NtRHy4wsMhw765IKLNmnjek7GvjWBYQ8Q5VBoYTFg9y1UQ==",
      "os": ["win32"],
      "cpu": ["x64"]
    },
    "@isaacs/cliui@8.0.2": {
      "integrity": "sha512-O8jcjabXaleOG9DQ0+ARXWZBTfnP4WNAqzuiJK7ll44AmxGKv/J2M4TPjxjY3znBCfvBXFzucm1twdyFybFqEA==",
      "dependencies": [
        "string-width@5.1.2",
        "string-width-cjs@npm:string-width@4.2.3",
        "strip-ansi@7.1.0",
        "strip-ansi-cjs@npm:strip-ansi@6.0.1",
        "wrap-ansi@8.1.0",
        "wrap-ansi-cjs@npm:wrap-ansi@7.0.0"
      ]
    },
    "@jridgewell/gen-mapping@0.3.8": {
      "integrity": "sha512-imAbBGkb+ebQyxKgzv5Hu2nmROxoDOXHh80evxdoXNOrvAnVx7zimzc1Oo5h9RlfV4vPXaE2iM5pOFbvOCClWA==",
      "dependencies": [
        "@jridgewell/set-array",
        "@jridgewell/sourcemap-codec",
        "@jridgewell/trace-mapping"
      ]
    },
    "@jridgewell/resolve-uri@3.1.2": {
      "integrity": "sha512-bRISgCIjP20/tbWSPWMEi54QVPRZExkuD9lJL+UIxUKtwVJA8wW1Trb1jMs1RFXo1CBTNZ/5hpC9QvmKWdopKw=="
    },
    "@jridgewell/set-array@1.2.1": {
      "integrity": "sha512-R8gLRTZeyp03ymzP/6Lil/28tGeGEzhx1q2k703KGWRAI1VdvPIXdG70VJc2pAMw3NA6JKL5hhFu1sJX0Mnn/A=="
    },
    "@jridgewell/sourcemap-codec@1.5.0": {
      "integrity": "sha512-gv3ZRaISU3fjPAgNsriBRqGWQL6quFx04YMPW/zD8XMLsU32mhCCbfbO6KZFLjvYpCZ8zyDEgqsgf+PwPaM7GQ=="
    },
    "@jridgewell/trace-mapping@0.3.25": {
      "integrity": "sha512-vNk6aEwybGtawWmy/PzwnGDOjCkLWSD2wqvjGGAgOAwCGWySYXfYoxt00IJkTF+8Lb57DwOb3Aa0o9CApepiYQ==",
      "dependencies": [
        "@jridgewell/resolve-uri",
        "@jridgewell/sourcemap-codec"
      ]
    },
    "@nodelib/fs.scandir@2.1.5": {
      "integrity": "sha512-vq24Bq3ym5HEQm2NKCr3yXDwjc7vTsEThRDnkp2DK9p1uqLR+DHurm/NOTo0KG7HYHU7eppKZj3MyqYuMBf62g==",
      "dependencies": [
        "@nodelib/fs.stat",
        "run-parallel"
      ]
    },
    "@nodelib/fs.stat@2.0.5": {
      "integrity": "sha512-RkhPPp2zrqDAQA/2jNhnztcPAlv64XdhIp7a7454A5ovI7Bukxgt7MX7udwAu3zg1DcpPU0rz3VV1SeaqvY4+A=="
    },
    "@nodelib/fs.walk@1.2.8": {
      "integrity": "sha512-oGB+UxlgWcgQkgwo8GcEGwemoTFt3FIO9ababBmaGwXIoBKZ+GTy0pP185beGg7Llih/NSHSV2XAs1lnznocSg==",
      "dependencies": [
        "@nodelib/fs.scandir",
        "fastq"
      ]
    },
    "@opentelemetry/api@1.9.0": {
      "integrity": "sha512-3giAOQvZiH5F9bMlMiv8+GSPMeqg0dbaeo58/0SlA9sxSqZhnUtxzX9/2FzyhS9sWQf5S0GJE0AKBrFqjpeYcg=="
    },
    "@opentelemetry/core@1.30.1_@opentelemetry+api@1.9.0": {
      "integrity": "sha512-OOCM2C/QIURhJMuKaekP3TRBxBKxG/TWWA0TL2J6nXUtDnuCtccy49LUJF8xPFXMX+0LMcxFpCo8M9cGY1W6rQ==",
      "dependencies": [
        "@opentelemetry/api",
        "@opentelemetry/semantic-conventions"
      ]
    },
    "@opentelemetry/resources@1.30.1_@opentelemetry+api@1.9.0": {
      "integrity": "sha512-5UxZqiAgLYGFjS4s9qm5mBVo433u+dSPUFWVWXmLAD4wB65oMCoXaJP1KJa9DIYYMeHu3z4BZcStG3LC593cWA==",
      "dependencies": [
        "@opentelemetry/api",
        "@opentelemetry/core",
        "@opentelemetry/semantic-conventions"
      ]
    },
    "@opentelemetry/sdk-trace-base@1.30.1_@opentelemetry+api@1.9.0": {
      "integrity": "sha512-jVPgBbH1gCy2Lb7X0AVQ8XAfgg0pJ4nvl8/IiQA6nxOsPvS+0zMJaFSs2ltXe0J6C8dqjcnpyqINDJmU30+uOg==",
      "dependencies": [
        "@opentelemetry/api",
        "@opentelemetry/core",
        "@opentelemetry/resources",
        "@opentelemetry/semantic-conventions"
      ]
    },
    "@opentelemetry/semantic-conventions@1.28.0": {
      "integrity": "sha512-lp4qAiMTD4sNWW4DbKLBkfiMZ4jbAboJIGOQr5DvciMRI494OapieI9qiODpOt0XBr1LjIDy1xAGAnVs5supTA=="
    },
    "@pkgjs/parseargs@0.11.0": {
      "integrity": "sha512-+1VkjdD0QBLPodGrJUeqarH8VAIvQODIbwh9XpP5Syisf7YoQgsJKPNFoqqLQlu+VQ/tVSshMR6loPMn8U+dPg=="
    },
    "@preact/signals-core@1.8.0": {
      "integrity": "sha512-OBvUsRZqNmjzCZXWLxkZfhcgT+Fk8DDcT/8vD6a1xhDemodyy87UJRJfASMuSD8FaAIeGgGm85ydXhm7lr4fyA=="
    },
    "@preact/signals@1.3.2_preact@10.26.6": {
      "integrity": "sha512-naxcJgUJ6BTOROJ7C3QML7KvwKwCXQJYTc5L/b0eEsdYgPB6SxwoQ1vDGcS0Q7GVjAenVq/tXrybVdFShHYZWg==",
      "dependencies": [
        "@preact/signals-core",
        "preact"
      ]
    },
    "@preact/signals@2.0.4_preact@10.26.6": {
      "integrity": "sha512-9241aGnIv7y0IGzaq2vkBMe8/0jGnnmEEUeFmAoTWsaj8q/BW2PVekL8nHVJcy69bBww6rwEy3A1tc6yPE0sJA==",
      "dependencies": [
        "@preact/signals-core",
        "preact"
      ]
    },
    "@trysound/sax@0.2.0": {
      "integrity": "sha512-L7z9BgrNEcYyUYtF+HaEfiS5ebkh9jXqbszz7pC0hRBPaatV0XjSD3+eHrpqFemQfgwiFF0QPIarnIihIDn7OA=="
    },
    "@ts-morph/common@0.26.1": {
      "integrity": "sha512-Sn28TGl/4cFpcM+jwsH1wLncYq3FtN/BIpem+HOygfBWPT5pAeS5dB4VFVzV8FbnOKHpDLZmvAl4AjPEev5idA==",
      "dependencies": [
        "fast-glob",
        "minimatch",
        "path-browserify"
      ]
    },
    "@types/node@22.15.15": {
      "integrity": "sha512-R5muMcZob3/Jjchn5LcO8jdKwSCbzqmPB6ruBxMcf9kbxtniZHP327s6C37iOfuw8mbKK3cAQa7sEl7afLrQ8A==",
      "dependencies": [
        "undici-types"
      ]
    },
    "ansi-regex@5.0.1": {
      "integrity": "sha512-quJQXlTSUGL2LH9SUXo8VwsY4soanhgo6LNSm84E1LBcE8s3O0wpdiRzyR9z/ZZJMlMWv37qOOb9pdJlMUEKFQ=="
    },
    "ansi-regex@6.1.0": {
      "integrity": "sha512-7HSX4QQb4CspciLpVFwyRe79O3xsIZDDLER21kERQ71oaPodF8jL725AgJMFAYbooIqolJoRLuM81SpeUkpkvA=="
    },
    "ansi-styles@4.3.0": {
      "integrity": "sha512-zbB9rCJAT1rbjiVDb2hqKFHNYLxgtk8NURxZ3IZwD3F6NtxbXZQCnnSi1Lkx+IDohdPlFp222wVALIheZJQSEg==",
      "dependencies": [
        "color-convert"
      ]
    },
    "ansi-styles@6.2.1": {
      "integrity": "sha512-bN798gFfQX+viw3R7yrGWRqnrN2oRkEkUjjl4JNn4E8GxxbjtG3FbrEIIY3l8/hrwUwIeCZvi4QuOTP4MErVug=="
    },
    "any-promise@1.3.0": {
      "integrity": "sha512-7UvmKalWRt1wgjL1RrGxoSJW/0QZFIegpeGvZG9kjp8vrRu55XTHbwnqq2GpXm9uLbcuhxm3IqX9OB4MZR1b2A=="
    },
    "anymatch@3.1.3": {
      "integrity": "sha512-KMReFUr0B4t+D+OBkjR3KYqvocp2XaSzO55UcB6mgQMd3KbcE+mWTyvVV7D/zsdEbNnV6acZUutkiHQXvTr1Rw==",
      "dependencies": [
        "normalize-path",
        "picomatch"
      ]
    },
    "arg@5.0.2": {
      "integrity": "sha512-PYjyFOLKQ9y57JvQ6QLo8dAgNqswh8M1RMJYdQduT6xbWSgK36P/Z/v+p888pM69jMMfS8Xd8F6I1kQ/I9HUGg=="
    },
    "autoprefixer@10.4.17_postcss@8.4.35": {
      "integrity": "sha512-/cpVNRLSfhOtcGflT13P2794gVSgmPgTR+erw5ifnMLZb0UnSlkK4tquLmkd3BhA+nLo5tX8Cu0upUsGKvKbmg==",
      "dependencies": [
        "browserslist",
        "caniuse-lite",
        "fraction.js",
        "normalize-range",
        "picocolors",
        "postcss@8.4.35",
        "postcss-value-parser"
      ],
      "bin": true
    },
    "balanced-match@1.0.2": {
      "integrity": "sha512-3oSeUO0TMV67hN1AmbXsK4yaqU7tjiHlbxRDZOpH0KW9+CeX4bRAaX0Anxt0tx2MrpRpWwQaPwIlISEJhYU5Pw=="
    },
    "binary-extensions@2.3.0": {
      "integrity": "sha512-Ceh+7ox5qe7LJuLHoY0feh3pHuUDHAcRUeyL2VYghZwfpkNIy/+8Ocg0a3UuSoYzavmylwuLWQOf3hl0jjMMIw=="
    },
    "boolbase@1.0.0": {
      "integrity": "sha512-JZOSA7Mo9sNGB8+UjSgzdLtokWAky1zbztM3WRLCbZ70/3cTANmQmOdR7y2g+J0e2WXywy1yS468tY+IruqEww=="
    },
    "brace-expansion@2.0.1": {
      "integrity": "sha512-XnAIvQ8eM+kC6aULx6wuQiwVsnzsi9d3WxzV3FpWTGA19F621kwdbsAcFKXgKUHZWsy+mY6iL1sHTxWEFCytDA==",
      "dependencies": [
        "balanced-match"
      ]
    },
    "braces@3.0.3": {
      "integrity": "sha512-yQbXgO/OSZVD2IsiLlro+7Hf6Q18EJrKSEsdoMzKePKXct3gvD8oLcOQdIzGupr5Fj+EDe8gO/lxc1BzfMpxvA==",
      "dependencies": [
        "fill-range"
      ]
    },
    "browserslist@4.24.5": {
      "integrity": "sha512-FDToo4Wo82hIdgc1CQ+NQD0hEhmpPjrZ3hiUgwgOG6IuTdlpr8jdjyG24P6cNP1yJpTLzS5OcGgSw0xmDU1/Tw==",
      "dependencies": [
        "caniuse-lite",
        "electron-to-chromium",
        "node-releases",
        "update-browserslist-db"
      ],
      "bin": true
    },
    "camelcase-css@2.0.1": {
      "integrity": "sha512-QOSvevhslijgYwRx6Rv7zKdMF8lbRmx+uQGx2+vDc+KI/eBnsy9kit5aj23AgGu3pa4t9AgwbnXWqS+iOY+2aA=="
    },
    "caniuse-api@3.0.0": {
      "integrity": "sha512-bsTwuIg/BZZK/vreVTYYbSWoe2F+71P7K5QGEX+pT250DZbfU1MQ5prOKpPR+LL6uWKK3KMwMCAS74QB3Um1uw==",
      "dependencies": [
        "browserslist",
        "caniuse-lite",
        "lodash.memoize",
        "lodash.uniq"
      ]
    },
    "caniuse-lite@1.0.30001717": {
      "integrity": "sha512-auPpttCq6BDEG8ZAuHJIplGw6GODhjw+/11e7IjpnYCxZcW/ONgPs0KVBJ0d1bY3e2+7PRe5RCLyP+PfwVgkYw=="
    },
    "chokidar@3.6.0": {
      "integrity": "sha512-7VT13fmjotKpGipCW9JEQAusEPE+Ei8nl6/g4FBAmIm0GOOLMua9NDDo/DWp0ZAxCr3cPq5ZpBqmPAQgDda2Pw==",
      "dependencies": [
        "anymatch",
        "braces",
        "glob-parent@5.1.2",
        "is-binary-path",
        "is-glob",
        "normalize-path",
        "readdirp"
      ],
      "optionalDependencies": [
        "fsevents"
      ]
    },
    "code-block-writer@13.0.3": {
      "integrity": "sha512-Oofo0pq3IKnsFtuHqSF7TqBfr71aeyZDVJ0HpmqB7FBM2qEigL0iPONSCZSO9pE9dZTAxANe5XHG9Uy0YMv8cg=="
    },
    "color-convert@2.0.1": {
      "integrity": "sha512-RRECPsj7iu/xb5oKYcsFHSppFNnsj/52OVTRKb4zP5onXwVF3zVmmToNcOfGC+CRDpfK/U584fMg38ZHCaElKQ==",
      "dependencies": [
        "color-name"
      ]
    },
    "color-name@1.1.4": {
      "integrity": "sha512-dOy+3AuW3a2wNbZHIuMZpTcgjGuLU/uBL/ubcZF9OXbDo8ff4O8yVp5Bf0efS8uEoYo5q4Fx7dY9OgQGXgAsQA=="
    },
    "colord@2.9.3": {
      "integrity": "sha512-jeC1axXpnb0/2nn/Y1LPuLdgXBLH7aDcHu4KEKfqw3CUhX7ZpfBSlPKyqXE6btIgEzfWtrX3/tyBCaCvXvMkOw=="
    },
    "commander@4.1.1": {
      "integrity": "sha512-NOKm8xhkzAjzFx8B2v5OAHT+u5pRQc2UCa2Vq9jYL/31o2wi9mxBA7LIFs3sV5VSC49z6pEhfbMULvShKj26WA=="
    },
    "commander@7.2.0": {
      "integrity": "sha512-QrWXB+ZQSVPmIWIhtEO9H+gwHaMGYiF5ChvoJ+K9ZGHG/sVsa6yiesAD1GC/x46sET00Xlwo1u49RVVVzvcSkw=="
    },
    "cross-spawn@7.0.6": {
      "integrity": "sha512-uV2QOWP2nWzsy2aMp8aRibhi9dlzF5Hgh5SHaB9OiTGEyDTiJJyx0uy51QXdyWbtAHNua4XJzUKca3OzKUd3vA==",
      "dependencies": [
        "path-key",
        "shebang-command",
        "which"
      ]
    },
    "css-declaration-sorter@7.2.0_postcss@8.4.35": {
      "integrity": "sha512-h70rUM+3PNFuaBDTLe8wF/cdWu+dOZmb7pJt8Z2sedYbAcQVQV/tEchueg3GWxwqS0cxtbxmaHEdkNACqcvsow==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "css-select@5.1.0": {
      "integrity": "sha512-nwoRF1rvRRnnCqqY7updORDsuqKzqYJ28+oSMaJMMgOauh3fvwHqMS7EZpIPqK8GL+g9mKxF1vP/ZjSeNjEVHg==",
      "dependencies": [
        "boolbase",
        "css-what",
        "domhandler",
        "domutils",
        "nth-check"
      ]
    },
    "css-tree@2.2.1": {
      "integrity": "sha512-OA0mILzGc1kCOCSJerOeqDxDQ4HOh+G8NbOJFOTgOCzpw7fCBubk0fEyxp8AgOL/jvLgYA/uV0cMbe43ElF1JA==",
      "dependencies": [
        "mdn-data@2.0.28",
        "source-map-js"
      ]
    },
    "css-tree@2.3.1": {
      "integrity": "sha512-6Fv1DV/TYw//QF5IzQdqsNDjx/wc8TrMBZsqjL9eW01tWb7R7k/mq+/VXfJCl7SoD5emsJop9cOByJZfs8hYIw==",
      "dependencies": [
        "mdn-data@2.0.30",
        "source-map-js"
      ]
    },
    "css-what@6.1.0": {
      "integrity": "sha512-HTUrgRJ7r4dsZKU6GjmpfRK1O76h97Z8MfS1G0FozR+oF2kG6Vfe8JE6zwrkbxigziPHinCJ+gCPjA9EaBDtRw=="
    },
    "cssesc@3.0.0": {
      "integrity": "sha512-/Tb/JcjK111nNScGob5MNtsntNM1aCNUDipB/TkwZFhyDrrE47SOx/18wF2bbjgc3ZzCSKW1T5nt5EbFoAz/Vg==",
      "bin": true
    },
    "cssnano-preset-default@6.1.2_postcss@8.4.35": {
      "integrity": "sha512-1C0C+eNaeN8OcHQa193aRgYexyJtU8XwbdieEjClw+J9d94E41LwT6ivKH0WT+fYwYWB0Zp3I3IZ7tI/BbUbrg==",
      "dependencies": [
        "browserslist",
        "css-declaration-sorter",
        "cssnano-utils",
        "postcss@8.4.35",
        "postcss-calc",
        "postcss-colormin",
        "postcss-convert-values",
        "postcss-discard-comments",
        "postcss-discard-duplicates",
        "postcss-discard-empty",
        "postcss-discard-overridden",
        "postcss-merge-longhand",
        "postcss-merge-rules",
        "postcss-minify-font-values",
        "postcss-minify-gradients",
        "postcss-minify-params",
        "postcss-minify-selectors",
        "postcss-normalize-charset",
        "postcss-normalize-display-values",
        "postcss-normalize-positions",
        "postcss-normalize-repeat-style",
        "postcss-normalize-string",
        "postcss-normalize-timing-functions",
        "postcss-normalize-unicode",
        "postcss-normalize-url",
        "postcss-normalize-whitespace",
        "postcss-ordered-values",
        "postcss-reduce-initial",
        "postcss-reduce-transforms",
        "postcss-svgo",
        "postcss-unique-selectors"
      ]
    },
    "cssnano-utils@4.0.2_postcss@8.4.35": {
      "integrity": "sha512-ZR1jHg+wZ8o4c3zqf1SIUSTIvm/9mU343FMR6Obe/unskbvpGhZOo1J6d/r8D1pzkRQYuwbcH3hToOuoA2G7oQ==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "cssnano@6.0.3_postcss@8.4.35": {
      "integrity": "sha512-MRq4CIj8pnyZpcI2qs6wswoYoDD1t0aL28n+41c1Ukcpm56m1h6mCexIHBGjfZfnTqtGSSCP4/fB1ovxgjBOiw==",
      "dependencies": [
        "cssnano-preset-default",
        "lilconfig",
        "postcss@8.4.35"
      ]
    },
    "csso@5.0.5": {
      "integrity": "sha512-0LrrStPOdJj+SPCCrGhzryycLjwcgUSHBtxNA8aIDxf0GLsRh1cKYhB00Gd1lDOS4yGH69+SNn13+TWbVHETFQ==",
      "dependencies": [
        "css-tree@2.2.1"
      ]
    },
    "cssom@0.5.0": {
      "integrity": "sha512-iKuQcq+NdHqlAcwUY0o/HL69XQrUaQdMjmStJ8JFmUaiiQErlhrmuigkg/CU4E2J0IyUKUrMAgl36TvN67MqTw=="
    },
    "didyoumean@1.2.2": {
      "integrity": "sha512-gxtyfqMg7GKyhQmb056K7M3xszy/myH8w+B4RT+QXBQsvAOdc3XymqDDPHx1BgPgsdAA5SIifona89YtRATDzw=="
    },
    "dlv@1.1.3": {
      "integrity": "sha512-+HlytyjlPKnIG8XuRG8WvmBP8xs8P71y+SKKS6ZXWoEgLuePxtDoUEiH7WkdePWrQ5JBpE6aoVqfZfJUQkjXwA=="
    },
    "dom-serializer@2.0.0": {
      "integrity": "sha512-wIkAryiqt/nV5EQKqQpo3SToSOV9J0DnbJqwK7Wv/Trc92zIAYZ4FlMu+JPFW1DfGFt81ZTCGgDEabffXeLyJg==",
      "dependencies": [
        "domelementtype",
        "domhandler",
        "entities@4.5.0"
      ]
    },
    "domelementtype@2.3.0": {
      "integrity": "sha512-OLETBj6w0OsagBwdXnPdN0cnMfF9opN69co+7ZrbfPGrdpPVNBUj02spi6B1N7wChLQiPn4CSH/zJvXw56gmHw=="
    },
    "domhandler@5.0.3": {
      "integrity": "sha512-cgwlv/1iFQiFnU96XXgROh8xTeetsnJiDsTc7TYCLFd9+/WNkIqPTxiM/8pSd8VIrhXGTf1Ny1q1hquVqDJB5w==",
      "dependencies": [
        "domelementtype"
      ]
    },
    "domutils@3.2.2": {
      "integrity": "sha512-6kZKyUajlDuqlHKVX1w7gyslj9MPIXzIFiz/rGu35uC1wMi+kMhQwGhl4lt9unC9Vb9INnY9Z3/ZA3+FhASLaw==",
      "dependencies": [
        "dom-serializer",
        "domelementtype",
        "domhandler"
      ]
    },
    "eastasianwidth@0.2.0": {
      "integrity": "sha512-I88TYZWc9XiYHRQ4/3c5rjjfgkjhLyW2luGIheGERbNQ6OY7yTybanSpDXZa8y7VUP9YmDcYa+eyq4ca7iLqWA=="
    },
    "electron-to-chromium@1.5.151": {
      "integrity": "sha512-Rl6uugut2l9sLojjS4H4SAr3A4IgACMLgpuEMPYCVcKydzfyPrn5absNRju38IhQOf/NwjJY8OGWjlteqYeBCA=="
    },
    "emoji-regex@8.0.0": {
      "integrity": "sha512-MSjYzcWNOA0ewAHpz0MxpYFvwg6yjy1NG3xteoqz644VCo/RPgnr1/GGt+ic3iJTzQ8Eu3TdM14SawnVUmGE6A=="
    },
    "emoji-regex@9.2.2": {
      "integrity": "sha512-L18DaJsXSUk2+42pv8mLs5jJT2hqFkFE4j21wOmgbUqsZ2hL72NsUU785g9RXgo3s0ZNgVl42TiHp3ZtOv/Vyg=="
    },
    "entities@4.5.0": {
      "integrity": "sha512-V0hjH4dGPh9Ao5p0MoRY6BVqtwCjhz6vI5LT8AJ55H+4g9/4vbHx1I54fS0XuclLhDHArPQCiMjDxjaL8fPxhw=="
    },
    "entities@6.0.0": {
      "integrity": "sha512-aKstq2TDOndCn4diEyp9Uq/Flu2i1GlLkc6XIDQSDMuaFE3OPW5OphLCyQ5SpSJZTb4reN+kTcYru5yIfXoRPw=="
    },
    "esbuild-wasm@0.23.1": {
      "integrity": "sha512-L3vn7ctvBrtScRfoB0zG1eOCiV4xYvpLYWfe6PDZuV+iDFDm4Mt3xeLIDllG8cDHQ8clUouK3XekulE+cxgkgw==",
      "bin": true
    },
    "esbuild-wasm@0.25.4": {
      "integrity": "sha512-2HlCS6rNvKWaSKhWaG/YIyRsTsL3gUrMP2ToZMBIjw9LM7vVcIs+rz8kE2vExvTJgvM8OKPqNpcHawY/BQc/qQ==",
      "bin": true
    },
    "esbuild@0.23.1": {
      "integrity": "sha512-VVNz/9Sa0bs5SELtn3f7qhJCDPCF5oMEl5cO9/SSinpE9hbPVvxbd572HH5AKiP7WD8INO53GgfDDhRjkylHEg==",
      "optionalDependencies": [
        "@esbuild/aix-ppc64@0.23.1",
        "@esbuild/android-arm@0.23.1",
        "@esbuild/android-arm64@0.23.1",
        "@esbuild/android-x64@0.23.1",
        "@esbuild/darwin-arm64@0.23.1",
        "@esbuild/darwin-x64@0.23.1",
        "@esbuild/freebsd-arm64@0.23.1",
        "@esbuild/freebsd-x64@0.23.1",
        "@esbuild/linux-arm@0.23.1",
        "@esbuild/linux-arm64@0.23.1",
        "@esbuild/linux-ia32@0.23.1",
        "@esbuild/linux-loong64@0.23.1",
        "@esbuild/linux-mips64el@0.23.1",
        "@esbuild/linux-ppc64@0.23.1",
        "@esbuild/linux-riscv64@0.23.1",
        "@esbuild/linux-s390x@0.23.1",
        "@esbuild/linux-x64@0.23.1",
        "@esbuild/netbsd-x64@0.23.1",
        "@esbuild/openbsd-arm64@0.23.1",
        "@esbuild/openbsd-x64@0.23.1",
        "@esbuild/sunos-x64@0.23.1",
        "@esbuild/win32-arm64@0.23.1",
        "@esbuild/win32-ia32@0.23.1",
        "@esbuild/win32-x64@0.23.1"
      ],
      "scripts": true,
      "bin": true
    },
    "esbuild@0.25.4": {
      "integrity": "sha512-8pgjLUcUjcgDg+2Q4NYXnPbo/vncAY4UmyaCm0jZevERqCHZIaWwdJHkf8XQtu4AxSKCdvrUbT0XUr1IdZzI8Q==",
      "optionalDependencies": [
        "@esbuild/aix-ppc64@0.25.4",
        "@esbuild/android-arm@0.25.4",
        "@esbuild/android-arm64@0.25.4",
        "@esbuild/android-x64@0.25.4",
        "@esbuild/darwin-arm64@0.25.4",
        "@esbuild/darwin-x64@0.25.4",
        "@esbuild/freebsd-arm64@0.25.4",
        "@esbuild/freebsd-x64@0.25.4",
        "@esbuild/linux-arm@0.25.4",
        "@esbuild/linux-arm64@0.25.4",
        "@esbuild/linux-ia32@0.25.4",
        "@esbuild/linux-loong64@0.25.4",
        "@esbuild/linux-mips64el@0.25.4",
        "@esbuild/linux-ppc64@0.25.4",
        "@esbuild/linux-riscv64@0.25.4",
        "@esbuild/linux-s390x@0.25.4",
        "@esbuild/linux-x64@0.25.4",
        "@esbuild/netbsd-arm64",
        "@esbuild/netbsd-x64@0.25.4",
        "@esbuild/openbsd-arm64@0.25.4",
        "@esbuild/openbsd-x64@0.25.4",
        "@esbuild/sunos-x64@0.25.4",
        "@esbuild/win32-arm64@0.25.4",
        "@esbuild/win32-ia32@0.25.4",
        "@esbuild/win32-x64@0.25.4"
      ],
      "scripts": true,
      "bin": true
    },
    "escalade@3.2.0": {
      "integrity": "sha512-WUj2qlxaQtO4g6Pq5c29GTcWGDyd8itL8zTlipgECz3JesAiiOKotd8JU6otB3PACgG6xkJUyVhboMS+bje/jA=="
    },
    "fast-glob@3.3.3": {
      "integrity": "sha512-7MptL8U0cqcFdzIzwOTHoilX9x5BrNqye7Z/LuC7kCMRio1EMSyqRK3BEAUD7sXRq4iT4AzTVuZdhgQ2TCvYLg==",
      "dependencies": [
        "@nodelib/fs.stat",
        "@nodelib/fs.walk",
        "glob-parent@5.1.2",
        "merge2",
        "micromatch"
      ]
    },
    "fastq@1.19.1": {
      "integrity": "sha512-GwLTyxkCXjXbxqIhTsMI2Nui8huMPtnxg7krajPJAjnEG/iiOS7i+zCtWGZR9G0NBKbXKh6X9m9UIsYX/N6vvQ==",
      "dependencies": [
        "reusify"
      ]
    },
    "fill-range@7.1.1": {
      "integrity": "sha512-YsGpe3WHLK8ZYi4tWDg2Jy3ebRz2rXowDxnld4bkQB00cc/1Zw9AWnC0i9ztDJitivtQvaI9KaLyKrc+hBW0yg==",
      "dependencies": [
        "to-regex-range"
      ]
    },
    "foreground-child@3.3.1": {
      "integrity": "sha512-gIXjKqtFuWEgzFRJA9WCQeSJLZDjgJUOMCMzxtvFq/37KojM1BFGufqsCy0r4qSQmYLsZYMeyRqzIWOMup03sw==",
      "dependencies": [
        "cross-spawn",
        "signal-exit"
      ]
    },
    "fraction.js@4.3.7": {
      "integrity": "sha512-ZsDfxO51wGAXREY55a7la9LScWpwv9RxIrYABrlvOFBlH/ShPnrtsXeuUIfXKKOVicNxQ+o8JTbJvjS4M89yew=="
    },
    "fsevents@2.3.3": {
      "integrity": "sha512-5xoDfX+fL7faATnagmWPpbFtwh/R77WmMMqqHGS65C3vvB0YHrgF+B1YmZ3441tMj5n63k0212XNoJwzlhffQw==",
      "os": ["darwin"],
      "scripts": true
    },
    "function-bind@1.1.2": {
      "integrity": "sha512-7XHNxH7qX9xG5mIwxkhumTox/MIRNcOgDrxWsMt2pAr23WHp6MrRlN7FBSFpCpr+oVO0F744iUgR82nJMfG2SA=="
    },
    "github-slugger@2.0.0": {
      "integrity": "sha512-IaOQ9puYtjrkq7Y0Ygl9KDZnrf/aiUJYUpVf89y8kyaxbRG7Y1SrX/jaumrv81vc61+kiMempujsM3Yw7w5qcw=="
    },
    "glob-parent@5.1.2": {
      "integrity": "sha512-AOIgSQCepiJYwP3ARnGx+5VnTu2HBYdzbGP45eLw1vr3zB3vZLeyed1sC9hnbcOc9/SrMyM5RPQrkGz4aS9Zow==",
      "dependencies": [
        "is-glob"
      ]
    },
    "glob-parent@6.0.2": {
      "integrity": "sha512-XxwI8EOhVQgWp6iDL+3b0r86f4d6AX6zSU55HfB4ydCEuXLXc5FcYeOu+nnGftS4TEju/11rt4KJPTMgbfmv4A==",
      "dependencies": [
        "is-glob"
      ]
    },
    "glob@10.4.5": {
      "integrity": "sha512-7Bv8RF0k6xjo7d4A/PxYLbUCfb6c+Vpd2/mB2yRDlew7Jb5hEXiCD9ibfO7wpk8i4sevK6DFny9h7EYbM3/sHg==",
      "dependencies": [
        "foreground-child",
        "jackspeak",
        "minimatch",
        "minipass",
        "package-json-from-dist",
        "path-scurry"
      ],
      "bin": true
    },
    "hasown@2.0.2": {
      "integrity": "sha512-0hJU9SCPvmMzIBdZFqNPXWa6dqh7WdH0cII9y+CyS8rG3nL48Bclra9HmKhVVUHyPWNH5Y7xDwAB7bfgSjkUMQ==",
      "dependencies": [
        "function-bind"
      ]
    },
    "html-escaper@3.0.3": {
      "integrity": "sha512-RuMffC89BOWQoY0WKGpIhn5gX3iI54O6nRA0yC124NYVtzjmFWBIiFd8M0x+ZdX0P9R4lADg1mgP8C7PxGOWuQ=="
    },
    "htmlparser2@10.0.0": {
      "integrity": "sha512-TwAZM+zE5Tq3lrEHvOlvwgj1XLWQCtaaibSN11Q+gGBAS7Y1uZSWwXXRe4iF6OXnaq1riyQAPFOBtYc77Mxq0g==",
      "dependencies": [
        "domelementtype",
        "domhandler",
        "domutils",
        "entities@6.0.0"
      ]
    },
    "is-binary-path@2.1.0": {
      "integrity": "sha512-ZMERYes6pDydyuGidse7OsHxtbI7WVeUEozgR/g7rd0xUimYNlvZRE/K2MgZTjWy725IfelLeVcEM97mmtRGXw==",
      "dependencies": [
        "binary-extensions"
      ]
    },
    "is-core-module@2.16.1": {
      "integrity": "sha512-UfoeMA6fIJ8wTYFEUjelnaGI67v6+N7qXJEvQuIGa99l4xsCruSYOVSQ0uPANn4dAzm8lkYPaKLrrijLq7x23w==",
      "dependencies": [
        "hasown"
      ]
    },
    "is-extglob@2.1.1": {
      "integrity": "sha512-SbKbANkN603Vi4jEZv49LeVJMn4yGwsbzZworEoyEiutsN3nJYdbO36zfhGJ6QEDpOZIFkDtnq5JRxmvl3jsoQ=="
    },
    "is-fullwidth-code-point@3.0.0": {
      "integrity": "sha512-zymm5+u+sCsSWyD9qNaejV3DFvhCKclKdizYaJUuHA83RLjb7nSuGnddCHGv0hk+KY7BMAlsWeK4Ueg6EV6XQg=="
    },
    "is-glob@4.0.3": {
      "integrity": "sha512-xelSayHH36ZgE7ZWhli7pW34hNbNl8Ojv5KVmkJD4hBdD3th8Tfk9vYasLM+mXWOZhFkgZfxhLSnrwRr4elSSg==",
      "dependencies": [
        "is-extglob"
      ]
    },
    "is-number@7.0.0": {
      "integrity": "sha512-41Cifkg6e8TylSpdtTpeLVMqvSBEVzTttHvERD741+pnZ8ANv0004MRL43QKPDlK9cGvNp6NZWZUBlbGXYxxng=="
    },
    "isexe@2.0.0": {
      "integrity": "sha512-RHxMLp9lnKHGHRng9QFhRCMbYAcVpn69smSGcq3f36xjgVVWThj4qqLbTLlq7Ssj8B+fIQ1EuCEGI2lKsyQeIw=="
    },
    "jackspeak@3.4.3": {
      "integrity": "sha512-OGlZQpz2yfahA/Rd1Y8Cd9SIEsqvXkLVoSw/cgwhnhFMDbsQFeZYoJJ7bIZBS9BcamUW96asq/npPWugM+RQBw==",
      "dependencies": [
        "@isaacs/cliui"
      ],
      "optionalDependencies": [
        "@pkgjs/parseargs"
      ]
    },
    "jiti@1.21.7": {
      "integrity": "sha512-/imKNG4EbWNrVjoNC/1H5/9GFy+tqjGBHCaSsN+P2RnPqjsLmv6UD3Ej+Kj8nBWaRAwyk7kK5ZUc+OEatnTR3A==",
      "bin": true
    },
    "lilconfig@3.1.3": {
      "integrity": "sha512-/vlFKAoH5Cgt3Ie+JLhRbwOsCQePABiU3tJ1egGvyQ+33R/vcwM2Zl2QR/LzjsBeItPt3oSVXapn+m4nQDvpzw=="
    },
    "lines-and-columns@1.2.4": {
      "integrity": "sha512-7ylylesZQ/PV29jhEDl3Ufjo6ZX7gCqJr5F7PKrqc93v7fzSymt1BpwEU8nAUXs8qzzvqhbjhK5QZg6Mt/HkBg=="
    },
    "linkedom@0.18.10": {
      "integrity": "sha512-ESCqVAtme2GI3zZnlVRidiydByV6WmPlmKeFzFVQslADiAO2Wi+H6xL/5kr/pUOESjEoVb2Eb3cYFJ/TQhQOWA==",
      "dependencies": [
        "css-select",
        "cssom",
        "html-escaper",
        "htmlparser2",
        "uhyphen"
      ]
    },
    "lodash.memoize@4.1.2": {
      "integrity": "sha512-t7j+NzmgnQzTAYXcsHYLgimltOV1MXHtlOWf6GjL9Kj8GK5FInw5JotxvbOs+IvV1/Dzo04/fCGfLVs7aXb4Ag=="
    },
    "lodash.uniq@4.5.0": {
      "integrity": "sha512-xfBaXQd9ryd9dlSDvnvI0lvxfLJlYAZzXomUYzLKtUeOQvOP5piqAWuGtrhWeqaXK9hhoM/iyJc5AV+XfsX3HQ=="
    },
    "lru-cache@10.4.3": {
      "integrity": "sha512-JNAzZcXrCt42VGLuYz0zfAzDfAvJWW6AfYlDBQyDV5DClI2m5sAmK+OIO7s59XfsRsWHp02jAJrRadPRGTt6SQ=="
    },
    "marked-mangle@1.1.10_marked@15.0.12": {
      "integrity": "sha512-TrpN67SMJJdzXXWIzOd/QmnpsC5o1B44PUYaG2bh1XEbqVjA0UCI2ijFuE5LWESwKeI2gCP5FqcUHRGQwFtDIA==",
      "dependencies": [
        "marked"
      ]
    },
    "marked@15.0.12": {
      "integrity": "sha512-8dD6FusOQSrpv9Z1rdNMdlSgQOIP880DHqnohobOmYLElGEqAL/JvxvuxZO16r4HtjTlfPRDC1hbvxC9dPN2nA==",
      "bin": true
    },
    "mdn-data@2.0.28": {
      "integrity": "sha512-aylIc7Z9y4yzHYAJNuESG3hfhC+0Ibp/MAMiaOZgNv4pmEdFyfZhhhny4MNiAfWdBQ1RQ2mfDWmM1x8SvGyp8g=="
    },
    "mdn-data@2.0.30": {
      "integrity": "sha512-GaqWWShW4kv/G9IEucWScBx9G1/vsFZZJUO+tD26M8J8z3Kw5RDQjaoZe03YAClgeS/SWPOcb4nkFBTEi5DUEA=="
    },
    "merge2@1.4.1": {
      "integrity": "sha512-8q7VEgMJW4J8tcfVPy8g09NcQwZdbwFEqhe/WZkoIzjn/3TGDwtOCYtXGxA3O8tPzpczCCDgv+P2P5y00ZJOOg=="
    },
    "micromatch@4.0.8": {
      "integrity": "sha512-PXwfBhYu0hBCPw8Dn0E+WDYb7af3dSLVWKi3HGv84IdF4TyFoC0ysxFd0Goxw7nSv4T/PzEJQxsYsEiFCKo2BA==",
      "dependencies": [
        "braces",
        "picomatch"
      ]
    },
    "minimatch@9.0.5": {
      "integrity": "sha512-G6T0ZX48xgozx7587koeX9Ys2NYy6Gmv//P89sEte9V9whIapMNF4idKxnW2QtCcLiTWlb/wfCabAtAFWhhBow==",
      "dependencies": [
        "brace-expansion"
      ]
    },
    "minipass@7.1.2": {
      "integrity": "sha512-qOOzS1cBTWYF4BH8fVePDBOO9iptMnGUEZwNc/cMWnTV2nVLZ7VoNWEPHkYczZA0pdoA7dl6e7FL659nX9S2aw=="
    },
    "mz@2.7.0": {
      "integrity": "sha512-z81GNO7nnYMEhrGh9LeymoE4+Yr0Wn5McHIZMK5cfQCl+NDX08sCZgUc9/6MHni9IWuFLm1Z3HTCXu2z9fN62Q==",
      "dependencies": [
        "any-promise",
        "object-assign",
        "thenify-all"
      ]
    },
    "nanoid@3.3.11": {
      "integrity": "sha512-N8SpfPUnUp1bK+PMYW8qSWdl9U+wwNWI4QKxOYDy9JAro3WMX7p2OeVRF9v+347pnakNevPmiHhNmZ2HbFA76w==",
      "bin": true
    },
    "node-releases@2.0.19": {
      "integrity": "sha512-xxOWJsBKtzAq7DY0J+DTzuz58K8e7sJbdgwkbMWQe8UYB6ekmsQ45q0M/tJDsGaZmbC+l7n57UV8Hl5tHxO9uw=="
    },
    "normalize-path@3.0.0": {
      "integrity": "sha512-6eZs5Ls3WtCisHWp9S2GUy8dqkpGi4BVSz3GaqiE6ezub0512ESztXUwUB6C6IKbQkY2Pnb/mD4WYojCRwcwLA=="
    },
    "normalize-range@0.1.2": {
      "integrity": "sha512-bdok/XvKII3nUpklnV6P2hxtMNrCboOjAcyBuQnWEhO665FwrSNRxU+AqpsyvO6LgGYPspN+lu5CLtw4jPRKNA=="
    },
    "nth-check@2.1.1": {
      "integrity": "sha512-lqjrjmaOoAnWfMmBPL+XNnynZh2+swxiX3WUE0s4yEHI6m+AwrK2UZOimIRl3X/4QctVqS8AiZjFqyOGrMXb/w==",
      "dependencies": [
        "boolbase"
      ]
    },
    "object-assign@4.1.1": {
      "integrity": "sha512-rJgTQnkUnH1sFw8yT6VSU3zD3sWmu6sZhIseY8VX+GRu3P6F7Fu+JNDoXfklElbLJSnc3FUQHVe4cU5hj+BcUg=="
    },
    "object-hash@3.0.0": {
      "integrity": "sha512-RSn9F68PjH9HqtltsSnqYC1XXoWe9Bju5+213R98cNGttag9q9yAOTzdbsqvIa7aNm5WffBZFpWYr2aWrklWAw=="
    },
    "package-json-from-dist@1.0.1": {
      "integrity": "sha512-UEZIS3/by4OC8vL3P2dTXRETpebLI2NiI5vIrjaD/5UtrkFX/tNbwjTSRAGC/+7CAo2pIcBaRgWmcBBHcsaCIw=="
    },
    "path-browserify@1.0.1": {
      "integrity": "sha512-b7uo2UCUOYZcnF/3ID0lulOJi/bafxa1xPe7ZPsammBSpjSWQkjNxlt635YGS2MiR9GjvuXCtz2emr3jbsz98g=="
    },
    "path-key@3.1.1": {
      "integrity": "sha512-ojmeN0qd+y0jszEtoY48r0Peq5dwMEkIlCOu6Q5f41lfkswXuKtYrhgoTpLnyIcHm24Uhqx+5Tqm2InSwLhE6Q=="
    },
    "path-parse@1.0.7": {
      "integrity": "sha512-LDJzPVEEEPR+y48z93A0Ed0yXb8pAByGWo/k5YYdYgpY2/2EsOsksJrq7lOHxryrVOn1ejG6oAp8ahvOIQD8sw=="
    },
    "path-scurry@1.11.1": {
      "integrity": "sha512-Xa4Nw17FS9ApQFJ9umLiJS4orGjm7ZzwUrwamcGQuHSzDyth9boKDaycYdDcZDuqYATXw4HFXgaqWTctW/v1HA==",
      "dependencies": [
        "lru-cache",
        "minipass"
      ]
    },
    "picocolors@1.1.1": {
      "integrity": "sha512-xceH2snhtb5M9liqDsmEw56le376mTZkEX/jEb/RxNFyegNul7eNslCXP9FDj/Lcu0X8KEyMceP2ntpaHrDEVA=="
    },
    "picomatch@2.3.1": {
      "integrity": "sha512-JU3teHTNjmE2VCGFzuY8EXzCDVwEqB2a8fsIvwaStHhAWJEeVd1o1QD80CU6+ZdEXXSLbSsuLwJjkCBWqRQUVA=="
    },
    "pify@2.3.0": {
      "integrity": "sha512-udgsAY+fTnvv7kI7aaxbqwWNb0AHiB0qBO89PZKPkoTmGOgdbrHDKD+0B2X4uTfJ/FT1R09r9gTsjUjNJotuog=="
    },
    "pirates@4.0.7": {
      "integrity": "sha512-TfySrs/5nm8fQJDcBDuUng3VOUKsd7S+zqvbOTiGXHfxX4wK31ard+hoNuvkicM/2YFzlpDgABOevKSsB4G/FA=="
    },
    "postcss-calc@9.0.1_postcss@8.4.35": {
      "integrity": "sha512-TipgjGyzP5QzEhsOZUaIkeO5mKeMFpebWzRogWG/ysonUlnHcq5aJe0jOjpfzUU8PeSaBQnrE8ehR0QA5vs8PQ==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-selector-parser",
        "postcss-value-parser"
      ]
    },
    "postcss-colormin@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-x9yX7DOxeMAR+BgGVnNSAxmAj98NX/YxEMNFP+SDCEeNLb2r3i6Hh1ksMsnW8Ub5SLCpbescQqn9YEbE9554Sw==",
      "dependencies": [
        "browserslist",
        "caniuse-api",
        "colord",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-convert-values@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-zx8IwP/ts9WvUM6NkVSkiU902QZL1bwPhaVaLynPtCsOTqp+ZKbNi+s6XJg3rfqpKGA/oc7Oxk5t8pOQJcwl/w==",
      "dependencies": [
        "browserslist",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-discard-comments@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-65w/uIqhSBBfQmYnG92FO1mWZjJ4GL5b8atm5Yw2UgrwD7HiNiSSNwJor1eCFGzUgYnN/iIknhNRVqjrrpuglw==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "postcss-discard-duplicates@6.0.3_postcss@8.4.35": {
      "integrity": "sha512-+JA0DCvc5XvFAxwx6f/e68gQu/7Z9ud584VLmcgto28eB8FqSFZwtrLwB5Kcp70eIoWP/HXqz4wpo8rD8gpsTw==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "postcss-discard-empty@6.0.3_postcss@8.4.35": {
      "integrity": "sha512-znyno9cHKQsK6PtxL5D19Fj9uwSzC2mB74cpT66fhgOadEUPyXFkbgwm5tvc3bt3NAy8ltE5MrghxovZRVnOjQ==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "postcss-discard-overridden@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-j87xzI4LUggC5zND7KdjsI25APtyMuynXZSujByMaav2roV6OZX+8AaCUcZSWqckZpjAjRyFDdpqybgjFO0HJQ==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "postcss-import@15.1.0_postcss@8.5.3": {
      "integrity": "sha512-hpr+J05B2FVYUAXHeK1YyI267J/dDDhMU6B6civm8hSY1jYJnBXxzKDKDswzJmtLHryrjhnDjqqp/49t8FALew==",
      "dependencies": [
        "postcss@8.5.3",
        "postcss-value-parser",
        "read-cache",
        "resolve"
      ]
    },
    "postcss-js@4.0.1_postcss@8.5.3": {
      "integrity": "sha512-dDLF8pEO191hJMtlHFPRa8xsizHaM82MLfNkUHdUtVEV3tgTp5oj+8qbEqYM57SLfc74KSbw//4SeJma2LRVIw==",
      "dependencies": [
        "camelcase-css",
        "postcss@8.5.3"
      ]
    },
    "postcss-load-config@4.0.2_postcss@8.5.3": {
      "integrity": "sha512-bSVhyJGL00wMVoPUzAVAnbEoWyqRxkjv64tUl427SKnPrENtq6hJwUojroMz2VB+Q1edmi4IfrAPpami5VVgMQ==",
      "dependencies": [
        "lilconfig",
        "postcss@8.5.3",
        "yaml"
      ],
      "optionalPeers": [
        "postcss@8.5.3"
      ]
    },
    "postcss-merge-longhand@6.0.5_postcss@8.4.35": {
      "integrity": "sha512-5LOiordeTfi64QhICp07nzzuTDjNSO8g5Ksdibt44d+uvIIAE1oZdRn8y/W5ZtYgRH/lnLDlvi9F8btZcVzu3w==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser",
        "stylehacks"
      ]
    },
    "postcss-merge-rules@6.1.1_postcss@8.4.35": {
      "integrity": "sha512-KOdWF0gju31AQPZiD+2Ar9Qjowz1LTChSjFFbS+e2sFgc4uHOp3ZvVX4sNeTlk0w2O31ecFGgrFzhO0RSWbWwQ==",
      "dependencies": [
        "browserslist",
        "caniuse-api",
        "cssnano-utils",
        "postcss@8.4.35",
        "postcss-selector-parser"
      ]
    },
    "postcss-minify-font-values@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-gklfI/n+9rTh8nYaSJXlCo3nOKqMNkxuGpTn/Qm0gstL3ywTr9/WRKznE+oy6fvfolH6dF+QM4nCo8yPLdvGJg==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-minify-gradients@6.0.3_postcss@8.4.35": {
      "integrity": "sha512-4KXAHrYlzF0Rr7uc4VrfwDJ2ajrtNEpNEuLxFgwkhFZ56/7gaE4Nr49nLsQDZyUe+ds+kEhf+YAUolJiYXF8+Q==",
      "dependencies": [
        "colord",
        "cssnano-utils",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-minify-params@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-bmSKnDtyyE8ujHQK0RQJDIKhQ20Jq1LYiez54WiaOoBtcSuflfK3Nm596LvbtlFcpipMjgClQGyGr7GAs+H1uA==",
      "dependencies": [
        "browserslist",
        "cssnano-utils",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-minify-selectors@6.0.4_postcss@8.4.35": {
      "integrity": "sha512-L8dZSwNLgK7pjTto9PzWRoMbnLq5vsZSTu8+j1P/2GB8qdtGQfn+K1uSvFgYvgh83cbyxT5m43ZZhUMTJDSClQ==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-selector-parser"
      ]
    },
    "postcss-nested@6.2.0_postcss@8.5.3": {
      "integrity": "sha512-HQbt28KulC5AJzG+cZtj9kvKB93CFCdLvog1WFLf1D+xmMvPGlBstkpTEZfK5+AN9hfJocyBFCNiqyS48bpgzQ==",
      "dependencies": [
        "postcss@8.5.3",
        "postcss-selector-parser"
      ]
    },
    "postcss-normalize-charset@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-a8N9czmdnrjPHa3DeFlwqst5eaL5W8jYu3EBbTTkI5FHkfMhFZh1EGbku6jhHhIzTA6tquI2P42NtZ59M/H/kQ==",
      "dependencies": [
        "postcss@8.4.35"
      ]
    },
    "postcss-normalize-display-values@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-8H04Mxsb82ON/aAkPeq8kcBbAtI5Q2a64X/mnRRfPXBq7XeogoQvReqxEfc0B4WPq1KimjezNC8flUtC3Qz6jg==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-positions@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-/JFzI441OAB9O7VnLA+RtSNZvQ0NCFZDOtp6QPFo1iIyawyXg0YI3CYM9HBy1WvwCRHnPep/BvI1+dGPKoXx/Q==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-repeat-style@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-YdCgsfHkJ2jEXwR4RR3Tm/iOxSfdRt7jplS6XRh9Js9PyCR/aka/FCb6TuHT2U8gQubbm/mPmF6L7FY9d79VwQ==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-string@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-vQZIivlxlfqqMp4L9PZsFE4YUkWniziKjQWUtsxUiVsSSPelQydwS8Wwcuw0+83ZjPWNTl02oxlIvXsmmG+CiQ==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-timing-functions@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-a+YrtMox4TBtId/AEwbA03VcJgtyW4dGBizPl7e88cTFULYsprgHWTbfyjSLyHeBcK/Q9JhXkt2ZXiwaVHoMzA==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-unicode@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-QVC5TQHsVj33otj8/JD869Ndr5Xcc/+fwRh4HAsFsAeygQQXm+0PySrKbr/8tkDKzW+EVT3QkqZMfFrGiossDg==",
      "dependencies": [
        "browserslist",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-url@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-kVNcWhCeKAzZ8B4pv/DnrU1wNh458zBNp8dh4y5hhxih5RZQ12QWMuQrDgPRw3LRl8mN9vOVfHl7uhvHYMoXsQ==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-normalize-whitespace@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-sXZ2Nj1icbJOKmdjXVT9pnyHQKiSAyuNQHSgRCUgThn2388Y9cGVDR+E9J9iAYbSbLHI+UUwLVl1Wzco/zgv0Q==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-ordered-values@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-VRZSOB+JU32RsEAQrO94QPkClGPKJEL/Z9PCBImXMhIeK5KAYo6slP/hBYlLgrCjFxyqvn5VC81tycFEDBLG1Q==",
      "dependencies": [
        "cssnano-utils",
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-reduce-initial@6.1.0_postcss@8.4.35": {
      "integrity": "sha512-RarLgBK/CrL1qZags04oKbVbrrVK2wcxhvta3GCxrZO4zveibqbRPmm2VI8sSgCXwoUHEliRSbOfpR0b/VIoiw==",
      "dependencies": [
        "browserslist",
        "caniuse-api",
        "postcss@8.4.35"
      ]
    },
    "postcss-reduce-transforms@6.0.2_postcss@8.4.35": {
      "integrity": "sha512-sB+Ya++3Xj1WaT9+5LOOdirAxP7dJZms3GRcYheSPi1PiTMigsxHAdkrbItHxwYHr4kt1zL7mmcHstgMYT+aiA==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser"
      ]
    },
    "postcss-selector-parser@6.1.2": {
      "integrity": "sha512-Q8qQfPiZ+THO/3ZrOrO0cJJKfpYCagtMUkXbnEfmgUjwXg6z/WBeOyS9APBBPCTSiDV+s4SwQGu8yFsiMRIudg==",
      "dependencies": [
        "cssesc",
        "util-deprecate"
      ]
    },
    "postcss-svgo@6.0.3_postcss@8.4.35": {
      "integrity": "sha512-dlrahRmxP22bX6iKEjOM+c8/1p+81asjKT+V5lrgOH944ryx/OHpclnIbGsKVd3uWOXFLYJwCVf0eEkJGvO96g==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-value-parser",
        "svgo"
      ]
    },
    "postcss-unique-selectors@6.0.4_postcss@8.4.35": {
      "integrity": "sha512-K38OCaIrO8+PzpArzkLKB42dSARtC2tmG6PvD4b1o1Q2E9Os8jzfWFfSy/rixsHwohtsDdFtAWGjFVFUdwYaMg==",
      "dependencies": [
        "postcss@8.4.35",
        "postcss-selector-parser"
      ]
    },
    "postcss-value-parser@4.2.0": {
      "integrity": "sha512-1NNCs6uurfkVbeXG4S8JFT9t19m45ICnif8zWLd5oPSZ50QnwMfK+H3jv408d4jw/7Bttv5axS5IiHoLaVNHeQ=="
    },
    "postcss@8.4.35": {
      "integrity": "sha512-u5U8qYpBCpN13BsiEB0CbR1Hhh4Gc0zLFuedrHJKMctHCHAGrMdG0PRM/KErzAL3CU6/eckEtmHNB3x6e3c0vA==",
      "dependencies": [
        "nanoid",
        "picocolors",
        "source-map-js"
      ]
    },
    "postcss@8.5.3": {
      "integrity": "sha512-dle9A3yYxlBSrt8Fu+IpjGT8SY8hN0mlaA6GY8t0P5PjIOZemULz/E2Bnm/2dcUOena75OTNkHI76uZBNUUq3A==",
      "dependencies": [
        "nanoid",
        "picocolors",
        "source-map-js"
      ]
    },
    "preact-render-to-string@6.5.13_preact@10.26.6": {
      "integrity": "sha512-iGPd+hKPMFKsfpR2vL4kJ6ZPcFIoWZEcBf0Dpm3zOpdVvj77aY8RlLiQji5OMrngEyaxGogeakTb54uS2FvA6w==",
      "dependencies": [
        "preact"
      ]
    },
    "preact@10.26.6": {
      "integrity": "sha512-5SRRBinwpwkaD+OqlBDeITlRgvd8I8QlxHJw9AxSdMNV6O+LodN9nUyYGpSF7sadHjs6RzeFShMexC6DbtWr9g=="
    },
    "prismjs@1.30.0": {
      "integrity": "sha512-DEvV2ZF2r2/63V+tK8hQvrR2ZGn10srHbXviTlcv7Kpzw8jWiNTqbVgjO3IY8RxrrOUF8VPMQQFysYYYv0YZxw=="
    },
    "queue-microtask@1.2.3": {
      "integrity": "sha512-NuaNSa6flKT5JaSYQzJok04JzTL1CA6aGhv5rfLW3PgqA+M2ChpZQnAC8h8i4ZFkBS8X5RqkDBHA7r4hej3K9A=="
    },
    "read-cache@1.0.0": {
      "integrity": "sha512-Owdv/Ft7IjOgm/i0xvNDZ1LrRANRfew4b2prF3OWMQLxLfu3bS8FVhCsrSCMK4lR56Y9ya+AThoTpDCTxCmpRA==",
      "dependencies": [
        "pify"
      ]
    },
    "readdirp@3.6.0": {
      "integrity": "sha512-hOS089on8RduqdbhvQ5Z37A0ESjsqz6qnRcffsMU3495FuTdqSm+7bhJ29JvIOsBDEEnan5DPu9t3To9VRlMzA==",
      "dependencies": [
        "picomatch"
      ]
    },
    "resolve@1.22.10": {
      "integrity": "sha512-NPRy+/ncIMeDlTAsuqwKIiferiawhefFJtkNSW0qZJEqMEb+qBt/77B/jGeeek+F0uOeN05CDa6HXbbIgtVX4w==",
      "dependencies": [
        "is-core-module",
        "path-parse",
        "supports-preserve-symlinks-flag"
      ],
      "bin": true
    },
    "reusify@1.1.0": {
      "integrity": "sha512-g6QUff04oZpHs0eG5p83rFLhHeV00ug/Yf9nZM6fLeUrPguBTkTQOdpAWWspMh55TZfVQDPaN3NQJfbVRAxdIw=="
    },
    "run-parallel@1.2.0": {
      "integrity": "sha512-5l4VyZR86LZ/lDxZTR6jqL8AFE2S0IFLMP26AbjsLVADxHdhB/c0GUsH+y39UfCi3dzz8OlQuPmnaJOMoDHQBA==",
      "dependencies": [
        "queue-microtask"
      ]
    },
    "shebang-command@2.0.0": {
      "integrity": "sha512-kHxr2zZpYtdmrN1qDjrrX/Z1rR1kG8Dx+gkpK1G4eXmvXswmcE1hTWBWYUzlraYw1/yZp6YuDY77YtvbN0dmDA==",
      "dependencies": [
        "shebang-regex"
      ]
    },
    "shebang-regex@3.0.0": {
      "integrity": "sha512-7++dFhtcx3353uBaq8DDR4NuxBetBzC7ZQOhmTQInHEd6bSrXdiEyzCvG07Z44UYdLShWUyXt5M/yhz8ekcb1A=="
    },
    "signal-exit@4.1.0": {
      "integrity": "sha512-bzyZ1e88w9O1iNJbKnOlvYTrWPDl46O1bG0D3XInv+9tkPrxrN8jUUTiFlDkkmKWgn1M6CfIA13SuGqOa9Korw=="
    },
    "source-map-js@1.2.1": {
      "integrity": "sha512-UXWMKhLOwVKb728IUtQPXxfYU+usdybtUrK/8uGE8CQMvrhOpwvzDBwj0QhSL7MQc7vIsISBG8VQ8+IDQxpfQA=="
    },
    "string-width@4.2.3": {
      "integrity": "sha512-wKyQRQpjJ0sIp62ErSZdGsjMJWsap5oRNihHhu6G7JVO/9jIB6UyevL+tXuOqrng8j/cxKTWyWUwvSTriiZz/g==",
      "dependencies": [
        "emoji-regex@8.0.0",
        "is-fullwidth-code-point",
        "strip-ansi@6.0.1"
      ]
    },
    "string-width@5.1.2": {
      "integrity": "sha512-HnLOCR3vjcY8beoNLtcjZ5/nxn2afmME6lhrDrebokqMap+XbeW8n9TXpPDOqdGK5qcI3oT0GKTW6wC7EMiVqA==",
      "dependencies": [
        "eastasianwidth",
        "emoji-regex@9.2.2",
        "strip-ansi@7.1.0"
      ]
    },
    "strip-ansi@6.0.1": {
      "integrity": "sha512-Y38VPSHcqkFrCpFnQ9vuSXmquuv5oXOKpGeT6aGrr3o3Gc9AlVa6JBfUSOCnbxGGZF+/0ooI7KrPuUSztUdU5A==",
      "dependencies": [
        "ansi-regex@5.0.1"
      ]
    },
    "strip-ansi@7.1.0": {
      "integrity": "sha512-iq6eVVI64nQQTRYq2KtEg2d2uU7LElhTJwsH4YzIHZshxlgZms/wIc4VoDQTlG/IvVIrBKG06CrZnp0qv7hkcQ==",
      "dependencies": [
        "ansi-regex@6.1.0"
      ]
    },
    "stylehacks@6.1.1_postcss@8.4.35": {
      "integrity": "sha512-gSTTEQ670cJNoaeIp9KX6lZmm8LJ3jPB5yJmX8Zq/wQxOsAFXV3qjWzHas3YYk1qesuVIyYWWUpZ0vSE/dTSGg==",
      "dependencies": [
        "browserslist",
        "postcss@8.4.35",
        "postcss-selector-parser"
      ]
    },
    "sucrase@3.35.0": {
      "integrity": "sha512-8EbVDiu9iN/nESwxeSxDKe0dunta1GOlHufmSSXxMD2z2/tMZpDMpvXQGsc+ajGo8y2uYUmixaSRUc/QPoQ0GA==",
      "dependencies": [
        "@jridgewell/gen-mapping",
        "commander@4.1.1",
        "glob",
        "lines-and-columns",
        "mz",
        "pirates",
        "ts-interface-checker"
      ],
      "bin": true
    },
    "supports-preserve-symlinks-flag@1.0.0": {
      "integrity": "sha512-ot0WnXS9fgdkgIcePe6RHNk1WA8+muPa6cSjeR3V8K27q9BB1rTE3R1p7Hv0z1ZyAc8s6Vvv8DIyWf681MAt0w=="
    },
    "svgo@3.3.2": {
      "integrity": "sha512-OoohrmuUlBs8B8o6MB2Aevn+pRIH9zDALSR+6hhqVfa6fRwG/Qw9VUMSMW9VNg2CFc/MTIfabtdOVl9ODIJjpw==",
      "dependencies": [
        "@trysound/sax",
        "commander@7.2.0",
        "css-select",
        "css-tree@2.3.1",
        "css-what",
        "csso",
        "picocolors"
      ],
      "bin": true
    },
    "tailwindcss@3.4.17_postcss@8.5.3": {
      "integrity": "sha512-w33E2aCvSDP0tW9RZuNXadXlkHXqFzSkQew/aIa2i/Sj8fThxwovwlXHSPXTbAHwEIhBFXAedUhP2tueAKP8Og==",
      "dependencies": [
        "@alloc/quick-lru",
        "arg",
        "chokidar",
        "didyoumean",
        "dlv",
        "fast-glob",
        "glob-parent@6.0.2",
        "is-glob",
        "jiti",
        "lilconfig",
        "micromatch",
        "normalize-path",
        "object-hash",
        "picocolors",
        "postcss@8.5.3",
        "postcss-import",
        "postcss-js",
        "postcss-load-config",
        "postcss-nested",
        "postcss-selector-parser",
        "resolve",
        "sucrase"
      ],
      "bin": true
    },
    "thenify-all@1.6.0": {
      "integrity": "sha512-RNxQH/qI8/t3thXJDwcstUO4zeqo64+Uy/+sNVRBx4Xn2OX+OZ9oP+iJnNFqplFra2ZUVeKCSa2oVWi3T4uVmA==",
      "dependencies": [
        "thenify"
      ]
    },
    "thenify@3.3.1": {
      "integrity": "sha512-RVZSIV5IG10Hk3enotrhvz0T9em6cyHBLkH/YAZuKqd8hRkKhSfCGIcP2KUY0EPxndzANBmNllzWPwak+bheSw==",
      "dependencies": [
        "any-promise"
      ]
    },
    "to-regex-range@5.0.1": {
      "integrity": "sha512-65P7iz6X5yEr1cwcgvQxbbIw7Uk3gOy5dIdtZ4rDveLqhrdJP+Li/Hx6tyK0NEb+2GCyneCMJiGqrADCSNk8sQ==",
      "dependencies": [
        "is-number"
      ]
    },
    "ts-interface-checker@0.1.13": {
      "integrity": "sha512-Y/arvbn+rrz3JCKl9C4kVNfTfSm2/mEp5FSz5EsZSANGPSlQrpRI5M4PKF+mJnE52jOO90PnPSc3Ur3bTQw0gA=="
    },
    "ts-morph@25.0.1": {
      "integrity": "sha512-QJEiTdnz1YjrB3JFhd626gX4rKHDLSjSVMvGGG4v7ONc3RBwa0Eei98G9AT9uNFDMtV54JyuXsFeC+OH0n6bXQ==",
      "dependencies": [
        "@ts-morph/common",
        "code-block-writer"
      ]
    },
    "uhyphen@0.2.0": {
      "integrity": "sha512-qz3o9CHXmJJPGBdqzab7qAYuW8kQGKNEuoHFYrBwV6hWIMcpAmxDLXojcHfFr9US1Pe6zUswEIJIbLI610fuqA=="
    },
    "undici-types@6.21.0": {
      "integrity": "sha512-iwDZqg0QAGrg9Rav5H4n0M64c3mkR59cJ6wQp+7C4nI0gsmExaedaYLNO44eT4AtBBwjbTiGPMlt2Md0T9H9JQ=="
    },
    "update-browserslist-db@1.1.3_browserslist@4.24.5": {
      "integrity": "sha512-UxhIZQ+QInVdunkDAaiazvvT/+fXL5Osr0JZlJulepYu6Jd7qJtDZjlur0emRlT71EN3ScPoE7gvsuIKKNavKw==",
      "dependencies": [
        "browserslist",
        "escalade",
        "picocolors"
      ],
      "bin": true
    },
    "util-deprecate@1.0.2": {
      "integrity": "sha512-EPD5q1uXyFxJpCrLnCc1nHnq3gOa6DZBocAIiI2TaSCA7VCJ1UJDMagCzIkXNsUYfD1daK//LTEQ8xiIbrHtcw=="
    },
    "which@2.0.2": {
      "integrity": "sha512-BLI3Tl1TW3Pvl70l3yq3Y64i+awpwXqsGBYWkkqMtnbXgrMD+yj7rhW0kuEDxzJaYXGjEW5ogapKNMEKNMjibA==",
      "dependencies": [
        "isexe"
      ],
      "bin": true
    },
    "wrap-ansi@7.0.0": {
      "integrity": "sha512-YVGIj2kamLSTxw6NsZjoBxfSwsn0ycdesmc4p+Q21c5zPuZ1pl+NfxVdxPtdHvmNVOQ6XSYG4AUtyt/Fi7D16Q==",
      "dependencies": [
        "ansi-styles@4.3.0",
        "string-width@4.2.3",
        "strip-ansi@6.0.1"
      ]
    },
    "wrap-ansi@8.1.0": {
      "integrity": "sha512-si7QWI6zUMq56bESFvagtmzMdGOtoxfR+Sez11Mobfc7tm+VkUckk9bW2UeffTGVUbOksxmSw0AA2gs8g71NCQ==",
      "dependencies": [
        "ansi-styles@6.2.1",
        "string-width@5.1.2",
        "strip-ansi@7.1.0"
      ]
    },
    "yaml@2.7.1": {
      "integrity": "sha512-10ULxpnOCQXxJvBgxsn9ptjq6uviG/htZKk9veJGhlqn3w/DxQ631zFF+nlQXLwmImeS5amR2dl2U8sg6U9jsQ==",
      "bin": true
    }
  },
  "redirects": {
    "https://esm.sh/@types/react@~19.0.7/index.d.ts": "https://esm.sh/@types/react@19.0.14/index.d.ts",
    "https://github.com/denoland/std/raw/refs/heads/main/_tools/check_docs.ts": "https://raw.githubusercontent.com/denoland/std/refs/heads/main/_tools/check_docs.ts"
  },
  "remote": {
    "https://deno.land/std@0.120.0/async/deadline.ts": "1d6ac7aeaee22f75eb86e4e105d6161118aad7b41ae2dd14f4cfd3bf97472b93",
    "https://deno.land/std@0.120.0/async/debounce.ts": "b2f693e4baa16b62793fd618de6c003b63228db50ecfe3bd51fc5f6dc0bc264b",
    "https://deno.land/std@0.120.0/async/deferred.ts": "ab60d46ba561abb3b13c0c8085d05797a384b9f182935f051dc67136817acdee",
    "https://deno.land/std@0.120.0/async/delay.ts": "f2d8ccaa8ebc26594bd8b0989edfd8a96257a714c1dee2fb54d986e5bdd840ac",
    "https://deno.land/std@0.120.0/async/mod.ts": "78425176fabea7bd1046ce3819fd69ce40da85c83e0f174d17e8e224a91f7d10",
    "https://deno.land/std@0.120.0/async/mux_async_iterator.ts": "62abff3af9ff619e8f2adc96fc70d4ca020fa48a50c23c13f12d02ed2b760dbe",
    "https://deno.land/std@0.120.0/async/pool.ts": "353ce4f91865da203a097aa6f33de8966340c91b6f4a055611c8c5d534afd12f",
    "https://deno.land/std@0.120.0/async/tee.ts": "3e9f2ef6b36e55188de16a667c702ace4ad0cf84e3720379160e062bf27348ad",
    "https://deno.land/std@0.120.0/http/http_status.ts": "2ff185827bff21c7be2807fcb09a6a2166464ba57fcd94afe805abab8e09070a",
    "https://deno.land/std@0.120.0/http/server.ts": "d0be8a9da160255623e645f5b515fa1c6b65eecfbb9cad87ef8002d4f8d56616",
    "https://deno.land/std@0.143.0/_util/assert.ts": "e94f2eb37cebd7f199952e242c77654e43333c1ac4c5c700e929ea3aa5489f74",
    "https://deno.land/std@0.143.0/datetime/formatter.ts": "7c8e6d16a0950f400aef41b9f1eb9168249869776ec520265dfda785d746589e",
    "https://deno.land/std@0.143.0/datetime/mod.ts": "dcab9ae7be83cbf74b7863e83bd16e7c646a8dea2f019092905630eb7a545739",
    "https://deno.land/std@0.143.0/datetime/tokenizer.ts": "7381e28f6ab51cb504c7e132be31773d73ef2f3e1e50a812736962b9df1e8c47",
    "https://deno.land/std@0.143.0/http/cookie.ts": "526f27762fad7bf84fbe491de7eba7c406057501eec6edcad7884a16b242fddf",
    "https://deno.land/std@0.93.0/_util/assert.ts": "2f868145a042a11d5ad0a3c748dcf580add8a0dbc0e876eaa0026303a5488f58",
    "https://deno.land/std@0.93.0/_util/os.ts": "e282950a0eaa96760c0cf11e7463e66babd15ec9157d4c9ed49cc0925686f6a7",
    "https://deno.land/std@0.93.0/fs/walk.ts": "8d37f2164a7397668842a7cb5d53b9e7bcd216462623b1b96abe519f76d7f8b9",
    "https://deno.land/std@0.93.0/path/_constants.ts": "1247fee4a79b70c89f23499691ef169b41b6ccf01887a0abd131009c5581b853",
    "https://deno.land/std@0.93.0/path/_interface.ts": "1fa73b02aaa24867e481a48492b44f2598cd9dfa513c7b34001437007d3642e4",
    "https://deno.land/std@0.93.0/path/_util.ts": "2e06a3b9e79beaf62687196bd4b60a4c391d862cfa007a20fc3a39f778ba073b",
    "https://deno.land/std@0.93.0/path/common.ts": "eaf03d08b569e8a87e674e4e265e099f237472b6fd135b3cbeae5827035ea14a",
    "https://deno.land/std@0.93.0/path/glob.ts": "4a524c1c9da3e79a9fdabdc6e850cd9e41bdf31e442856ffa19c5b123268ca95",
    "https://deno.land/std@0.93.0/path/mod.ts": "4465dc494f271b02569edbb4a18d727063b5dbd6ed84283ff906260970a15d12",
    "https://deno.land/std@0.93.0/path/posix.ts": "f56c3c99feb47f30a40ce9d252ef6f00296fa7c0fcb6dd81211bdb3b8b99ca3b",
    "https://deno.land/std@0.93.0/path/separator.ts": "8fdcf289b1b76fd726a508f57d3370ca029ae6976fcde5044007f062e643ff1c",
    "https://deno.land/std@0.93.0/path/win32.ts": "77f7b3604e0de40f3a7c698e8a79e7f601dc187035a1c21cb1e596666ce112f8",
    "https://deno.land/x/case@2.1.1/lowerCase.ts": "86d5533f9587ed60003181591e40e648838c23f371edfa79d00288153d113b16",
    "https://deno.land/x/case@2.1.1/normalCase.ts": "6a8b924da9ab0790d99233ae54bfcfc996d229cb91b2533639fe20972cc33dac",
    "https://deno.land/x/case@2.1.1/snakeCase.ts": "ee2ab4e2c931d30bb79190d090c21eb5c00d1de1b7a9a3e7f33e035ae431333b",
    "https://deno.land/x/case@2.1.1/types.ts": "8e2bd6edaa27c0d1972c0d5b76698564740f37b4d3787d58d1fb5f48de611e61",
    "https://deno.land/x/case@2.1.1/vendor/camelCaseRegexp.ts": "7d9ff02aad4ab6429eeab7c7353f7bcdd6cc5909a8bd3dda97918c8bbb7621ae",
    "https://deno.land/x/case@2.1.1/vendor/camelCaseUpperRegexp.ts": "292de54a698370f90adcdf95727993d09888b7f33d17f72f8e54ba75f7791787",
    "https://deno.land/x/case@2.1.1/vendor/nonWordRegexp.ts": "c1a052629a694144b48c66b0175a22a83f4d61cb40f4e45293fc5d6b123f927e",
    "https://deno.land/x/imagescript@1.3.0/ImageScript.js": "cf90773c966031edd781ed176c598f7ed495e7694cd9b86c986d2d97f783cca0",
    "https://deno.land/x/imagescript@1.3.0/mod.ts": "18a6cb83c55e690c873505f6fe867364c678afb64934fe7aef593a6b92f79995",
    "https://deno.land/x/imagescript@1.3.0/png/src/crc.mjs": "5cf50de181d61dd00e66a240d811018ba5070afa8bba302f393604404604de84",
    "https://deno.land/x/imagescript@1.3.0/png/src/mem.mjs": "4968d400dae069b4bf0ef4767c1802fd2cc7d15d90eda4cfadf5b4cd19b96c6d",
    "https://deno.land/x/imagescript@1.3.0/png/src/png.mjs": "96ef0ceff1b5a6cd9304749e5f187b4ab238509fb5f9a8be8ee934240271ed8d",
    "https://deno.land/x/imagescript@1.3.0/png/src/zlib.mjs": "9867dc3fab1d31b664f9344b0d7e977f493d9c912a76c760d012ed2b89f7061c",
    "https://deno.land/x/imagescript@1.3.0/utils/buffer.js": "952cb1beb8827e50a493a5d1f29a4845e8c648789406d389dd51f51205ba02d8",
    "https://deno.land/x/imagescript@1.3.0/utils/crc32.js": "573d6222b3605890714ebc374e687ec2aa3e9a949223ea199483e47ca4864f7d",
    "https://deno.land/x/imagescript@1.3.0/utils/png.js": "fbed9117e0a70602645d70df9c103ff6e79c03e987bd5c1685dcb4200729b6de",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/font.js": "9e75d842608c057045698d6a7cdf5ffd27241b5cdea0391c89a1917b31294524",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/gif.js": "8b86f7b96486bb8ff50fbc7c7487f86cb5cef85e6acd71e1def78a1aa2f12e4f",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/jpeg.js": "75295e2fcf96b4f7bb894b3844fdaa8140d63169d28b466b5d5be89d59a7b6e6",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/png.js": "0659536a8dd8f892c8346e268b2754b4414fad0ec1e9794dfcde1ba1c804ee02",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/svg.js": "f5c8a9d1977b51a7c07549ceb6bbbaca9497321a193f28b3dc229a42d91bcf14",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/tiff.js": "c2d7bdaef094df25aae1752e75167f485e89275d76a1379e39d8949580b7af4f",
    "https://deno.land/x/imagescript@1.3.0/utils/wasm/zlib.js": "749875f83abffe24d3b977475a0cbd5f9b52bee1fbdbef61ec183cbfc17805f6",
    "https://deno.land/x/imagescript@1.3.0/v2/framebuffer.mjs": "add44ff184636659714b3c6d4b896f628545451abffbc30b5bcc2e8d9a73d012",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/blur.mjs": "80716f1ffab8a2aeb54a036f583bf51a2b9dd37e005adc000add803df8e8a12f",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/color.mjs": "5e72cdcbf97dc939a2795223f01e3cb0544c0c56b03ea2aa026050df58348814",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/crop.mjs": "69431fa6f687fd9f0c31eff0ec27d7ac925275005e53a37f0c3fab4cc4d9a9ea",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/fill.mjs": "cf1b9488314753fbc9ebf03410ac74c2a34ea5a69fb6892cd6e8366cd1930d93",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/flip.mjs": "825a34a66567dcf15e76a719f1bf2f66fb106503cd69942292b1b0ae05c5718e",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/index.mjs": "423ba687119be2bba8cec72890577d3afa3621b6b8108912242fe937a183f2aa",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/iterator.mjs": "c2adf3d90ce00719a02c48c97634574176a3501ff026676259bd71aa8f5d69b9",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/overlay.mjs": "7e6e2c2ffd25006d52597ab8babc5f8f503d388a3fdf2fbc0eaea02799a020c9",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/resize.mjs": "814e78ebce8eaf8f1f918688db7b52a141405e06a36ed4b25d04413d69e7d17b",
    "https://deno.land/x/imagescript@1.3.0/v2/ops/rotate.mjs": "a1b65616717bd2eed8db406affea3263b4674dada46b56441ef38167a187455d",
    "https://deno.land/x/imagescript@1.3.0/v2/util/mem.mjs": "4968d400dae069b4bf0ef4767c1802fd2cc7d15d90eda4cfadf5b4cd19b96c6d",
    "https://esm.sh/@docsearch/js@3.5.2/es2020/js.mjs": "9b278cf3c0b26feded7d8efeac8e2b50f76bbafcf173a95002944bcc3482830a",
    "https://esm.sh/@docsearch/js@3.5.2?target=es2020": "4bad084f771a1923fe042ece62a9078f482f8642cb0b1acb890905e58586fee7",
    "https://raw.githubusercontent.com/denoland/ga4/main/mod.ts": "36f72ba1c90b5ebdb811427f367cd95fa6772d2de2fb45d6e57550501ee6d476",
    "https://raw.githubusercontent.com/denoland/std/refs/heads/main/_tools/check_docs.ts": "59c29d6a5de45d04c5cab1078c4aacbed9edd0a6b83a6469f55318665e5be6b2",
    "https://raw.githubusercontent.com/denoland/std/refs/heads/main/_tools/utils.ts": "c2e38ed7e7a9a8c0fbaf8d70aa808fb02f2cbb5e71ef18d634feb4b479c6a001"
  },
  "workspace": {
    "dependencies": [
      "jsr:@astral/astral@~0.5.3",
      "jsr:@deno/doc@0.172",
      "jsr:@fresh/core@^2.0.0-alpha.29",
      "jsr:@fresh/plugin-tailwind@^0.0.1-alpha.7",
      "jsr:@luca/esbuild-deno-loader@0.11",
      "jsr:@marvinh-test/fresh-island@^0.0.1",
      "jsr:@std/async@^1.0.13",
      "jsr:@std/cli@^1.0.17",
      "jsr:@std/collections@^1.0.11",
      "jsr:@std/crypto@1",
      "jsr:@std/datetime@~0.225.2",
      "jsr:@std/encoding@1",
      "jsr:@std/expect@^1.0.16",
      "jsr:@std/fmt@^1.0.7",
      "jsr:@std/front-matter@^1.0.5",
      "jsr:@std/fs@1",
      "jsr:@std/html@1",
      "jsr:@std/http@^1.0.15",
      "jsr:@std/jsonc@1",
      "jsr:@std/media-types@1",
      "jsr:@std/path@1",
      "jsr:@std/semver@1",
      "jsr:@std/streams@1",
      "jsr:@std/testing@^1.0.12",
      "jsr:@std/uuid@^1.0.7",
      "npm:@opentelemetry/api@^1.9.0",
      "npm:@preact/signals@^2.0.4",
      "npm:autoprefixer@10.4.17",
      "npm:cssnano@6.0.3",
      "npm:esbuild-wasm@0.25.4",
      "npm:esbuild@0.25.4",
      "npm:github-slugger@2",
      "npm:linkedom@~0.18.10",
      "npm:marked-mangle@^1.1.9",
      "npm:marked@^15.0.11",
      "npm:postcss@8.4.35",
      "npm:preact-render-to-string@^6.5.11",
      "npm:preact@^10.26.6",
      "npm:prismjs@^1.29.0",
      "npm:tailwindcss@^3.4.1",
      "npm:ts-morph@^25.0.1"
    ]
  }
}



================================================
FILE: LICENSE
================================================
MIT License

Copyright (c) 2021-2023 Luca Casonato

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



================================================
FILE: versions.json
================================================
[
  "1.7.3",
  "1.7.2",
  "1.7.1",
  "1.7.0",
  "1.6.8",
  "1.6.7",
  "1.6.6",
  "1.6.5",
  "1.6.4",
  "1.6.3",
  "1.6.2",
  "1.6.1",
  "1.6.0",
  "1.5.4",
  "1.5.3",
  "1.5.2",
  "1.5.1",
  "1.5.0",
  "1.4.3",
  "1.4.2",
  "1.4.1",
  "1.4.0",
  "1.3.1",
  "1.3.0",
  "1.2.0",
  "1.1.6",
  "1.1.5",
  "1.1.4",
  "1.1.3",
  "1.1.2",
  "1.1.1",
  "1.1.0",
  "1.0.2",
  "1.0.1",
  "1.0.0",
  "1.0.0-rc.6",
  "1.0.0-rc.5",
  "1.0.0-rc.4",
  "1.0.0-rc.3",
  "1.0.0-rc.2",
  "1.0.0-rc.1"
]



================================================
FILE: docs/toc.ts
================================================
import FRESH_VERSIONS from "../versions.json" with { type: "json" };

type RawTableOfContents = Record<
  string,
  {
    label: string;
    content: Record<string, RawTableOfContentsEntry>;
  }
>;

interface RawTableOfContentsEntry {
  title: string;
  link?: string;
  pages?: [string, string, string?][];
}

const toc: RawTableOfContents = {
  canary: {
    label: "canary",
    content: {
      "the-canary-version": {
        title: "The canary version",
      },
      introduction: {
        title: "Introduction",
        link: "latest",
      },
      "getting-started": {
        title: "Getting Started",
        link: "latest",
        pages: [
          ["create-a-project", "Create a project", "link:latest"],
          ["running-locally", "Running locally", "link:latest"],
          ["create-a-route", "Create a route", "link:latest"],
          ["dynamic-routes", "Dynamic routes", "link:latest"],
          ["custom-handlers", "Custom handlers", "link:latest"],
          ["form-submissions", "Form submissions", "link:latest"],
          ["adding-interactivity", "Adding interactivity", "link:latest"],
          ["deploy-to-production", "Deploy to production", "link:latest"],
        ],
      },
      concepts: {
        title: "Concepts",
        link: "latest",
        pages: [
          ["architecture", "Architecture", "link:latest"],
          ["server-components", "Server Components", "link:latest"],
          ["routing", "Routing", "link:latest"],
          ["routes", "Routes", "link:latest"],
          ["app-wrapper", "App wrapper", "link:latest"],
          ["layouts", "Layouts", "link:latest"],
          ["forms", "Forms", "link:latest"],
          ["islands", "Interactive islands", "link:latest"],
          ["static-files", "Static files", "link:latest"],
          ["middleware", "Middlewares", "link:latest"],
          ["error-pages", "Error pages", "link:latest"],
          ["partials", "Partials", "link:latest"],
          ["data-fetching", "Data fetching", "link:latest"],
          ["ahead-of-time-builds", "Ahead-of-time Builds", "link:latest"],
          ["deployment", "Deployment", "link:latest"],
          ["plugins", "Plugins", "link:latest"],
          ["updating", "Updating Fresh", "link:latest"],
          ["server-configuration", "Server configuration", "link:latest"],
        ],
      },
      integrations: {
        title: "Integrations",
        link: "latest",
      },
      examples: {
        title: "Examples",
        link: "latest",
        pages: [
          ["migration-guide", "Migration Guide", "link:canary"],
          ["modifying-the-head", "Modifying the <head>", "link:latest"],
          ["writing-tests", "Writing tests", "link:latest"],
          [
            "changing-the-src-dir",
            "Changing the source directory",
            "link:latest",
          ],
          ["init-the-server", "Initializing the server", "link:latest"],
          ["dealing-with-cors", "Dealing with CORS", "link:latest"],
          ["creating-a-crud-api", "Creating a CRUD API", "link:latest"],
          ["handling-complex-routes", "Handling complex routes", "link:latest"],
          ["rendering-markdown", "Rendering markdown", "link:latest"],
          ["rendering-raw-html", "Rendering raw HTML", "link:latest"],
          [
            "sharing-state-between-islands",
            "Sharing state between islands",
            "link:latest",
          ],
          ["using-csp", "Using CSP", "link:latest"],
          ["active-links", "Styling active links", "link:latest"],
        ],
      },
    },
  },
  latest: {
    label: FRESH_VERSIONS[0],
    content: {
      introduction: {
        title: "Introduction",
      },
      "getting-started": {
        title: "Getting Started",
        pages: [
          ["create-a-project", "Create a project"],
          ["running-locally", "Running locally"],
          ["create-a-route", "Create a route"],
          ["dynamic-routes", "Dynamic routes"],
          ["custom-handlers", "Custom handlers"],
          ["form-submissions", "Form submissions"],
          ["adding-interactivity", "Adding interactivity"],
          ["deploy-to-production", "Deploy to production"],
        ],
      },
      concepts: {
        title: "Concepts",
        pages: [
          ["architecture", "Architecture"],
          ["server-components", "Server Components"],
          ["routing", "Routing"],
          ["routes", "Routes"],
          ["app-wrapper", "App wrapper"],
          ["layouts", "Layouts"],
          ["forms", "Forms"],
          ["islands", "Interactive islands"],
          ["static-files", "Static files"],
          ["middleware", "Middlewares"],
          ["error-pages", "Error pages"],
          ["partials", "Partials"],
          ["data-fetching", "Data fetching"],
          ["ahead-of-time-builds", "Ahead-of-time Builds"],
          ["deployment", "Deployment"],
          ["plugins", "Plugins"],
          ["updating", "Updating Fresh"],
          ["server-configuration", "Server configuration"],
        ],
      },
      integrations: {
        title: "Integrations",
      },
      examples: {
        title: "Examples",
        pages: [
          ["migrating-to-tailwind", "Migrating to Tailwind"],
          ["modifying-the-head", "Modifying the <head>"],
          ["writing-tests", "Writing tests"],
          ["changing-the-src-dir", "Changing the source directory"],
          ["using-twind-v1", "Using Twind v1"],
          ["init-the-server", "Initializing the server"],
          ["using-fresh-canary-version", "Using Fresh canary version"],
          ["dealing-with-cors", "Dealing with CORS"],
          ["creating-a-crud-api", "Creating a CRUD API"],
          ["handling-complex-routes", "Handling complex routes"],
          ["rendering-markdown", "Rendering markdown"],
          ["rendering-raw-html", "Rendering raw HTML"],
          ["sharing-state-between-islands", "Sharing state between islands"],
          ["using-csp", "Using CSP"],
          ["active-links", "Styling active links"],
          [
            "client-side-components-and-libraries",
            "Client only side components",
          ],
        ],
      },
    },
  },
};

export default toc;



================================================
FILE: docs/canary/examples/migration-guide.md
================================================
---
description: |
  Migration guide for Fresh 2.x
---

We tried to keep breaking changes in Fresh 2 as minimal as possible, but some
changes need to be updated manually. Fresh 2 comes with many quality of life
[improvements](TODO) that make it easier to extend and adapt Fresh. We've
created this upgrade guide as part of upgrading our own apps here at Deno.

Use this guide to migrate a Fresh 1.x app to Fresh 2.

## Applying automatic updates

Most changes can be applied automatically with the update script. Start the
update by running it in your project directory:

```sh
deno run -Ar jsr:@fresh/update
```

This will apply most API changes made in Fresh 2
[automatically](#automatic-updates) like changing `$fresh/server.ts` imports to
`fresh`.

## Getting `main.ts` and `dev.ts` ready

Configuring Fresh doesn't require a dedicated config file anymore. You can
delete the `fresh.config.ts` file. The `fresh.gen.ts` manifest file isn't needed
anymore either.

```diff
  routes/
  dev.ts
- fresh.gen.ts
- fresh.config.ts
  main.ts
```

Fresh 2 takes great care in ensuring that code that's only needed during
development is separate from production code. This split makes deployments much
smaller, quicker to upload and allows them to boot up much quicker in
production.

### Updating `dev.ts`

Development related configuration can be passed to the `Builder` class instance
in `dev.ts`. This file is also where you typically set up development-only
plugins like [tailwindcss](https://tailwindcss.com/).

The full `dev.ts` file for newly generated Fresh 2 projects looks like this:

```ts
import { Builder } from "fresh/dev";
import { tailwind } from "@fresh/plugin-tailwind";
import { app } from "./main.ts";

// Pass development only configuration here
const builder = new Builder({ target: "safari12" });

// Example: Enabling the tailwind plugin for Fresh
tailwind(builder, app, {});

// Create optimized assets for the browser when
// running `deno run -A dev.ts build`
if (Deno.args.includes("build")) {
  await builder.build(app);
} else {
  // ...otherwise start the development server
  await builder.listen(app);
}
```

### Updating `main.ts`

Similarly, configuration related to running Fresh in production can be passed to
`new App()`:

```ts
// main.ts
import { App, fsRoutes, staticFiles } from "fresh";

export const app = new App()
  // Add static file serving middleware
  .use(staticFiles());

// Enable file-system based routing
await fsRoutes(app, {
  loadIsland: (path) => import(`./islands/${path}`),
  loadRoute: (path) => import(`./routes/${path}`),
});

// If this module is called directly, start the server
if (import.meta.main) {
  await app.listen();
}
```

## Merging error pages

Both the `_500.tsx` and `_404.tsx` template have been unified into a single
`_error.tsx` template.

```diff
  routes/
-   ├── _404.tsx
-   ├── _500.tsx
+   ├── _error.tsx
    └── ...
```

Inside the `_error.tsx` template you can show different content based on errors
or status codes with the following code:

```tsx
export default function ErrorPage(props: PageProps) {
  const error = props.error; // Contains the thrown Error or HTTPError
  if (error instanceof HttpError) {
    const status = error.status; // HTTP status code

    // Render a 404 not found page
    if (status === 404) {
      return <h1>404 - Page not found</h1>;
    }
  }

  return <h1>Oh no...</h1>;
}
```

## Removal of `<Head>` component

The `<Head>` component was used in Fresh 1.x to add additional tags to the
`<head>` portion of an HTML document from anywhere on the page. This feature was
removed in preparation and due to performance concerns as it required a complex
machinery in the background to work.

Instead, passing head-related data is best done via `ctx.state`

```tsx
// about.tsx
export const handler = {
  GET(ctx) {
    // Set a route specific data in a handler
    ctx.state.title = "About Me";
    return page();
  },
};

// Render that in _app.tsx
export default function AppWrapper(ctx: FreshContext) {
  return (
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        {ctx.state.title ? <title>{ctx.state.title}</title> : null}
      </head>
      <body>
        <ctx.Component />
      </body>
    </html>
  );
}
```

## Update deployment settings

Fresh 2 requires assets to be build during deployment instead of building them
on demand. Run the `deno task build` command as part of your deployment process.
If you have already set up Fresh's 1.x "Ahead-of-Time Builds", then no changes
are necessary.

## Trailing slash handling

The handling trailing slashes has been extracted to an optional middleware that
you can add if needed. This middleware can be used to ensure that URLs always
have a trailing slash at the end or that they will never have one.

```diff
-  import { App, staticFiles } from "fresh";
+  import { App, staticFiles, trailingSlashes } from "fresh";

  export const app = new App({ root: import.meta.url })
    .use(staticFiles())
+   .use(trailingSlashes("never"));
```

## Automatic updates

> [info]: The changes listed here are applied automatically when running the
> [`@fresh/update`](https://jsr.io/@fresh/update) script and you shouldn't need
> to have to do these yourself.

### Unified middleware signatures

Middleware, handler and route component signatures have been unified to all look
the same. Instead of receiving two arguments, they receive one. The `Request`
object is stored on the context object as `ctx.req`.

```diff
- const middleware = (req, ctx) => new Response("ok");
+ const middleware = (ctx) => new Response("ok");
```

Same is true for handlers:

```diff
  export const handler = {
-   GET(req, ctx) {
+   GET(ctx) {
      return new Response("ok");
    },
  };
```

...and async route components:

```diff
-  export default async function MyPage(req: Request, ctx: RouteContext) {
+  export default async function MyPage(props: PageProps) {
    const value = await loadFooValue();
    return <p>foo is: {value}</p>;
  }
```

All the various context interfaces have been consolidated and simplified:

| Fresh 1.x                                     | Fresh 2.x      |
| --------------------------------------------- | -------------- |
| `AppContext`, `LayoutContext`, `RouteContext` | `FreshContext` |

### Context methods

The `ctx.renderNotFound()` method has been removed in favor of throwing an
`HttpError` instance. This allows all middlewares to optionally participate in
error handling. Other properties have been moved or renamed to make it easier to
re-use existing objects internally as a minor performance optimization.

| Fresh 1.x              | Fresh 2.x                  |
| ---------------------- | -------------------------- |
| `ctx.renderNotFound()` | `throw new HttpError(404)` |
| `ctx.basePath`         | `ctx.config.basePath`      |
| `ctx.remoteAddr`       | `ctx.info.remoteAddr`      |

## Getting help

If you run into problems with upgrading your app, reach out to us by creating an
issue here https://github.com/denoland/fresh/issues/new . That way we can
improve this migration guide for everyone.



================================================
FILE: docs/canary/the-canary-version/index.md
================================================
---
description: |
  Learn more about Fresh canary releases
---

The canary version represents the current development state of Fresh. It's
intended for testing work in progress features before it lands in a stable
release. Whenever new code is merged into the `main` branch on
[GitHub](https://github.com/denoland/fresh) a new canary version is created that
matches the hash of the git commit.

With the addition of new features that are not yet in a stable release, we need
a place to update our documentation. That's what this section of the
documentation is about. It contains all information about unreleased features
and changes that will land in a stable release.



================================================
FILE: docs/latest/concepts/ahead-of-time-builds.md
================================================
---
description: |
  Fresh optimize assets ahead of time, which makes pages load way quicker.
---

Fresh enables you to pre-optimize frontend assets before the code is deployed.
During that process the code for Islands will be compressed and optimized, so
that Fresh can send as little code as possible to the browser. Depending on the
amount of code an island needs, this process can take several seconds if done on
the fly server-side.

Doing those optimizations ahead-of-time and deploying the already optimized
assets alongside with your code, allows Fresh to treat them as like any other
static file and can serve it immediately without any further processing. On
pages with islands, having to do no processing greatly speeds up page load
times.

Plugins can build static assets during ahead-of-time builds. This can be used to
pre-process or generate CSS files, for example.

## Creating an optimized build

To have Fresh optimize all the assets, run one of the following commands:

```sh
# As a task in newer Fresh projects
deno task build
# or invoke it manually
deno run -A dev.ts build
```

This will create a `_fresh` folder in the project directory. That folder
contains the optimized assets and a `snapshot.json` file which includes some
metadata for Fresh.

Any other static files generated by plugins will be stored in the
`_fresh/static` subfolder. They will be served the same as other
[static files](/docs/concepts/static-files.md).

> [info]: The `_fresh` folder should not be committed to the repository. Add an
> entry in the `.gitignore` file to ensure that it is not committed. Create that
> file at the root of your git repository if not present.
>
> ```gitignore .gitignore
> # Ignore fresh build directory
> _fresh/
> ```

## Running Fresh with optimized assets

When Fresh is started in non-development mode (usually via `main.ts`), Fresh
will automatically pick up optimized assets when a `_fresh` folder exists. If
found, Fresh will print the following message to the terminal:

```sh Terminal output
Using snapshot found at /path/to/project/_fresh
```

## Deploying an optimized Fresh project

If you are deploying a Fresh project to Deno Deploy, you can use ahead-of-time
builds to optimize the assets before deploying them. This will make your
application load quicker.

Open the Deno Deploy dashboard for your project and head to the "Git
Integration" section in the project settings. Enter `deno task build` in the
"Build command" field and save. This will switch your Deno Deploy project to use
ahead-of-time builds.

## Migrating existing projects with Plugins

If you're using Fresh plugins, extract them into a `fresh.config.ts` file, so
that both the `dev.ts` and `main.ts` script have access to them.

```ts fresh.config.ts
import { defineConfig } from "$fresh/server.ts";
import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.ts";

export default defineConfig({
  plugins: [twindPlugin(twindConfig)],
});
```

```ts main.ts
import { start } from "$fresh/server.ts";
import manifest from "./fresh.gen.ts";
import config from "./fresh.config.ts";

await start(manifest, config);
```

```ts dev.ts
import dev from "$fresh/dev.ts";
import config from "./fresh.config.ts";

await dev(import.meta.url, "./main.ts", config);
```



================================================
FILE: docs/latest/concepts/app-wrapper.md
================================================
---
description: |
  Add a global app wrapper to provide common meta tags or context for application routes.
---

An app wrapper is defined in an `_app.tsx` file in `routes/` folder and is
typically used to create the outer structure of an HTML document. It must
contain a default export that is a regular Preact component. Only one such
wrapper is allowed per application.

The component to be wrapped is received via props, in addition to a few other
things. This allows for the introduction of a global container functioning as a
template which can be conditioned based on state and params. Note that any state
set by middleware is available via `props.state`.

```tsx routes/_app.tsx
import { PageProps } from "$fresh/server.ts";

export default function App({ Component, state }: PageProps) {
  // do something with state here
  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>My Fresh app</title>
      </head>
      <body>
        <Component />
      </body>
    </html>
  );
}
```

## Async app wrapper

Similar to routes and layouts, the app wrapper can be made asynchronous. This
changes the function signature so that the first argument is the `Request`
instance and the second one is the `FreshContext`.

```tsx routes/_app.tsx
import { FreshContext } from "$fresh/server.ts";

export default async function App(req: Request, ctx: FreshContext) {
  const data = await loadData();

  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>My Fresh app</title>
      </head>
      <body>
        <h1>Hello {data.name}</h1>
        <ctx.Component />
      </body>
    </html>
  );
}
```

### Define helper

To make it quicker to type the async app wrapper, Fresh includes a `defineApp`
helper which already infers the correct types for you.

```tsx routes/_app.tsx
import { defineApp } from "$fresh/server.ts";

export default defineApp(async (req, ctx) => {
  const data = await loadData();

  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>My Fresh app</title>
      </head>
      <body>
        <h1>Hello {data.name}</h1>
        <ctx.Component />
      </body>
    </html>
  );
});
```

## Disabling the app wrapper

Rendering the app wrapper can be skipped on a route or layout basis. To do that,
set `skipAppWrapper: true` to the layout or route config.

```tsx routes/my-special-route.tsx
import { RouteConfig } from "$fresh/server.ts";

export const config: RouteConfig = {
  skipAppWrapper: true, // Skip the app wrapper during rendering
};

export default function Page() {
  // ...
}
```



================================================
FILE: docs/latest/concepts/architecture.md
================================================
---
description: |
  Fresh's architecture is designed to make it easy to build fast, scalable, and reliable applications.
---

Fresh is designed to make it easy to build fast, scalable, and reliable
applications. To do this, it makes opinionated decisions about how one should
build web applications. These decisions are backed by strong empirical data
gathered from experts in the field. Some examples of these principles are:

- Page load times should be reduced to a minimum.
- The work performed on the client should be minimized.
- Errors should have a small blast radius - stuff should gracefully degrade.

The single biggest architecture decision that Fresh makes is its usage of the
[islands architecture][islands] pattern. This means that Fresh applications ship
pure HTML to the client by default. Parts of a server-rendered page can then be
independently re-hydrated with interactive widgets (islands). This means that
the client is only responsible for rendering parts of the page that are
interactive enough to warrant the extra effort. Any content that is purely
static does not have related client-side JavaScript and is thus very
lightweight.

<!-- TODO(lucacasonato): elaborate on request handling, form actions, etc. -->

[islands]: https://www.patterns.dev/posts/islands-architecture/



================================================
FILE: docs/latest/concepts/data-fetching.md
================================================
---
description: |
  Data fetching in Fresh happens inside of route handler functions. These can pass route data to the page via page props.
---

Server side data fetching in Fresh is accomplished through asynchronous handler
functions. These handler functions can call a `ctx.render()` function with the
data to be rendered as an argument. This data can then be retrieved by the page
component through the `data` property on the `props`.

Here is an example:

```tsx routes/projects/[id].tsx
interface Project {
  name: string;
  stars: number;
}

export const handler: Handlers<Project> = {
  async GET(_req, ctx) {
    const project = await db.projects.findOne({ id: ctx.params.id });
    if (!project) {
      return ctx.renderNotFound({
        message: "Project does not exist",
      });
    }
    return ctx.render(project);
  },
};

export default function ProjectPage(props: PageProps<Project>) {
  return (
    <div>
      <h1>{props.data.name}</h1>
      <p>{props.data.stars} stars</p>
    </div>
  );
}
```

The type parameter on the `PageProps`, `Handlers`, `Handler`, and `FreshContext`
can be used to enforce a TypeScript type to use for the render data. Fresh
enforces during type checking that the types in all of these fields are
compatible within a single page.

## Asynchronous routes

As a shortcut for combining a `GET` handler with a route, you can define your
route as `async`. An `async` route (a route that returns a promise) will be
called with the `Request` and a `RouteContext` (similar to a `HandlerContext`).
Here is the above example rewritten using this shortcut:

```tsx routes/projects/[id].tsx
interface Project {
  name: string;
  stars: number;
}

export default async function ProjectPage(_req, ctx: FreshContext) {
  const project: Project | null = await db.projects.findOne({
    id: ctx.params.id,
  });

  if (!project) {
    return <h1>Project not found</h1>;
  }

  return (
    <div>
      <h1>{project.name}</h1>
      <p>{project.stars} stars</p>
    </div>
  );
}
```



================================================
FILE: docs/latest/concepts/deployment.md
================================================
---
description: |
  Fresh can be deployed to a variety of platforms easily.
---

While Fresh is designed to be deployed to [Deno Deploy][deno-deploy], it can be
deployed to any system or platform that can run a Deno based web server.

Here are instructions for specific providers / systems:

- [Deno Deploy](#deno-deploy)
- [Docker](#docker)

## Deno Deploy

The recommended way to deploy Fresh is by using Deno Deploy. Deno Deploy
provides a GitHub integration that can deploy your Fresh projects to its
globally distributed edge network in seconds, automatically.

View [the getting started guide][deploy-to-production] for instructions on how
to deploy Fresh to Deno Deploy.

## Docker

You can deploy Fresh to any platform that can run Docker containers. Docker is a
tool to containerize projects and portably run them on any supported platform.

When packaging your Fresh app for Docker, it is important that you set the
`DENO_DEPLOYMENT_ID` environment variable in your container. This variable needs
to be set to an opaque string ID that represents the version of your application
that is currently being run. This could be a Git commit hash, or a hash of all
files in your project. It is critical for the function of Fresh that this ID
changes when _any_ file in your project changes - if it doesn't, incorrect
caching **will** cause your project to not function correctly.

Here is an example `Dockerfile` for a Fresh project:

```dockerfile Dockerfile
FROM denoland/deno:1.38.3

ARG GIT_REVISION
ENV DENO_DEPLOYMENT_ID=${GIT_REVISION}

WORKDIR /app

COPY . .
RUN deno cache main.ts

EXPOSE 8000

CMD ["run", "-A", "main.ts"]
```

To build your Docker image inside of a Git repository:

```sh Terminal
$ docker build --build-arg GIT_REVISION=$(git rev-parse HEAD) -t my-fresh-app .
```

Then run your Docker container:

```sh Terminal
$ docker run -t -i -p 80:8000 my-fresh-app
```

To deploy to a cloud provider, push it to a container registry and follow their
documentation.

- [Amazon Web Services][aws-container-registry]
- [Google Cloud][gcp-container-registry]

## Self Contained Executable

With Deno 2.1, you can create a self-contained executable of your Fresh project
that includes all assets and dependencies. This executable can run on any
platform without requiring Deno to be installed.

```sh Terminal
$ deno task build
$ deno compile --include static --include _fresh --include deno.json -A main.ts
```

[aws-container-registry]: https://docs.aws.amazon.com/AmazonECS/latest/userguide/create-container-image.html#create-container-image-push-ecr
[gcp-container-registry]: https://cloud.google.com/container-registry/docs/pushing-and-pulling
[deno-deploy]: https://deno.com/deploy
[deploy-to-production]: /docs/getting-started/deploy-to-production



================================================
FILE: docs/latest/concepts/error-pages.md
================================================
---
description: |
  Error pages can be used to customize the page that is shown when an error occurs in the application.
---

Fresh supports customizing the `404 Not Found`, and the
`500 Internal Server Error` pages. These are shown when a request is made but no
matching route exists, and when a middleware, route handler, or page component
throws an error respectively.

### 404: Not Found

The 404 page can be customized by creating a `_404.tsx` file in the `routes/`
folder. The file must have a default export that is a regular Preact component.
A props object of type `PageProps` is passed in as an argument.

```tsx routes/_404.tsx
import { PageProps } from "$fresh/server.ts";

export default function NotFoundPage({ url }: PageProps) {
  return <p>404 not found: {url.pathname}</p>;
}
```

#### Manually render 404 pages

The `_404.tsx` file will be invoked automatically when no route matches the URL.
In some cases, one needs to manually trigger the rendering of the 404 page, for
example when the route did match, but the requested resource does not exist.
This can be achieved with `ctx.renderNotFound`.

```tsx routes/blog/[slug].tsx
import { Handlers, PageProps } from "$fresh/server.ts";

export const handler: Handlers = {
  async GET(req, ctx) {
    const blogpost = await fetchBlogpost(ctx.params.slug);
    if (!blogpost) {
      return ctx.renderNotFound({
        custom: "prop",
      });
    }
    return ctx.render({ blogpost });
  },
};

export default function BlogpostPage({ data }) {
  return (
    <article>
      <h1>{data.blogpost.title}</h1>
      {/* rest of your page */}
    </article>
  );
}
```

This can also be achieved by throwing an error, if you're uninterested in
passing specific data to your 404 page:

```tsx
import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  GET(_req, _ctx) {
    throw new Deno.errors.NotFound();
  },
};
```

### 500: Internal Server Error

The 500 page can be customized by creating a `_500.tsx` file in the `routes/`
folder. The file must have a default export that is a regular Preact component.
A props object of type `PageProps` is passed in as an argument.

```tsx routes/_500.tsx
import { PageProps } from "$fresh/server.ts";

export default function Error500Page({ error }: PageProps) {
  return <p>500 internal error: {(error as Error).message}</p>;
}
```



================================================
FILE: docs/latest/concepts/forms.md
================================================
---
description: |
  Robustly handle user inputs using HTML `<form>` elements client side, and form
  submission handlers server side.
---

For stronger resiliency and user experience, Fresh relies on native browser
support for form submissions with the HTML `<form>` element.

In the browser, a `<form>` submit will send an HTML action (usually `GET` or
`POST`) to the server, which responds with a new page to render.

## POST request with `application/x-www-form-urlencoded`

Forms typically submit as a `GET` request with data encoded in the URL's search
parameters, or as a `POST` request with either an
`application/x-www-form-urlencoded` or `multipart/form-data` body.

This example demonstrates how to handle `application/x-www-form-urlencoded`
`<form>` submissions:

```tsx routes/subscribe.tsx
import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  async GET(req, ctx) {
    return await ctx.render();
  },
  async POST(req, ctx) {
    const form = await req.formData();
    const email = form.get("email")?.toString();

    // Add email to list.

    // Redirect user to thank you page.
    const headers = new Headers();
    headers.set("location", "/thanks-for-subscribing");
    return new Response(null, {
      status: 303, // See Other
      headers,
    });
  },
};

export default function Subscribe() {
  return (
    <>
      <form method="post">
        <input type="email" name="email" value="" />
        <button type="submit">Subscribe</button>
      </form>
    </>
  );
}
```

When the user submits the form, Deno will retrieve the `email` value using the
request's `formData()` method, add the email to a list, and redirect the user to
a thank you page.

## Handling file uploads

File uploads can be handled in a very similar manner to the example above. Note
that this time, we have to explicitly declare the form's encoding to be
`multipart/form-data`.

```tsx routes/subscribe.tsx
import { Handlers, type PageProps } from "$fresh/server.ts";

interface Props {
  message: string | null;
}

export const handler: Handlers<Props> = {
  async GET(req, ctx) {
    return await ctx.render({
      message: null,
    });
  },
  async POST(req, ctx) {
    const form = await req.formData();
    const file = form.get("my-file") as File;

    if (!file) {
      return ctx.render({
        message: `Please try again`,
      });
    }

    const name = file.name;
    const contents = await file.text();

    console.log(contents);

    return ctx.render({
      message: `${name} uploaded!`,
    });
  },
};

export default function Upload(props: PageProps<Props>) {
  const { message } = props.data;
  return (
    <>
      <form method="post" encType="multipart/form-data">
        <input type="file" name="my-file" />
        <button type="submit">Upload</button>
      </form>
      {message ? <p>{message}</p> : null}
    </>
  );
}
```

## A note of caution

These examples are simplified to demonstrate how Deno and Fresh handle HTTP
requests. In the Real World™, you'll want to validate your data (_especially the
file type_) and protect against cross-site request forgery. Consider yourself
warned.



================================================
FILE: docs/latest/concepts/index.md
================================================
---
description: |
  This chapter goes over some fundamental concepts of Fresh.
---

This chapter goes over some fundamental concepts of Fresh. It covers the
overarching architecture design of Fresh applications, as well as reference
documentation about the various features of Fresh.



================================================
FILE: docs/latest/concepts/islands.md
================================================
---
description: |
  Islands enable client side interactivity in Fresh. They are hydrated on the client in addition to being rendered on the server.
---

Islands enable client side interactivity in Fresh. Islands are isolated Preact
components that are rendered on the server and then hydrated on the client. This
is different from all other components in Fresh, as they are usually rendered on
the server only.

Islands are defined by creating a file in the `islands/` folder in a Fresh
project. The name of this file must be a PascalCase or kebab-case name of the
island.

```tsx islands/my-island.tsx
import { useSignal } from "@preact/signals";

export default function MyIsland() {
  const count = useSignal(0);

  return (
    <div>
      Counter is at {count}.{" "}
      <button onClick={() => (count.value += 1)}>+</button>
    </div>
  );
}
```

An island can be used in a page like a regular Preact component. Fresh will take
care of automatically re-hydrating the island on the client.

```tsx route/index.tsx
import MyIsland from "../islands/my-island.tsx";

export default function Home() {
  return <MyIsland />;
}
```

## Passing JSX to islands

Islands support passing JSX elements via the `children` property.

```tsx islands/my-island.tsx
import { useSignal } from "@preact/signals";
import { ComponentChildren } from "preact";

interface Props {
  children: ComponentChildren;
}

export default function MyIsland({ children }: Props) {
  const count = useSignal(0);

  return (
    <div>
      Counter is at {count}.{" "}
      <button onClick={() => (count.value += 1)}>+</button>
      {children}
    </div>
  );
}
```

This allows you to pass static content rendered by the server to an island in
the browser.

```tsx routes/index.tsx
import MyIsland from "../islands/my-island.tsx";

export default function Home() {
  return (
    <MyIsland>
      <p>This text is rendered on the server</p>
    </MyIsland>
  );
}
```

You can also create shared components in your `components/` directory, which can
be used in both static content and interactive islands. When these components
are used within islands, interactivity can be added, such as `onClick` handlers
(using an `onClick` handler on a button outside of an island will not fire).

```tsx islands/my-island.tsx
import { useSignal } from "@preact/signals";
import { ComponentChildren } from "preact";
import Card from "../components/Card.tsx";
import Button from "../components/Button.tsx";

interface Props {
  children: ComponentChildren;
}

export default function MyIsland({ children }: Props) {
  const count = useSignal(0);

  return (
    <Card>
      Counter is at {count}.{" "}
      <Button onClick={() => (count.value += 1)}>+</Button>
      {children}
    </Card>
  );
}
```

## Passing other props to islands

Passing props to islands is supported, but only if the props are serializable.
Fresh can serialize the following types of values:

- Primitive types `string`, `boolean`, `bigint`, and `null`
- Most `number`s (`Infinity`, `-Infinity`, and `NaN` are silently converted to
  `null`)
- Plain objects with string keys and serializable values
- Arrays containing serializable values
- Uint8Array
- JSX Elements (restricted to `props.children`)
- Preact Signals (if the inner value is serializable)

Circular references are supported. If an object or signal is referenced multiple
times, it is only serialized once and the references are restored upon
deserialization. Passing complex objects like `Date`, custom classes, or
functions is not supported.

During server side rendering, Fresh annotates the HTML with special comments
that indicate where each island will go. This gives the code sent to the client
enough information to put the islands where they are supposed to go without
requiring hydration for the static children of interactive islands. No
Javascript is sent to the client when no interactivity is needed.

```html
<!--frsh-myisland_default:default:0-->
<div>
  Counter is at 0.
  <button>+</button>
  <!--frsh-slot-myisland_default:children-->
  <p>This text is rendered on the server</p>
  <!--/frsh-slot-myisland_default:children-->
</div>
<!--/frsh-myisland_default:default:0-->
```

### Nesting islands

Islands can be nested within other islands as well. In that scenario they act
like a normal Preact component, but still receive the serialized props if any
were present.

```tsx islands/other-island.tsx
import { useSignal } from "@preact/signals";
import { ComponentChildren } from "preact";

interface Props {
  children: ComponentChildren;
  foo: string;
}

function randomNumber() {
  return Math.floor(Math.random() * 100);
}

export default function OtherIsland({ children, foo }: Props) {
  const number = useSignal(randomNumber());

  return (
    <div>
      <p>String from props: {foo}</p>
      <p>
        <button onClick={() => (number.value = randomNumber())}>Random</button>
        {" "}
        number is: {number}.
      </p>
    </div>
  );
}
```

In essence, Fresh allows you to mix static and interactive parts in your app in
a way that's most optimal for your app. We'll keep sending only the JavaScript
that is needed for the islands to the browser.

```tsx route/index.tsx
import MyIsland from "../islands/my-island.tsx";
import OtherIsland from "../islands/other-island.tsx";

export default function Home() {
  return (
    <div>
      <MyIsland>
        <OtherIsland foo="this prop will be serialized" />
      </MyIsland>
      <p>Some more server rendered text</p>
    </div>
  );
}
```

## Rendering islands on client only

When using client-only APIs, like `EventSource` or `navigator.getUserMedia`,
this component will not run on the server as it will produce an error like:

```
An error occurred during route handling or page rendering. ReferenceError: EventSource is not defined
    at Object.MyIsland (file:///Users/someuser/fresh-project/islandsmy-island.tsx:6:18)
    at m (https://esm.sh/v129/preact-render-to-string@6.2.0/X-ZS8q/denonext/preact-render-to-string.mjs:2:2602)
    at m (https://esm.sh/v129/preact-render-to-string@6.2.0/X-ZS8q/denonext/preact-render-to-string.mjs:2:2113)
    ....
```

Use the [`IS_BROWSER`](https://deno.land/x/fresh/runtime.ts?doc=&s=IS_BROWSER)
flag as a guard to fix the issue:

```tsx islands/my-island.tsx
import { IS_BROWSER } from "$fresh/runtime.ts";

export function MyIsland() {
  // Return any prerenderable JSX here which makes sense for your island
  if (!IS_BROWSER) return <div></div>;

  // All the code which must run in the browser comes here!
  // Like: EventSource, navigator.getUserMedia, etc.
  return <div></div>;
}
```



================================================
FILE: docs/latest/concepts/layouts.md
================================================
---
description: |
  Add a layout to provide common meta tags, context for application sub routes, and common layout.
---

A layout is defined in a `_layout.tsx` file in any sub directory (at any level)
under the `routes/` folder. It must contain a default export that is a regular
Preact component. Only one such layout is allowed per sub directory.

```txt Project structure
└── routes
    ├── sub
    │   ├── page.tsx
    │   └── index.tsx
    ├── other
    │   ├── _layout.tsx  # will be applied on top of `routes/_layout.tsx`
    │   └── page.tsx
    ├── _layout.tsx  # will be applied to all routes
    └── _app.tsx
```

The component to be wrapped is received via props, in addition to a few other
things. This allows for the introduction of a global container functioning as a
template which can be conditioned based on state and params. Note that any state
set by middleware is available via `props.state`.

```tsx routes/sub/_layout.tsx
import { PageProps } from "$fresh/server.ts";

export default function Layout({ Component, state }: PageProps) {
  // do something with state here
  return (
    <div class="layout">
      <Component />
    </div>
  );
}
```

## Async layouts

In case you need to fetch data asynchronously before rendering the layout, you
can use an async layout to do so.

```tsx routes/sub/_layout.tsx
import { FreshContext } from "$fresh/server.ts";

export default async function Layout(req: Request, ctx: FreshContext) {
  // do something with state here
  const data = await loadData();

  return (
    <div class="layout">
      <p>{data.greeting}</p>
      <ctx.Component />
    </div>
  );
}
```

### Define helper

To make it a little quicker to write async layouts, Fresh ships with a
`defineLayout` helper which automatically infers the correct types for the
function arguments.

```tsx
import { defineLayout } from "$fresh/server.ts";

export default defineLayout(async (req, ctx) => {
  const data = await loadData();

  return (
    <div class="layout">
      <p>{data.greeting}</p>
      <ctx.Component />
    </div>
  );
});
```

## Opting out of layout inheritance

Sometimes you want to opt out of the layout inheritance mechanism for a
particular route. This can be done via route configuration. Picture a directory
structure like this:

```txt Project structure
└── routes
    ├── sub
    │   ├── _layout_.tsx
    │   ├── special.tsx  # should not inherit layouts
    │   └── index.tsx
    └── _layout.tsx
```

To make `routes/sub/special.tsx` opt out of rendering layouts we can set
`skipInheritedLayouts: true`.

```tsx routes/sub/special.tsx
import { RouteConfig } from "$fresh/server.ts";

export const config: RouteConfig = {
  skipInheritedLayouts: true, // Skip already inherited layouts
};

export default function MyPage() {
  return <p>Hello world</p>;
}
```

You can skip already inherited layouts inside a layout file:

```tsx routes/special/_layout.tsx
import { LayoutConfig } from "$fresh/server.ts";

export const config: LayoutConfig = {
  skipInheritedLayouts: true, // Skip already inherited layouts
};

export default function MyPage() {
  return <p>Hello world</p>;
}
```



================================================
FILE: docs/latest/concepts/middleware.md
================================================
---
description: |
  Add middleware routes to intercept requests or responses for analytics purposes, access control, or anything else.
---

A middleware is defined in a `_middleware.ts` file. It will intercept the
request in order for you to perform custom logic before or after the route
handler. This allows modifying or checking requests and responses. Common
use-cases for this are logging, authentication, and performance monitoring.

Each middleware gets passed a `next` function in the context argument that is
used to trigger child handlers. The `ctx` also has a `state` property that can
be used to pass arbitrary data to downstream (or upstream) handlers. This
`state` is included in `PageProps` by default, which is available to both the
special [\_app](/docs/concepts/app-wrapper.md) wrapper and normal
[routes](/docs/concepts/routes.md). `ctx.state` is normally set by modifying its
properties, e.g. `ctx.state.loggedIn = true`, but you can also replace the
entire object like `ctx.state = { loggedIn: true }`.

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

interface State {
  data: string;
}

export async function handler(
  req: Request,
  ctx: FreshContext<State>,
) {
  ctx.state.data = "myData";
  const resp = await ctx.next();
  resp.headers.set("server", "fresh server");
  return resp;
}
```

```ts routes/myHandler.ts
export const handler: Handlers<any, { data: string }> = {
  GET(_req, ctx) {
    return new Response(`middleware data is ${ctx.state.data}`);
  },
};
```

Middlewares are scoped and can be layered. This means a project can have
multiple middlewares, each covering a different set of routes. If multiple
middlewares cover a route, they will all be run, in order of specificity (least
specific first).

For example, take a project with the following routes:

```txt Project Structure
└── routes
    ├── _middleware.ts
    ├── index.ts
    └── admin
        ├── _middleware.ts
        └── index.ts
        └── signin.ts
```

For a request to `/` the request will flow like this:

1. The `routes/_middleware.ts` middleware is invoked.
2. Calling `ctx.next()` will invoke the `routes/index.ts` handler.

For a request to `/admin` the request flows like this:

1. The `routes/_middleware.ts` middleware is invoked.
2. Calling `ctx.next()` will invoke the `routes/admin/_middleware.ts`
   middleware.
3. Calling `ctx.next()` will invoke the `routes/admin/index.ts` handler.

For a request to `/admin/signin` the request flows like this:

1. The `routes/_middleware.ts` middleware is invoked.
2. Calling `ctx.next()` will invoke the `routes/admin/_middleware.ts`
   middleware.
3. Calling `ctx.next()` will invoke the `routes/admin/signin.ts` handler.

A single middleware file can also define multiple middlewares (all for the same
route) by exporting an array of handlers instead of a single handler. For
example:

```ts routes/_middleware.ts
export const handler = [
  async function middleware1(req, ctx) {
    // do something
    return ctx.next();
  },
  async function middleware2(req, ctx) {
    // do something
    return ctx.next();
  },
];
```

It should be noted that `middleware` has access to route parameters. If you're
running a fictitious `routes/[tenant]/admin/_middleware.ts` like this:

```ts routes/[tenant]/admin/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

export async function handler(_req: Request, ctx: FreshContext) {
  const currentTenant = ctx.params.tenant;
  // do something with the tenant
  const resp = await ctx.next();
  return resp;
}
```

and the request is to `mysaas.com/acme/admin/`, then `currentTenant` will have
the value of `acme` in your middleware.

## Middleware Destination

To set the stage for this section, let's focus on the part of `FreshContext`
that looks like this:

```ts
export interface FreshContext<State = Record<string, unknown>> {
  ...
  next: () => Promise<Response>;
  state: State;
  destination: router.DestinationKind;
  remoteAddr: {
    transport: "tcp" | "udp";
    hostname: string;
    port: number;
  };
  ...
}
```

and `router.DestinationKind` is defined like this:

```ts
export type DestinationKind = "internal" | "static" | "route" | "notFound";
```

This is useful if you want your middleware to only run when a request is headed
for a `route`, as opposed to something like `http://localhost:8001/favicon.ico`.

### Example

Initiate a new Fresh project (`deno run -A -r https://fresh.deno.dev/`) and then
create a `_middleware.ts` file in the `routes` folder like this:

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

export async function handler(req: Request, ctx: FreshContext) {
  console.log(ctx.destination);
  console.log(req.url);
  const resp = await ctx.next();
  return resp;
}
```

If you start up your server (`deno task start`) you'll see the following:

```sh Terminal
Task start deno run -A --watch=static/,routes/ dev.ts
Watcher Process started.
The manifest has been generated for 4 routes and 1 islands.

 🍋 Fresh ready
    Local: http://localhost:8000/

route
http://localhost:8000/
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/deserializer.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/signals.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/plugin-twind-main.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/main.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/island-counter.js
internal
http://localhost:8000/_frsh/refresh.js
static
http://localhost:8000/logo.svg?__frsh_c=3c7400558fc00915df88cb181036c0dbf73ab7f5
internal
http://localhost:8000/_frsh/alive
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/chunk-PDMKJVJ5.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/chunk-UGFDDSOV.js
internal
http://localhost:8000/_frsh/js/3c7400558fc00915df88cb181036c0dbf73ab7f5/chunk-RCK7U3UF.js
```

That first `route` request is for when `Fresh` responds with the root level
`index.tsx` route. The rest, as you can see, are either `internal` or `static`
requests. You can use `ctx.destination` to filter these out if your middleware
is only supposed to deal with routes.

## Middleware Redirects

If you want to redirect a request from a middleware, you can do so by returning:

```ts
export function handler(req: Request): Response {
  return Response.redirect("https://example.com", 307);
}
```

`307` stands for temporary redirect. You can also use `301` for permanent
redirect. You can also redirect to a relative path by doing:

```ts
export function handler(req: Request): Response {
  return new Response("", {
    status: 307,
    headers: { Location: "/my/new/relative/path" },
  });
}
```



================================================
FILE: docs/latest/concepts/partials.md
================================================
---
description: |
  Partials allow areas of a page to be updated without causing the browser to reload the page. They enable optimized fine grained UI updates and can be used to do client-side navigation.
---

Partials allow areas of the page to be updated with new content by the server
without causing the browser to reload the page. They make your website feel more
app-like because only the parts of the page that need to be updated will be
updated.

## Enabling partials

Partials are enabled by adding a `f-client-nav` attribute to an HTML element and
wrapping one or more areas in the page with a
`<Partial name="my-partial">`-component.

The quickest way to get started is to enable partials for every page in
`routes/_app.tsx` by making the following changes.

```diff routes/_app.tsx
  import { PageProps } from "$fresh/server.ts";
+ import { Partial } from "$fresh/runtime.ts";

  export default function App({ Component }: PageProps) {
    return (
      <html>
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <title>My Fresh app</title>
        </head>
-       <body>
+       <body f-client-nav>
+         <Partial name="body">
            <Component />
+         </Partial>
        </body>
      </html>
    );
  }
```

By adding the `f-client-nav` attribute, we enable partials for every element
beneath the `<body>`-tag. To mark an area of the page as a partial we wrap it
with a `<Partial>`-component with a unique name.

Behind the scenes, when the user clicks an `<a>`-tag, Fresh fetches the new page
and only pulls out the relevant content out of the HTML response. When it finds
a matching partial area it will update the content inside the partial.

> [info]: The `name` prop of the `<Partial>` component is expected to be unique
> among Partials. That's how Fresh knows which parts of the response need to go
> on the current page.

> [info]: Passing `f-client-nav={false}` disables client side navigation for all
> elements below the current node.

### Optimizing partial requests

By default, with `f-client-nav` set, Fresh fetches the full next page and only
picks out the relevant parts of the response. We can optimize this pattern
further by only rendering the parts we need, instead of always rendering the
full page. This is done by adding the `f-partial` attribute to a link.

```diff
- <a href="/docs/routes">Routes</a>
+ <a href="/docs/routes" f-partial="/partials/docs/routes">Routes</a>
```

When the `f-partial` attribute is present, Fresh will navigate to the page URL
defined in the `href` attribute, but fetch the updated UI from the URL specified
in `f-partial` instead. This can be a highly optimized route that only delivers
the content you care about.

Let's use a typical documentation page layout as an example. It often features a
main content area and a sidebar of links to switch between pages of the
documentation (marked green here).

![A sketched layout of a typical documentation page with the sidebar on the left composed of green links and a main content area on the right. The main content area is labeled as Partial "docs-content"](/docs/fresh-partial-docs.png)

The code for such a page (excluding styling) might look like this:

```tsx routes/docs/[id].tsx
export default defineRoute(async (req, ctx) => {
  const content = await loadContent(ctx.params.id);

  return (
    <div>
      <aside>
        <a href="/docs/page1">Page 1</a>
        <a href="/docs/page2">Page 2</a>
      </aside>
      <Partial name="docs-content">
        {content}
      </Partial>
    </div>
  );
});
```

An optimal route that only renders the content instead of the outer layout with
the sidebar might look like this respectively.

```tsx routes/partials/docs/[id].tsx
import { defineRoute, RouteConfig } from "$fresh/server.ts";
import { Partial } from "$fresh/runtime.ts";

// We only want to render the content, so disable
// the `_app.tsx` template as well as any potentially
// inherited layouts
export const config: RouteConfig = {
  skipAppWrapper: true,
  skipInheritedLayouts: true,
};

export default defineRoute(async (req, ctx) => {
  const content = await loadContent(ctx.params.id);

  // Only render the new content
  return (
    <Partial name="docs-content">
      {content}
    </Partial>
  );
});
```

By adding the `f-partial` attribute we tell Fresh to fetch the content from our
newly added `/partials/docs/[id].tsx` route.

```diff routes/docs/[id].tsx
  <aside>
-   <a href="/docs/page1">Page 1</a>
-   <a href="/docs/page2">Page 2</a>
+   <a href="/docs/page1" f-partial="/partials/docs/page1">Page 1</a>
+   <a href="/docs/page2" f-partial="/partials/docs/page2">Page 2</a>
  </aside>
```

With this in place, Fresh will navigate to the new page when clicking any of the
two links and _only_ load the content rendered by our optimized partial route.

> Currently, `f-partial` is scoped to `<a>`, `<button>` and `<form>` elements.
> This might be extended to more elements in the future.

## Sending multiple Partials at the same time

A neat aspect of partials in Fresh is that a response can return as many
partials as desired. That way you can update multiple unrelated areas on your
page in one single HTTP response. A scenario where this is useful are online
shops for example.

```tsx routes/partials/cart.tsx
export default function AddToCartPartial() {
  return (
    <>
      <Partial name="cart-items" mode="append">
        {/* Render the new cart item here */}
      </Partial>
      <Partial name="total-price">
        <p>Total: {totalPrice} €</p>
      </Partial>
    </>
  );
}
```

Both partials will be applied to the current page.

## Replacement mode

By default the whole content inside a partial will be replaced, but there are
scenarios where you want to prepend or append new content instead. This can be
achieved by adding the `mode` prop to a `Partial` component.

- `replace` - Swap out the content of the existing partial (default)
- `prepend` - Insert the new content before the existing content
- `append` - Insert the new content after the existing content

Personally, we’ve found that the `append` mode is really useful when you have an
UI which displays log messages or similar list-like data.

```tsx
export default function LogView() {
  const lines = getNewLogLines();

  return (
    <Partial name="logs-list" mode="append">
      {lines.map((line) => {
        return <li key={line}>{line}</li>;
      })}
    </Partial>
  );
}
```

> [info]: When picking the `prepend` or `append` mode, make sure to add keys to
> the elements.

## Bypassing or disabling Partials

If you want to exempt a particular element from triggering a partial request
like on a particular link, form or button, you can opt out of it by setting
`f-client-nav={false}` on the element or one of the ancestor elements.

```tsx
<body f-client-nav>
  {/* This will cause a partial navigation */}
  <a href="/docs/page1">With partials</a>

  {/* This WONT cause a partial navigation */}
  <a href="/docs/page1" f-client-nav={false}>No partials</a>

  {/* This WONT cause a partial navigation on any elements below */}
  <div f-client-nav={false}>
    <div>
      <a href="/docs/page1">No partials</a>
    </div>
  </div>
</body>;
```

Whenever an element is clicked Fresh checks if it has the `f-client-nav`
attribute and if it is set to `true`. If the element itself doesn't have such an
attribute, it will check if any of the ancestor elements has it. If an element
was found with a truthy `f-client-nav` attribute a partial request will be
triggered. If there is no such attribute or if it's set to `false`, no partial
request will occur.



================================================
FILE: docs/latest/concepts/plugins.md
================================================
---
description: Plugins can add new functionality to Fresh without requiring significant complexity.
---

Plugins can dynamically add new functionality to Fresh without exposing
significant complexity to the user. Users can add plugins by importing and
initializing them in their `main.ts` file:

```ts main.ts
import { start } from "$fresh/server.ts";
import manifest from "./fresh.gen.ts";

import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.js";

await start(manifest, {
  plugins: [
    // This line configures Fresh to use the first-party twind plugin.
    twindPlugin(twindConfig),
  ],
});
```

Currently, the only available first-party plugin is the Twind plugin.
Third-party plugins are also supported - they can be imported from any HTTP
server, like any other Deno module.

Plugin hooks are executed in the order that the plugins are defined in the
`plugins` array. This means that the first plugin in the array will be executed
first, and the last plugin in the array will be executed last. For many plugins,
this does not matter, but for some plugins it may.

## Creating a plugin

Fresh plugins are in essence a collection of hooks that allow the plugin to hook
into various systems inside of Fresh.

A Fresh plugin is just a JavaScript object that conforms to the
[Plugin](https://deno.land/x/fresh/server.ts?s=Plugin) interface. The only
required property of a plugin is its name. Names must only contain the
characters `a`-`z`, and `_`.

```ts
import { Plugin } from "$fresh/server.ts";

const plugin: Plugin = {
  name: "my_plugin",
};
```

A plugin containing only a name is technically valid, but not very useful. To be
able to do anything with a plugin, it must register some hooks, middlewares, or
routes.

### Hook: `render`

The render hook allows plugins to:

- Control timing of the synchronous render of a page.
- Inject additional CSS and JS into the rendered page.

This is commonly used to set thread local variables for the duration of the
render (for example preact global context, preact option hooks, or for style
libraries like Twind). After render is complete, the plugin can inject inline
CSS and JS modules (with attached state) into the page.

The render hook is called with the
[`PluginRenderContext`](https://deno.land/x/fresh/server.ts?s=PluginRenderContext)
object, which contains a `render()` method. This method must be invoked during
the render hook to actually render the page. It is a terminal error to not call
the `render()` method during the render hook.

The `render()` method returns a
[`PluginRenderFunctionResult`](https://deno.land/x/fresh/server.ts?s=PluginRenderFunctionResult)
object which contains the HTML text of the rendered page, as well as a boolean
indicating whether the page contains any islands that will be hydrated on the
client.

The `render` hook needs to synchronously return a
[`PluginRenderResult`](https://deno.land/x/fresh/server.ts?s=PluginRenderResult)
object. Additional CSS and JS modules can be added to be injected into the page
by adding them to `styles`, `links` and `scripts` arrays in this object. The
plugin can also replace the HTML in side the `<body>`-element of the page by
including a `htmlText` string in this object.

`styles` are injected into the `<head>` of the page as inline CSS. Each entry
can define the CSS text to inject, as well as an optional `id` for the style
tag, and an optional `media` attribute for the style tag.

`links` are injected into the `<head>` of the page as `<link>` tags. A link tag
is created for each entry, with attributes from the entry's properties.

`scripts` define JavaScript/TypeScript modules to be injected into the page. The
possibly loaded modules need to be defined up front in the `Plugin#entrypoints`
property. Each defined module must be a JavaScript/TypeScript module that has a
default export of a function that takes one (arbitrary) argument, and returns
nothing (or a promise resolving to nothing). Fresh will call this function with
the state defined in the `scripts` entry. The state can be any arbitrary JSON
serializable JavaScript value.

For an example of a plugin that uses the `render` hook, see the first-party
[Twind plugin](https://github.com/denoland/fresh/blob/1.x/plugins/twind.ts).

### Hook: `renderAsync`

This hook is largely the same as the `render` hook, with a couple of key
differences to make asynchronous style and script generation possible. It must
asynchronously return its
[`PluginRenderResult`](https://deno.land/x/fresh/server.ts?s=PluginRenderResult),
either from an `async/await` function or wrapped within a promise.

The render hook is called with the
[`PluginAsyncRenderContext`](https://deno.land/x/fresh/server.ts?s=PluginAsyncRenderContext)
object, which contains a `renderAsync()` method. This method must be invoked
during the render hook to actually render the page. It is a terminal error to
not call the `renderAsync()` method during the render hook.

This is useful for when plugins are generating styles and scripts with
asynchronous dependencies based on the `htmlText`. Unlike the synchronous render
hook, async render hooks for multiple pages can be running at the same time.
This means that unlike the synchronous render hook, you can not use global
variables to propagate state between the render hook and the renderer.

The `renderAsync` hooks start before any page rendering occurs, and finish after
all rendering is complete -- they wrap around the underlying JSX->string
rendering, plugin `render` hooks, and the
[`RenderFunction`](https://deno.land/x/fresh/server.ts?s=RenderFunction) that
may be provided to Fresh's `start` entrypoint in the `main.ts` file.

### Hook: `buildStart`

This hook is run at the start of the Fresh
[ahead-of-time build task](/docs/concepts/ahead-of-time-builds). It may be
synchronous or asynchronous.

The build start hook is called with the
[`ResolvedFreshConfig`](https://deno.land/x/fresh/src/server/types.ts?s=ResolvedFreshConfig)
object, which contains the full Fresh configuration.

This hook may be used to generate precompiled static assets. Any files saved to
the `static` subfolder of `config.build.outDir` (typically `_fresh`) will be
served the same as other [static files](/docs/concepts/static-files).

### Hook: `buildEnd`

This hook is run at the end of the Fresh
[ahead-of-time build task](/docs/concepts/ahead-of-time-builds). It may be
synchronous or asynchronous.

### Routes and Middlewares

You can create routes and middlewares that get loaded and rendered like the
normal [routes](/docs/concepts/routes) and
[middlewares](/docs/concepts/middleware).

The plugin routes and middlewares need a defined path in the format of a file
name without a filetype inside the routes directory(E.g. `blog/index`,
`blog/[slug]`).

For more examples see the [Concepts: Routing](/docs/concepts/routing) page.

To create a middleware you need to create a `MiddlewareHandler` function.

And to create a route you can create both a Handler and/or component.

Below is an example plugin that creates a route and middleware

```ts my-route-and-middleware-plugin.ts
import { MiddlewareHandlerContext, Plugin } from "$fresh/server.ts";
import { handler as testMiddleware } from "./sample_routes/_middleware.ts";
import { SimpleRoute } from "./sample_routes/simple-route.tsx";
export type { Options };

interface Options {
  title: string;
}
export type PluginMiddlewareState = {
  num: number;
  test: string;
};

const twoPointlessMiddlewares = [
  async (
    _req: Request,
    ctx: MiddlewareHandlerContext<PluginMiddlewareState>,
  ) => {
    ctx.state.num = ctx.state.num === undefined ? 1 : ctx.state.num + 1;
    return await ctx.next();
  },
  async (
    _req: Request,
    ctx: MiddlewareHandlerContext<PluginMiddlewareState>,
  ) => {
    ctx.state.num = ctx.state.num === undefined ? 1 : ctx.state.num + 1;
    return await ctx.next();
  },
];

export default function routePlugin(
  options: Options,
): Plugin<PluginMiddlewareState> {
  return {
    name: "routePlugin",
    middlewares: [{
      middleware: { handler: testMiddleware },
      path: "/",
    }, {
      middleware: {
        handler: twoPointlessMiddlewares,
      },
      path: "lots-of-middleware",
    }],
    routes: [
      { path: "no-leading-slash-here", component: SimpleRoute },
    ],
  };
}
```

### Islands

Islands from plugins can be loaded by specifying a list of file paths in your
plugin. Those files will be treated by Fresh as if they had been placed inside
the `islands/` directory. They will be processed and bundled for the browser in
the same way.

```tsx my-island-plugin.ts
import { Plugin } from "$fresh/server.ts";

export default function myIslandPlugin(): Plugin {
  return {
    name: "my-island-plugin",
    islands: {
      baseLocation: import.meta.url,
      paths: [
        "./plugin/MyPluginIsland.tsx",
        "./plugin/OtherPluginIsland.tsx",
      ],
    },
  };
}
```



================================================
FILE: docs/latest/concepts/routes.md
================================================
---
description: |
  Routes are the basic building block of Fresh applications. They are used to define the behaviour the application when a given path is requested.
---

At their core, routes describe how a request for a given path should be handled,
and what the response should be. To do this, routes have two main parts: the
handler, and the component. A route can have either one, or both, but never
neither.

The handler is a function that is called for every request to the route. It
needs to return a response that is then sent to the client. The response could
be anything: a plain text string, a JSON object, an HTML page, a WebSocket
connection, a streaming file, or pretty much anything else. The handler is
passed a `render` function that it can call to invoke rendering a component.

The component is the template for a page. It is a JSX element that is rendered
on the server. The page component gets passed props that can be used by it to
determine exactly what should be rendered. By default components receive props
consisting of: the request URL, the matching route (as a string), the matches
from the URL pattern match, any state set by middleware, and any data passed to
the handler's `render` function.

## Handler route

Let's look at a basic route that returns a plain text string:

```tsx routes/plain.tsx
import { FreshContext, Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  GET(_req: Request, _ctx: FreshContext) {
    return new Response("Hello World");
  },
};
```

To define a handler, one needs to export a `handler` function or object from the
route module. If the handler is an object, each key in the object is the name of
the HTTP method that the handler should be called for. For example the `GET`
handler above is called for `GET` requests. If the handler is a function, it is
called for all requests regardless of the method. If an HTTP method does not
have a corresponding handler, a 405 HTTP error is returned.

## Component route

Now, let's render some HTML using the route component:

```tsx routes/html.tsx
import { PageProps } from "$fresh/server.ts";

export default function Page(props: PageProps) {
  return <div>You are on the page '{props.url.href}'.</div>;
}
```

The page component needs to be the default export of the route module. It is
passed props that can be used to render the page.

As you can see in the second example, if no handler is explicitly defined a
default handler is used that just renders out the page component if present. You
can also override the default handler though to modify how exactly rendering
should work.

## Mixed handler and component route

In the below example, a custom handler is used to add a custom header to the
response after rendering the page component.

```tsx routes/html.tsx
import { HandlerContext, Handlers, PageProps } from "$fresh/server.ts";

export const handler: Handlers = {
  async GET(_req: Request, ctx: HandlerContext) {
    const resp = await ctx.render();
    resp.headers.set("X-Custom-Header", "Hello World");
    return resp;
  },
};

export default function Page(props: PageProps) {
  return <div>You are on the page '{props.url.href}'.</div>;
}
```

## Async route components

Having a separate route handler and component function is nice, when you want to
test these in isolation, but can become a bit cumbersome to maintain. They
require some additional indirection of declaring an interface for the component
`Data` when you're passing it around through `ctx.render()`.

```tsx routes/page.tsx
interface Data {
  foo: number;
}

export const handler: Handlers<Data> = {
  async GET(req, ctx) {
    const value = await loadFooValue();
    return ctx.render({ foo: value });
  },
};

export default function MyPage(props: PageProps<Data>) {
  return <p>foo is: {props.data.foo}</p>;
}
```

When a route has both a component and a `GET` handler, they are typically very
closely coupled. With async route components you can merge the two together and
avoid having to create the `Data` interface boilerplate.

```tsx routes/page.tsx
// Async route component
export default async function MyPage(req: Request, ctx: RouteContext) {
  const value = await loadFooValue();
  return <p>foo is: {value}</p>;
}
```

The code gets a little shorter with async route components. Conceptually, you
can think of async route components inlining the `GET` handler into the
component function. Note, that you can still add additional HTTP handlers in the
same file like before.

```tsx routes/page.tsx
export const handler: Handlers = {
  async POST(req) {
    // ... do something here
  },
};

export default async function MyPage(req: Request, ctx: RouteContext) {
  const value = await loadFooValue();
  return <p>foo is: {value}</p>;
}
```

### Returning Response objects

Quite often a route handler needs to render a 404 page or bail out of rendering
in another manner. This can be done by returning a `Response` object.

```tsx route/page.tsx
// Async route component
export default async function MyPage(req: Request, ctx: RouteContext) {
  const value = await loadFooValue();

  // Return 404 if `value` is null
  if (value === null) {
    return ctx.renderNotFound();
  }

  // Returning a response object directly works too
  if (value === "redirect") {
    const headers = new Headers();
    headers.set("location", "/some-other-page");
    return new Response(null, {
      status: 302,
      headers,
    });
  }

  return <p>foo is: {value}</p>;
}
```

### Define helper

To make it a little quicker to write async routes, Fresh ships with a
`defineRoute` helper which automatically infers the correct types for the
function arguments.

```tsx
import { defineRoute } from "$fresh/server.ts";

export default defineRoute(async (req, ctx) => {
  const data = await loadData();

  return (
    <div class="page">
      <h1>Hello {data.name}</h1>
    </div>
  );
});
```



================================================
FILE: docs/latest/concepts/routing.md
================================================
---
description: |
  File based routing is the simplest way to do routing in Fresh apps. Additionally custom patterns can be configured per route.
---

Routing is the mechanism that determines what route a given incoming request is
handled by. Fresh routes requests based on their URL path. By default routes
specify which paths they are invoked for using the name of the file. Routes can
also define a custom [URL pattern][urlpattern] to match against for more
advanced use cases.

The file based routing in Fresh is very similar to the file based routing seen
in other frameworks, namely Next.js. File names are used to determine which
route a given request should be handled by. The pattern is determined based on
the path of the file on disk, relative to the `routes/` directory.

File names are mapped to route patterns as follows:

- File extensions are ignored.
- Literals in the file path are treated as string literals to match.
- Files named `<path>/index.<ext>` behave identically to a file named
  `<path>.<ext>`.
- Path segments can be made dynamic by surrounding an identifier with `[` and
  `]`.
- Paths where the last path segment follows the structure `[...<ident>]` are
  treated as having a wildcard suffix.

Here is a table of file names, which route patterns they map to, and which paths
they might match:

| File name                   | Route pattern          | Matching paths                          |
| --------------------------- | ---------------------- | --------------------------------------- |
| `index.ts`                  | `/`                    | `/`                                     |
| `about.ts`                  | `/about`               | `/about`                                |
| `blog/index.ts`             | `/blog`                | `/blog`                                 |
| `blog/[slug].ts`            | `/blog/:slug`          | `/blog/foo`, `/blog/bar`                |
| `blog/[slug]/comments.ts`   | `/blog/:slug/comments` | `/blog/foo/comments`                    |
| `old/[...path].ts`          | `/old/:path*`          | `/old/foo`, `/old/bar/baz`              |
| `docs/[[version]]/index.ts` | `/docs{/:version}?`    | `/docs`, `/docs/latest`, `/docs/canary` |

Advanced use-cases can require that a more complex pattern be used for matching.
A custom [URL pattern][urlpattern] can be specified in the route configuration.
This pattern will be used instead of the file path based pattern:

```ts routes/x.ts
import { RouteConfig } from "$fresh/server.ts";

export const config: RouteConfig = {
  routeOverride: "/x/:module@:version/:path*",
};

// ...
```

## Route Groups

When working with [layouts](/docs/concepts/layouts) or
[middlewares](/docs/concepts/middleware), you'll sometimes come across a
situation where you want your routes to inherit from a layout other than what's
suggested by the URL segment.

Let's illustrate that with an example:

```txt
/about -> layout A
/career -> layout A
/archive -> layout B
/contact -> layout B
```

Without any way to group routes this is a problem because every route segment
can only have one `_layout` file.

```txt Project structure
└── routes
    ├── _layout.tsx  # applies to all routes here :(
    ├── about.tsx
    ├── career.tsx
    ├── archive.tsx
    └── contact.tsx
```

We can solve this problem with route groups. A route group is a folder which has
a name that is wrapped in parentheses. For example `(info)` would be considered
a route group and so would `(marketing)`. This enables us to group related
routes in a folder and use a different `_layout` file for each group.

```txt Project structure
└── routes
    ├── (marketing)
    │   ├── _layout.tsx  # only applies to about.tsx and career.tsx
    │   ├── about.tsx
    │   └── career.tsx
    └── (info)
        ├── _layout.tsx  # only applies to archive.tsx and contact.tsx
        ├── archive.tsx
        └── contact.tsx
```

> [warn]: Be careful about routes in different groups which match to the same
> URL. Such scenarios will lead to ambiguity as to which route file should be
> picked.
>
> ```txt Project structure
> └── routes
>     ├── (group-1)
>     │   └── about.tsx  # Bad: Maps to same `/about` url
>     └── (group-2)
>         └── about.tsx  # Bad: Maps to same `/about` url
> ```

[urlpattern]: https://developer.mozilla.org/en-US/docs/Web/API/URL_Pattern_API

## Co-location

If you want to store components and islands closer to their routes, you may want
to use co-location.

When the name of a route group folder starts with an underscore, like
`(_components)`, Fresh will ignore that folder and it’s effectively treated as
private. This means you can use these private route folders to store components
related to a particular route.

Following the above example, say you have some components you only want to use
in your marketing pages, you could create a route group folder `(_components)`
to house these.

The one special name is `(_islands)` which tells Fresh to treat all files in
that folder as an island.

```txt Project structure
└── routes
    ├── (marketing)
    │   ├── _layout.tsx
    │   ├── about.tsx
    │   ├── career.tsx
    │   ├── (_components)
    │   │   └── newsletter-cta.tsx
    │   └── (_islands)
    │       └── interactive-stats.tsx # Fresh treats this as an island
    └── shop
        ├── (_components)
        │   └── product-card.tsx
        └── (_islands)
            └── cart.tsx # Fresh treats this as an island
```

Combined together, this gives you the ability to organise your code on a feature
basis and put all related components, islands or anything else into a shared
folder.



================================================
FILE: docs/latest/concepts/server-components.md
================================================
---
description: |
  Fresh's architecture is designed to leverage server components by default.
---

If you've read about Fresh's [architecture](/docs/concepts/architecture) then
you know that it's based on the islands architecture pattern. The flip side of
this is that everything else is, by default, a server component. When you
[create a route](/docs/getting-started/create-a-route), all of the components
used are rendered on the server. No JavaScript is sent to the client, unless you
specifically include something from the `/islands/` folder.

Internally, Fresh's rendering heavily leverages
[preact-render-to-string](https://github.com/preactjs/preact-render-to-string).
This is the exact library mentioned on Preact's
[Server-Side Rendering](https://preactjs.com/guide/v10/server-side-rendering/)
article.



================================================
FILE: docs/latest/concepts/server-configuration.md
================================================
---
description: |
  The ability to configure the core Fresh server leads to its flexibility.
---

In this page we discuss how the server can be configured during startup.

The signature of the primary method looks like this:

```ts main.ts
export async function start(manifest: Manifest, config: FreshConfig = {});
```

## Configuration

`Manifest` comes from `fresh.gen.ts`, so nothing to do there. `config` is where
things get interesting.
[`FreshConfig`](https://deno.land/x/fresh/server.ts?s=FreshConfig) looks like
this:

```ts
export interface FreshConfig {
  build?: {
    /**
     * The directory to write generated files to when `dev.ts build` is run.
     * This can be an absolute path, a file URL or a relative path.
     */
    outDir?: string;
    /**
     * This sets the target environment for the generated code. Newer
     * language constructs will be transformed to match the specified
     * support range. See https://esbuild.github.io/api/#target
     * @default {"es2022"}
     */
    target?: string | string[];
  };
  render?: RenderFunction;
  plugins?: Plugin[];
  staticDir?: string;
  router?: RouterOptions;
  server?: Partial<Deno.ServeTlsOptions>;
}
```

And for completeness here are the remaining two types:

```ts
export type RenderFunction = (
  ctx: RenderContext,
  render: InnerRenderFunction,
) => void | Promise<void>;

export interface RouterOptions {
  /**
   *  Controls whether Fresh will append a trailing slash to the URL.
   *  @default {false}
   */
  trailingSlash?: boolean;
  /**
   *  Configures the pattern of files to ignore in islands and routes.
   *
   *  By default Fresh will ignore test files,
   *  for example files with a `.test.ts` or a `_test.ts` suffix.
   *
   *  @default {/(?:[^/]*_|[^/]*\.|)test\.(?:ts|tsx|mts|js|mjs|jsx|)\/*$/}
   */
  ignoreFilePattern?: RegExp;
  /**
   * Serve fresh from a base path instead of from the root.
   *   "/foo/bar" -> http://localhost:8000/foo/bar
   * @default {undefined}
   */
  basePath?: string;
}
```

## Build

### outDir

As the comment suggests, this can be used to configure where generated files are
written:

```tsx
await dev(import.meta.url, "./main.ts", {
  build: {
    outDir: Deno.env.get("FRESH_TEST_OUTDIR") ?? undefined,
  },
});
```

### target

This should be a valid ES Build target.

```tsx
await dev(import.meta.url, "./main.ts", {
  build: {
    target: "es2015",
  },
});
```

## Plugins

See the [docs](/docs/concepts/plugins) on this topic for more detail. But as a
quick example, you can do something like this to load plugins:

```ts main.ts
await start(manifest, { plugins: [twindPlugin(twindConfig)] });
```

## StaticDir

This allows you to specify the location where your site's static assets are
stored. Here's an example:

```ts main.ts
await start(manifest, { staticDir: "./custom_static" });
```

## Render

This is by far the most complicated option currently available. It allows you to
configure how your components get rendered.

## RouterOptions

### TrailingSlash

By default Fresh uses URLs like `https://www.example.com/about`. If you'd like,
you can configure this to `https://www.example.com/about/` by using the
`trailingSlash` setting.

```ts main.ts
await start(manifest, { router: { trailingSlash: true } });
```

### ignoreFilePattern

By default Fresh ignores test files which are co-located next routes and
islands. If you want, you can change the pattern Fresh uses ignore these files

### basePath

This setting allows you to serve a Fresh app from sub-path of a domain. A value
of `/foo/bar` would serve the app from `http://localhost:8000/foo/bar` instead
of `http://localhost:8000/` for example.

The `basePath` will be automatically applied to absolute links in your app. For
example, when the `basePath` is `/foo/bar`, linking to `/about` will
automatically become `/foo/bar/about`.

```jsx
<a href="/about">About</a>;
```

Rendered HTML:

```html
<a href="/foo/bar/about">About</a>
```

The `basePath` is also applied to the `src` and `srcset` attribute of
`<img>`-tags, the `href` attribute of `<link>` and the `src` attribute of
`<script>` tags.

## Server

Now that Deno has stabilized
[Deno.serve](https://docs.deno.com/api/deno/~/Deno.serve) and Fresh has switched
to using this API, all server configuration options are embedded in `server`
inside the `FreshConfig`. The fully expanded set of parameters looks like this:

```ts
server: {
  /** Server private key in PEM format */
  cert: string;

  /** Cert chain in PEM format */
  key: string;

  /** The port to listen on.
   *
   * @default {8000} */
  port?: number;

  /** A literal IP address or host name that can be resolved to an IP address.
   *
   * __Note about `0.0.0.0`__ While listening `0.0.0.0` works on all platforms,
   * the browsers on Windows don't work with the address `0.0.0.0`.
   * You should show the message like `server running on localhost:8080` instead of
   * `server running on 0.0.0.0:8080` if your program supports Windows.
   *
   * @default {"0.0.0.0"} */
  hostname?: string;

  /** An {@linkcode AbortSignal} to close the server and all connections. */
  signal?: AbortSignal;

  /** Sets `SO_REUSEPORT` on POSIX systems. */
  reusePort?: boolean;

  /** The handler to invoke when route handlers throw an error. */
  onError?: (error: unknown) => Response | Promise<Response>;

  /** The callback which is called when the server starts listening. */
  onListen?: (params: { hostname: string; port: number }) => void;
}
```

Use these to configure your server as you see fit.



================================================
FILE: docs/latest/concepts/static-files.md
================================================
---
description: |
  Fresh has built-in support for serving static files. This is useful for serving images, CSS, and other static assets.
---

Fresh automatically serves static assets placed in a `static/` directory in the
project root. These assets are served at the root of the webserver, with a
higher priority than routes. This means that if a given request matches a file
in the `static/` folder, it is always served, even if there is a route that
would also match the request.

Static asset responses automatically get a `content-type` header assigned based
on the file extension of the file on disk. Assets are also automatically
streamed from disk to the client to improve performance and efficiency for both
user and server.

Fresh also adds an `etag` header to assets automatically and handles the
`If-None-Match` header for incoming requests.

### Caching

By default, no caching headers are added to assets. This can be disadvantageous
in many scenarios, so Fresh makes it easy to serve assets with long cache
lifetimes too.

The first approach to do this is manual. The client runtime exports an `asset`
function that takes an absolute path to the static asset and returns a "locked"
version of this path that contains a build ID for cache busting. When the asset
is requested at this "locked" path, it will be served with a cache lifetime of
one year.

```jsx routes/page.tsx
import { asset } from "$fresh/runtime.ts";

export default function Page() {
  return (
    <p>
      <a href={asset("/brochure.pdf")}>View brochure</a>
    </p>
  );
}
```

Fresh also does this automatically for `src` and `srcset` attributes in `<img>`
and `<source>` HTML tags. These will automatically use "locked" paths if Fresh
deems it safe to do so. You can always opt out of this behaviour per tag, by
adding the `data-fresh-disable-lock` attribute.

```jsx
<img src="/user.png" />;
```



================================================
FILE: docs/latest/concepts/updating.md
================================================
---
description: |
  New versions of Fresh are regularly released. This page explains how to update your project.
---

Fresh consists of multiple pieces which are independently versioned and
released.

- Fresh (https://deno.land/x/fresh)
- Preact (https://esm.sh/preact)
- preact-render-to-string (https://esm.sh/preact-render-to-string)

Some plugins also have their own dependencies that can be updated independently.

- Twind (https://esm.sh/twind) (for the twind plugin)

For the most part these pieces can be updated independently. Certain versions of
Fresh may require a minimum version of a given dependency. This is documented
below.

| Fresh version | Preact            | preact-render-to-string | Deno      |
| ------------- | ----------------- | ----------------------- | --------- |
| 1.0.0-1.0.2   | >=10.8.1 <11.0.0  | >=5.2.0 <6.0.0          | >= 1.23.0 |
| 1.1.0-1.1.5   | >=10.8.1 <11.0.0  | >=5.2.0 <6.0.0          | >= 1.25.0 |
| 1.2.0         | >=10.15.0 <11.0.0 | >=6.1.0                 | >= 1.25.0 |

## Updating dependencies

To update your dependencies, you have two options:

- Run the Fresh updater to update your project dependencies.
- Manually update the dependency versions in your `deno.json` file.

### Auto updater

The auto updater is a command line tool that will update your project's
`deno.json` file to the latest versions of Fresh and its dependencies. It may
also contain code mods for your project that will update your code to the latest
recommended patterns for Fresh projects.

To run the auto updater, run the following command from the root of your
project:

```sh Terminal
$ deno run -A -r https://fresh.deno.dev/update
```

You will be prompted to confirm the changes that will be made to your project.

### Manual update

To manually update your project's dependencies, you can edit the `deno.json`
file in the root of your projects directory. Dependency versions are encoded
into the URLs in this file. For example, here is how to update a project from
Fresh 1.0.2 to 1.1.3, and update Preact to the latest version:

```diff deno.json
  {
    "imports": {
-     "$fresh/": "https://deno.land/x/fresh@1.0.2/",
+     "$fresh/": "https://deno.land/x/fresh@1.1.5/",

-     "preact": "https://esm.sh/preact@10.8.1",
-     "preact/": "https://esm.sh/preact@10.8.1/",
+     "preact": "https://esm.sh/preact@10.11.0",
+     "preact/": "https://esm.sh/preact@10.11.0/",

-     "preact-render-to-string": "https://esm.sh/*preact-render-to-string@5.2.0",
+     "preact-render-to-string": "https://esm.sh/*preact-render-to-string@6.1.0",

      "twind": "https://esm.sh/twind@0.16.17",
      "twind/": "https://esm.sh/twind@0.16.17/"
    }
  }
```

## Automatic update checks

Fresh will periodically check if a new Fresh version is available if it's
running outside of CI. This happens once per day and can be disabled by setting
the `FRESH_NO_UPDATE_CHECK=true` environment variable.

## Code mods

Code mods are small scripts that can be run to update your project's code to
match the latest recommended patterns for Fresh projects. Code mods can be run
through the auto updater. Sometimes the code mod can not cover all cases, so you
may need to manually update some code. This section explains the code mods
currently available.

### Classical JSX -> Automatic JSX

> This code mod is only available in Fresh 1.1.0 and above.

The classical JSX transform that relies on a `/** @jsx h */` pragma is no longer
the recommended way to use JSX in Fresh projects. Instead, starting with version
1.1.0, Fresh projects should use the automatic JSX transform that requires no
JSX pragma or preact import.

```diff
- /** @jsx h */
- import { h } from "preact";

  export default function Page() {
    return <div>Hello world!</div>;
  }
```

This code mod will update your deno.json file to include the relevant compiler
options to enable the automatic JSX transform. It will then go through your
project and remove any `/** @jsx h */` pragmas and `import { h } from "preact"`
statements.

### Classic twind -> Twind plugin

> This code mod is only available in Fresh 1.1.0 and above.

Fresh version 1.1.0 introduced a new plugin for using twind with Fresh. This
plugin is much nicer to use than the raw twind integration that was previously
available.

This code mod will update your project to use the new twind plugin. It will
update your `main.ts` file to import the twind plugin and add it to the plugins
array. It will also update your files to remove many unnecessary uses of the
`tw` function, and remove unnecessary twind imports. While the code mod can
handle most cases, you may need to manually update some code. Additionally you
will need to manually update your `twind.config.ts` if you use a custom
configuration.



================================================
FILE: docs/latest/examples/active-links.md
================================================
---
description: |
  Style active links with ease in Fresh
---

Fresh automatically enhances the accessibility of `<a>` elements by adding the
aria-current attribute when rendering links that match the current URL. This
attribute is recognized by assistive technologies and clearly indicates the
current page within a set of pages.

- aria-current="page" - Added to links with an exact path match, enhancing
  accessibility by indicating the current page to assistive technologies.

As we aim to improve accessibility, we encourage the use of aria-current for
styling current links where applicable.

## Styling with CSS

The aria-current attribute is easily styled with CSS using attribute selectors,
providing a native way to visually differentiate the active link.

```css
/* Give links pointing to the current page a green color */
a[aria-current="page"] {
  color: green;
}

/* Color all ancestor links of the current page */
a[aria-current="true"] {
  color: peachpuff;
}
```

## Tailwind / Twind

In Tailwind or similar CSS frameworks like Twind, you can apply styles to
elements with the ﻿aria-current attribute using bracket notation in your class
definitions. However, the specific syntax varies slightly between Tailwind and
Twind. For Tailwind, use the syntax:

```tsx
function Menu() {
  return (
    <a href="/foo" class="aria-[current]:text-green-600">
      Link to some page
    </a>
  );
}
```

For Twind, the syntax is:

```tsx
function Menu() {
  return (
    <a href="/foo" class="[aria-current]:text-green-600">
      Link to some page
    </a>
  );
}
```

### Twind Plugin

The original twind plugin (`import twindPlugin from "$fresh/plugins/twind.ts";`)
supports the above style:

```tsx
class="[aria-current='page']:text-green-600"
```

### TwindV1 Plugin

The new twind plugin (`import twindPlugin from "$fresh/plugins/twindv1.ts";`)
requires a slightly different syntax (note the position of the left bracket):

```tsx
class="aria-[current='page']:text-green-600"
```



================================================
FILE: docs/latest/examples/authentication-with-supabase.md
================================================
---
description: |
  Learn how to implement the PKCE authentication flow using Supabase.
---

Fresh is a great tool for quickly building lightweight, server-side rendered web
apps and Supabase provides an easy way to add authentication (and/or a
PostgreSQL database backend) to your app.

In this example, we'll create a small app that implements the PKCE
authentication flow using Supabase.

The PKCE authentication flow is designed specifically for applications that
cannot store a client secret, such as native mobile apps or server-side rendered
web apps. You can read up on the specifics of PKCE
[here](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow-with-pkce)
or have a look at
[its specification](https://datatracker.ietf.org/doc/html/rfc7636). Our example
is based on the information you can piece together from the
[Supabase documentation](https://supabase.com/docs/guides/auth/server-side/oauth-with-pkce-flow-for-ssr)
on the topic.

The purpose of the example app we're building here is to showcase the basic
building blocks of an implementation. As such, it is limited in functionality
and purposefully leaves out things like
[password resets](https://supabase.com/docs/guides/auth/server-side/email-based-auth-with-pkce-flow-for-ssr),
[proper error handling](https://fresh.deno.dev/docs/concepts/error-pages) as
well as validating input form data. You can find the
[full code here](https://github.com/morlinbrot/supa-fresh-pkce), where the
missing functionality is implemented.

## Supabase

First of all, we need a Supabase account
[which can be created for free here](https://supabase.com/). A handy way to
supply the credentials to our app is via `.env` file (never check in `.env`
files to version control).

```txt .env.example
SUPABASE_URL=https://<projectName>.supabase.co
SUPABASE_ANON_KEY=<api_key>
```

Update the imports section of your `deno.json` file to include the following:

```json deno.json
"imports": {
  "supabase": "npm:@supabase/supabase-js@2",
  "supabase/ssr": "npm:@supabase/ssr",
}
```

Since Deno 1.38, we reading .env files is built-in and can be enabled with the
`--env` flag. Here's the complete command to run our app:

```shell
deno run --unstable-kv --allow-env --allow-read --allow-write --allow-run --allow-net --watch=static/,routes/ dev.ts
```

### `@supabase/ssr`

Supabase provides the `@supabase/ssr` package for working with its API in an SSR
context. It exposes the `createServerClient` method that we can use on the
server side. Set it up like so:

```ts lib/supabase.ts
import { deleteCookie, getCookies, setCookie } from "$std/http/cookie.ts";
import { assert } from "$std/assert/assert.ts";
import { type CookieOptions, createServerClient } from "supabase/ssr";

export function createSupabaseClient(
  req: Request,
  // Keep this optional parameter in mind, we'll get back to it.
  resHeaders = new Headers(),
) {
  const SUPABASE_URL = Deno.env.get("SUPABASE_URL");
  const SUPABASE_ANON_KEY = Deno.env.get("SUPABASE_ANON_KEY");

  assert(
    SUPABASE_URL && SUPABASE_ANON_KEY,
    "SUPABASE URL and SUPABASE_ANON_KEY environment variables must be set.",
  );

  return createServerClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
    auth: { flowType: "pkce" },
    cookies: {
      get(name: string) {
        return decodeURIComponent(getCookies(req.headers)[name]);
      },
      set(name: string, value: string, options: CookieOptions) {
        setCookie(resHeaders, {
          name,
          value: encodeURIComponent(value),
          ...options,
        });
      },
      remove(name: string, options: CookieOptions) {
        deleteCookie(resHeaders, name, options);
      },
    },
  });
}
```

Note: We are specifying the `flowType` to be `pkce` and that we're using
[`encodeURIComponent()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)
to serialize and store the session object as a cookie.

Crucially, _we need to create a new instance of this client for each request!_

## Sign Up

In our endpoints, we can now use this client to talk to the Supabase API. Here's
the `/api/sign-up` handler:

```ts routes/api/sign-up.ts
import { FreshContext, Handlers } from "$fresh/server.ts";
import { createSupabaseClient } from "lib/supabase.ts";

export const handler: Handlers = {
  async POST(req: Request, _ctx: FreshContext) {
    const form = await req.formData();
    const email = form.get("email");
    const password = form.get("password");

    const headers = new Headers();
    headers.set("location", "/sign-in"); // Redirect to /sign-in on success.

    const supabase = createSupabaseClient(req);
    const { error } = await supabase.auth.signUp({
      email: String(email),
      password: String(password),
    });

    if (error) throw error; // Have a look at the full app for proper error handling.

    return new Response(null, { status: 303, headers });
  },
};
```

Create a form to call our API endpoint and render it at `/sign-up`:

```tsx routes/sign-up.tsx
export default function SignUpPage() {
  return (
    <form action="/api/sign-up" method="post">
      <input autofocus type="email" name="email" />
      <input type="password" name="password" />
      <button type="submit">Submit</button>
    </form>
  );
}
```

## Confirmation

To complete the sign-up process, we need a `/confirm` route to intercept
successful email confirmations:

```ts routes/api/confirm.ts
import { Handlers } from "$fresh/server.ts";
import { createSupabaseClient } from "lib/supabase.ts";

export const handler: Handlers = {
  async GET(req: Request) {
    const { searchParams } = new URL(req.url);
    const token_hash = searchParams.get("token_hash");
    const type = searchParams.get("type") as EmailOtpType | null;
    const next = searchParams.get("next") ?? "/welcome";

    const redirectTo = new URL(req.url);
    redirectTo.pathname = next;

    if (token_hash && type) {
      const supabase = createSupabaseClient(req);
      const { error } = await supabase.auth.verifyOtp({ type, token_hash });
      if (error) throw error; // Have a look at the full app for proper error handling.
    }

    redirectTo.searchParams.delete("next");
    return Response.redirect(redirectTo);
  },
};
```

Have a look at the Supabase docs on the
[details on how to configure email templates and other endpoints](https://supabase.com/docs/guides/auth/server-side/email-based-auth-with-pkce-flow-for-ssr)
like `/password-reset` you would need for a full implementation.

## Sign In

The `/api/sign-in` route is pretty straight-forward, too:

```ts routes/api/sign-in.ts
import { Handlers } from "$fresh/server.ts";
import { createSupabaseClient } from "lib/supabase.ts";

export const handler: Handlers = {
  async POST(req) {
    const form = await req.formData();
    const email = form.get("email")!;
    const password = form.get("password")!;

    const headers = new Headers();
    headers.set("location", "/");

    const supabase = createSupabaseClient(req, headers);
    const { error } = await supabase.auth.signInWithPassword({
      email,
      password,
    });

    if (error) throw error; // Have a look at the full app for proper error handling.

    return new Response(null, { status: 303, headers });
  },
};
```

Note: We're passing `headers` this time. The Supabase client will set the
session as a cookie for us, which we will want to pick up in the middleware that
we are writing next.

## Middleware

We can now write a middleware that will check the auth status of any request,
guarding any protected routes. You can read up on middlewares and where to put
them [in the docs](https://fresh.deno.dev/docs/concepts/middleware).

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";
import { createSupabaseClient } from "lib/supabase.ts";

export const handler = [
  async function authMiddleware(req: Request, ctx: FreshContext) {
    const url = new URL(req.url);
    const headers = new Headers();
    headers.set("location", "/");

    const supabase = createSupabaseClient(req, headers);
    // Note: Always use `getUser` instead of `getSession` as this calls the Supabase API and revalidates the token.
    const { error, data: { user } } = await supabase.auth.getUser();

    const isProtectedRoute = url.pathname.includes("secret");

    // Don't mind 401 as it just means no credentials were provided. E.g. There was no session cookie.
    if (error && error.status !== 401) throw error; // Have a look at the full app for proper error handling.

    if (isProtectedRoute && !user) {
      return new Response(null, { status: 303, headers });
    }

    ctx.state.user = user;

    return ctx.next();
  },
];
```

That's it! These are the building blocks for implementing the PKCE
authentication flow in a Fresh app using Supabase. Again, have a look at the
[full code here](https://github.com/morlinbrot/supa-fresh-pkce) for a fully
featured version of the app.



================================================
FILE: docs/latest/examples/changing-the-src-dir.md
================================================
---
description: |
  Change the source directory to effectively manage your project.
---

When you initialize a project with `deno run -A -r https://fresh.deno.dev`,
you'll end up with a project like the following:

```txt Project Structure
.
├── README.md
├── components
│   └── Button.tsx
├── deno.json
├── dev.ts
├── fresh.gen.ts
├── islands
│   └── Counter.tsx
├── main.ts
├── routes
│   ├── greet
│   │   ├── [name].tsx
│   ├── api
│   │   └── joke.ts
│   ├── _404.tsx
│   └── index.tsx
└── static
    ├── favicon.ico
    └── logo.svg
```

## Using a `src` directory

If you'd like your code to live in an `src` directory (or any other directory of
your choosing), then you'll need to do the following things:

1. Move all your files, except `deno.json` and `README.md`, to the `src`
   directory.
2. Modify the `start` task in `deno.json` to point to the new directory.

Here's what the diff of `deno.json` looks like:

```diff deno.json
 {
   "lock": false,
   "tasks": {
-    "start": "deno run -A --watch=static/,routes/ dev.ts"
+    "start": "deno run -A --watch=src/static/,src/routes/ src/dev.ts"
   },
   "imports": {
     "$fresh/": "file:///Users/reed/code/fresh/",
```

The resulting file structure looks like this:

```txt Project Structure
.
├── README.md
├── deno.json
└── src
    ├── components
    │   └── Button.tsx
    ├── dev.ts
    ├── fresh.gen.ts
    ├── islands
    │   └── Counter.tsx
    ├── main.ts
    ├── routes
    │   ├── greet
    │   │   ├── [name].tsx
    │   ├── api
    │   │   └── joke.ts
    │   ├── _404.tsx
    │   └── index.tsx
    └── static
        ├── favicon.ico
        └── logo.svg
```

Success! Your code now lives elsewhere.



================================================
FILE: docs/latest/examples/client-side-components-and-libraries.md
================================================
---
description: |
  Client side components and libraries
---

Some components depend on client environments, browser-specific features, or
dynamic user interactions, making them incompatible or non-functional during
server-side rendering.

By employing conditional rendering and state management techniques, we can
ensure graceful handling of library or data loading, improving workflow and
usability of such components.

Let's explore an example utilizing this solution with Leaflet, a popular mapping
library, in a Fresh application. The objective is to ensure the proper rendering
of Leaflet components on the client side while gracefully handling them on the
server side.

The full code is available at the end of the page

## Explanation

The first step is creating the context variable to enhance usability across
various components within a Fresh application. By initializing these variables
with a null value and integrating type references, developers can streamline the
use of client side features while adapting to scenarios where server side
rendering might not be feasible.

> [warn]: Proper typing might not be easily available, so we might need to
> define our own types or not use types at all.

```ts
export const leafletContext = createContext<typeof Leaflet | null>(null);
```

Then, we should implement a Provider Component, this will handle loading and
passing down values to be used in other components, other than that, we also
need to handle the server side case as well.

In this example, for the server side we are simply rendering a placeholder in
place of our component tree. As for the context value, we are using html tags to
inject the library on the window and a onLoad callback to set the value of our
state, and this value will be handled/shared with our other components.

> [warn]: Be careful with providers, the manner in which they load/inject both
> script and css may cause issues. Leaflet, for instance, will throw errors if
> we try to load it again.

```tsx
function LeafletProvider(props: { children: ComponentChildren }) {
  if (!IS_BROWSER) {
    return (
      <p>Leaflet must be loaded on the client. No children will render</p>
    );
  }
  const value = useSignal<typeof Leaflet | null>(null)
  return (
    <>
      {/* Load Leaflet CSS */}
      <link
        rel="stylesheet"
        href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
        integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
        crossorigin=""
      />
      {/* Load Leaflet JS */}
      <script
        onLoad={() => value.value = window.L}
        src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
        integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
        crossorigin=""
      />
      {/* Provide Leaflet context to children */}
      <leafletContext.Provider value={value}>
        {props.children}
      </LeafletContext.Provider>
    </>
  );
}
```

In order to utilize the context, call the useContext hook with the context
variable this will give us access to the value set in the Provider. Handling
cases where the context has not loaded values yet is a good practice as well, in
this way we can have a smooth integration and manipulation of client-side data
and logic on our server-side code.

```tsx
function MapComponent() {
  const leaf = useContext(leafletContext);
  if (!leaf) return <p>Context not ready. Component placeholder</p>;
  useEffect(() => {
    const map = leaf.map("map").setView(leaf.latLng(0, 0), 2);
    leaf.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution:
        '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map);
  });
  return <div id="map" class="relative w-[80vw] h-[50vh]" />;
}
```

Here is an example island encapsulating both the provider and component in order
to demonstrate a simple usage. In real cases, it's usually better to add the
Provider directly to our Page and then use Components that depend on that
provider inside it.

```tsx
export default function MapIsland() {
  return (
    <LeafletProvider>
      <MapComponent />
    </LeafletProvider>
  );
}
```

## Full code:

```tsx MapIsland.tsx
import * as Leaflet from "https://esm.sh/v135/@types/leaflet@1.9.4/index.d.ts";
import { IS_BROWSER } from "$fresh/runtime.ts";
import { useContext, useEffect } from "preact/hooks";
import { ComponentChildren, createContext } from "preact";
import { useSignal } from "@preact/signals";

// Create a context to hold Leaflet data/functions
const LeafletContext = createContext<typeof Leaflet | null>(null);

// LeafletProvider component manages Leaflet loading and context
function LeafletProvider(props: { children: ComponentChildren }) {
  if (!IS_BROWSER) {
    return <p>Leaflet must be loaded on the client. No children will render</p>;
  }
  const value = useSignal<typeof Leaflet | null>(null);
  return (
    <>
      {/* Load Leaflet CSS */}
      <link
        rel="stylesheet"
        href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
        integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
        crossorigin=""
      />
      {/* Load Leaflet JS */}
      <script
        onLoad={() => value.value = window.L}
        src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
        integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
        crossorigin=""
      />
      {/* Provide Leaflet context to children */}
      <LeafletContext.Provider value={value}>
        {props.children}
      </LeafletContext.Provider>
    </>
  );
}

// MapComponent utilizes Leaflet context for rendering the map
function MapComponent() {
  const leaf = useContext(LeafletContext);
  if (!leaf) return <div>Component placeholder</div>;
  useEffect(() => {
    const map = leaf.map("map").setView(leaf.latLng(0, 0), 2);
    leaf.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution:
        '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map);
  });
  return <div id="map" class="relative w-[80vw] h-[50vh]" />;
}

// MapIsland is the parent component integrating LeafletProvider and MapComponent
export default function MapIsland() {
  return (
    <LeafletProvider>
      <MapComponent />
    </LeafletProvider>
  );
}
```



================================================
FILE: docs/latest/examples/creating-a-crud-api.md
================================================
---
description: |
  Use HTTP CRUD methods to perform operations on resources. Learn how to use HTTP handlers to create a RESTful API.
---

The MDN [docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods) are a
great resource to learn more about HTTP methods. We'll touch on the four
fundamental methods necessary to create a basic CRUD (create, read, update,
delete) API here. Additionally, we'll briefly mention CORS requests and how
`OPTIONS` comes into play.

Using HTTP methods is a common way to create a REST API. Fresh supports common
HTTP methods in handlers out of the box. Async HTTP requests are also supported.
Read more about custom handlers [here](/docs/getting-started/custom-handlers).

In this example we'll be creating a small API that uses
[Deno KV](https://deno.com/kv) to store users in a database.

Our project structure will look like this (in addition to the rest of the Fresh
code from a new project):

```txt Project Structure
routes
└── api
    └── users
        ├── [id].ts
        └── index.ts
```

In each section about a method, only the relevant handler will be shown. The
full files are available at the bottom for reference.

## POST

`POST` (create) is used to create a resource.

```tsx routes/api/users/index.ts
export const handler: Handlers<User | null> = {
  async POST(req, _ctx) {
    const user = (await req.json()) as User;
    const userKey = ["user", user.id];
    const ok = await kv.atomic().set(userKey, user).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(JSON.stringify(user));
  },
};
```

Test this with Postman (or your favorite client) with a URL like
`http://localhost:8000/api/users` and a method of `POST`. Make sure to have a
payload like:

```json
{
  "id": "2",
  "name": "TestUserName"
}
```

You should receive the same thing back:

```json
{ "id": "2", "name": "TestUserName" }
```

## GET

`GET` (read) is used to retrieve a resource and is by far the most common HTTP
method. You can use `GET` to fetch database content, markdown, or static files.

```tsx routes/api/users/[id].ts
export const handler: Handlers<User | null> = {
  async GET(_req, ctx) {
    const id = ctx.params.id;
    const key = ["user", id];
    const user = (await kv.get<User>(key)).value!;
    return new Response(JSON.stringify(user));
  },
};
```

Let's practice retrieving our user! A `GET` request to
`http://localhost:8000/api/users/2` should return:

```json
{ "id": "2", "name": "TestUserName" }
```

## PUT (and PATCH)

`PUT` (update) and `PATCH` are used to update a resource. While similar, there
are differences and you should use the one that best suits your use case. Read
more about HTTP methods on
[MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).

The short version of it: `PUT` requires the entire object to be submitted, while
`PATCH` requires only the properties that are different to be submitted.

An example of an update endpoint using `PUT`:

```tsx routes/api/users/[id].ts
export const handler: Handlers<User | null> = {
  async PUT(req, ctx) {
    const id = ctx.params.id;
    const user = (await req.json()) as User;
    const userKey = ["user", id];
    const userRes = await kv.get(userKey);
    if (!userRes.value) return new Response(`no user with id ${id} found`);
    const ok = await kv.atomic().check(userRes).set(userKey, user).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(JSON.stringify(user));
  },
};
```

Time to change their name. We'll now `PUT` a request to
`http://localhost:8000/api/users/2` like:

```json
{
  "id": "2",
  "name": "New Name"
}
```

We should receive:

```json
{ "id": "2", "name": "New Name" }
```

If, on the other hand, we chose to implement this as a `PATCH` operation, the
request would just involve the changed property like this:

```json
{
  "name": "New Name"
}
```

No need to send in the id in this case.

## DELETE

`DELETE` (delete) is used to delete a resource.

```tsx routes/api/users/[id].ts
export const handler: Handlers<User | null> = {
  async DELETE(_req, ctx) {
    const id = ctx.params.id;
    const userKey = ["user", id];
    const userRes = await kv.get(userKey);
    if (!userRes.value) return new Response(`no user with id ${id} found`);
    const ok = await kv.atomic().check(userRes).delete(userKey).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(`user ${id} deleted`);
  },
};
```

Try sending `DELETE` to `http://localhost:8000/api/users/2` without a body.
We'll get back:

```txt
user 2 deleted
```

## OPTIONS

Options can be used for some advanced cases, including implementing preflight
request checks for complex CORS use cases. See more on the
[CORS documentation](/docs/examples/dealing-with-cors).

## Full File Reference

<details>
<summary>[id].ts</summary>

```ts
import { Handlers } from "$fresh/server.ts";

type User = {
  id: string;
  name: string;
};

const kv = await Deno.openKv();

export const handler: Handlers<User | null> = {
  async GET(_req, ctx) {
    const id = ctx.params.id;
    const key = ["user", id];
    const user = (await kv.get<User>(key)).value!;
    return new Response(JSON.stringify(user));
  },
  async DELETE(_req, ctx) {
    const id = ctx.params.id;
    const userKey = ["user", id];
    const userRes = await kv.get(userKey);
    if (!userRes.value) return new Response(`no user with id ${id} found`);
    const ok = await kv.atomic().check(userRes).delete(userKey).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(`user ${id} deleted`);
  },
  async PUT(req, ctx) {
    const id = ctx.params.id;
    const user = (await req.json()) as User;
    const userKey = ["user", id];
    const userRes = await kv.get(userKey);
    if (!userRes.value) return new Response(`no user with id ${id} found`);
    const ok = await kv.atomic().check(userRes).set(userKey, user).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(JSON.stringify(user));
  },
};
```

</details>

<details>
<summary>index.ts</summary>

```ts
import { Handlers } from "$fresh/server.ts";

type User = {
  id: string;
  name: string;
};

const kv = await Deno.openKv();

export const handler: Handlers<User | null> = {
  async GET(_req, _ctx) {
    const users = [];
    for await (const res of kv.list({ prefix: ["user"] })) {
      users.push(res.value);
    }
    return new Response(JSON.stringify(users));
  },
  async POST(req, _ctx) {
    const user = (await req.json()) as User;
    const userKey = ["user", user.id];
    const ok = await kv.atomic().set(userKey, user).commit();
    if (!ok) throw new Error("Something went wrong.");
    return new Response(JSON.stringify(user));
  },
};
```

</details>



================================================
FILE: docs/latest/examples/dealing-with-cors.md
================================================
---
description: |
  CORS enabling routes in your Fresh project.
---

So you've encountered some CORS problems and are on the hunt for the solution?
You're in the right spot.

Here's a good [resource](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
talking about CORS in general, in case you don't fully understand what's wrong.

## Simple CORS -- Middleware

As per the above link, "simple" requests involve `GET`, `HEAD`, or `POST`
requests. You can CORS enable all the routes affected by some `middleware` by
doing the following:

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

export async function handler(req: Request, ctx: FreshContext) {
  const origin = req.headers.get("Origin") || "*";
  const resp = await ctx.next();
  const headers = resp.headers;

  headers.set("Access-Control-Allow-Origin", origin);
  headers.set("Access-Control-Allow-Credentials", "true");
  headers.set(
    "Access-Control-Allow-Headers",
    "Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization, accept, origin, Cache-Control, X-Requested-With",
  );
  headers.set(
    "Access-Control-Allow-Methods",
    "POST, OPTIONS, GET, PUT, DELETE",
  );

  return resp;
}
```

## Complex CORS -- Middleware

What about for one of the other HTTP methods? Then you'll need to be able to
deal with "preflight requests". Let's imagine you're trying to support a
`DELETE` route. Then you'd need to do something like this:

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

export async function handler(req: Request, ctx: FreshContext) {
  if (req.method == "OPTIONS") {
    const resp = new Response(null, {
      status: 204,
    });
    const origin = req.headers.get("Origin") || "*";
    const headers = resp.headers;
    headers.set("Access-Control-Allow-Origin", origin);
    headers.set("Access-Control-Allow-Methods", "DELETE");
    return resp;
  }
  const origin = req.headers.get("Origin") || "*";
  const resp = await ctx.next();
  const headers = resp.headers;

  headers.set("Access-Control-Allow-Origin", origin);
  headers.set("Access-Control-Allow-Credentials", "true");
  headers.set(
    "Access-Control-Allow-Headers",
    "Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization, accept, origin, Cache-Control, X-Requested-With",
  );
  headers.set(
    "Access-Control-Allow-Methods",
    "POST, OPTIONS, GET, PUT, DELETE",
  );

  return resp;
}
```

These complex results require a two step process:

1. the browser makes an `OPTIONS` request to find out about the allowed methods
2. the browser makes the actual request

So you can see the middleware has some special handling to deal with `OPTIONS`
requests.

## CORS in Routes

Of course there's no reason why you need to use middleware in order to solve
this. The headers can be set directly in the
[handler](/docs/getting-started/custom-handlers) as well.



================================================
FILE: docs/latest/examples/handling-complex-routes.md
================================================
---
description: |
  Sometimes URL based routing isn't enough.
---

The page on [routing](/docs/concepts/routing) hints at complex routing based on
URL patterns using a `RouteConfig` object. Let's dive into this in a bit more
detail.

A `RouteConfig` has a `routeOverride` string property, which makes use of the
[URL Pattern API](https://developer.mozilla.org/en-US/docs/Web/API/URL_Pattern_API).
Here you can define named groups, wildcards, regex groups, and other bits.

## Simple Route Config

Let's look at the example from the routing page more closely. We'll flesh out
the handler so that we end up with something like the following:

```ts routes/x.tsx
import { FreshContext, RouteConfig } from "$fresh/server.ts";

export const handler = {
  GET(_req: Request, { params }: FreshContext) {
    console.log(params);
    return new Response(params.path);
  },
};

export const config: RouteConfig = {
  routeOverride: "/x/:module@:version/:path*",
};
```

Now if we hit the server with a request like
`http://localhost:8000/x/bestModule@1.33.7/asdf`, then logging the params will
show the following:

```
{
  module: "bestModule",
  version: "1.33.7",
  path: "asdf"
}
```

## Complex Route Config

Let's look at something a bit more complex:

```ts routes/api.tsx
import { FreshContext, RouteConfig } from "$fresh/server.ts";

export const handler = {
  GET(_req: Request, { params }: FreshContext) {
    console.log(params);
    return new Response(params.path);
  },
};

export const config: RouteConfig = {
  routeOverride: "/api/db/:resource(jobs?|bar)/:id(\\d+)?",
};
```

Values are available via `params.resource` and `params.id`.

Here are some example URLs that match this:

- /api/db/bar/1
- /api/db/jobs/1
- /api/db/job/1
- /api/db/job
- /api/db/jobs
- /api/db/bar

Here are some that don't:

- /api/db/other/123
- /api/db/jobs/abc
- /api/db

## Regex

At this point is should be clear that this is essentially an exercise in
understanding regex. There are [numerous](https://regexr.com/)
[resources](https://regex101.com/) [available](https://chat.openai.com/) for
getting assistance with regex.



================================================
FILE: docs/latest/examples/index.md
================================================
---
description: |
  In this chapter of the Fresh documentation, you can find examples of features that you may like in your Fresh project.
---

In this chapter of the Fresh documentation, you can find examples of features
that you may like in your Fresh project. If there's a specific example you'd
like to see here, please open
[a GitHub discussion](https://github.com/denoland/fresh/discussions/new?category=ideas).

- [Modifying the `<head>`](./examples/modifying-the-head)
- [Setting the language](./examples/setting-the-language)
- [Writing tests](./examples/writing-tests)
- [Changing the source directory](./examples/changing-the-src-dir)
- [Initializing the server](./examples/init-the-server)
- [Using Fresh canary version](./examples/using-fresh-canary-version)
- [Dealing with CORS](./examples/dealing-with-cors)
- [Creating a CRUD API](./examples/creating-a-crud-api)
- [Handling complex routes](./examples/handling-complex-routes)
- [Rendering markdown](./examples/rendering-markdown)
- [Sharing state between islands](./examples/sharing-state-between-islands)
- [Using CSP](./examples/using-csp)



================================================
FILE: docs/latest/examples/init-the-server.md
================================================
---
description: |
  For when you have some complicated setup that needs to be performed once.
---

Let's pretend you've just initialized a new Fresh project. You want to do some
complicated setup that runs once, before the server is started. This is,
fortunately, quite easy. Here's how. Modify your `fresh.config.ts` like this:

```diff fresh.config.ts
 import twindConfig from "./twind.config.ts";
+import { Context } from "./routes/_middleware.ts";
+
+await Context.init();

 export default defineConfig({
   plugins: [twindPlugin(twindConfig)],
```

So your full `fresh.config.ts` should look like this:

```ts fresh.config.ts
import { defineConfig } from "$fresh/server.ts";
import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.ts";
import { Context } from "./routes/_middleware.ts";

await Context.init();

export default defineConfig({
  plugins: [twindPlugin(twindConfig)],
});
```

But what's going on in this new `_middleware.ts` we've created?

```ts routes/_middleware.ts
import { FreshContext } from "$fresh/server.ts";

export interface State {
  context: Context;
}

export class Context {
  private static context: Context;
  private complicatedStartupValue: number;

  public constructor() {
    console.log("i'm logged during initialization, and not during handling!");
    // presumably this involves connecting to a
    // database or doing some heavy computation
    this.complicatedStartupValue = 42;
  }

  public static async init() {
    Context.context = new Context();
  }

  public static instance() {
    if (this.context) return this.context;
    else throw new Error("Context is not initialized!");
  }
}

export async function handler(
  _req: Request,
  ctx: FreshContext<State>,
) {
  ctx.state.context = Context.instance();
  if (ctx.destination === "route") {
    console.log("i'm logged during a request!");
    console.log(ctx.state.context);
  }
  const resp = await ctx.next();
  return resp;
}
```

So now in this `handler` (or any other `handler` functions you create) you can
have access to the complicated initialization step by calling
`Context.instance()`.

## Proving it out

### Dev

When you run `deno task start` you should see the following output:

```
Task start deno run -A --watch=static/,routes/ dev.ts
Watcher Process started.
i'm logged during initialization, and not during handling!
The manifest has been generated for 6 routes and 1 islands.

 🍋 Fresh ready
    Local: http://localhost:8000/
```

Going to `http://localhost:8000/` should produce:

```
i'm logged during a request!
Context { complicatedStartupValue: 42 }
```

### Build

When you run `deno task build` you should see:

```
Task build deno run -A dev.ts build
i'm logged during initialization, and not during handling!
The manifest has been generated for 6 routes and 1 islands.
Assets written to: /path/to/my/project/_fresh
```

There's no handling of routes associated with this, but note that the
initialization occurred.

### Preview

Finally when you run `deno task preview` you should see:

```
Task preview deno run -A main.ts
i'm logged during initialization, and not during handling!
Using snapshot found at /Users/reed/code/temp/1763/_fresh

 🍋 Fresh ready
    Local: http://localhost:8000/
```

Going to `http://localhost:8000/` should produce:

```
i'm logged during a request!
Context { complicatedStartupValue: 42 }
```



================================================
FILE: docs/latest/examples/migrating-to-tailwind.md
================================================
---
description: |
  Migrating from twind to Tailwind CSS
---

Starting with version 1.6 Fresh comes with a proper Tailwind CSS plugin out of
the box. When you create a new Fresh project, checking the Tailwind CSS option
will now install the Tailwind CSS plugin instead of twind like it did before.

## Requirements before migrating

The tailwind plugin requires Fresh's
[ahead of time builds](/docs/concepts/ahead-of-time-builds) to be set up,
otherwise it won't work. Make sure to switch your projects to ahead of time
builds in your project before continuing this guide. If your project is already
configured to use ahead of time builds, then you're good to go.

## Migrating to Tailwind CSS

1. Create a `<project>/tailwind.config.ts` file in your project folder:

```ts tailwind.config.ts
import { type Config } from "tailwindcss";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx}",
  ],
} satisfies Config;
```

2. Create a css file in your static directory `<project>/static/styles.css`:

```css static/styles.css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

3. Add the created stylesheet in your HTML in `<project>/routes/_app.tsx`:

```diff routes/_app.tsx
  import { AppProps } from "$fresh/server.ts";
  
  export default function App({ Component }: AppProps) {
    return (
      <html>
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <title>My Fresh Project</title>
+         <link rel="stylesheet" href="/styles.css" />
        </head>
        <body>
          <Component />
      </body>
      </html>
    );
  }
```

4. Replace the `twind` plugin with `tailwind`

```diff fresh.config.ts
  import { defineConfig } from "$fresh/server.ts";
- import twind from "$fresh/plugins/twind.ts";
+ import tailwind from "$fresh/plugins/tailwind.ts";

  export default defineConfig({
-   plugins: [twind()],
+   plugins: [tailwind()],
  });
```

5. Update your `deno.json` file and add the following `tailwindcss` imports. To
   make the
   [vscode Tailwind CSS extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
   work, we also need to set `"nodeModulesDir": "auto"`. This will create a
   `node_modules` directory in your project folder:

```diff deno.json
  {
+   "nodeModulesDir": "auto",
    "imports": {
      "$fresh/": "https://deno.land/x/fresh@1.5.2/",
      "preact": "https://esm.sh/preact@10.22.0",
      "preact/": "https://esm.sh/preact@10.22.0/",
-     "twind": "https://esm.sh/twind@0.16.19",
-     "twind/": "https://esm.sh/twind@0.16.19/"
+     "tailwindcss": "npm:tailwindcss@3.4.1"
    }
  }
```

6. Add `node_modules` to your `.gitignore` or create one if the file is not
   present in your project root directory.

```diff .gitignore
+ node_modules/
```

That's it! Now you can use Tailwind CSS in your project.

> [info]: If you're a vscode user, be sure to install the
> [official Tailwind CSS extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)
> to get full intellisense support. For it to work you also need to set
> `"nodeModulesDir": "auto"` in your `deno.json`.

> [warn]: Tailwind CSS doesn't support the grouping syntax from twind:
> `text(lg uppercase gray-100)`. These need to be rewritten to their expanded
> values like `text-lg uppercase text-gray-100`. Selecting `data-*` or `aria-*`
> attributes works a little different with Tailwind CSS as well.
>
> | Twind                       | Tailwind CSS                |
> | --------------------------- | --------------------------- |
> | `[data-current]:bg-red-600` | `data-[current]:bg-red-300` |
> | `[aria-current]:bg-red-600` | `aria-[current]:bg-red-300` |

> [warn]: Tailwind CSS does not allow you to generate and apply CSS classes
> dynamically, which means you need to explicitly specify the class you want to
> apply. In other words, to use dynamic classes, you need to ensure that they
> are present in the final CSS file.
>
> | Twind                             | Tailwind CSS                                                 |
> | --------------------------------- | ------------------------------------------------------------ |
> | `<a class={`link-${color}`}></a>` | `<a class={color === 'blue' ?`link-blue`:`link-green`}></a>` |

## Frequently Asked Questions (FAQ)

### What are the differences between twind and Tailwind CSS?

Twind is a project that tries to enable you to use Tailwind-like styling
capabilities in a single script that can also be used in the browser. The key
difference between the two is that twind generates CSS on the fly on every
request and was shipped to the browser to make newly generated classes by
islands work in Fresh. Overall, this wasn't an ideal setup for building
performant sites.

In contrast to that, Tailwind CSS extracts generates the resulting CSS file
ahead of time, which only happens once per deployment. There is no runtime
component needed, which makes your Fresh project respond faster to requests.

During the Tailwind CSS v2 days twind pushed a lot of great ideas like allowing
any number to be used for classes like `opacity-82` and others, but it hasn't
kept up with recent developments of Tailwind CSS. In fact, twind has been
unmaintained for more than a year by now. We never could get autocompletion with
twind to work either.

### Why did Fresh use twind instead of Tailwind CSS?

When Fresh was originally built, Deno didn't support npm modules or node APIs.
This meant that Tailwind CSS didn't work with Deno. Now, many years later, Deno
does ship with support for both of that and we can use the same npm
`tailwindcss` module as everyone else.



================================================
FILE: docs/latest/examples/modifying-the-head.md
================================================
---
description: |
  Add components like <title> or <meta> to a <head> tag using Fresh's <Head> component.
---

We can use the `<Head />` component in `$fresh/runtime.ts` to add elements as
children of the `<head>` element. By adding elements as children of Fresh's
`<Head />` tag, these automatically get injected into the `<head>` element of
the web page. Some uses include:

- Setting the document title using `<title>`
- Specifying page metadata using `<meta>`
- Linking to resources like stylesheets using `<link>`
- Including third-party JavaScript code using `<script>`

```tsx routes/index.tsx
import { Head } from "$fresh/runtime.ts";

export default function Home() {
  return (
    <>
      <Head>
        <meta charset="UTF-8" />
        <title>Fresh App</title>
        <meta
          name="description"
          content="This is a brief description of Fresh"
        />
        <link rel="stylesheet" href="styles.css" />
        <script src="script.js"></script>
      </Head>
      <div class="p-4 mx-auto max-w-screen-md">
        <h1>Hello World</h1>
      </div>
    </>
  );
}
```

## Avoiding duplicate tags

You might end up with duplicate tags, when multiple `<Head />` components are
rendered on the same page. This can happen when you render `<Head />` in a route
and another `<Head />` in another component for example.

```tsx
// routes/page-a.tsx
<Head>
  <meta name="og:title" content="This is a title" />
</Head>

// components/MyTitle.tsx
<Head>
  <meta name="og:title" content="Other title" />
</Head>
```

To ensure that the tag is not duplicated, Fresh supports setting the `key` prop.
By giving matching elements the same `key` prop, only the last one will be
rendered.

```diff
  // routes/page-a.tsx
  <Head>
-   <meta name="og:title" content="This is a title" />
+   <meta name="og:title" content="This is a title" key="title" />
  </Head>

  // components/MyTitle.tsx
  <Head>
-   <meta name="og:title" content="Other title" />
+   <meta name="og:title" content="Other title" key="title" />
  </Head>
```

The rendered page will only include the `<meta>`-tag with `"Other title"`.

> [info]: The `<title>`-tag is automatically deduplicated, even without a `key`
> prop.



================================================
FILE: docs/latest/examples/rendering-markdown.md
================================================
---
description: |
  How to render markdown on your Fresh site.
---

What if you want to render some markdown on your site? There are a few
possibilities:

1. the markdown is coming from a remote source
2. the markdown is defined in a string
3. the markdown is on a file

The following file uses
[dynamic routing](https://fresh.deno.dev/docs/getting-started/dynamic-routes) to
handle the three cases. It's assumed this file is called `[slug].tsx`:

```ts routes/[slug].tsx
import { Handlers, PageProps } from "$fresh/server.ts";
import { extract } from "$std/front_matter/yaml.ts";
import { CSS, render } from "$gfm";
import { Head } from "$fresh/runtime.ts";

interface Page {
  markdown: string;
  data: Record<string, unknown>;
}

export const handler: Handlers<Page> = {
  async GET(_req, ctx) {
    let rawMarkdown = "";
    if (ctx.params.slug === "remote") {
      const resp = await fetch(
        `https://raw.githubusercontent.com/denoland/fresh/main/docs/latest/introduction/index.md`,
      );
      if (resp.status !== 200) {
        return ctx.render(undefined);
      }
      rawMarkdown = await resp.text();
    } else if (ctx.params.slug === "string") {
      rawMarkdown = `---
description: test
---

## big text

Look, it's working. _This is in italics._
      
      `;
    } else if (ctx.params.slug === "file") {
      rawMarkdown = await Deno.readTextFile("text.md");
    } else {
      return ctx.render(undefined);
    }
    const { attrs, body } = extract(rawMarkdown);
    return ctx.render({ markdown: body, data: attrs });
  },
};

export default function MarkdownPage({ data }: PageProps<Page | null>) {
  if (!data) {
    return <h1>File not found.</h1>;
  }

  return (
    <>
      <Head>
        <style dangerouslySetInnerHTML={{ __html: CSS }} />
      </Head>
      <main>
        <div>{JSON.stringify(data.data)}</div>
        <div
          class="markdown-body"
          dangerouslySetInnerHTML={{ __html: render(data?.markdown) }}
        />
      </main>
    </>
  );
}
```

The contents of the `text.md` file are the following:

```md text.md
---
description: testFromText
---

# Really Big Text

**bold**
```

You'll also need to import the `Github Flavored Markdown` module:

```bash
deno add jsr:@deno/gfm
```

Andy has a helpful [post](https://deno.com/blog/build-a-blog-with-fresh) on the
Deno Blog which goes into a slightly more realistic example.



================================================
FILE: docs/latest/examples/rendering-raw-html.md
================================================
---
description: |
  How to render raw HTML in Fresh.
---

Text content in Fresh is always escaped, whether serverside rendered or rendered
in islands. While this generally desired, it can create issues in certain
situations.

## Warning

The TL;DR is to use Preact's `dangerouslySetInnerHTML`. As the name implies, it
should not be used lightly.

Setting arbitrary HTML can be dangerous. Make sure you trust the source.
Rendering user-supplied HTML to the DOM makes your site vulnerable to cross-
site scripting. The markup must first be sanitizied, or better yet, something
you trust.

## Example: Rendering JSON-LD

Suppose we need to add some microdata markup to a page. The following will
result in **escaped characters, and will not work**:

```tsx
const json = `
{
  "@context": "http://schema.org",
  "@type": "PostalAddress",
  "streetAddress": "8888 University Drive",
  "addressLocality": "Burnaby",
  "addressRegion": "British Columbia"
}
`;

export default function JsonLd() {
  return <script type="application/ld+json">{json}</script>;
}
```

Instead, we can use `dangerouslySetInnerHTML`:

```tsx
export default function JsonLd() {
  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: json }}
    />
  );
}
```

## Another example: Code highlighting

Syntax highlighters parse strings into HTML tags, allowing them to be
individually styled with CSS. We can build a simple Preact syntax highlighter
like so:

```tsx
import Prism from "https://esm.sh/prismjs@1.29.0";

interface Props {
  code: string;
  lang: string;
}

export default function Code({ code, lang }: Props) {
  const parsed = Prism.highlight(code, Prism.languages[lang], lang);

  return (
    <pre data-lang={lang} className={`language-${lang}`}>
      <code
        dangerouslySetInnerHTML={{
          __html: parsed,
        }}
      />
    </pre>
  );
}
```

Of course, we will also have to add some CSS to make this look nice.



================================================
FILE: docs/latest/examples/setting-the-language.md
================================================
---
description: |
  Set the lang attribute in the <html> tag.
---

When you initialize a project with `deno run -A -r https://fresh.deno.dev`,
you'll end up with a `main.ts` like the following:

```ts main.ts
/// <reference no-default-lib="true" />
/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
/// <reference lib="dom.asynciterable" />
/// <reference lib="deno.ns" />

import { start } from "$fresh/server.ts";
import manifest from "./fresh.gen.ts";

import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.ts";

await start(manifest, { plugins: [twindPlugin(twindConfig)] });
```

This is a great start if your site is in English, but let's say you want to
change the language, as per the `<html lang=asdf>` tag. Then you'll need to do
something like this:

```ts main.ts
/// <reference no-default-lib="true" />
/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
/// <reference lib="dom.asynciterable" />
/// <reference lib="deno.ns" />

import { start } from "$fresh/server.ts";
import manifest from "./fresh.gen.ts";

import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.ts";

await start(manifest, {
  plugins: [twindPlugin(twindConfig)],
  render: (ctx, render) => {
    ctx.lang = "de";
    render();
  },
});
```

If you're curious how this works, start by checking out `TemplateOptions` in
`render.ts`.



================================================
FILE: docs/latest/examples/sharing-state-between-islands.md
================================================
---
description: |
  When you need to have state shared between islands, this page provides a few recipes.
---

All of this content is lifted from this great
[example](https://fresh-with-signals.deno.dev/) by Luca. The source can be found
[here](https://github.com/lucacasonato/fresh-with-signals).

## Multiple Sibling Islands with Independent State

Imagine we have `Counter.tsx` like this:

```tsx islands/Counter.tsx
import { useSignal } from "@preact/signals";
import { Button } from "../components/Button.tsx";

interface CounterProps {
  start: number;
}

// This island is used to display a counter and increment/decrement it. The
// state for the counter is stored locally in this island.
export default function Counter(props: CounterProps) {
  const count = useSignal(props.start);
  return (
    <div class="flex gap-2 items-center w-full">
      <p class="flex-grow-1 font-bold text-xl">{count}</p>
      <Button onClick={() => count.value--}>-1</Button>
      <Button onClick={() => count.value++}>+1</Button>
    </div>
  );
}
```

Note how `useSignal` is within the `Counter` component. Then if we instantiate
some counters like this...

```tsx
<Counter start={3} />
<Counter start={4} />
```

they'll keep track of their own independent state. Not much sharing going on
here, yet.

## Multiple Sibling Islands with Shared State

But we can switch things up by looking at a `SynchronizedSlider.tsx` like this:

```tsx islands/SynchronizedSlider.tsx
import { Signal } from "@preact/signals";

interface SliderProps {
  slider: Signal<number>;
}

// This island displays a slider with a value equal to the `slider` signal's
// value. When the slider is moved, the `slider` signal is updated.
export default function SynchronizedSlider(props: SliderProps) {
  return (
    <input
      class="w-full"
      type="range"
      min={1}
      max={100}
      value={props.slider.value}
      onInput={(e) => (props.slider.value = Number(e.currentTarget.value))}
    />
  );
}
```

Now if we were to do the following...

```tsx routes/index.tsx
export default function Home() {
  const sliderSignal = useSignal(50);
  return (
    <div>
      <SynchronizedSlider slider={sliderSignal} />
      <SynchronizedSlider slider={sliderSignal} />
      <SynchronizedSlider slider={sliderSignal} />
    </div>
  );
}
```

they would all use the same value.

## Independent Islands

We can also create a `signal` in a utility file and export it for consumption
across multiple places.

```ts utils/cart.ts
import { signal } from "@preact/signals";

export const cart = signal<string[]>([]);
```

```tsx islands/AddToCart.tsx
import { Button } from "../components/Button.tsx";
import { cart } from "../utils/cart.ts";

interface AddToCartProps {
  product: string;
}

// This island is used to add a product to the cart state.
export default function AddToCart(props: AddToCartProps) {
  return (
    <Button
      onClick={() => (cart.value = [...cart.value, props.product])}
      class="w-full"
    >
      Add{cart.value.includes(props.product) ? " another" : ""} "{props.product}
      " to cart
    </Button>
  );
}
```

```tsx islands/Cart.tsx
import { Button } from "../components/Button.tsx";
import { cart } from "../utils/cart.ts";
import * as icons from "../components/Icons.tsx";

// This island is used to display the cart contents and remove items from it.
export default function Cart() {
  return (
    <h1 class="text-xl flex items-center justify-center">
      Cart
    </h1>

    <ul class="w-full bg-gray-50 mt-2 p-2 rounded min-h-[6.5rem]">
      {cart.value.length === 0 && (
        <li class="text-center my-4">
          <div class="text-gray-400">
            <icons.Cart class="w-8 h-8 inline-block" />
            <div>
              Your cart is empty.
            </div>
          </div>
        </li>
      )}
      {cart.value.map((product, index) => (
        <CartItem product={product} index={index} />
      ))}
    </ul>
  );
}

interface CartItemProps {
  product: string;
  index: number;
}

function CartItem(props: CartItemProps) {
  const remove = () => {
    const newCart = [...cart.value];
    newCart.splice(props.index, 1);
    cart.value = newCart;
  };

  return (
    <li class="flex items-center justify-between gap-1">
      <icons.Lemon class="text-gray-500" />
      <div class="flex-1">
        {props.product}
      </div>
      <Button onClick={remove} aria-label="Remove" class="border-none">
        <icons.X class="inline-block w-4 h-4" />
      </Button>
    </li>
  );
}
```

Now we can add the islands to our site by doing the following:

```tsx
<AddToCart product="Lemon" />
<AddToCart product="Lime" />
<Cart />
```

What happens as a result? The `cart` signal is shared across the two `AddToCart`
islands _and_ the `Cart` island.



================================================
FILE: docs/latest/examples/using-csp.md
================================================
---
description: |
  Change the source directory to effectively manage your project.
---

As per the
[MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP):

> Content Security Policy (CSP) is an added layer of security that helps to
> detect and mitigate certain types of attacks, including Cross-Site Scripting
> (XSS) and data injection attacks. These attacks are used for everything from
> data theft, to site defacement, to malware distribution.
>
> To enable CSP, you need to configure your web server to return the
> Content-Security-Policy HTTP header. (Sometimes you may see mentions of the
> X-Content-Security-Policy header, but that's an older version and you don't
> need to specify it anymore.)

Fortunately Fresh has built in support for CSP. We don't need to worry about
setting headers ourselves. We just have to configure our routes correctly. Let's
dive into a few examples to see how this works.

Fresh's CSP implementation supports the following

<details>
<summary>directives</summary>

```ts
export interface ContentSecurityPolicyDirectives {
  // Fetch directives
  /**
   * Defines the valid sources for web workers and nested browsing contexts
   * loaded using elements such as <frame> and <iframe>.
   */
  childSrc?: string[];
  /**
   * Restricts the URLs which can be loaded using script interfaces.
   */
  connectSrc?: string[];
  /**
   * Serves as a fallback for the other fetch directives.
   */
  defaultSrc?: string[];
  /**
   * Specifies valid sources for fonts loaded using @font-face.
   */
  fontSrc?: string[];
  /**
   * Specifies valid sources for nested browsing contexts loading using elements
   * such as <frame> and <iframe>.
   */
  frameSrc?: string[];
  /**
   * Specifies valid sources of images and favicons.
   */
  imgSrc?: string[];
  /**
   * Specifies valid sources of application manifest files.
   */
  manifestSrc?: string[];
  /**
   * Specifies valid sources for loading media using the <audio> , <video> and
   * <track> elements.
   */
  mediaSrc?: string[];
  /**
   * Specifies valid sources for the <object>, <embed>, and <applet> elements.
   */
  objectSrc?: string[];
  /**
   * Specifies valid sources to be prefetched or prerendered.
   */
  prefetchSrc?: string[];
  /**
   * Specifies valid sources for JavaScript.
   */
  scriptSrc?: string[];
  /**
   * Specifies valid sources for JavaScript <script> elements.
   */
  scriptSrcElem?: string[];
  /**
   * Specifies valid sources for JavaScript inline event handlers.
   */
  scriptSrcAttr?: string[];
  /**
   * Specifies valid sources for stylesheets.
   */
  styleSrc?: string[];
  /**
   * Specifies valid sources for stylesheets <style> elements and <link>
   * elements with rel="stylesheet".
   */
  styleSrcElem?: string[];
  /**
   * Specifies valid sources for inline styles applied to individual DOM
   * elements.
   */
  styleSrcAttr?: string[];
  /**
   * Specifies valid sources for Worker, SharedWorker, or ServiceWorker scripts.
   */
  workerSrc?: string[];

  // Document directives
  /**
   * Restricts the URLs which can be used in a document's <base> element.
   */
  baseUri?: string[];
  /**
   * Enables a sandbox for the requested resource similar to the <iframe>
   * sandbox attribute.
   */
  sandbox?: string[];

  // Navigation directives
  /**
   * Restricts the URLs which can be used as the target of a form submissions
   * from a given context.
   */
  formAction?: string[];
  /**
   * Specifies valid parents that may embed a page using <frame>, <iframe>,
   * <object>, <embed>, or <applet>.
   */
  frameAncestors?: string[];
  /**
   * Restricts the URLs to which a document can initiate navigation by any
   * means, including <form> (if form-action is not specified), <a>,
   * window.location, window.open, etc.
   */
  navigateTo?: string[];

  /**
   * The URI to report CSP violations to.
   */
  reportUri?: string;
}
```

</details>

For our examples, we'll just be focused on `styleSrc`, but the technique can be
applied to any of the directives.

We'll start off by having an example stylesheet defined like this:

```css static/example.css
h1 {
  font-size: 25px;
  font-weight: normal;
  margin-top: 5px;
  margin-left: 25px;
}
```

## No CSP

To kick things off, we'll create the following control route which doesn't do
anything with CSP. We include a stylesheet to confirm that our sheet correctly
styles the response.

```tsx routes/noCSP.tsx
import { RouteContext } from "$fresh/server.ts";

export default function Home(req: Request, ctx: RouteContext) {
  return (
    <>
      <h1>This page doesn't use CSP at all. Styles will be applied.</h1>
      <link rel="stylesheet" type="text/css" href="example.css" />
    </>
  );
}
```

We can hit `http://localhost:8000/noCSP` and we should see the following:

```txt
This page doesn't use CSP at all. Styles will be applied.
```

## Incorrect CSP

Let's invoke the `useCSP` hook in our response to try to secure our page. Watch
closely, we're using the wrong URL! This will cause the browser to reject the
stylesheet, due to the header that Fresh produces. We get a `(blocked:csp)`
status when the browser tries to request this resource.

```tsx routes/incorrectCSP.tsx
import { RouteConfig, RouteContext } from "$fresh/server.ts";
import { useCSP } from "$fresh/runtime.ts";

export default function Home(req: Request, ctx: RouteContext) {
  useCSP((csp) => {
    if (!csp.directives.styleSrc) {
      csp.directives.styleSrc = [];
    }
    csp.directives.styleSrc.push("http://www.example.com");
  });
  return (
    <>
      <h1>This page violates our configured CSP. Styles won't be applied.</h1>
      <link rel="stylesheet" type="text/css" href="example.css" />
    </>
  );
}

export const config: RouteConfig = {
  csp: true,
};
```

We can hit `http://localhost:8000/incorrectCSP` and we should see the following:

```txt
This page violates our configured CSP. Styles won't be applied.
```

## Correct CSP

Let's fix our simple mistake and use the correct URL. Everything is working
correctly here.

```tsx routes/correctCSP.tsx
import { RouteConfig, RouteContext } from "$fresh/server.ts";
import { useCSP } from "$fresh/runtime.ts";

export default function Home(req: Request, ctx: RouteContext) {
  useCSP((csp) => {
    if (!csp.directives.styleSrc) {
      csp.directives.styleSrc = [];
    }
    csp.directives.styleSrc.push("http://localhost:8000/example.css");
  });
  return (
    <>
      <h1>This page adheres to our configured CSP. Styles will be applied.</h1>
      <link rel="stylesheet" type="text/css" href="example.css" />
    </>
  );
}

export const config: RouteConfig = {
  csp: true,
};
```

We can hit `http://localhost:8000/correctCSP` and we should see the following:

```txt
This page adheres to our configured CSP. Styles will be applied.
```

## No Route Config

What happens if we forget to use a `RouteConfig` in our route?

```tsx routes/cspNoRouteConfig.tsx
import { RouteContext } from "$fresh/server.ts";
import { useCSP } from "$fresh/runtime.ts";

export default function Home(req: Request, ctx: RouteContext) {
  useCSP((csp) => {
    if (!csp.directives.styleSrc) {
      csp.directives.styleSrc = [];
    }
    csp.directives.styleSrc.push("http://www.example.com");
  });
  return (
    <>
      <h1>
        This page violates our configured CSP. But we don't have a{" "}
        <code>RouteConfig</code>{" "}
        enabled, so Fresh doesn't know to use the CSP. Styles will be applied.
      </h1>
      <link rel="stylesheet" type="text/css" href="example.css" />
    </>
  );
}
```

We can hit `http://localhost:8000/cspNoRouteConfig` and we should see the
following:

```txt
This page violates our configured CSP. But we don't have a RouteConfig enabled, so Fresh doesn't know to use the CSP. Styles will be applied.
```

## Reporting

Let's touch on the reporting aspect of CSP. CSP (and Fresh's framework) support
a `reportOnly` flag and a `reportUri` endpoint. This is a destination that
should be able to receive `POST` requests. If the `reportOnly` flag is enabled,
then the browser will ignore the CSP headers and log any issues to the
`reportUri` destination.

```tsx routes/incorrectCSPwithReport.tsx
import { RouteConfig, RouteContext } from "$fresh/server.ts";
import { useCSP } from "$fresh/runtime.ts";

export default function Home(req: Request, ctx: RouteContext) {
  useCSP((csp) => {
    csp.reportOnly = true;
    if (!csp.directives.styleSrc) {
      csp.directives.styleSrc = [];
    }
    csp.directives.reportUri = "http://localhost:8000/reportHandler";
    csp.directives.styleSrc.push("http://www.example.com");
  });
  return (
    <>
      <h1>
        This page violates our configured CSP. But we're using "reportOnly".
        Styles will be applied.
      </h1>
      <link rel="stylesheet" type="text/css" href="example.css" />
    </>
  );
}

export const config: RouteConfig = {
  csp: true,
};
```

```ts routes/reportHandler.ts
import { FreshContext } from "$fresh/server.ts";

export const handler = {
  async POST(req: Request, _ctx: FreshContext) {
    const body = await req.json();
    const report = JSON.stringify(body, null, 2);

    await Deno.writeTextFile("./csp-reports.txt", report + "\n", {
      append: true,
    });
    return new Response(null, { status: 200 });
  },
};
```

We can hit `http://localhost:8000/incorrectCSPwithReport` and we should see the
following:

```txt
This page violates our configured CSP. But we're using "reportOnly". Styles will be applied.
```

We can then check our server and we'll see that `csp-reports.txt` has an entry
like this:

```json csp-reports.txt
{
  "csp-report": {
    "document-uri": "http://localhost:8000/incorrectCSPwithReport",
    "referrer": "http://localhost:8000/incorrectCSPwithReport",
    "violated-directive": "style-src-elem",
    "effective-directive": "style-src-elem",
    "original-policy": "default-src 'none'; style-src 'unsafe-inline' http://www.example.com; report-uri http://localhost:8000/reportHandler; script-src 'nonce-0f2d8259315d40479e8c21979128ac0d'; connect-src 'self'",
    "disposition": "report",
    "blocked-uri": "http://localhost:8000/example.css",
    "line-number": 37,
    "source-file": "http://localhost:8000/incorrectCSPwithReport",
    "status-code": 200,
    "script-sample": ""
  }
}
```



================================================
FILE: docs/latest/examples/using-fresh-canary-version.md
================================================
---
description: |
  For cases where the latest release doesn't fit your needs.
---

Pretend you have a use case where you need to modify your project to use a
canary version of Fresh. Or you want to use a slightly different initialization
script. This page has you covered.

## Canary Fresh in `deno.json`

### Specific commit

Let's say you like living life in the fast lane, and want a particular commit.
How can you modify your project to no longer use the current release, but
instead this one particular commit? Just make the following changes to your
`deno.json`:

```diff deno.json
     "update": "deno run -A -r https://fresh.deno.dev/update ."
   },
   "imports": {
-    "$fresh/": "https://deno.land/x/fresh@1.2.0/",
+    "$fresh/": "https://raw.githubusercontent.com/denoland/fresh/the-particular-commit-hash-here/",
     "preact": "https://esm.sh/preact@10.22.0",
     "preact/": "https://esm.sh/preact@10.22.0/",
```

### Forked Fresh

Or what if you have a PR created but it's not getting merged into `main`. Don't
worry, you can use the same approach to reference any branch in a fork as well.
Here's an example of referencing a feature in a forked repository that hasn't
been merged yet (at the time of writing this):

```diff deno.json
     "update": "deno run -A -r https://fresh.deno.dev/update ."
   },
   "imports": {
-    "$fresh/": "https://deno.land/x/fresh@1.2.0/",
+    "$fresh/": "https://raw.githubusercontent.com/deer/fresh/state_in_props/",
     "preact": "https://esm.sh/preact@10.22.0",
     "preact/": "https://esm.sh/preact@10.22.0/",
```

## Creating a new project

What if you're getting into open source development, and you've of course
decided to contribute to the best, freshest project around. Maybe you want to
create a test project based on your local changes.

### Creating a project from source

Instead of doing it like this:

```sh Terminal
deno run -A -r https://fresh.deno.dev/
```

do it like this:

```sh Terminal
deno run -A -r path/to/fresh/init.ts
```

(or wherever your local code lives)

### Creating a project from the latest commit

Of course there's no reason why you have to check out the Fresh source. You can
create a project from the latest commit by combining the techniques on this page
like this:

```sh Terminal
deno run -A -r https://raw.githubusercontent.com/denoland/fresh/main/init.ts
```



================================================
FILE: docs/latest/examples/using-twind-v1.md
================================================
---
description: |
  With a few tweaks one can use twind v1
---

When you initialize a project with `deno run -A -r https://fresh.deno.dev`,
you'll end up with a `main.ts` like the following:

```ts main.ts
/// <reference no-default-lib="true" />
/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
/// <reference lib="dom.asynciterable" />
/// <reference lib="deno.ns" />

import "$std/dotenv/load.ts";

import { start } from "$fresh/server.ts";
import manifest from "./fresh.gen.ts";
import config from "./fresh.config.ts";

await start(manifest, config);
```

And the Fresh config is like this:

```ts fresh.config.ts
import { defineConfig } from "$fresh/server.ts";
import twindPlugin from "$fresh/plugins/twind.ts";
import twindConfig from "./twind.config.ts";

export default defineConfig({
  plugins: [twindPlugin(twindConfig)],
});
```

Let's bump that up to v1:

```diff
diff --git a/fresh.config.ts b/fresh.config.ts
index 548e16a..e00d557 100644
--- a/fresh.config.ts
+++ b/fresh.config.ts
@@ -1,5 +1,5 @@
 import { defineConfig } from "$fresh/server.ts";
-import twindPlugin from "$fresh/plugins/twind.ts";
+import twindPlugin from "$fresh/plugins/twindv1.ts";
 import twindConfig from "./twind.config.ts";

 export default defineConfig({
```

The twind config object has changed significantly in v1, so we must also change
`twind.config.ts`. A good base looks like this (just replace whatever is there
with this):

```ts twind.config.ts
import { defineConfig, Preset } from "https://esm.sh/@twind/core@1.1.3";
import presetTailwind from "https://esm.sh/@twind/preset-tailwind@1.1.4";
import presetAutoprefix from "https://esm.sh/@twind/preset-autoprefix@1.0.7";

export default {
  ...defineConfig({
    presets: [presetTailwind() as Preset, presetAutoprefix()],
  }),
  selfURL: import.meta.url,
};
```

(Note: the `as Preset` cast is required to fix a typing issue with twind.)

To see what other presets exist, you can go to the
[twind docs](https://twind.style/presets).



================================================
FILE: docs/latest/examples/writing-tests.md
================================================
---
description: |
  You can write HTTP tests for your Fresh project by creating an application handler.
---

You can write tests for your Fresh project by creating an application handler
through
[`createHandler()`](https://deno.land/x/fresh/server.ts?doc=&s=createHandler).

## 1. Create your routes

```tsx routes/index.tsx
import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  async POST(req) {
    const form = await req.formData();

    // Processing something

    return new Response(null, {
      status: 303,
      headers: { location: "/" },
    });
  },
};

export default function HomePage() {
  return <div>Hello Deno!</div>;
}
```

```tsx routes/foo.tsx
export default function FooPage() {
  return <div>Hello Foo!</div>;
}
```

## 2. Write your tests

```ts tests/main_test.ts
import { createHandler, ServeHandlerInfo } from "$fresh/server.ts";
import manifest from "../fresh.gen.ts";
import config from "../fresh.config.ts";
import { assert, assertEquals } from "$std/testing/asserts.ts";

const CONN_INFO: ServeHandlerInfo = {
  remoteAddr: { hostname: "127.0.0.1", port: 53496, transport: "tcp" },
};

Deno.test("HTTP assert test.", async (t) => {
  const handler = await createHandler(manifest, config);

  await t.step("#1 GET /", async () => {
    const resp = await handler(new Request("http://127.0.0.1/"), CONN_INFO);
    assertEquals(resp.status, 200);
  });

  await t.step("#2 POST /", async () => {
    const formData = new FormData();
    formData.append("text", "Deno!");
    const req = new Request("http://127.0.0.1/", {
      method: "POST",
      body: formData,
    });
    const resp = await handler(req, CONN_INFO);
    assertEquals(resp.status, 303);
  });

  await t.step("#3 GET /foo", async () => {
    const resp = await handler(new Request("http://127.0.0.1/foo"), CONN_INFO);
    const text = await resp.text();
    assert(text.includes("<div>Hello Foo!</div>"));
  });
});
```

## 3. Run the tests

```sh Terminal
$ deno test --allow-read --allow-env --allow-net
running 1 test from ./tests/main_test.ts
HTTP assert test. ...
  #1 GET / ... ok (31ms)
  #2 POST / ... ok (35ms)
  #3 GET /foo ... ok (12ms)
HTTP assert test. ... ok (118ms)

ok | 1 passed (3 steps) | 0 failed (236ms)
```

## createHandler in detail

This function is typed as follows:

```ts
export async function createHandler(
  manifest: Manifest,
  config: FreshConfig = {},
): Promise<
  (req: Request, connInfo?: ServeHandlerInfo) => Promise<Response>
```

When you're using it, you'll likely be importing the manifest from your project.
You can of course import the config (`fresh.config.ts`) as well, but you're also
free to provide your own bag of options.
[`FreshConfig`](https://deno.land/x/fresh/server.ts?s=FreshConfig) is declared
as follows:

```ts
export interface FreshConfig {
  build?: {
    outDir?: string;
    target?: string | string[];
  };
  render?: RenderFunction;
  plugins?: Plugin[];
  staticDir?: string;
  router?: RouterOptions;
  server?: Partial<Deno.ServeTlsOptions>;
}
```

For more on how these work, see the page about
[server configuration](/docs/concepts/server-configuration).



================================================
FILE: docs/latest/getting-started/adding-interactivity.md
================================================
---
description: |
  Add JavaScript based interactivity to your project without sacrificing user
  experience, by using Fresh's powerful islands system.
---

Up to now none of the pages in the demo project have contained any client side
JavaScript. This is great for resiliency and performance, but it can also limit
the possibilities of interactivity. In many current generation web frameworks,
you get the choice of shipping no JavaScript to the client or shipping a
renderer for the entire page.

This is not very flexible, especially considering that most pages will only have
small pieces of content that require interactivity. For example, an otherwise
static page might need a little bit of JavaScript to power an image carousel or
"buy now" button. This model is often called
[islands architecture][islands-architecture]. This refers to a page having
little "islands" of interactivity, in a sea of otherwise static content.

Fresh embraces this model. All pages are rendered server side, but you can
create "island components" that are _also_ rendered client side. To do this,
Fresh projects have a special `islands/` folder. The modules in this folder each
encapsulate a single island component. The name of the module should be the
[pascal case][pascal-case] or [kebab case][kebab-case] name of the island
component. For example a counter component would be defined in the file
`islands/Counter.tsx`. A buy now button could be defined in the file
`islands/buy-now-button.tsx`.

Here is an example of an island component that counts down to a specific time.

```tsx islands/Countdown.tsx
import { useSignal } from "@preact/signals";
import { useEffect } from "preact/hooks";

const timeFmt = new Intl.RelativeTimeFormat("en-US");

// The target date is passed as a string instead of as a `Date`, because the
// props to island components need to be JSON (de)serializable.
export default function Countdown(props: { target: string }) {
  const target = new Date(props.target);
  const now = useSignal(new Date());

  // Set up an interval to update the `now` date every second with the current
  // date as long as the component is mounted.
  useEffect(() => {
    const timer = setInterval(() => {
      if (now.value > target) {
        clearInterval(timer);
      }
      now.value = new Date();
    }, 1000);
    return () => clearInterval(timer);
  }, [props.target]);

  const secondsLeft = Math.floor(
    (target.getTime() - now.value.getTime()) / 1000,
  );

  // If the target date has passed, we stop counting down.
  if (secondsLeft <= 0) {
    return <span>🎉</span>;
  }

  // Otherwise, we format the remaining time using `Intl.RelativeTimeFormat` and
  // render it.
  return <span>{timeFmt.format(secondsLeft, "seconds")}</span>;
}
```

To include this in a page component, one can just use the component normally.
Fresh will take care of automatically mounting the island component on the
client with the correct props:

```tsx routes/countdown.tsx
import Countdown from "../islands/Countdown.tsx";

export default function Page() {
  const date = new Date();
  date.setHours(date.getHours() + 1);
  return (
    <p>
      The big event is happening <Countdown target={date.toISOString()} />.
    </p>
  );
}
```

The page that is rendered on the client now has an interactive countdown.

[islands-architecture]: https://jasonformat.com/islands-architecture
[pascal-case]: https://en.wiktionary.org/wiki/Pascal_case
[kebab-case]: https://en.wiktionary.org/wiki/kebab_case



================================================
FILE: docs/latest/getting-started/create-a-project.md
================================================
---
description: |
  Create a new Fresh project by running the Fresh project creation tool. This
  scaffolds out the various files and folders a Fresh project needs.
---

New Fresh projects can be created by using the Fresh project creation tool. It
will scaffold out a new project with some example files to get you started.

To create a new project, run:

```sh Terminal
deno run -A -r https://fresh.deno.dev
cd fresh-project
deno task start
```

This will scaffold out the new project, then switch into the newly created
directory, and then start the development server.

This will create a directory containing some files and directories. There are 4
files that are strictly necessary to run a Fresh project:

- **`dev.ts`**: This is the development entry point for your project. This is
  the file that you run to start your project. This file doesn't need to be
  called `dev.ts`, but this is the convention.
- **`main.ts`**: This is the production entry point for your project. It is the
  file that you link to Deno Deploy. This file doesn't actually need to be
  `main.ts`, but this is the convention.
- **`fresh.gen.ts`**: This is the manifest file that contains information about
  your routes and islands. This file is automatically generated in development
  based on your `routes/` and `islands/` folders.

A **`deno.json`** file is also created in the project directory. This file does
two things:

- It defines the "imports" field. This is an [import map][import-map] that is
  used to manage dependencies for the project. This allows for easy importing
  and updating of dependencies.
- It registers a "start" [task][task-runner] to run the project without having
  to type a long `deno run` command.

Two important folders are also created that contain your routes and islands
respectively:

- **`routes/`**: This folder contains all of the routes in your project. The
  name of each file in this folder corresponds to the path where that page will
  be accessed. Code inside of this folder is never directly shipped to the
  client. You'll learn more about how routes work in the next section.
- **`islands/`**: This folder contains all of the interactive islands in your
  project. The name of each file corresponds to the name of the island defined
  in that file. Code inside of this folder can be run from both client and
  server. You'll learn more about islands later in this chapter.

Finally a **`static/`** folder is created that contains static files that are
automatically served "as is". [Learn more about static files][static-files].

[import-map]: https://docs.deno.com/runtime/fundamentals/modules
[task-runner]: https://docs.deno.com/runtime/reference/cli/task
[static-files]: ../concepts/static-files



================================================
FILE: docs/latest/getting-started/create-a-route.md
================================================
---
description: |
  Create a new route to a Fresh project by creating a new file in the `routes/`
  folder.
---

After getting the project running locally, the next step is to add a new route
to the project. Routes encapsulate the logic for handling requests to a
particular path in your project. They can be used to handle API requests or
render HTML pages. For now we are going to do the latter.

Routes are defined as files in the `routes` directory. The file name of the
module is important: it is used to determine the path that the route will
handle. For example, if the file name is `index.js`, the route will handle
requests to `/`. If the file name is `about.js`, the route will handle requests
to `/about`. If the file name is `contact.js` and is placed inside of the
`routes/about/` folder, the route will handle requests to `/about/contact`. This
concept is called _File-system routing_. You can learn more about it on the
[_Concepts: Routing_][concepts-routing] page.

Route files that render HTML are JavaScript or TypeScript modules that export a
JSX component as their default export. This component will be rendered for every
request to the route's path. The component receives a few properties that can be
used to customize the rendered output, such as the current route, the url of the
request, state set by middleware, and handler data (more on the last two later).

In the demo project we'll create a route to handle the `/about` page. To do
this, one needs to create a new `routes/about.tsx` file. In this file, we can
declare a component that should be rendered every time a user visits the page.
This is done with JSX.

> [info]: To learn more about JSX, you can read [this article][jsx] in the React
> documentation. Beware that Fresh does not use React, but rather
> [Preact][preact], a lighter weight virtual dom library that works similar to
> React.

```tsx routes/about.tsx
export default function AboutPage() {
  return (
    <main>
      <h1>About</h1>
      <p>This is the about page.</p>
    </main>
  );
}
```

The new page will be visible at `http://localhost:8000/about`.

<!-- You can find more in depth information about routes on the
[_Concepts: Routes_][concepts-routes] documentation page. The following
pages in the _Getting Started_ guide will also explain more features of routes. -->

[concepts-routing]: /docs/concepts/routing
[jsx]: https://react.dev/learn/writing-markup-with-jsx
[preact]: https://preactjs.com/

<!-- [concepts-routes]: /docs/concepts/routes -->



================================================
FILE: docs/latest/getting-started/custom-handlers.md
================================================
---
description: |
  Add custom handlers to a route to customize HTTP headers, implement API
  routes, do data fetching for a rendered page, or handle form submissions.
---

Routes actually consist of two parts: handlers, and the page component. Up to
now, only the page component has been discussed in this chapter.

Handlers are functions in the form of `Request => Response` or
`Request => Promise<Response>` that are called when a request is made to a
particular route. There can be one handler that covers all HTTP methods or one
handler per method.

The handler has access to the `Request` object that backs the request to the
route and must return a `Response` object. The response object can either be
created manually (for example a JSON response for an API route), or it can be
created by rendering the page component. By default, all routes that don't
define a custom handler use a default handler that just renders the page
component.

To define a handler in a route module, one must export it as a named export with
the name `handler`. Handlers can have two forms: a plain function (catchall for
all HTTP methods) or a plain object where each property is a function named by
the HTTP method it handles.

Here is an example of a custom `GET` handler that renders the page component and
then adds a custom header to the response before returning it:

```tsx routes/about.tsx
import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  async GET(_req, ctx) {
    const resp = await ctx.render();
    resp.headers.set("X-Custom-Header", "Hello");
    return resp;
  },
};

export default function AboutPage() {
  return (
    <main>
      <h1>About</h1>
      <p>This is the about page.</p>
    </main>
  );
}
```

Note that handlers do not need to call `ctx.render()`. This feature can be used
to create API routes. Here is an API route that returns a random UUID as a JSON
response:

```ts routes/api/random-uuid.ts
import { Handlers } from "$fresh/server.ts";

export const handler: Handlers = {
  GET(_req) {
    const uuid = crypto.randomUUID();
    return new Response(JSON.stringify(uuid), {
      headers: { "Content-Type": "application/json" },
    });
  },
};
```

Handlers can do much more, including fetching data from a database or external
API and passing it to their route.



================================================
FILE: docs/latest/getting-started/deploy-to-production.md
================================================
---
description: |
  Deploy a Fresh application to Deno Deploy in seconds, making it available on
  the edge globally - resulting in fantastic user latency worldwide.
---

As a final step in the getting started guide, we'll deploy the demo site to the
public internet using [Deno Deploy][deno-deploy]. Deno Deploy is a globally
distributed edge runtime built by the Deno company that allows developers to
quickly and painlessly deploy web applications to the internet. Deno Deploy has
edge nodes all over the world that serve traffic. Because of this, users
worldwide have fantastic latency because their traffic is served from a server
that is physically close to them.

To deploy to Deno Deploy, we'll make use of the GitHub integration. To use this
the code needs to be pushed to a repository on GitHub. Once this has been done,
one must go to the [Deno Deploy dashboard][deno-deploy-dashboard] and create a
new project.

Click on the "New Project" button and select the GitHub repository that contains
the Fresh project. Select the "Fresh" framework preset, and click on "Advanced
options". Enter `deno task build` in the "Build command" field. Press "Create
project".

The project will now deploy to Deno Deploy. After this is done, the project will
be available at https://$PROJECT_NAME.deno.dev.

Every time the code in the GitHub repository is updated, it will be deployed
either as a preview or production deployment. Production deployments are only
created for changes to the default/production branch (often `main`).

[deno-deploy]: https://deno.com/deploy
[deno-deploy-dashboard]: https://dash.deno.com/projects



================================================
FILE: docs/latest/getting-started/dynamic-routes.md
================================================
---
description: |
  Create a dynamic route in Fresh by adding a dynamic segment to the route name
  in the routes' file name on disk: `/greet/[name].tsx`.
---

The `/about` route created on the last page is pretty static. It does not matter
what query or path parameters are passed to the route, it will always render the
same page. Let's create a `/greet/:name` that will render a page with a greeting
that contains the name passed in the path.

Before diving in, a quick refresher on "dynamic" routes. Dynamic routes don't
just match a single static path, but rather a whole bunch of different paths
based on a pattern. For example, the `/greet/:name` route will match the paths
`/greet/Luca` and `/greet/John`, but not `/greet/Luca/John`.

Fresh supports dynamic routes out of the box through file system routing. To
make any path segment dynamic, just put square brackets around that segment in
the file name. For example the `/greet/:name` route maps to the file name
`routes/greet/[name].tsx`.

Just like the static `/about` route, the dynamic `/greet/:name` route will
render a page. The module must once again expose a component as a default
export. This time the component will receive the matched path segment properties
as arguments in its `props` object though.

```tsx routes/greet/[name].tsx
import { PageProps } from "$fresh/server.ts";

export default function GreetPage(props: PageProps) {
  const { name } = props.params;
  return (
    <main>
      <p>Greetings to you, {name}!</p>
    </main>
  );
}
```

The `PageProps` interface actually contains a bunch of useful properties that
can be used to customize the rendered output. Next to the matched url pattern
parameters, the raw `url`, and the `route` name can also be found in here.

Navigating to `http://localhost:8000/greet/Luca` will now render a page showing
"Greetings to you, Luca!".

The [_Concepts: Routing_][concepts-routing] page has more information about
dynamic routes, especially about how to create more advanced dynamic routes.

[concepts-routing]: /docs/concepts/routing



================================================
FILE: docs/latest/getting-started/form-submissions.md
================================================
---
description: |
  Robustly handle user inputs using HTML `<form>` elements client side, and form
  submission handlers server side.
---

Forms are a common mechanism for letting users interact with applications. In
the last few years it has become more and more common for web applications to
move form submission entirely to the client. This can have useful properties for
interactivity, but it is much worse for resiliency and user experience as a
whole. Browsers have great built in systems for form submission, revolving
around the HTML `<form>` element.

Fresh builds the core of its form submission infrastructure around the native
`<form>` element. This page explains how to use `<form>` in Fresh, and the next
chapter explains how to progressively enhance your forms with client side
JavaScript to make them more interactive.

The way forms work in the browser, is that they perform an HTML navigation
action when the user submits the form. In most cases this means that when the
form is submitted, a `GET` or `POST` request is sent to the server with the form
data, which then responds with a new page to render.

Fresh can handle both `GET` and `POST` requests through the
[custom handlers][custom-handlers] feature of routes. The handlers can perform
any necessary processing on the form data, and then pass data to the
`ctx.render()` call to render a new page.

Here is an example implementing a search form that filters an array of names
server side:

```tsx routes/search.tsx
import { Handlers, PageProps } from "$fresh/server.ts";

const NAMES = ["Alice", "Bob", "Charlie", "Dave", "Eve", "Frank"];

interface Data {
  results: string[];
  query: string;
}

export const handler: Handlers<Data> = {
  GET(req, ctx) {
    const url = new URL(req.url);
    const query = url.searchParams.get("q") || "";
    const results = NAMES.filter((name) => name.includes(query));
    return ctx.render({ results, query });
  },
};

export default function Page({ data }: PageProps<Data>) {
  const { results, query } = data;
  return (
    <div>
      <form>
        <input type="text" name="q" value={query} class="border p-1" />
        <button type="submit" class="ml-1 px-2 py-1 bg-gray-100 border">
          Search
        </button>
      </form>
      <ul>
        {results.map((name) => <li key={name}>{name}</li>)}
      </ul>
    </div>
  );
}
```

When the user submits the form, the browser will navigate to `/search` with the
query set as the `q` query parameter in the URL. The `GET` handler will then
filter the names array based on the query, and pass it to the page component for
rendering.

[Learn more about using forms in Fresh][concepts-forms].

<!-- TODO(lucacasonato): link to todo app example when that is built again -->

[custom-handlers]: /docs/getting-started/custom-handlers
[concepts-forms]: /docs/concepts/forms



================================================
FILE: docs/latest/getting-started/index.md
================================================
---
description: |
  In this chapter of the Fresh documentation, you'll be introduced to the
  framework. Create a new project, run it locally, edit and create pages, fetch
  data, handle user interactions, and deploy it.
---

In this chapter of the Fresh documentation, you'll be introduced to the
framework. You'll learn how to create a new project, run it locally, edit and
create pages, fetch data, handle user interactions, and how to then deploy the
project to [Deno Deploy](https://deno.com/deploy).

The documentation assumes you have the latest version
[Deno](https://docs.deno.com/runtime/#install-deno) installed on your system and
set up your
[Editor to work with Deno](https://docs.deno.com/runtime/getting_started/setup_your_environment/).



================================================
FILE: docs/latest/getting-started/running-locally.md
================================================
---
description: |
  To start a Fresh project, just run `deno task start`. This will start the
  project with default permission flags, in watch mode.
---

The next step after scaffolding out a new project, is to actually start it. To
do this you can just `deno task start`. Environment variables will be
automatically read from `.env`.

```sh Terminal
$ deno task start
Watcher Process started.
 🍋 Fresh ready
     Local: http://localhost:8000
```

If you want to start manually without Deno task, `deno run` the `main.ts` with
the appropriate flags. You will need to provide permission flags for:

- **`--allow-net`**: This is required to start the HTTP server.
- **`--allow-read`**: This is required to read (static) files from disk.
- **`--allow-env`**: This is required to read environment variables that can be
  used to configure your project.
- **`--allow-run`**: This is required to shell out to `deno` and `esbuild` under
  the hood during development to do type stripping. In production this is done
  using a WebAssembly binary.

For development, you also want to run with the [`--watch` flag][--watch], so the
Fresh server will automatically reload whenever you make a change to your code.
By default `--watch` only watches over files in your module graph. Some project
files like static files are not part of the module graph, but you probably want
to restart/reload whenever you make a change to them too. This can be done by
passing the extra folder as an argument: `--watch=static/`. You should also add
`routes/` to the watch list, so that the server restarts automatically whenever
you add a new route.

If you want to change the port or host, modify the config bag of the `start()`
call in `main.ts` to include an explicit port number:

```js main.ts
await start(manifest, { server: { port: 3000 } });
```

You can also change the port by setting the `PORT` environment variable:

```sh Terminal
$ PORT=3000 deno task start
```

Combining all of this we get the following `deno run` command:

```sh Terminal
$ deno run --allow-net --allow-read --allow-env --allow-run --watch=static/,routes/ main.ts
Watcher Process started.
 🍋 Fresh ready
     Local: http://localhost:8000
```

If you now visit http://localhost:8000, you can see the running project. Try
change some of the text in `routes/index.tsx` and see how the page updates
automatically when you save the file.

[--watch]: https://docs.deno.com/runtime/getting_started/command_line_interface/#watch-mode



================================================
FILE: docs/latest/integrations/index.md
================================================
---
description: |
  Various projects can integrate into Fresh to easily add functionality to your
  project.
---

This section of the docs showcases modules or other projects that integrate with
Fresh. These integrations can be used to add functionality to your project.

## `fresh_charts`

Fresh charts allows for easy integration of [Chart.js][chart-js] charts into
your Fresh project. It provides a `Chart` JSX component that can be used to
render charts on the server and client.

A live demo can be found here: https://fresh-charts.deno.dev/

Documentation for the module can be found here: https://deno.land/x/fresh_charts

[chart-js]: https://www.chartjs.org/ "Chart.js"

## `fresh_marionette`

Fresh Marionette allows you to start writing end 2 end browser tests in your
Fresh projects with a single import. Then you can run those browser tests in a
GitHub Actions workflow.

Documentation for the module can be found here:
https://deno.land/x/fresh_marionette

An example project that runs the tests in a GitHub Actions workflow:
https://marionette.deno.dev

Fresh Marionette works with VSCode too! - https://youtu.be/OG77NdqL164



================================================
FILE: docs/latest/introduction/index.md
================================================
---
description: |
  Fresh is a full stack modern web framework for JavaScript and TypeScript
  developers, designed to build high-quality, performant,
  and personalized web applications.
---

Fresh is a full stack modern web framework for JavaScript and TypeScript
developers. It's designed for building high-quality, performant, and
personalized web applications. You can use it to create your home page, a blog,
an e-commerce shop, a large web application like GitHub or Twitter and more.

At its core, Fresh is a combination of a routing framework and templating engine
that renders pages on demand on the server. These server rendered pages can
contain areas that are made interactive on the client (also known as the
[Island Architecture](https://jasonformat.com/islands-architecture)). Fresh uses
[Preact][preact] as the JSX rendering engine.

Fresh projects can be deployed manually to any platform with [Deno][deno], but
it is intended to be deployed to an edge runtime like [Deno Deploy][deno-deploy]
for the best experience.

Some stand out features:

- Zero config necessary
- Tiny & fast (no client JS is required by the framework)
- Optional client side hydration of individual components
- Highly resilient because of progressive enhancement and use of native browser
  features
- TypeScript out of the box
- File-system routing à la Next.js

[preact]: https://preactjs.com
[deno]: https://deno.com
[deno-deploy]: https://deno.com/deploy



================================================
FILE: examples/README.md
================================================
# Examples for Fresh

This package contains examples for using Fresh with [JSR](https://jsr.io/).

Learn more about the Fresh framework here:
[https://fresh.deno.dev/](https://fresh.deno.dev/)

## Usage: Island example

```tsx
import { App } from "fresh";
// Import the island function
import { DemoIsland } from "jsr:@fresh/examples/island";

export const app = new App({ root: import.meta.url })
  .use(staticFiles());

// Register the island
app.island(
  // Module specifier for esbuild, could also be a file path
  "jsr:@fresh/examples/island",
  // Name of the island
  "DemoIsland",
  // Island component function
  DemoIsland,
);

// Use the island somewhere in your components
app.get("/", (ctx) => ctx.render(<DemoIsland />));

await app.listen();
```

## Usage: App1 or App2 example

```tsx
import { App } from "fresh";
// Import the example apps
import { app1 } from "jsr:@fresh/examples/app1";
import { app2 } from "jsr:@fresh/examples/app2";

export const app = new App({ root: import.meta.url })
  .use(staticFiles());

// Merge apps from JSR into this one
app.mountApp("/app1", app1);
app.mountApp("/app2", app1);

await app.listen();
```

## License

MIT, see the [LICENSE](./LICENSE) file.



================================================
FILE: examples/deno.json
================================================
{
  "name": "@fresh/examples",
  "version": "1.0.1",
  "license": "MIT",
  "exports": {
    "./island": "./src/island.tsx",
    "./app1": "./src/app1.tsx",
    "./app2": "./src/app2.tsx"
  }
}



================================================
FILE: examples/LICENSE
================================================
MIT License

Copyright (c) 2021-2024 the Deno authors

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



================================================
FILE: examples/src/app1.tsx
================================================
import { App } from "fresh";
import { Doc } from "./shared.tsx";

export const app1: App<unknown> = new App()
  .get("/", (ctx) =>
    ctx.render(
      <Doc>
        <h1>App1</h1>
        <p>This app is loaded from JSR</p>
      </Doc>,
    ));



================================================
FILE: examples/src/app2.tsx
================================================
import { App } from "fresh";
import { Doc } from "./shared.tsx";

export const app2: App<unknown> = new App()
  .get("/", (ctx) =>
    ctx.render(
      <Doc>
        <h1>App2</h1>
        <p>This app is loaded from JSR</p>
      </Doc>,
    ));



================================================
FILE: examples/src/island.tsx
================================================
import { useSignal } from "@preact/signals";
import type { JSX } from "preact";

export function DemoIsland(): JSX.Element {
  const count = useSignal(0);

  return (
    <div style="display: flex; gap: 1rem; padding: 2rem;">
      <button type="button" onClick={() => (count.value -= 1)}>-1</button>
      <p style="font-variant-numeric: tabular-nums;">{count}</p>
      <button type="button" onClick={() => (count.value += 1)}>+1</button>
    </div>
  );
}



================================================
FILE: examples/src/shared.tsx
================================================
import type { ComponentChildren } from "preact";

export function Doc(props: { children?: ComponentChildren }) {
  return (
    <html>
      <head>
        <meta charset="utf-8" />
        <title>Fresh Examples</title>
      </head>
      <body>{props.children}</body>
    </html>
  );
}



================================================
FILE: init/README.md
================================================
# Create a new Fresh project.

This is a CLI tool to bootstrap a new Fresh project. To do so, run this command:

```sh
deno run -Ar jsr:@fresh/init
```

Go to [https://fresh.deno.dev/](https://fresh.deno.dev/) for more information
about Fresh.



================================================
FILE: init/deno.json
================================================
{
  "name": "@fresh/init",
  "version": "2.0.0-alpha.34",
  "license": "MIT",
  "exports": "./src/mod.ts",
  "exclude": ["**/tmp/*"],
  "publish": {
    "include": [
      "src/**/*.ts",
      "deno.json",
      "README.md"
    ],
    "exclude": ["**/*_test.*", "*.todo"]
  }
}



================================================
FILE: init/src/init.ts
================================================
// deno-lint-ignore-file no-console
import * as colors from "@std/fmt/colors";
import * as path from "@std/path";

// Keep these as is, as we replace these version in our release script
const FRESH_VERSION = "2.0.0-alpha.34";
const FRESH_TAILWIND_VERSION = "0.0.1-alpha.7";
const PREACT_VERSION = "10.26.6";
const PREACT_SIGNALS_VERSION = "2.0.4";

function css(strs: TemplateStringsArray, ...exprs: string[]): string {
  let out = "";

  for (let i = 0; i < exprs.length; i++) {
    out += strs[i];
    out += String(exprs[i]);
  }
  out += strs.at(-1) ?? "";

  return out;
}

export class InitError extends Error {}

function error(message: string): never {
  console.error(`%cerror%c: ${message}`, "color: red; font-weight: bold", "");
  throw new InitError();
}

export const HELP_TEXT = `@fresh/init

Initialize a new Fresh project. This will create all the necessary files for a
new project.

To generate a project in the './foobar' subdirectory:
  deno run -Ar jsr:@fresh/init ./foobar

To generate a project in the current directory:
  deno run -Ar jsr:@fresh/init .

USAGE:
    deno run -Ar jsr:@fresh/init [DIRECTORY]

OPTIONS:
    --force      Overwrite existing files
    --tailwind   Use Tailwind for styling
    --vscode     Setup project for VS Code
    --docker     Setup Project to use Docker
`;

export const CONFIRM_EMPTY_MESSAGE =
  "The target directory is not empty (files could get overwritten). Do you want to continue anyway?";
export const CONFIRM_TAILWIND_MESSAGE = `Set up ${
  colors.cyan("Tailwind CSS")
} for styling?`;
export const CONFIRM_VSCODE_MESSAGE = `Do you use ${colors.cyan("VS Code")}?`;

export async function initProject(
  cwd = Deno.cwd(),
  input: (string | number)[],
  flags: {
    docker?: boolean | null;
    force?: boolean | null;
    tailwind?: boolean | null;
    vscode?: boolean | null;
  } = {},
): Promise<void> {
  console.log();
  console.log(
    colors.bgRgb8(
      colors.rgb8(" 🍋 Fresh: The next-gen web framework. ", 0),
      121,
    ),
  );
  console.log();

  let unresolvedDirectory = Deno.args[0];
  if (input.length !== 1) {
    const userInput = prompt(
      "Project Name:",
      "fresh-project",
    );
    if (!userInput) {
      error(HELP_TEXT);
    }

    unresolvedDirectory = userInput;
  }

  const projectDir = path.resolve(cwd, unresolvedDirectory);

  try {
    const dir = [...Deno.readDirSync(projectDir)];
    const isEmpty = dir.length === 0 ||
      dir.length === 1 && dir[0].name === ".git";
    if (
      !isEmpty &&
      !(flags.force === null ? confirm(CONFIRM_EMPTY_MESSAGE) : flags.force)
    ) {
      error("Directory is not empty.");
    }
  } catch (err) {
    if (!(err instanceof Deno.errors.NotFound)) {
      throw err;
    }
  }

  const useDocker = flags.docker;
  let useTailwind = flags.tailwind || false;
  if (flags.tailwind == null) {
    if (
      confirm(CONFIRM_TAILWIND_MESSAGE)
    ) {
      useTailwind = true;
    }
  }

  const useVSCode = flags.vscode == null
    ? confirm(CONFIRM_VSCODE_MESSAGE)
    : flags.vscode;

  const writeFile = async (
    pathname: string,
    content:
      | string
      | Uint8Array
      | ReadableStream<Uint8Array>
      | Record<string, unknown>,
  ) => await writeProjectFile(projectDir, pathname, content);

  const GITIGNORE = `# dotenv environment variable files
.env
.env.development.local
.env.test.local
.env.production.local
.env.local

# Fresh build directory
_fresh/
# npm + other dependencies
node_modules/
vendor/
`;

  await writeFile(".gitignore", GITIGNORE);

  if (useDocker) {
    const DENO_VERSION = Deno.version.deno;
    const DOCKERFILE_TEXT = `
FROM denoland/deno:${DENO_VERSION}

ARG GIT_REVISION
ENV DENO_DEPLOYMENT_ID=\${GIT_REVISION}

WORKDIR /app

COPY . .
RUN deno cache main.ts

EXPOSE 8000

CMD ["run", "-A", "main.ts"]

`;
    await writeFile("Dockerfile", DOCKERFILE_TEXT);
  }

  const TAILWIND_CONFIG_TS = `import type { Config } from "tailwindcss";

export default {
  content: [
    "{routes,islands,components}/**/*.{ts,tsx}",
  ],
} satisfies Config;`;
  if (useTailwind) {
    await writeFile("tailwind.config.ts", TAILWIND_CONFIG_TS);
  }

  // deno-fmt-ignore
  const GRADIENT_CSS = css`.fresh-gradient {
  background-color: rgb(134, 239, 172);
  background-image: linear-gradient(
    to right bottom,
    rgb(219, 234, 254),
    rgb(187, 247, 208),
    rgb(254, 249, 195)
  );
}`;
  // deno-fmt-ignore
  const NO_TAILWIND_STYLES = css`*,
*::before,
*::after {
  box-sizing: border-box;
}
* {
  margin: 0;
}
button {
  color: inherit;
}
button, [role="button"] {
  cursor: pointer;
}
code {
  font-family:
    ui-monospace,
    SFMono-Regular,
    Menlo,
    Monaco,
    Consolas,
    "Liberation Mono",
    "Courier New",
    monospace;
  font-size: 1em;
}
img,
svg {
  display: block;
}
img,
video {
  max-width: 100%;
  height: auto;
}

html {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  font-family:
    ui-sans-serif,
    system-ui,
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    Roboto,
    "Helvetica Neue",
    Arial,
    "Noto Sans",
    sans-serif,
    "Apple Color Emoji",
    "Segoe UI Emoji",
    "Segoe UI Symbol",
    "Noto Color Emoji";
}
.transition-colors {
  transition-property: background-color, border-color, color, fill, stroke;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}
.my-6 {
  margin-bottom: 1.5rem;
  margin-top: 1.5rem;
}
.text-4xl {
  font-size: 2.25rem;
  line-height: 2.5rem;
}
.mx-2 {
  margin-left: 0.5rem;
  margin-right: 0.5rem;
}
.my-4 {
  margin-bottom: 1rem;
  margin-top: 1rem;
}
.mx-auto {
  margin-left: auto;
  margin-right: auto;
}
.px-4 {
  padding-left: 1rem;
  padding-right: 1rem;
}
.py-8 {
  padding-bottom: 2rem;
  padding-top: 2rem;
}
.bg-\\[\\#86efac\\] {
  background-color: #86efac;
}
.text-3xl {
  font-size: 1.875rem;
  line-height: 2.25rem;
}
.py-6 {
  padding-bottom: 1.5rem;
  padding-top: 1.5rem;
}
.px-2 {
  padding-left: 0.5rem;
  padding-right: 0.5rem;
}
.py-1 {
  padding-bottom: 0.25rem;
  padding-top: 0.25rem;
}
.border-gray-500 {
  border-color: #6b7280;
}
.bg-white {
  background-color: #fff;
}
.flex {
  display: flex;
}
.gap-8 {
  grid-gap: 2rem;
  gap: 2rem;
}
.font-bold {
  font-weight: 700;
}
.max-w-screen-md {
  max-width: 768px;
}
.flex-col {
  flex-direction: column;
}
.items-center {
  align-items: center;
}
.justify-center {
  justify-content: center;
}
.border-2 {
  border-width: 2px;
}
.rounded {
  border-radius: 0.25rem;
}
.hover\\:bg-gray-200:hover {
  background-color: #e5e7eb;
}
.tabular-nums {
  font-variant-numeric: tabular-nums;
}

${GRADIENT_CSS}`;

  // deno-fmt-ignore
  const TAILWIND_CSS = css`@tailwind base;
@tailwind components;
@tailwind utilities;
${GRADIENT_CSS}`;

  const cssStyles = useTailwind ? TAILWIND_CSS : NO_TAILWIND_STYLES;
  await writeFile("static/styles.css", cssStyles);

  const STATIC_LOGO =
    `<svg width="40" height="40" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M34.092 8.845C38.929 20.652 34.092 27 30 30.5c1 3.5-2.986 4.222-4.5 2.5-4.457 1.537-13.512 1.487-20-5C2 24.5 4.73 16.714 14 11.5c8-4.5 16-7 20.092-2.655Z" fill="#FFDB1E"/>
  <path d="M14 11.5c6.848-4.497 15.025-6.38 18.368-3.47C37.5 12.5 21.5 22.612 15.5 25c-6.5 2.587-3 8.5-6.5 8.5-3 0-2.5-4-5.183-7.75C2.232 23.535 6.16 16.648 14 11.5Z" fill="#fff" stroke="#FFDB1E"/>
  <path d="M28.535 8.772c4.645 1.25-.365 5.695-4.303 8.536-3.732 2.692-6.606 4.21-7.923 4.83-.366.173-1.617-2.252-1.617-1 0 .417-.7 2.238-.934 2.326-1.365.512-4.223 1.29-5.835 1.29-3.491 0-1.923-4.754 3.014-9.122.892-.789 1.478-.645 2.283-.645-.537-.773-.534-.917.403-1.546C17.79 10.64 23 8.77 25.212 8.42c.366.014.82.35.82.629.41-.14 2.095-.388 2.503-.278Z" fill="#FFE600"/>
  <path d="M14.297 16.49c.985-.747 1.644-1.01 2.099-2.526.566.121.841-.08 1.29-.701.324.466 1.657.608 2.453.701-.715.451-1.057.852-1.452 2.106-1.464-.611-3.167-.302-4.39.42Z" fill="#fff"/>
</svg>`;
  await writeFile("static/logo.svg", STATIC_LOGO);

  try {
    const res = await fetch("https://fresh.deno.dev/favicon.ico");
    const buf = await res.arrayBuffer();
    await writeFile("static/favicon.ico", new Uint8Array(buf));
  } catch {
    // Skip this and be silent if there is a network issue.
  }

  const MAIN_TS = `import { App, fsRoutes, staticFiles } from "fresh";
import { define, type State } from "./utils.ts";

export const app = new App<State>();

app.use(staticFiles());

// this is the same as the /api/:name route defined via a file. feel free to delete this!
app.get("/api2/:name", (ctx) => {
  const name = ctx.params.name;
  return new Response(
    \`Hello, \${name.charAt(0).toUpperCase() + name.slice(1)}!\`,
  );
});

// this can also be defined via a file. feel free to delete this!
const exampleLoggerMiddleware = define.middleware((ctx) => {
  console.log(\`\${ctx.req.method} \${ctx.req.url}\`);
  return ctx.next();
});
app.use(exampleLoggerMiddleware);

await fsRoutes(app, {
  loadIsland: (path) => import(\`./islands/\${path}\`),
  loadRoute: (path) => import(\`./routes/\${path}\`),
});

if (import.meta.main) {
  await app.listen();
}`;
  await writeFile("main.ts", MAIN_TS);

  const COMPONENTS_BUTTON_TSX =
    `import type { ComponentChildren } from "preact";

export interface ButtonProps {
  onClick?: () => void;
  children?: ComponentChildren;
  disabled?: boolean;
}

export function Button(props: ButtonProps) {
  return (
    <button
      {...props}
      class="px-2 py-1 border-gray-500 border-2 rounded bg-white hover:bg-gray-200 transition-colors"
    />
  );
}`;
  await writeFile("components/Button.tsx", COMPONENTS_BUTTON_TSX);

  const UTILS_TS = `import { createDefine } from "fresh";

// deno-lint-ignore no-empty-interface
export interface State {}

export const define = createDefine<State>();`;
  await writeFile("utils.ts", UTILS_TS);

  const ROUTES_HOME = `import { useSignal } from "@preact/signals";
import { define } from "../utils.ts";
import Counter from "../islands/Counter.tsx";

export default define.page(function Home() {
  const count = useSignal(3);

  return (
    <div class="px-4 py-8 mx-auto fresh-gradient">
      <div class="max-w-screen-md mx-auto flex flex-col items-center justify-center">
        <img
          class="my-6"
          src="/logo.svg"
          width="128"
          height="128"
          alt="the Fresh logo: a sliced lemon dripping with juice"
        />
        <h1 class="text-4xl font-bold">Welcome to Fresh</h1>
        <p class="my-4">
          Try updating this message in the
          <code class="mx-2">./routes/index.tsx</code> file, and refresh.
        </p>
        <Counter count={count} />
      </div>
    </div>
  );
});`;
  await writeFile("routes/index.tsx", ROUTES_HOME);

  const APP_WRAPPER = `import type { PageProps } from "fresh";

export