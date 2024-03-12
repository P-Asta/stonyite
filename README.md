# stonyite
macos dotfile manager make with rust


## install

- cargo
```bash
cargo install --git https://github.com/5-23/stonyite
```

- brew
```bash
brew tap 5-23/stonyite
brew install stonyite
```


## dotfile-example
https://github.com/5-23/dotfiles-mac

## commands
stonyite commands

### setup
`install dependencies` and `set config file`
```
stonyite setup
```

### install
`just install dependencies`
```
stonyite install
```


### config
`just set config file`
```
stonyite config
```


## flags
stonyite flags

### manager
set pakage manager
> When you set up a package manager, it reads and installs dependencies-{manager}.(default: brew)

```bash
stonyite setup --manager nix-env
```


### flag
set command's flag
```bash
stonyite setup -flag debug
```
