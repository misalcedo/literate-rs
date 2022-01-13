# literate-rs
A [literate programming](https://en.wikipedia.org/wiki/Literate_programming) tool that allows writing code in your Markdown files.

## How it works
At its core `literate` reads a file from standard in and outputs code blocks to standard out as a single stream.
The code blocks may be filtered by various qualities such as language, attributes, etc.

## The real magic
Although `literate` itself is very simple, the magic really happens when you apply the tool to entire directories.
For example, a Rust project could add literate to their `build.rs` in order to write their examples as Markdown files that become Rust files at build time.
Since Rust has a very mature documentation culture, the actual source may not benefit as much from `literate`.
However, one can easily envision a tool that flips Rust's documentation on its head.
Instead of doc comments, Rust source could be defined as Markdown files.
During build, the file could be parsed to add the surrounding text as a doc comment to the code block code.
User's could plug in different strategies for traversing up the tree to find the relevant documentation.

## Testing
```console
cat .\examples\tortuga.ta.md | literate --language tortuga --required
```