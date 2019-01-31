# echoip

The `echoip` application is used to get the public IP address of a host
hidden behind NAT. It runs a server on a publicly available host,
listening on port 5300. The client then sends a packet to the server,
which responds with a packet containing the IP address of the host which
contacted it.

At this time it only works with IPv4 over UDP, and the returned IP address
is formatted in dotted quad form. Long term, we would like to see it work
over TCP as well, and possibly deliver the IP address in binary format as
well as text. There is no special hurry with getting it working over IPv6,
as NAT over IPv6 is discouraged and rarely used.

## Installation

TODO: Write installation instructions here

## Usage

TODO: Write usage instructions here

## Development

TODO: Write development instructions here

## Contributing

1. Fork it (<https://github.com/mjochimsen/echoip/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [Michael Jochimsen](https://github.com/mjochimsen) - creator and maintainer
