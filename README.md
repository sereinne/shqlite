# Shqlite
A simple TUI [`sqlite`](https://sqlite.org) client in Java

> [!NOTE]
> `shqlite` is not stable yet for personal usage. Nevertheless, you could try it out with limited capabilities!

# Demonstration
![Example demonstration of `dbcli`](demo.gif)

## Java Dependencies
- [sqlite-jdbc](https://github.com/xerial/sqlite-jdbc)
- [jline3](https://github.com/jline/jline3)
- [picocli](https://github.com/remkop/picocli)

## Installation
Make sure, [gradle](https://gradle.org) is installed.

Clone this project
```
$ git clone https://github.com/sereinne/shqlite
$ cd shqlite
```

Build this project using `gradle`
```
$ gradle installDist
```

run `dbcli`
```
$ ./app/build/install/shqlite/bin/shqlite <path_to_db_file>
```

## Dot Commands Table
| Commands     | Implemented        |
| --------     | ------------------ |
|.archive      | :x:                |
|.auth         | :white_check_mark: |
|.backup       | :white_check_mark: |
|.bail         | :x:                |
|.cd           | :x:                |
|.changes      | :x:                |
|.check        | :x:                |
|.clone        | :x:                |
|.connection   | :x:                |
|.crlf         | :x:                |
|.databases    | :white_check_mark: |
|.dbconfig     | :x:                |
|.dbinfo       | :x:                |
|.dbtotxt      | :x:                |
|.dump         | :x:                |
|.echo         | :x:                |
|.eqp          | :x:                |
|.excel        | :x:                |
|.exit         | :white_check_mark: |
|.expert       | :x:                |
|.explain      | :x:                |
|.filectrl     | :x:                |
|.fullschema   | :x:                |
|.headers      | :x:                |
|.help         | :white_check_mark: |
|.import       | :x:                |
|.imposter     | :x:                |
|.indexes      | :x:                |
|.intck        | :x:                |
|.limit        | :x:                |
|.lint         | :x:                |
|.load         | :x:                |
|.log          | :x:                |
|.mode         | :x:                |
|.nonce        | :x:                |
|.nullvalue    | :x:                |
|.once         | :x:                |
|.open         | :x:                |
|.output       | :x:                |
|.parameter    | :x:                |
|.print        | :x:                |
|.progress     | :x:                |
|.prompt       | :x:                |
|.quit         | :white_check_mark: |
|.read         | :x:                |
|.recover      | :x:                |
|.restore      | :white_check_mark: |
|.save         | :white_check_mark: |
|.scanstats    | :x:                |
|.schema       | :white_check_mark: |
|.separator    | :x:                |
|.session      | :x:                |
|.sha3sum      | :x:                |
|.shell        | :x:                |
|.show         | :x:                |
|.stats        | :x:                |
|.system       | :x:                |
|.tables       | :white_check_mark: |
|.timeout      | :x:                |
|.timer        | :x:                |
|.trace        | :x:                |
|.unmodule     | :x:                |
|.version      | :white_check_mark: |
|.vfsinfo      | :x:                |
|.vfslist      | :x:                |
|.vfsname      | :x:                |
|.width        | :x:                |
|.www          | :x:                |
