# theme-manager

Simplified CLI for theme management for Linux Mint Cinnamon

## Installation

This tool has not been published in ant public registry (e.g. [crates.io](https://crates.io/)) yet.

However you can install it by cloning this repo and running `cargo install`:

```
git clone git@github.com:kkafar/theme-manager.git theme-manager
cargo install --path theme-manager
```

To run this tool periodicaly [`cron`](https://en.wikipedia.org/wiki/Cron) can be used:

```
SHELL=/bin/bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:<YOUR_HOME_DIR>/.cargo/bin

# Change theme. Check twice in an hour.
01 * * * * theme-manager --verbose --log-file <PATH_TO_LOG_FILE> --config <PATH_TO_CONFIG> set
31 * * * * theme-manager --verbose --log-file <PATH_TO_LOG_FILE> --config <PATH_TO_CONFIG> set
```


## Usage

```
Usage: theme-manager [OPTIONS] <COMMAND>

Commands:
  set   Sets theme by name or basing on current time
  get   Retrieves current configuration and prints it to logfile or stdout
  help  Print this message or the help of the given subcommand(s)

Options:
      --config <FILE>    Path to config file - see project readme for config file description
  -v, --verbose          Run in verbose mode
      --log-file <FILE>  Path to log file - if not specified logs are printed to stdout
  -h, --help             Print help information
  -V, --version          Print version information
```

### Examples

* `theme-manager --config <CONFIG> set dark`
  * sets theme to `dark`, basing on theme definition found in `<CONFIG>`
* `theme-manager --config <CONFIG> set`
  * sets theme to one that is assigned for current time
  * if there is no such theme, default one is used
  * if default is not specified - no changes are performed
* `theme-manager --log-file <FILE> set`
  * sets theme to one that is assigned for current time;
  * if there is no such theme, default one is used
  * if default is not specified - no changes are performed
  * **config is loaded from default location: `$HOME/.config/theme-manager/config.json`**

### Config specification

See [config example](config-example/config.json) for supported fields & options.


**Note**: `kitty` param in theme specification is optional (rest of them are required) - it is option for setting theme of terminal emulator of my choice.
