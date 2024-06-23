# Rustpass

Project for 2024 RUST course during Computer Science studies on AGH University.

---
It is a simple password manager with symmetric encryption of passwords, git synchronization and functionality to export passwords as `tar` archive and them import them.

Use any command with `-h` option to print available options.

# Instalation

```shell
git clone https://github.com/JBRS307/rustpass.git
```

After that, open the project root directory and use
```shell
cargo build --release
```

# Commands

To run any command use
```shell
./rustpass <COMMAND>
```

## INIT

Initializes the password and key storage. Should be used also to initialize subfolders

```shell
./rustpass init [OPTIONS]
```

## ADD

Adds password to given subfolder (if no subfolder specified, then places password in main password storage). Adds password ONLY if it doesn't already exist.

```shell
./rustpass add [OPTIONS] <NAME> <PASSWORD> <REPEAT_PASSWORD>
```

## UPDATE

Works like `ADD`, however will update password if already exists.

```shell
./rustpass update [OPTIONS] <NAME> <PASSWORD> <REPEAT_PASSWORD>
```

## REMOVE

Removes given password in given subfolder, if no subfolder specified removes password from main password storage.

```shell
./rustpass remove [OPTIONS] <NAME>
```

## GENERATE

Generates password of desired length. Password can be saved by specifing `NAME` and `SUBFOLDER`.

```shell
./rustpass generate [OPTIONS] <LENGTH> [NAME] [SUBFOLDER]
```

## GET

Gets the password by `NAME`.

```shell
./rustpass get [OPTIONS] <NAME>
```

## LIST

Lists stored passwords names as tree.  
WARNING, `tree` must be installed for this to work,
to install tree use
```shell
sudo apt install tree
```

```shell
./rustpass list [SUBFOLDER]
```

## CLEAR

Completely clears password storage or given subfolder, use with caution.

```shell
./rustpass clear [OPTIONS]
```

## CONFIG

Changes the location of **.pass_key** directory (default is home directory), which is stored in **.pass_config** file. Can also print the current location with `-g` option.

```shell
./rustpass config [OPTIONS] [PATH]
```

## GIT

Enables easy usage of git inside of password and key storages. Git must be installed for the command to work properly.  
To install git use
```shell
sudo apt install git
```

```shell
./rustpass git [OPTIONS] [ARGS...]
```


## EXPORT

Export password or key storage to **tar.gz** archive. Default export location is home directory. Requires `tar` installed.  
To install tar
```shell
sudo apt install tar
```

```shell
./rustpass export [OPTIONS]
```

WARNING - archives SHOULD NOT be renamed.
## IMPORT

Imports key or passwords from **tar.gz** archive.

```shell
./rustpass import <PATH>
```

## HELP

Prints help message.

```shell
./rustpass help
```


