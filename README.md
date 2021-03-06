# envch

![Screenshot of envch in the terminal](assets/readme-simple-example.png)

An intuitive program for setting, modifying, listing, and removing environment variables on Linux

## Installation

### Cargo
If you don't have Cargo already, install it using this command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install envch

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

#### Note about `sudo`
In order to run this command with `sudo`, you'll have to explicitly tell `sudo` to preserve the `PATH` variable (this is necessary for running `envch`). For example, to run `sudo envch set MY_VAR MY_VALUE`, you would instead type the following:

```bash
sudo --preserve-env=PATH env envch set MY_VAR MY_VALUE
```

If you'd like to make this easier, you can run this (preferably in .bashrc or .zshrc) to alias `sudo_envch` to the command above.

```bash
alias sudo_envch='sudo --preserve-env=PATH env envch'
```

Now, you can run `sudo_envch set MY_VAR MY_VALUE` without errors.

### List
![Screenshot of output by list command](assets/readme-list-example.png)  
You can run `list` by entering `envch list`. This command lists your environment variables. The names of the variables are color-coded. Blue means that it's a system-wide environment variable, yellow means that it is a user-wide variable, and pink means that it is declared in a terminal initialization script (like .bashrc or .zshenv). By default, `list` does not show the `PATH` variable because it usually needs to be treated differently than other variables.

#### Show column names
![Screenshot of list command with -c flag](assets/readme-list-column-example.png)  
If you'd like the columns to be titled (like Name and Value), you can use the `-c` or `--show-columns` flags.

#### Show declared in
![Screenshot of list command with -sc flags](assets/readme-list-declared-example.png)  
If you'd like to see the specific file where your environment variables are declared, you can use the `-s` or `--show-declared-in` flags.

#### Show path
To include the `PATH` variable in the output, use the `-p` or `--show-path` flags.

### Set
![Screenshot of setting an environment variable using sudo](assets/readme-set-example.png)  
You can run `set` by entering `envch set`. `set` updates an environment if it exists. If no environment variable with the specified name is found, a new environment variable is declared with the specified name and value (by default, this is user-scoped). This command usually requires `sudo`.

#### Arguments
`set` takes two arguments: `<name>` and `<value>`. To set an environment variable, use `envch set <name> <value>`.

#### Set scope
![Screenshot of setting a terminal-scoped environment variable](assets/readme-set-scope-example.png)  
When creating a new environment variable using the `set` command, you might want to specify which scope the new variable should be set in. There are three scopes: system, user (default), and terminal. A system environment variable can be accessed by all users on a system (these are declared in /etc/environment). A user environment variable (declared in /etc/profile.d) is accessable all users on a system. A terminal environment variable is specific to your terminal. Your terminal will be termined by the `SHELL` environment variable, which points to `bash` by default. This means that the shell will not be determined by your active terminal, but rather the shell that you've set to be default. Supported shells include bash and zsh. If you'd like to see another shell supported, please create an issue on the Github repository.

### Remove
![Screenshot of removing an environment variable](assets/readme-remove-example.png)  
`remove` removes an environment variable from your computer. It takes one argument, which is the name of the environment variable to remove. For example, you can run `envch remove MY_ENV_VAR` to remove an environment variable called MY_ENV_VAR.

### Help
To get help with `envch` in general or a specific command, you can add the `-h` or `--help` flag to any command. `envch --help` will print general help about the different commands you can use. `envch list --help`, for example, will print information about the `list` subcommand, like the flags it accepts.

### Debug mode
To enable debug mode, use the `-d` or `--debug` flags.
