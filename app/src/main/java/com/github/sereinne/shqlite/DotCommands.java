package com.github.sereinne.shqlite;

import com.github.sereinne.shqlite.OutputTable.Format;
import java.io.BufferedReader;
import java.io.File;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.Arrays;
import java.util.Scanner;
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

    private void unimplemented(String dotCommand) {
        terminal
            .writer()
            .println(
                "TODO: This command " + dotCommand + "is not implemented!"
            );
        terminal.flush();
    }

    public DotCommands(Statement stmt, Terminal terminal) {
        this.terminal = terminal;
        this.stmt = stmt;
    }

    public void executeDotCommand(String query) throws Exception {
        String[] splitted = query.split(" ");
        String dotCommand = splitted[0];
        String[] dotCommandArgs = Arrays.copyOfRange(
            splitted,
            1,
            splitted.length
        );
        switch (dotCommand) {
            case ".archive" -> unimplemented(dotCommand);
            case ".auth" -> unimplemented(dotCommand);
            case ".bail" -> unimplemented(dotCommand);
            case ".cd" -> unimplemented(dotCommand);
            case ".changes" -> unimplemented(dotCommand);
            case ".check" -> unimplemented(dotCommand);
            case ".clone" -> unimplemented(dotCommand);
            case ".connection" -> unimplemented(dotCommand);
            case ".crlf" -> unimplemented(dotCommand);
            case ".databases" -> dotDatabases();
            case ".dbconfig" -> dotDBConfig();
            case ".dbinfo" -> unimplemented(dotCommand);
            case ".dbtotxt" -> unimplemented(dotCommand);
            case ".dump" -> unimplemented(dotCommand);
            case ".echo" -> unimplemented(dotCommand);
            case ".eqp" -> unimplemented(dotCommand);
            case ".excel" -> unimplemented(dotCommand);
            case ".expert" -> unimplemented(dotCommand);
            case ".explain" -> unimplemented(dotCommand);
            case ".exit" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotExit(dotCommandArgs);
            }
            case ".filectrl" -> unimplemented(dotCommand);
            case ".fullschema" -> unimplemented(dotCommand);
            case ".headers" -> unimplemented(dotCommand);
            case ".help" -> dotHelp();
            case ".import" -> unimplemented(dotCommand);
            case ".imposter" -> unimplemented(dotCommand);
            case ".indexes" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotIndexes(dotCommandArgs);
            }
            case ".intck" -> unimplemented(dotCommand);
            case ".limit" -> unimplemented(dotCommand);
            case ".lint" -> unimplemented(dotCommand);
            case ".load" -> unimplemented(dotCommand);
            case ".log" -> unimplemented(dotCommand);
            case ".mode" -> unimplemented(dotCommand);
            case ".nonce" -> unimplemented(dotCommand);
            case ".nullvalue" -> unimplemented(dotCommand);
            case ".once" -> unimplemented(dotCommand);
            case ".output" -> unimplemented(dotCommand);
            case ".parameter" -> unimplemented(dotCommand);
            case ".print" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotPrint(dotCommandArgs);
            }
            case ".progress" -> unimplemented(dotCommand);
            case ".prompt" -> unimplemented(dotCommand);
            case ".quit" -> dotQuit();
            case ".read" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotRead(dotCommandArgs);
            }
            case ".recover" -> unimplemented(dotCommand);
            case ".restore" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotRestore(dotCommandArgs);
            }
            case ".save", ".backup" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotSaveOrBackup(dotCommandArgs);
            }
            case ".scanstats" -> unimplemented(dotCommand);
            case ".schema" -> dotSchema();
            case ".separator" -> unimplemented(dotCommand);
            case ".session" -> unimplemented(dotCommand);
            case ".sha3sum" -> unimplemented(dotCommand);
            case ".shell", ".system" -> {
                if (!assertRequiredArgument(dotCommand, dotCommandArgs)) return;
                dotRunShell(dotCommandArgs);
            }
            case ".show" -> unimplemented(dotCommand);
            case ".stats" -> unimplemented(dotCommand);
            case ".tables" -> dotTables();
            case ".timeout" -> unimplemented(dotCommand);
            case ".timer" -> unimplemented(dotCommand);
            case ".trace" -> unimplemented(dotCommand);
            case ".unmodule" -> unimplemented(dotCommand);
            case ".version" -> dotVersion();
            case ".vfsinfo" -> unimplemented(dotCommand);
            case ".vfslist" -> unimplemented(dotCommand);
            case ".vfsname" -> unimplemented(dotCommand);
            case ".width" -> unimplemented(dotCommand);
            case ".www" -> unimplemented(dotCommand);
            default -> {
                terminal
                    .writer()
                    .println(
                        "Error: unknown command or invalid arguments: " +
                            "\"" +
                            query +
                            "\". " +
                            "Enter \".help\" for help"
                    );
            }
        }
    }

    public void dotDBConfig() throws Exception {
        OutputTable dbconfig = new OutputTable(
            Format.CENTER,
            "pragma",
            "value"
        );

        String[] dbconfigPragmas = {
            "attach_create",
            "attach_write",
            "comments",
            "defensive",
            "dqs_ddl",
            "dqs_dml",
            "enable_fkey",
            "enable_qpsg",
            "enable_trigger",
            "enable_view",
            "fts3_tokenizer",
            "legacy_alter_table",
            "legacy_file_format",
            "load_extension",
            "no_ckpt_on_close",
            "reset_database",
            "reverse_scanorder",
            "stmt_scanstatus",
            "trigger_eqp",
            "trusted_schema",
            "writable_schema",
        };

        for (String pragma : dbconfigPragmas) {
            String res = getPragmaValue(pragma);
            dbconfig.addRow(pragma, res);
        }

        terminal.writer().println(dbconfig.toString());
        terminal.flush();
    }

    private String getPragmaValue(String pragma) {
        try (ResultSet rs = stmt.executeQuery("PRAGMA " + pragma)) {
            if (rs.next()) {
                int intValue = rs.getInt(1);
                if (rs.wasNull()) {
                    return "NULL";
                }
                return intValue == 0 ? "off" : "on";
            }
            // No rows returned
            return "N/A";
        } catch (SQLException e) {
            // PRAGMA not supported or error
            return "N/A";
        }
    }

    public void dotIndexes(String[] args) throws Exception {
        OutputTable allIndexes = new OutputTable(
            Format.CENTER,
            Arrays.asList("indexes")
        );

        ResultSet indexes = stmt.executeQuery(
            "SELECT name FROM sqlite_schema WHERE type  AND name NOT LIKE 'sqlite_%' ORDER BY 1"
        );

        while (indexes.next()) {
            String tableName = indexes.getString("name");
            allIndexes.addRow(tableName);
        }

        terminal.writer().println(allIndexes.toString());
        terminal.flush();
    }

    public void dotRunShell(String[] args) throws Exception {
        // redicret stderr to stdout
        Process proc = new ProcessBuilder(args)
            .redirectErrorStream(true)
            .start();
        InputStream outputStream = proc.getInputStream();
        InputStreamReader streamReader = new InputStreamReader(outputStream);
        BufferedReader bufread = new BufferedReader(streamReader);

        bufread.lines().forEach(line -> terminal.writer().println(line));
        int exitCode = proc.waitFor();
        terminal
            .writer()
            .println("process terminated with exit code " + exitCode);
    }

    public void dotPrint(String[] args) {
        String joined = String.join(" ", args);
        terminal.writer().println(joined);
        terminal.flush();
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

    public void dotRead(String[] args) throws Exception {
        String filepath = args[0];
        Scanner sc = new Scanner(new File(filepath));
        sc.useDelimiter(";");

        while (sc.hasNext()) {
            String sqlStatement = sc.next().trim();
            if (!sqlStatement.isEmpty() && !sqlStatement.isBlank()) {
                stmt.addBatch(sqlStatement);
                terminal
                    .writer()
                    .println("successfully added statement into batch");
                terminal.flush();
            }
        }

        stmt.executeBatch();
        terminal.writer().println("successfully executed all batch");

        sc.close();
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

        Completion.DOT_COMMANDS.forEach(candidate -> {
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
