package com.github.sereinne.shqlite;

import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collector;
import java.util.stream.Collectors;
import org.jline.reader.Candidate;
import org.jline.reader.Completer;
import org.jline.reader.LineReader;
import org.jline.reader.ParsedLine;
import org.jline.reader.impl.completer.StringsCompleter;

public class Completion implements Completer {

    public static final List<String> SQLITE_KEYWORDS = Arrays.asList(
        "ABORT",
        "ACTION",
        "ADD",
        "AFTER",
        "ALL",
        "ALTER",
        "ALWAYS",
        "ANALYZE",
        "AND",
        "AS",
        "ASC",
        "ATTACH",
        "AUTOINCREMENT",
        "BEFORE",
        "BEGIN",
        "BETWEEN",
        "BY",
        "CASCADE",
        "CASE",
        "CAST",
        "CHECK",
        "COLLATE",
        "COLUMN",
        "COMMIT",
        "CONFLICT",
        "CONSTRAINT",
        "CREATE",
        "CROSS",
        "CURRENT",
        "CURRENT_DATE",
        "CURRENT_TIME",
        "CURRENT_TIMESTAMP",
        "DATABASE",
        "DEFAULT",
        "DEFERRABLE",
        "DEFERRED",
        "DELETE",
        "DESC",
        "DETACH",
        "DISTINCT",
        "DO",
        "DROP",
        "EACH",
        "ELSE",
        "END",
        "ESCAPE",
        "EXCEPT",
        "EXCLUDE",
        "EXCLUSIVE",
        "EXISTS",
        "EXPLAIN",
        "FAIL",
        "FILTER",
        "FIRST",
        "FOLLOWING",
        "FOR",
        "FOREIGN",
        "FROM",
        "FULL",
        "GENERATED",
        "GLOB",
        "GROUP",
        "GROUPS",
        "HAVING",
        "IF",
        "IGNORE",
        "IMMEDIATE",
        "IN",
        "INDEX",
        "INDEXED",
        "INITIALLY",
        "INNER",
        "INSERT",
        "INSTEAD",
        "INTERSECT",
        "INTO",
        "IS",
        "ISNULL",
        "JOIN",
        "KEY",
        "LAST",
        "LEFT",
        "LIKE",
        "LIMIT",
        "MATCH",
        "MATERIALIZED",
        "NATURAL",
        "NO",
        "NOT",
        "NOTHING",
        "NOTNULL",
        "NULL",
        "NULLS",
        "OF",
        "OFFSET",
        "ON",
        "OR",
        "ORDER",
        "OTHERS",
        "OUTER",
        "OVER",
        "PARTITION",
        "PLAN",
        "PRAGMA",
        "PRECEDING",
        "PRIMARY",
        "QUERY",
        "RAISE",
        "RANGE",
        "RECURSIVE",
        "REFERENCES",
        "REGEXP",
        "REINDEX",
        "RELEASE",
        "RENAME",
        "REPLACE",
        "RESTRICT",
        "RETURNING",
        "RIGHT",
        "ROLLBACK",
        "ROW",
        "ROWS",
        "SAVEPOINT",
        "SELECT",
        "SET",
        "TABLE",
        "TEMP",
        "TEMPORARY",
        "THEN",
        "TIES",
        "TO",
        "TRANSACTION",
        "TRIGGER",
        "UNBOUNDED",
        "UNION",
        "UNIQUE",
        "UPDATE",
        "USING",
        "VACUUM",
        "VALUES",
        "VIEW",
        "VIRTUAL",
        "WHEN",
        "WHERE",
        "WINDOW",
        "WITH",
        "WITHOUT"
    );

    // All possible dot commands
    public static final List<Candidate> DOT_COMMANDS = Arrays.asList(
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

    public static final List<String> STR_DOT_COMMANDS = DOT_COMMANDS.stream()
        .map(e -> e.value())
        .collect(Collectors.toList());

    Map<String, List<String>> candidatePair = new HashMap<>();
    Statement stmt;

    public Completion(Statement stmt) {
        this.stmt = stmt;
        this.candidatePair.put("SQLKEYWORDS", SQLITE_KEYWORDS);
        this.candidatePair.put("DOTCOMMANDS", STR_DOT_COMMANDS);
        this.candidatePair.put("TABLES", Arrays.asList());
        this.candidatePair.put("COLUMNS", Arrays.asList());
    }

    public List<String> getTableNames() {
        List<String> tableNames = new ArrayList<>();
        try {
            ResultSet tables = stmt.executeQuery(
                "SELECT name FROM sqlite_schema WHERE type in ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY 1"
            );

            while (tables.next()) {
                String tableName = tables.getString("name");
                tableNames.add(tableName);
            }
        } catch (Exception e) {
            System.out.println("no tables available");
        }
        return tableNames;
    }

    public List<String> getAllColumns() {
        List<String> columnNames = new ArrayList<>();
        List<String> tableNames = this.candidatePair.get("TABLES");
        columnNames.add("*");
        try {
            for (String tableName : tableNames) {
                ResultSet rs = stmt.executeQuery("SELECT * FROM " + tableName);
                ResultSetMetaData metadata = rs.getMetaData();
                int cols = metadata.getColumnCount();

                for (int i = 1; i <= cols; i++) {
                    String columnName = metadata.getColumnName(i);
                    columnNames.add(columnName);
                }
            }
        } catch (Exception e) {
            System.out.println("no columns available");
        }
        return columnNames;
    }

    @Override
    public void complete(
        LineReader reader,
        ParsedLine line,
        List<Candidate> candidates
    ) {
        String unparsedLine = line.line();
        String word = line.word();
        String mode = (String) reader.getVariable("COMPLETION_MODE");
        if (word.startsWith(".")) {
            mode = "DOTCOMMANDS";
        }

        if (isAfterWord(unparsedLine, "SELECT")) {
            this.candidatePair.put("COLUMNS", getAllColumns());
            mode = "COLUMNS";
        }

        if (
            isAfterWord(unparsedLine, "FROM") ||
            isAfterWord(unparsedLine, "INTO") ||
            isAfterWord(unparsedLine, "JOIN")
        ) {
            this.candidatePair.put("TABLES", getTableNames());
            mode = "TABLES";
        }

        List<String> completionCandidates = this.candidatePair.get(mode);
        Completer sc = new StringsCompleter(completionCandidates);
        sc.complete(reader, line, candidates);
    }

    private boolean isAfterWord(String unparsedLine, String word) {
        String[] splitted = unparsedLine.split("\\s+");
        String last = splitted[splitted.length - 1];
        if (last.equals(word)) {
            return true;
        }
        return false;
    }
}
