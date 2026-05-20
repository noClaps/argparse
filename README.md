# argparse

A very simple command line argument parsing library in Rust.

## Usage

Add it to your project with:

```sh
cargo add --git https://github.com/noClaps/argparse
```

You can then use it in your code:

```rust
use argparse::ArgParse;

let mut args = ArgParse::new();
args.parse();
```

### Configuration

You can set some configuration options:

- `help`: Set whether to show help with the `--help` and `-h` flags. Help generation is enabled by default, but you can set it explicitly using this configuration option.

  ```rust
  use argparse::ArgParse;

  let mut args = ArgParse::new().help(true);

  // Flags:
  //   -h, --help    Display this help and exit.
  ```

- `about`: Set a description for your command.

  ```rust
  use argparse::ArgParse;

  let mut args = ArgParse::new().about("A short description of this program.");

  // Usage: ...
  //
  // A short description of this program.
  //
  // ...
  ```

- `positional`: Add a positional argument to your command. It has the following parameters:
  - `name`: The name of the positional argument. This will be displayed in the help menu, as well as acting as the key to retrieve the value in the positional.
  - `help`: The help text to display for the positional argument. This will be displayed in the help menu, if it has been enabled.

  ```rust
  use argparse::ArgParse;

  let mut args = ArgParse::new().positional("name", "A short description of this positional argument.");

  // Positionals:
  //   <name>    A short description of this positional argument.
  ```

- `flag`: Add a flag to your command. It has the following parameters:
  - `name`: The name of the flag. This will act as the key to retrieve the value in the flag. It will also be displayed as the name of the argument in the help menu, if the flag is not a switch.
  - `long`: The long name of the flag. This corresponds to the `--long` form of the flag, and will be displayed as such in the help menu.
  - `short`: The optional short name of the flag. If it is set, then it corresponds to the `-short` form of the flag, and will be displayed as such in the help menu.
  - `help`: The help text to display for this flag. This will be displayed in the help menu, if it has been enabled.
  - `switch`: Whether or not this flag is boolean. If this is true, then the flag will not have an argument following it.

  ```rust
  use argparse::ArgParse;

  let mut args = ArgParse::new()
      .flag("name", "name", Some("n"), "A short description of this flag.", false)
      .flag("age", "age", None, "A short description of this flag.", false)
      .flag("above height limit", "above-height-limit", None, "A short description of this flag.", true);

  // Flags:
  //   -n, --name <name>       A short description of this flag.
  //   --age <age>             A short description of this flag.
  //   --above-height-limit    A short description of this flag.
  ```

### Getting values

Once you've set up your `ArgParse` instance, call the `parse()` method. Once you've done that, you can call `get_positional()` and `get_flag()` with the names you set in the configuration to retrieve the values you set.

```rust
use argparse::ArgParse;

let mut args = ArgParse::new()
    .positional("name", "A short description of this positional argument.")
    .flag("age", "age", Some("a"), "A short description of this flag.", false); // set your configuration options

args.parse();
let name: String = args.get_positional("name").unwrap();
let age: usize = args.get_flag("age").unwrap();
```

If you do not call `parse()` first, both `get_positional()` and `get_flag()` will return `None`. They will also return `None` if the value of the positional or flag could not be parsed to the correct type, e.g. attempting to parse `--age "abcd"` to `usize`.
