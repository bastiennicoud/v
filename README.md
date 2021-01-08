# v

> Experiment rust with a little brew formulae version switcher...

## Build

```
$ git clone git@github.com:bastiennicoud/v.git

$ cargo run -- php 8.0
```

## Underlying brew commands

```
# Get a json representation of all the installed formulaes
$ brew info --json --installed
```
