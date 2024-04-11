# serve

## Usage

Serve files from a directory like darkhttpd, but replying to CORS OPTION pings too. (And allowing everything.)

Usage: serve [OPTIONS]

Options:
  -p, --path <PATH_TO_SERVE>   directory to serve [default: .]
  -l, --listens <SOCKET_ADDR>  where to listen [default: [::]:8080]
  -h, --help                   Print help
  -V, --version                Print version

## Notes

There is no listing of the files in the directory, so the consumer must know the correct files names to access.
Maybe I add it later.

## Example

`serve -p . -l 127.0.0.1:8080` will serve the files from the current directory at port 8080 bound to loopback device,
so other hosts in the network will not be able to access it.
