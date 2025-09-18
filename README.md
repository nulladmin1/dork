# dork

A simple local Downloads organizer

## Run (from `source`)

Clone repository, install `Rust`, and `cargo run` to run

## Run (using `Nix`)

```nix
nix run github:nulladmin1/dork
```

or add the `package` to `environment.systemPackages` or `home.packages` for permanent installation

## Config (stored in `~/.config/dork/config.toml`)

```toml
# Default config

# Settings for sorting
[sorting]

# Optional folder for miscallenous items
misc = "Misc"

# A map of a subdirectory in Downloads to a list of file extensions
[sorting.by-type]
Videos = ["mp4", "mkv"] # Create a Videos/ subdirectory with files of "mp4" and "mkv" types
Documents = ["docx", "pdf", "txt", "pptx", "xlsx"] # Create a Documents/ subdirectory with corresponding types
Code = ["py", "java", "class", "rs", "toml", "yaml", "json"] # ...
Images = ["png", "jpg", "jpeg", "svg", "gif"] #...
```

## TODO

- Undo feature
- Sort by date
- PostSortActions
- `add` command

