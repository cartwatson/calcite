# Rust Static Site Generator

An intentionally basic static site generator written in rust 🦀  

## Usage

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

## Goals

### V0

- basic static site generator, can generate a directory that can be hosted as a site
- adheres to enough rules of CommonMark
  - headings
  - paragraphs
  - links
- get a good name for the project

### V1

- available on nixpkgs
- more rules of CommonMark
  - bold
  - italic
  - inline code
  - code blocks
  - blockquote
  - images
  - horizontal lines
- basic documentation on usage and themeing

## Development

Requires a `test.md` file. Can be any md file but ideally includes a variety of md components, including nested components (ie code blocks inside blockquotes).

```
  nix develop
  cargo run -- content/test.md
```

