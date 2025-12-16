# Calcite

An intentionally basic static site generator written in rust 🦀 with the help of nix ❄️

## Why should I use calcite?

Calcite aims to be a middle ground between hacking away in html, as described in the [html hobbyist](https://www.htmlhobbyist.com/), and wrangling a more complex framework like [hugo](https://gohugo.io/) or [wordpress](https://wordpress.com/). The goal is to give users complete control of their theming while keeping their content separate.

## Usage

### Site Setup

Create two directories for your site, `content` and `template`.

The content directory _must_ include at least one markdown, `.md`, file. The contents and structure of this directory will be copied to the `out/` directory when built.

Template _must_ include `template.html`, this file must contain a `<div id="content">` and closing div tag. This tag is used to place the contents of the markdown file inside. A simple example of this file is as follows:

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <title>Title</title>
  <meta charset="utf-8" />
  <link rel="stylesheet" href="css/general.css" />
  <link rel="shortcut icon" type="image/x-icon" href="media/favicon.ico">
</head>

<body>
  <div>
    <header>
      <h1>Header</h1>
      <!-- navbar -->
    </header>

    <div id="content">
    <!-- markdown content will be turned into html and placed here -->
    </div>

    <footer>
      <p>generic copyright</p>
    </footer>
  </div>
</body>

</html>
```

## Tool Usage

### Nix

Run `nix run github:cartwatson/calcite/v0.0.0` in the directory you setup above
Remove `v0.0.0` to run with the latest changes

Obligatory [Security Warning](https://determinate.systems/posts/nix-run/#security-warning), the best way to avoid said security issue is to [pin to a version](https://determinate.systems/posts/nix-run/#using-git-revisions-as-a-versioning-mechanism)

### Deploy to Github Pages

1. Create a github pages repo: [github guide](https://docs.github.com/en/pages/quickstart)
2. Create a file in your repo `.github/workflows/deploy.yml`
3. Add the below code block to the `.github/workflows/deploy.yml` file
   - make sure to add a version to `run: nix run github:cartwatson/calcite` if desired
5. Github will now automatically update your site every time you push more content or change your theme

```yaml
name: "Build + Deploy with Calcite"

on:
  push:
    branches:
      - gh-pages

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: false

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: checkout
        uses: actions/checkout@v4

      - name: install nix
        uses: cachix/install-nix-action@v31

      - name: setup pages
        id: pages
        uses: actions/configure-pages@v5

      - name: Build with calcite
        run: nix run github:cartwatson/calcite

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./out

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Other methods

All other methods of install are currently unsupported. It's in the roadmap to generate binaries for Linux and MacOS for future releases.

## Development

Using [Nix](https://nixos.org/download/) is the only supported development process.

- use `nix develop` to get all depencies for your editor
- use `nix build` to build and link to your `/nix/store`, symlinked to `result` in the current dir
  - `./result/bin/calcite` in the same directory where you have your `template/` and `content/` directories
- use `nix-shell -p httplz --command "httplz out/"` to serve the site locally (or use your favorite http server)

Create a `content/test.md` file. Ideally it includes a variety of md components, including nested components (ie code blocks inside blockquotes). Provided example below

````markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

<p>plain paragraph</p>

> blockquote
> blockquote with `inline code`
> blockquote with *bold*

```
code block
code block
code block
```

- bullet list
  - bullet list
- bullet list

1. ordered list
  2. ordered list
3. ordered list

[link](https://google.com)
regular ***bold + italic* just bold** regular
regular **bold** regular
regular *italic* regular
regular paragraph
regular paragraph

regular paragraph

````

Create a `template/template.html` file as described [here](#site-setup)

## Goals

<details>
<summary>future features</summary>
<br>
<ul>
  <li>[ ] way to pull a theme from github, ie <code>nix run --theme=github:cartwatson/calcite-theme-gruvbox/v1.0.0</code>; aim is to make deployment even more simple</li>
  <li>[ ] annotation citations</li>
  <li>[ ] syntax highlighting in codeblocks</li>
  <li>[ ] allow template files in sub directories; eg <code>blog/template.html</code></li>
  <li>[ ] easy copy from codeblocks</li>
  <li>[ ] debug mode/better logging</li>
  <li>[ ] add ability to link to headings, needs design work for how to copy links</li>
  <li>[ ] more markdown</li>
  <ul>
    <li>[ ] unordered lists</li>
    <li>[ ] ordered lists</li>
  </ul>
</ul>
</details>

<details open>
<summary>v1.0.0</summary>
<br>
<ul>
  <li>[X] available on nixpkgs - counting the ability to <code>nix run github:cartwatson/calcite</code> as enough...</li>
  <li>[ ] clean up code architecture, no more hacks</li>
  <li>[ ] if unsuccessful at anypoint, remove any artifacts created</li>
  <li>[ ] prompt remove output dir if it exists</li>
  <li>[ ] correct indentation on html files, use prettier probably</li>
  <li>[ ] more rules of CommonMark</li>
    <ul>
      <li>[X] bold</li>
      <li>[X] italic</li>
      <li>[X] inline code</li>
      <li>[ ] code blocks</li>
      <li>[ ] blockquote</li>
      <li>[X] images</li>
      <li>[X] horizontal lines</li>
    </ul>
  <li>[X] basic documentation on usage and themeing</li>
</ul>
</details>

<details>
  <summary>v0.0.0 - beta</summary>
  <br>
  <ul>
    <li>[X] basic static site generator, can generate a directory that can be hosted as a site</li>
    <li>[X] adheres to enough rules of CommonMark</li>
    <ul>
      <li>[X] headings</li>
      <li>[X] paragraphs</li>
      <li>[X] links</li>
    </ul>
    <li>[X] get a good name for the project</li>
    <li>scope creep</li>
    <ul>
      <li>[X] inline html pass through</li>
      <li>[X] standalone html pass through</li>
      <li>[X] github action template for easy pages deployment; see <a href="https://github.com/cartwatson/cartwatson.github.io/blob/72adc55acf01db57022b42e61ad8e729f7cf63c3/.github/workflows/deploy.yml">here</a></li>
    </ul>
  </ul>
</details>

## Naming

Calcite has a PH of 9.91 making it naturally pretty _basic_, also it's a great looking [minecraft block](https://minecraft.wiki/w/Calcite)
