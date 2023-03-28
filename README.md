# README

Help generate .gitignore files for git repositories.The gitignore template file is from
the https://github.com/github/gitignore.git

## INSTALL

If there is no rust environment, please install rust.

If there is a rust environment/installed

```shell
cargo install creat_gitignore
```

## USAGE

### init

Please execute the init command when using it for the first time.

```shell
cgi init
```

Re-execution will pull the latest template data from GitHub again.

### ls

View the list of templates.

```shell
cgi ls
```

### add
```shell
cgi add a.gitignore
```
Add a.gitignore to the custom template library.

### help

```shell
cgi help
```

### example

If you want to add a Java project gitignore template file to a directory /a/b/c

```shell
cd /a/b/c
cgi Java
```

Run the ls command to view the supported templates

## UNINSTALL

```shell
cargo unistall creat_gitignore
```
