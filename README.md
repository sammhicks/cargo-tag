# cargo-tag

A tool which tags the latest commit with version information if the version field in `Cargo.toml` changes.

## Installation ##

    git clone https://github.com/sammhicks/cargo-tag
    cargo install --path .

## Usage ##

Having commited a change where the version field in `Cargo.toml` changes:

    cargo tag

The commit will be tagged with the new version.

In a workspace, the tag is prefixed with the name of the package.

### Git Hooks ###

Git allows you to automatically run scripts when certain actions happen.

This tool was designed to be run by the post-commit git hook.

To do so, create a file at `.git/hooks/post-commit` with the following:

    #!/bin/sh
    cargo tag
