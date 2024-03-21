# cidrrr

Does the same as [cidrr](https://github.com/stuarthicks/cidrr), but with one more `r` :D

Makes inspection of IPv4 and IPv6 CIDR blocks a bit easier :)

## Installation

```shell
cargo install cidrrr
```

## Usage

```shell
# shows the first and last IP of the block
cidrrr 10.105.4.0/24

10.105.4.1
10.105.4.254
```

```shell
# shows all IPs of the block as a JSON array
# running the tool without "--danger-zone" will yield at most "1.048.576" addresses
cidrrr --all --output json 10.105.4.0/24

["10.105.4.1", "10.105.4.2", "10.105.4.3", ..., "10.105.4.253", "10.105.4.254"]
```
