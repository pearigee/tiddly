## Tiddly

Tiddly is a simple DAV webserver for [TiddlyWiki 5](https://tiddlywiki.com/). This is intended for local use only. For public facing wikis, consider the [official Node.js server](https://www.npmjs.com/package/tiddlywiki).

Use the CLI to serve your wiki HTML and automatically create backups before each save.

Run `tiddly --help` for usage information.

```
tiddly 0.1.0
A small, leightweight TiddlyWiki server that supports `PUT` (DAV) saves

USAGE:
    tiddly [OPTIONS] --target <TARGET> --backup-dir <BACKUP_DIR>

OPTIONS:
    -b, --backup-dir <BACKUP_DIR>    Directory to store backups
    -h, --help                       Print help information
    -p, --port <PORT>                Port to serve on [default: 8000]
    -t, --target <TARGET>            Target TiddlyWiki HTML file
    -V, --version                    Print version information
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

## How does it work?
TiddlyWiki 5 has a `PUT` saver included by default. The source can be found [here](https://github.com/Jermolene/TiddlyWiki5/blob/39e4e69ae79d3a0cf060a091c9c613b09848d275/core/modules/savers/put.js).

TiddlyWiki fires an `OPTIONS` request when opened. If the response includes a [truthy](https://developer.mozilla.org/en-US/docs/Glossary/Truthy) `dav` header and has status `200` the `PUT` save functionality is enabled. See the [source](https://github.com/Jermolene/TiddlyWiki5/blob/39e4e69ae79d3a0cf060a091c9c613b09848d275/core/modules/savers/put.js#L58) for a more detailed explanation.

When TiddlyWiki saves, it `PUT`'s itself as a static HTML file to the server.

When Tiddly recieves this request it first backs up the current file. Then it streams the uploaded document into a file that replaces the previous target. The new file will be served on following `GET` requests.

## Why make this?
Why not? It was fun. Do I have to have a reason? Here's a half baked one: 

I wanted a small, low memory, executable I could run in the background. I could run it on my Steam Deck without impacting gaming performance (I think). :D