# Rust Static Site Generator

An intentionally basic static site generator written in rust 🦀  

## Goals

### V0

- basic static site generator, can generate a directory that can be hosted as a site
- adheres to enough rules of CommonMark
  - headings
  = paragraphs
  - links
- get a good name for the project

### V1

- available on nixpkgs for common usage and usage in github workflows
- more rules of CommonMark
  - bold
  - italic
  - blockquote
  - code blocks
  - inline code

## Development

Requires a `test.md` file. Can be any md file but ideally includes a variety of md components, including nested.

```
  nix develop
  cargo run -- test.md
```

