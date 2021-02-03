# envch

![Screenshot of envch in the terminal](assets/readme-simple-example.png)

An intuitive program that allows users to create, modify, list, and remove environment variables

## Installation

### Cargo
If you don't have Cargo already, install it using this command:

#### macOS and Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows
If you're on Windows, you can install Cargo using [this guide](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup).


#### Install envch

Then, install envch using:
```bash
cargo install envch
```

### Manually
To manually install envch, run the following:
```bash
git clone https://github.com/ZackMurry/envch.git
cd envch

cargo build --release

cd target/release
./envch
```

## Usage
Envch provides three commands: `list`, `set`, and `remove`.

### List
You can run `list` by entering `envch list`. This command lists your environment variables. The names of the variables are color-coded. Blue means that it's a system-wide environment variable, yellow means that it is a user-wide variable, and pink means that it is declared in a terminal initialization script (like .bashrc or .zshenv). By default, `list` does not show the `PATH` variable because it usually needs to be treated differently than other variables.

#### Show column names
If you'd like the columns to be titled (like Name and Value), you can use the `-c` or `--show-columns` flags.

#### Show declared in
If you'd like to see the specific file where your environment variables are declared, you can use the `-s` or `--show-declared-in` flags.

#### Show path
To include the `PATH` variable in the output, use the `-p` or `--show-path` flags.

### Set
You can run `set` by entering `envch set`. `set` updates an environment if it exists. If no environment variable with the specified name is found, a new environment variable is declared with the specified name and value (by default, this is user-scoped).

#### Arguments
`set` takes two arguments: `<name>` and `<value>`. To set an environment variable, use `envch set <name> <value>`.

#### Set scope
When creating a new environment variable using the `set` command, you might want to specify which scope the new variable should be set in. There are three scopes: system, user (default), and terminal. A system environment variable can be accessed by all users on a system (these are declared in /etc/environment). A user environment variable (declared in /etc/profile.d) is accessable all users on a system. A terminal environment variable is specific to your terminal. Your terminal will be termined by the `SHELL` environment variable, which points to `bash` by default. This means that the shell will not be determined by your active terminal, but rather the shell that you've set to be default. Supported shells include bash and zsh. If you'd like to see another shell supported, please create an issue on the Github repository.

### Remove
`remove` removes an environment variable from your computer. It takes one argument, which is the name of the environment variable to remove. For example, you can run `envch remove MY_ENV_VAR` to remove an environment variable called MY_ENV_VAR.

### Help
To get help with `envch` in general or a specific command, you can add the `-h` or `--help` flag to any command. `envch --help` will print general help about the different commands you can use. `envch list --help`, for example, will print information about the `list` subcommand, like the flags it accepts.

### Debug mode
To enable debug mode, use the `-d` or `--debug` flags.
