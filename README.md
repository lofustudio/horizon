# ☀️ Horizon

A stupidly simple, multi-platform music app. Built with Tauri and React.

## Commits

Since the 4th June, we've started to use conventional commit messages to help authors and maintainers understand commits with additional context.
This convention dovetails with SemVer, by describing the features, fixes, and breaking changes made in commit messages.  

An example of a "conventional" commit message looks like this:
```bash
<type>[optional scope]: <description>
# fix(front): hide window bar on mobile

[optional body]
# Fixes 1234abc.

[optional footer(s)]
# Co-authored-by: Ty Mason <hi@tygr.dev>
```

Horizon is following the standard [types](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13#types) and [general rules](https://www.conventionalcommits.org/en/v1.0.0/#specification) of conventional commits, however, we are adding two more:  
* `release`: The version number on all of the different metadata files like `package.json` and `Cargo.toml` have been incremented.
* `revert`: For commit reverts, **must** have a footer that references the commit SHAs that are being reverted.

You can find lots of information about conventional commits at [conventionalcommits.org](https://www.conventionalcommits.org/en/v1.0.0/) or this [gist](https://gist.github.com/qoomon/5dfcdf8eec66a051ecd85625518cfd13#conventional-commit-messages).
