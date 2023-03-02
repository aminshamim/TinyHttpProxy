# TinyHttpProxy
TinyHttpProxy is a lightweight HTTP/HTTPS proxy server written in Rust. It can be used to intercept and modify HTTP/HTTPS traffic, as well as to serve as a reverse proxy for HTTP/HTTPS servers.

## Features
- HTTP/HTTPS proxy server
- Reverse proxy for HTTP/HTTPS servers
- DNS caching
- Easy to configure with a simple configuration file
- Lightweight and fast, written in Rust

## Installation
To install TinyHttpProxy, you'll need to have Rust installed on your system. Once Rust is installed, you can use Cargo to build and install TinyHttpProxy:


## Configuration
TinyHttpProxy can be configured using a simple configuration file located at /etc/tinyhttpproxy/proxies.conf. The configuration file contains a list of proxies, each of which is defined by a section like this:

```
[myproxy]
path = /myproxy
target = https://example.com
```

This defines a proxy that listens on the path /myproxy and forwards requests to https://example.com. You can add as many proxies as you like by adding additional sections to the configuration file.

## Contributing
Contributions are welcome! If you find a bug or would like to suggest a new feature, please open an issue on GitHub. Pull requests are also welcome.

## License
TinyHttpProxy is licensed under the GNU Public License version 3 (GPLv3). See the LICENSE file for details.
