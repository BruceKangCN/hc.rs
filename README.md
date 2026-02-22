# HC

Simple HTTP based health check tool. Works like `ping`.

## Usage

| Option | Description | Default|
| ------ | ----------- | ------ |
| `-h, --help` | print help | |
| `-V, --version` | print version | |
| `-c, --count` | request count | unlimited |
| `-i, --interval` | request interval in milliseconds | 1000 |
| `END_POINT` | HTTP end point | |

## Detail

- Timeout is hardcoded to 5 seconds for now.
- This program uses HTTP GET method for a better compatibility. Some servers
  (and/or end points) disallow HEAD requests.
