# WebDir

WebDir is a super simple web server that serves a directory tree of files.

It is intended to be used test and lab environments. It's not recommended for production environments.

## Usage
```
Usage: webdir [OPTIONS] <DIR>

Arguments:
  <DIR>  Directory to serve

Options:
  -4, --ipv4 <IPV4>            IP address to listen on [default: 127.0.0.1]
  -p, --port <PORT>            Port to listen on [default: 13337]
  -l, --log-level <LOG_LEVEL>  Log level (error, warn, info, debug, trace) [default: info]
  -h, --help                   Print help
  -V, --version                Print version
```

```
webdir /path/to/directory

2024-09-27T01:55:15.877504Z  INFO webdir: Serving directory: "/path/to/directory"
2024-09-27T01:55:15.877545Z  INFO webdir: Listening on http://127.0.0.1:13337
```