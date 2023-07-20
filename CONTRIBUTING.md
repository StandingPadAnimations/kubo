# Contributing for Kubo
If you're reading this, then it sounds like you're interested in contributing to Kubo. If so, here's a guide on contributing.

# Building Kubo
Clone the repo using Git, or fork the repo if you'd prefer. Kubo only requires one command to build, `cargo build`, and that will put a binary in `target/debug/`. For a release build, do `cargo build --release`, and the binaryu will be found in `target/release`.

# Implementing new features
If you want to implement a new feature to Kubo, that's great, but first ask yourself the following question:
"Is this a feature that only I want/need or is it a feature that users want?"

If it's the former, then by all means fork Kubo. Kubo is open source under GNU GPL V3, which not only allows modifications for one's personal use, but encourages it. Kubo is mostly a personal project, so we typically don't add features unless a sizable portion of the community wants them.

# Submitting a pull request
Before submitting a pull request, make sure the following is true:
1. All commit messages need to **justify** why a change was made, not repeat what the change was. In addition, the following format is required:
```
Basic summary of changes in less then 50 chars

Further explanation with each line being 72 characters long, to fit in 
people's terminals
```
2. All added dependencies are compatible with GNU GPL V3
3. `cargo build` works

When making a pull request, think about the following:
- Title: what does this pull request add or change?
- Description: why were changes made? Justify the changes, don't just repeat them

With that, you should be ready to make a pull request and contribute to Kubo!

# Section for companies
Companies and FOSS are an... interesting mix, with many companies not caring about FOSS projects or contributing back.

Our philosophy here at Kubo is this: do whatever you want, but follow GPL

If a company wants to use this project but not contribute back, that's fine, but at the very least it's important to follow GPL if modifications are made, especailly for support. Kubo is FOSS, but there's still a license.
