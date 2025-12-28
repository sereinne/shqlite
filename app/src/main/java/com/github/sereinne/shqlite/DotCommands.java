package com.github.sereinne.shqlite;

import com.github.sereinne.shqlite.OutputTable.Format;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.Arrays;
import java.util.List;
import org.jline.reader.Candidate;
import org.jline.terminal.Terminal;

public class DotCommands {

    Terminal terminal;
    Statement stmt;

    private boolean assertRequiredArgument(
        String dotCommand,
        String[] dotCommandArgs
    ) {
        if (dotCommandArgs.length == 0) {
            terminal
                .writer()
                .println(
                    "an argument is required for " + dotCommand + " command..."
                );
            return false;
        }
        return true;
    }

    public DotCommands(Statement stmt, Terminal terminal) {
        this.terminal = terminal;
        this.stmt = stmt;
    }

    // All possible dot commands
    public static final List<Candidate> candidates = Arrays.asList(
        // new Candidate(value, displ, group, descr, suffix, key, complete, sort)
        new Candidate(
            ".archive",
            ".archive ...",
            null,
            "Manage SQL archives",
            null,
            null,
            true
        ),
        new Candidate(
            ".auth",
            ".auth ON|OFF",
            null,
            "Show authorizer callbacks",
            null,
            null,
            true
        ),
        new Candidate(
            ".backup",
            ".backup ?DB? FILE",
            null,
            "Backup DB (default \"main\") to FILE",
            null,
            null,
            true
        ),
        new Candidate(
            ".bail",
            ".bail on|off",
            null,
            "Stop after hitting an error.  Default OFF",
            null,
            null,
            true
        ),
        new Candidate(
            ".cd",
            ".cd DIRECTORY",
            null,
            "Change the working directory to DIRECTORY",
            null,
            null,
            true
        ),
        new Candidate(
            ".changes",
            ".changes on|off",
            null,
            "Show number of rows changed by SQL",
            null,
            null,
            true
        ),
        new Candidate(
            ".check",
            ".check GLOB",
            null,
            "Fail if output since .testcase does not match",
            null,
            null,
            true
        ),
        new Candidate(
            ".clone",
            ".clone NEWDB",
            null,
            "Clone data into NEWDB from the existing database",
            null,
            null,
            true
        ),
        new Candidate(
            ".connection",
            ".connection [close] [#]",
            null,
            "Open or close an auxiliary database connection",
            null,
            null,
            true
        ),
        new Candidate(
            ".crlf",
            ".crlf ?on|off?",
            null,
            "Whether or not to use \\r\\n line endings",
            null,
            null,
            true
        ),
        new Candidate(
            ".databases",
            ".databases",
            null,
            "List names and files of attached databases",
            null,
            null,
            true
        ),
        new Candidate(
            ".dbconfig",
            ".dbconfig ?op? ?val?",
            null,
            "List or change sqlite3_db_config() options",
            null,
            null,
            true
        ),
        new Candidate(
            ".dbinfo",
            ".dbinfo ?DB?",
            null,
            "Show status information about the database",
            null,
            null,
            true
        ),
        new Candidate(
            ".dbtotxt",
            ".dbtotxt",
            null,
            "Hex dump of the database file",
            null,
            null,
            true
        ),
        new Candidate(
            ".dump",
            ".dump ?OBJECTS?",
            null,
            "Render database content as SQL",
            null,
            null,
            true
        ),
        new Candidate(
            ".echo",
            ".echo on|off",
            null,
            "Turn command echo on or off",
            null,
            null,
            true
        ),
        new Candidate(
            ".eqp",
            ".eqp on|off|full|...",
            null,
            "Enable or disable automatic EXPLAIN QUERY PLAN",
            null,
            null,
            true
        ),
        new Candidate(
            ".excel",
            ".excel",
            null,
            "Display the output of next command in spreadsheet",
            null,
            null,
            true
        ),
        new Candidate(
            ".exit",
            ".exit ?CODE?",
            null,
            "Exit this program with return-code CODE",
            null,
            null,
            true
        ),
        new Candidate(
            ".expert",
            ".expert",
            null,
            "EXPERIMENTAL. Suggest indexes for queries",
            null,
            null,
            true
        ),
        new Candidate(
            ".explain",
            ".explain ?on|off|auto?",
            null,
            "Change the EXPLAIN formatting mode.  Default: auto",
            null,
            null,
            true
        ),
        new Candidate(
            ".filectrl",
            ".filectrl CMD ...",
            null,
            "Run various sqlite3_file_control() operations",
            null,
            null,
            true
        ),
        new Candidate(
            ".fullschema",
            ".fullschema ?--indent?",
            null,
            "Show schema and the content of sqlite_stat tables",
            null,
            null,
            true
        ),
        new Candidate(
            ".headers",
            ".headers on|off",
            null,
            "Turn display of headers on or off",
            null,
            null,
            true
        ),
        new Candidate(
            ".help",
            ".help ?-all? ?PATTERN?",
            null,
            "Show help text for PATTERN",
            null,
            null,
            true
        ),
        new Candidate(
            ".import",
            ".import FILE TABLE",
            null,
            "Import data from FILE into TABLE",
            null,
            null,
            true
        ),
        new Candidate(
            ".imposter",
            ".imposter INDEX TABLE",
            null,
            "Create imposter table TABLE on index INDEX",
            null,
            null,
            true
        ),
        new Candidate(
            ".indexes",
            ".indexes ?TABLE?",
            null,
            "Show names of indexes",
            null,
            null,
            true
        ),
        new Candidate(
            ".intck",
            ".intck ?STEPS_PER_UNLOCK?",
            null,
            "Run an incremental integrity check on the db",
            null,
            null,
            true
        ),
        new Candidate(
            ".limit",
            ".limit ?LIMIT? ?VAL?",
            null,
            "Display or change the value of an SQLITE_LIMIT",
            null,
            null,
            true
        ),
        new Candidate(
            ".lint",
            ".lint OPTIONS",
            null,
            "Report potential schema issues.",
            null,
            null,
            true
        ),
        new Candidate(
            ".load",
            ".load FILE ?ENTRY?",
            null,
            "Load an extension library",
            null,
            null,
            true
        ),
        new Candidate(
            ".log",
            ".log FILE|on|off",
            null,
            "Turn logging on or off.  FILE can be stderr/stdout",
            null,
            null,
            true
        ),
        new Candidate(
            ".mode",
            ".mode ?MODE? ?OPTIONS?",
            null,
            "Set output mode",
            null,
            null,
            true
        ),
        new Candidate(
            ".nonce",
            ".nonce STRING",
            null,
            "Suspend safe mode for one command if nonce matches",
            null,
            null,
            true
        ),
        new Candidate(
            ".nullvalue",
            ".nullvalue STRING",
            null,
            "Use STRING in place of NULL values",
            null,
            null,
            true
        ),
        new Candidate(
            ".once",
            ".once ?OPTIONS? ?FILE?",
            null,
            "Output for the next SQL command only to FILE",
            null,
            null,
            true
        ),
        new Candidate(
            ".open",
            ".open ?OPTIONS? ?FILE?",
            null,
            "Close existing database and reopen FILE",
            null,
            null,
            true
        ),
        new Candidate(
            ".output",
            ".output ?FILE?",
            null,
            "Send output to FILE or stdout if FILE is omitted",
            null,
            null,
            true
        ),
        new Candidate(
            ".parameter",
            ".parameter CMD ...",
            null,
            "Manage SQL parameter bindings",
            null,
            null,
            true
        ),
        new Candidate(
            ".print",
            ".print STRING...",
            null,
            "Print literal STRING",
            null,
            null,
            true
        ),
        new Candidate(
            ".progress",
            ".progress N",
            null,
            "Invoke progress handler after every N opcodes",
            null,
            null,
            true
        ),
        new Candidate(
            ".prompt",
            ".prompt MAIN CONTINUE",
            null,
            "Replace the standard prompts",
            null,
            null,
            true
        ),
        new Candidate(
            ".quit",
            ".quit",
            null,
            "Stop interpreting input stream, exit if primary.",
            null,
            null,
            true
        ),
        new Candidate(
            ".read",
            ".read FILE",
            null,
            "Read input from FILE or command output",
            null,
            null,
            true
        ),
        new Candidate(
            ".recover",
            ".recover",
            null,
            "Recover as much data as possible from corrupt db.",
            null,
            null,
            true
        ),
        new Candidate(
            ".restore",
            ".restore ?DB? FILE",
            null,
            "Restore content of DB (default \"main\") from FILE",
            null,
            null,
            true
        ),
        new Candidate(
            ".save",
            ".save ?OPTIONS? FILE",
            null,
            "Write database to FILE (an alias for .backup ...)",
            null,
            null,
            true
        ),
        new Candidate(
            ".scanstats",
            ".scanstats on|off|est",
            null,
            "Turn sqlite3_stmt_scanstatus() metrics on or off",
            null,
            null,
            true
        ),
        new Candidate(
            ".schema",
            ".schema ?PATTERN?",
            null,
            "Show the CREATE statements matching PATTERN",
            null,
            null,
            true
        ),
        new Candidate(
            ".separator",
            ".separator COL ?ROW?",
            null,
            "Change the column and row separators",
            null,
            null,
            true
        ),
        new Candidate(
            ".session",
            ".session ?NAME? CMD ...",
            null,
            "Create or control sessions",
            null,
            null,
            true
        ),
        new Candidate(
            ".sha3sum",
            ".sha3sum ...",
            null,
            "Compute a SHA3 hash of database content",
            null,
            null,
            true
        ),
        new Candidate(
            ".shell",
            ".shell CMD ARGS ...",
            null,
            "Run CMD ARGS... in a system shell",
            null,
            null,
            true
        ),
        new Candidate(
            ".show",
            ".show ",
            null,
            "Show the current values for various settings",
            null,
            null,
            true
        ),
        new Candidate(
            ".stats",
            ".stats ?ARG?",
            null,
            "Show stats or turn stats on or off",
            null,
            null,
            true
        ),
        new Candidate(
            ".system",
            ".system CMD ARGS ...",
            null,
            "Run CMD ARGS... in a system shell",
            null,
            null,
            true
        ),
        new Candidate(
            ".tables",
            ".tables ?TABLE?",
            null,
            "List names of tables matching LIKE pattern TABLE",
            null,
            null,
            true
        ),
        new Candidate(
            ".timeout",
            ".timeout MS",
            null,
            "Try opening locked tables for MS milliseconds",
            null,
            null,
            true
        ),
        new Candidate(
            ".timer",
            ".timer on|off",
            null,
            "Turn SQL timer on or off",
            null,
            null,
            true
        ),
        new Candidate(
            ".trace",
            ".trace ?OPTIONS?",
            null,
            "Output each SQL statement as it is run",
            null,
            null,
            true
        ),
        new Candidate(
            ".unmodule",
            ".unmodule NAME ...",
            null,
            "Unregister virtual table modules",
            null,
            null,
            true
        ),
        new Candidate(
            ".version",
            ".version",
            null,
            "Show source, library and compiler versions",
            null,
            null,
            true
        ),
        new Candidate(
            ".vfsinfo",
            ".vfsinfo ?AUX?",
            null,
            "Information about the top-level VFS",
            null,
            null,
            true
        ),
        new Candidate(
            ".vfslist",
            ".vfslist",
            null,
            "List all available VFSes",
            null,
            null,
            true
        ),
        new Candidate(
            ".vfsname",
            ".vfsname ?AUX?",
            null,
            "Print the name of the VFS stack",
            null,
            null,
            true
        ),
        new Candidate(
            ".width",
            ".width NUM1 NUM2 ...",
            null,
            "Set minimum column widths for columnar output",
            null,
            null,
            true
        ),
        new Candidate(
            ".www",
            ".www",
            null,
            "Display output of the next command in web browser",
            null,
            null,
            true
        )
    );

    public void handleDotCommands(Connection dbConn, String query)
        throws Exception {
        String[] splitted = query.split(" ");
        String dotCommand = splitted[0];
        String[] dotCommandArgs = Arrays.copyOfRange(
            splitted,
            1,
            splitted.length
        );
        switch (dotCommand) {
            case ".archive" -> {}
            case ".auth" -> {}
            case ".bail" -> {}
            case ".cd" -> {}
            case ".changes" -> {}
            case ".check" -> {}
            case ".clone" -> {}
            case ".connection" -> {}
            case ".crlf" -> {}
            case ".databases" -> dotDatabases();
            case ".dbconfig" -> {}
            case ".dbinfo" -> {}
            case ".dbtotxt" -> {}
            case ".dump" -> {}
            case ".echo" -> {}
            case ".eqp" -> {}
            case ".excel" -> {}
            case ".expert" -> {}
            case ".explain" -> {}
            case ".exit" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotExit(dotCommandArgs);
            }
            case ".filectrl" -> {}
            case ".fullschema" -> {}
            case ".headers" -> {}
            case ".help" -> dotHelp();
            case ".import" -> {}
            case ".imposter" -> {}
            case ".indexes" -> {}
            case ".intck" -> {}
            case ".limit" -> {}
            case ".lint" -> {}
            case ".load" -> {}
            case ".log" -> {}
            case ".mode" -> {}
            case ".nonce" -> {}
            case ".nullvalue" -> {}
            case ".once" -> {}
            case ".output" -> {}
            case ".parameter" -> {}
            case ".print" -> {}
            case ".progress" -> {}
            case ".prompt" -> {}
            case ".quit" -> dotQuit();
            case ".read" -> {}
            case ".recover" -> {}
            case ".restore" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotRestore(dotCommandArgs);
            }
            case ".save", ".backup" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotSaveOrBackup(dotCommandArgs);
            }
            case ".scanstats" -> {}
            case ".schema" -> dotSchema();
            case ".separator" -> {}
            case ".session" -> {}
            case ".sha3sum" -> {}
            case ".shell" -> {}
            case ".show" -> {}
            case ".stats" -> {}
            case ".system" -> {}
            case ".tables" -> dotTables();
            case ".timeout" -> {}
            case ".timer" -> {}
            case ".trace" -> {}
            case ".unmodule" -> {}
            case ".version" -> dotVersion();
            case ".vfsinfo" -> {}
            case ".vfslist" -> {}
            case ".vfsname" -> {}
            case ".width" -> {}
            case ".www" -> {}
            default -> {
                terminal
                    .writer()
                    .println(
                        "Error: unknown command or invalid arguments:\t" +
                            " \" " +
                            query +
                            " \". " +
                            "Enter \".help'\" for help"
                    );
            }
        }
    }

    public void dotRestore(String[] args) throws Exception {
        String path = args[0];
        terminal.writer().println("restoring database using " + path);
        stmt.execute("restore from " + path);
        terminal.writer().println("successfully restore database " + path);
        terminal.flush();
    }

    public void dotSaveOrBackup(String[] args) throws Exception {
        String path = args[0];
        terminal.writer().println("saving database to " + path);
        stmt.execute("backup to " + path);
        terminal.writer().println("saving to " + path);
        terminal.flush();
    }

    public static Connection dotOpen(String query) throws Exception {
        String[] splitted = query.split(" ");
        String[] args = Arrays.copyOfRange(splitted, 1, splitted.length);
        return DriverManager.getConnection("jdbc:sqlite:" + args[0]);
    }

    public void dotVersion() throws Exception {
        OutputTable sqliteVersion = new OutputTable(
            Format.CENTER,
            "version",
            "date",
            "timestamp",
            "hash"
        );

        ResultSet resultVersion = stmt.executeQuery("SELECT sqlite_version()");

        String semver = "";
        while (resultVersion.next()) {
            semver = resultVersion.getString(1);
        }

        ResultSet resultSourceId = stmt.executeQuery(
            "SELECT sqlite_source_id()"
        );

        while (resultSourceId.next()) {
            String[] fullSourceId = resultSourceId.getString(1).split(" ");
            String date = fullSourceId[0];
            String timestamp = fullSourceId[1];
            String hash = fullSourceId[2];
            sqliteVersion.addRow(semver, date, timestamp, hash);
        }

        terminal.writer().println(sqliteVersion.toString());
        terminal.flush();
    }

    public void dotDatabases() throws Exception {
        OutputTable allDatabases = new OutputTable(
            Format.CENTER,
            "seq",
            "name",
            "file"
        );

        ResultSet databasesInfo = stmt.executeQuery(
            "SELECT seq , name , file FROM pragma_database_list"
        );

        while (databasesInfo.next()) {
            String seq = databasesInfo.getString("seq");
            String databaseName = databasesInfo.getString("name");
            String file = databasesInfo.getString("file");

            allDatabases.addRow(seq, databaseName, file);
        }

        terminal.writer().println(allDatabases.toString());
        terminal.flush();
    }

    public void dotExit(String[] args) {
        int exitCode = Integer.parseInt(args[0]);
        terminal
            .writer()
            .println("Successfully exited with exit code" + exitCode);
        System.exit(exitCode);
    }

    public void dotQuit() {
        terminal.writer().println("Successfully quit!");
        System.exit(1);
    }

    public void dotTables() throws Exception {
        OutputTable allTables = new OutputTable(
            Format.CENTER,
            Arrays.asList("tables")
        );

        ResultSet tables = stmt.executeQuery(
            "SELECT name FROM sqlite_schema WHERE type in ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY 1"
        );

        while (tables.next()) {
            String tableName = tables.getString("name");
            allTables.addRow(tableName);
        }

        terminal.writer().println(allTables.toString());
        terminal.flush();
    }

    public void dotSchema() throws Exception {
        OutputTable allSchemas = new OutputTable(
            Format.RIGHT,
            Arrays.asList("schemas")
        );

        ResultSet tables = stmt.executeQuery(
            "SELECT sql FROM sqlite_schema ORDER BY tbl_name, type DESC, name"
        );

        while (tables.next()) {
            String tableName = tables.getString("sql");
            String fmtted = tableName.replaceAll("\\s*\\R\\s*", " ").trim();
            allSchemas.addRow(fmtted);
        }

        terminal.writer().println(allSchemas.toString());
        terminal.flush();
    }

    public void dotHelp() {
        OutputTable helpTable = new OutputTable(
            Format.RIGHT,
            Arrays.asList("Dot commands", "Arguments", "Description")
        );

        candidates.forEach(candidate -> {
            String display = candidate.displ();
            String description = candidate.descr();
            int firstSpace = display.indexOf(" ");
            if (firstSpace == -1) {
                helpTable.addRow(display, "None", description);
            } else {
                String command = display.substring(0, firstSpace);
                String arguments = display.substring(firstSpace + 1);

                helpTable.addRow(command, arguments, description);
            }
        });

        terminal.writer().println(helpTable.toString());
        terminal.flush();
    }
}
