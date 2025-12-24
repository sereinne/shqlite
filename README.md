# DBCLI
A simple TUI sqlite client in Java

![Example demonstration of `dbcli`](demo.gif)

## Dependencies
- [sqlite-jdbc](https://github.com/xerial/sqlite-jdbc)
- [jline3](https://github.com/jline/jline3)
- [picocli](https://github.com/remkop/picocli)

## Installation
Make sure, [gradle](https://gradle.org) is installed.

Clone this project
```
$ git clone https://github.com/sereinne/dbcli
$ cd dbcli
```

Build this project using `gradle`
```
$ gradle installDist
```

run `dbcli`
```
$ ./app/build/install/dbcli/bin/dbcli
```

## Todos
- [ ] Implement dot commands
- [x] Able to execute SQL queries
- [ ] Better Autocompletions
