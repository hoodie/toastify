<div align="center">

# toastify

[![build](https://img.shields.io/github/actions/workflow/status/hoodie/toastify/ci.yml?branch=main)](https://github.com/hoodie/toastify/actions)
[![Crates.io](https://img.shields.io/crates/d/toastify)](https://crates.io/crates/toastify)
[![contributors](https://img.shields.io/github/contributors/hoodie/toastify)](https://github.com/hoodie/toastify/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2026)

[![version](https://img.shields.io/crates/v/toastify)](https://crates.io/crates/toastify/)
[![license](https://img.shields.io/crates/l/toastify.svg?style=flat)](https://crates.io/crates/toastify/)

</div>

A commandline tool that shows desktop notifications using [notify-rust](https://docs.rs/notify-rust/).


### `toastify send`

The available options differ per platform. 

```text
Shows a notification

Usage: toastify send [OPTIONS] <TITLE> [BODY]

Arguments:
  <TITLE>  Title of the Notification
  [BODY]   Message body. Use `-` to read it from stdin

Options:
  -a, --app-name <APP_NAME>      Set a specific app-name manually
  -s, --sound-name <SOUND_NAME>  Set a specific sound manually
      --subtitle <SUBTITLE>      Subtitle shown below the title
      --image <IMAGE>            Image shown inside the notification
  -b, --bundle-id <BUNDLE_ID>    Bundle identifier of the app to send the notification as, e.g. com.apple.Terminal
  -h, --help                     Print help
```

On Linux/BSD there are additionally `--expire-time`, `--icon`, `--id`, `--categories`, `--hint`, `--urgency` and `--debug`;
on Windows `--expire-time`, `--urgency`, `--subtitle`, `--image` and `--app-id`.

### Examples

```sh
# read the body from stdin
make 2>&1 | tail -n 1 | toastify send "Build finished" -
```

***

## Contribution
Any help in form of descriptive and friendly [issues](https://github.com/hoodie/toastify/issues) or comprehensive pull requests are welcome! 


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in toastify by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions
The Changelog of this library is generated from its commit log, there any commit message must conform with https://www.conventionalcommits.org/en/v1.0.0/. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).
