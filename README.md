## Tiddly

Tiddly is a simple DAV webserver for [TiddlyWiki 5](https://tiddlywiki.com/).

The CLI serves your wiki HTML and automatically creates a backup before each save.

It can be used like so:
```
tiddly [target.html] [backup directory]
```

## Installation
To start, clone the repo.
```
git clone https://github.com/pearigee/tiddly.git
```
Afterwards, navigate into the cloned directory and use cargo to compile and install the binary.
```
cd tiddly
cargo install --path .
```