# Calcite

An intentionally basic static site generator written in rust 🦀 with the help of nix ❄️  

## Why should I use calcite?

Calcite aims to be a middle ground between deploying a basic html website, like one described in the [html hobbyist](https://www.htmlhobbyist.com/), and wihtout the overhead or complexity of a tool like [hugo](https://gohugo.io/) or [wordpress](https://wordpress.com/). Giving users easy and complete control of their themeing while keeping it separate from their content.

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

### Tool Usage

<!-- TODO: put version number at end of command when available, ie `nix run github:cartwatson/calcite/v1.0.0` -->
Run `nix run github:cartwatson/calcite` in the directory you setup above

Obligatory [Security Warning](https://determinate.systems/posts/nix-run/#security-warning), the best way to avoid said security issue is to [pin to a version](https://determinate.systems/posts/nix-run/#using-git-revisions-as-a-versioning-mechanism)

## Goals

<details>
<summary>future features</summary>
<br>
<ul>
  <li>[ ] way to pull a theme from github, ie <code>nix run --theme=github:cartwatson/calcite-theme-gruvbox/v1.0.0</code>; aim is to make deployment even more simple</li>
  <li>[ ] syntax highlighting in codeblocks</li>
  <li>[ ] allow template files in sub directories; eg <code>blog/template.html</code></li>
  <li>[ ] easy copy from codeblocks</li>
  <li>[ ] inline html pass through</li>
  <li>[ ] standalone html pass through</li>
  <li>[ ] github action template for easy pages deployment</li>
  <li>[ ] subheading for title, likely h2 with id</li>
  <li>[ ] more markdown</li>
  <ul>
    <li>[ ] unordered lists</li>
    <li>[ ] ordered lists</li>
  </ul>
</ul>
</details>

<details>
<summary>v1.0.0</summary>
<br>
<ul>
  <li>[X] available on nixpkgs - counting the ability to <code>nix run github:cartwatson/calcite</code> as enough...</li>
  <li>[ ] clean up code architecture, no more hacks</li>
  <li>[ ] if unsuccessful at anypoint, remove any artifacts created</li>
  <li>[ ] more rules of CommonMark</li>
    <ul>
      <li>[ ] bold</li>
      <li>[ ] italic</li>
      <li>[ ] inline code</li>
      <li>[ ] code blocks</li>
      <li>[ ] blockquote</li>
      <li>[ ] images</li>
      <li>[ ] horizontal lines</li>
    </ul>
  <li>[X] basic documentation on usage and themeing</li>
</ul>
</details>

<details open>
  <summary>v0.0.0 - beta</summary>
  <br>
  <ul>
    <li>[ ] basic static site generator, can generate a directory that can be hosted as a site</li>
    <li>[X] adheres to enough rules of CommonMark</li>
    <ul>
      <li>[X] headings</li>
      <li>[X] paragraphs</li>
      <li>[X] links</li>
    </ul>
    <li>[X] get a good name for the project</li>
  </ul>
</details>

## Development

Requires a `test.md` file. Can be any md file but ideally includes a variety of md components, including nested components (ie code blocks inside blockquotes).

```bash
nix develop
nix run
```

example `content/test.md`

````markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

<p>test</p>

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

regular ***bold + italic* just bold** regular
regular **bold** regular
regular *italic* regular
regular paragraph
regular paragraph

regular paragraph

````

## Naming

Calcite has a PH of 9.91 making it naturally pretty _basic_, also it's a great [minecraft block](https://minecraft.wiki/w/Calcite)
