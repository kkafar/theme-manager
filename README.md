# theme-manager

Simplified CLI for theme management for Linux Mint Cinnamon

## Usage

```
Usage: theme-manager [OPTIONS] <COMMAND>

Commands:
  set   Sets theme by name or basing on current time
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
