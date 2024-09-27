# WebDir

WebDir is a super simple web server that serves a directory tree of files.

It is intended to be used test and lab environments. It's not recommended for production environments.

## Usage
```
Usage: webdir [OPTIONS] --dir <DIR>

Options:
  -d, --dir <DIR>              Directory to serve
  -4, --ipv4 <IPV4>            IP address to listen on [default: 127.0.0.1]
  -p, --port <PORT>            Port to listen on [default: 13337]
  -l, --log-level <LOG_LEVEL>  Log level (error, warn, info, debug, trace) [default: info]
  -h, --help                   Print help
  -V, --version                Print version
```