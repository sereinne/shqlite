package com.github.sereinne.shqlite;

import java.sql.ResultSet;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import org.jline.reader.Candidate;
import org.jline.reader.Completer;
import org.jline.reader.LineReader;
import org.jline.reader.ParsedLine;
import org.jline.reader.impl.completer.StringsCompleter;

public class Completion implements Completer {

    public static final List<Candidate> SQLITE_KEYWORDS = Arrays.asList(
        new Candidate("ABORT"),
        new Candidate("ACTION"),
        new Candidate("ADD"),
        new Candidate("AFTER"),
        new Candidate("ALL"),
        new Candidate("ALTER"),
        new Candidate("ALWAYS"),
        new Candidate("ANALYZE"),
        new Candidate("AND"),
        new Candidate("AS"),
        new Candidate("ASC"),
        new Candidate("ATTACH"),
        new Candidate("AUTOINCREMENT"),
        new Candidate("BEFORE"),
        new Candidate("BEGIN"),
        new Candidate("BETWEEN"),
        new Candidate("BY"),
        new Candidate("CASCADE"),
        new Candidate("CASE"),
        new Candidate("CAST"),
        new Candidate("CHECK"),
        new Candidate("COLLATE"),
        new Candidate("COLUMN"),
        new Candidate("COMMIT"),
        new Candidate("CONFLICT"),
        new Candidate("CONSTRAINT"),
        new Candidate("CREATE"),
        new Candidate("CROSS"),
        new Candidate("CURRENT"),
        new Candidate("CURRENT_DATE"),
        new Candidate("CURRENT_TIME"),
        new Candidate("CURRENT_TIMESTAMP"),
        new Candidate("DATABASE"),
        new Candidate("DEFAULT"),
        new Candidate("DEFERRABLE"),
        new Candidate("DEFERRED"),
        new Candidate("DELETE"),
        new Candidate("DESC"),
        new Candidate("DETACH"),
        new Candidate("DISTINCT"),
        new Candidate("DO"),
        new Candidate("DROP"),
        new Candidate("EACH"),
        new Candidate("ELSE"),
        new Candidate("END"),
        new Candidate("ESCAPE"),
        new Candidate("EXCEPT"),
        new Candidate("EXCLUDE"),
        new Candidate("EXCLUSIVE"),
        new Candidate("EXISTS"),
        new Candidate("EXPLAIN"),
        new Candidate("FAIL"),
        new Candidate("FILTER"),
        new Candidate("FIRST"),
        new Candidate("FOLLOWING"),
        new Candidate("FOR"),
        new Candidate("FOREIGN"),
        new Candidate("FROM"),
        new Candidate("FULL"),
        new Candidate("GENERATED"),
        new Candidate("GLOB"),
        new Candidate("GROUP"),
        new Candidate("GROUPS"),
        new Candidate("HAVING"),
        new Candidate("IF"),
        new Candidate("IGNORE"),
        new Candidate("IMMEDIATE"),
        new Candidate("IN"),
        new Candidate("INDEX"),
        new Candidate("INDEXED"),
        new Candidate("INITIALLY"),
        new Candidate("INNER"),
        new Candidate("INSERT"),
        new Candidate("INSTEAD"),
        new Candidate("INTERSECT"),
        new Candidate("INTO"),
        new Candidate("IS"),
        new Candidate("ISNULL"),
        new Candidate("JOIN"),
        new Candidate("KEY"),
        new Candidate("LAST"),
        new Candidate("LEFT"),
        new Candidate("LIKE"),
        new Candidate("LIMIT"),
        new Candidate("MATCH"),
        new Candidate("MATERIALIZED"),
        new Candidate("NATURAL"),
        new Candidate("NO"),
        new Candidate("NOT"),
        new Candidate("NOTHING"),
        new Candidate("NOTNULL"),
        new Candidate("NULL"),
        new Candidate("NULLS"),
        new Candidate("OF"),
        new Candidate("OFFSET"),
        new Candidate("ON"),
        new Candidate("OR"),
        new Candidate("ORDER"),
        new Candidate("OTHERS"),
        new Candidate("OUTER"),
        new Candidate("OVER"),
        new Candidate("PARTITION"),
        new Candidate("PLAN"),
        new Candidate("PRAGMA"),
        new Candidate("PRECEDING"),
        new Candidate("PRIMARY"),
        new Candidate("QUERY"),
        new Candidate("RAISE"),
        new Candidate("RANGE"),
        new Candidate("RECURSIVE"),
        new Candidate("REFERENCES"),
        new Candidate("REGEXP"),
        new Candidate("REINDEX"),
        new Candidate("RELEASE"),
        new Candidate("RENAME"),
        new Candidate("REPLACE"),
        new Candidate("RESTRICT"),
        new Candidate("RETURNING"),
        new Candidate("RIGHT"),
        new Candidate("ROLLBACK"),
        new Candidate("ROW"),
        new Candidate("ROWS"),
        new Candidate("SAVEPOINT"),
        new Candidate("SELECT"),
        new Candidate("SET"),
        new Candidate("TABLE"),
        new Candidate("TEMP"),
        new Candidate("TEMPORARY"),
        new Candidate("THEN"),
        new Candidate("TIES"),
        new Candidate("TO"),
        new Candidate("TRANSACTION"),
        new Candidate("TRIGGER"),
        new Candidate("UNBOUNDED"),
        new Candidate("UNION"),
        new Candidate("UNIQUE"),
        new Candidate("UPDATE"),
        new Candidate("USING"),
        new Candidate("VACUUM"),
        new Candidate("VALUES"),
        new Candidate("VIEW"),
        new Candidate("VIRTUAL"),
        new Candidate("WHEN"),
        new Candidate("WHERE"),
        new Candidate("WINDOW"),
        new Candidate("WITH"),
        new Candidate("WITHOUT")
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

    Map<String, Completer> completers = new HashMap<>();
    Statement stmt;

    public Completion(Statement stmt) {
        this.stmt = stmt;
        this.completers.put(
            "SQLITE_KEYWORDS",
            new StringsCompleter(SQLITE_KEYWORDS)
        );
        this.completers.put("DOT_COMMANDS", new StringsCompleter(DOT_COMMANDS));
        this.completers.put("TABLES", new StringsCompleter(""));
    }

    public List<Candidate> getTables() {
        List<Candidate> tableNames = new ArrayList<>();
        try {
            ResultSet tables = stmt.executeQuery(
                "SELECT name FROM sqlite_schema WHERE type in ('table', 'view') AND name NOT LIKE 'sqlite_%' ORDER BY 1"
            );

            while (tables.next()) {
                String tableName = tables.getString("name");
                tableNames.add(new Candidate(tableName));
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return tableNames;
    }

    @Override
    public void complete(
        LineReader reader,
        ParsedLine line,
        List<Candidate> candidates
    ) {
        String rest = line.line();
        String word = line.word();

        String mode = (String) reader.getVariable("COMPLETION_MODE");
        if (word.startsWith(".")) {
            mode = "DOT_COMMANDS";
        }

        if (isLastWord(rest, "SELECT")) {
            candidates.add(new Candidate("*"));
        }

        if (
            isLastWord(rest, "FROM") ||
            isLastWord(rest, "JOIN") ||
            isLastWord(rest, "INTO")
        ) {
            Completer tables = new StringsCompleter(getTables());
            this.completers.put("TABLES", tables);
            mode = "TABLES";
        }

        Completer completer = this.completers.get(mode);
        completer.complete(reader, line, candidates);
    }

    private boolean isLastWord(String buffer, String word) {
        String[] split = buffer.split("\\s+");
        String last = split[split.length - 1];
        if (last.equals(word)) {
            return true;
        }
        return false;
    }
}
