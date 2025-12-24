package com.github.sereinne.dbcli;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;
import org.jline.reader.Completer;
import org.jline.reader.LineReader;
import org.jline.reader.LineReaderBuilder;
import org.jline.terminal.Terminal;
import org.jline.terminal.TerminalBuilder;

public class Dbcli {

    public static void handleDotCommands(Terminal terminal, String query) {
        switch (query) {
            case ".archive" -> {}
            case ".auth" -> {}
            case ".backup" -> {}
            case ".bail" -> {}
            case ".cd" -> {}
            case ".changes" -> {}
            case ".check" -> {}
            case ".clone" -> {}
            case ".connection" -> {}
            case ".crlf" -> {}
            case ".databases" -> {}
            case ".dbconfig" -> {}
            case ".dbinfo" -> {}
            case ".dbtotxt" -> {}
            case ".dump" -> {}
            case ".echo" -> {}
            case ".eqp" -> {}
            case ".excel" -> {}
            // case ".exit" -> {}
            case ".expert" -> {}
            case ".explain" -> {}
            case ".filectrl" -> {}
            case ".fullschema" -> {}
            case ".headers" -> {}
            case ".help" -> {}
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
            case ".open" -> {}
            case ".output" -> {}
            case ".parameter" -> {}
            case ".print" -> {}
            case ".progress" -> {}
            case ".prompt" -> {}
            // case ".quit" -> {}
            case ".read" -> {}
            case ".recover" -> {}
            case ".restore" -> {}
            case ".save" -> {}
            case ".scanstats" -> {}
            case ".schema" -> {}
            case ".separator" -> {}
            case ".session" -> {}
            case ".sha3sum" -> {}
            case ".shell" -> {}
            case ".show" -> {}
            case ".stats" -> {}
            case ".system" -> {}
            case ".tables" -> {}
            case ".timeout" -> {}
            case ".timer" -> {}
            case ".trace" -> {}
            case ".unmodule" -> {}
            case ".version" -> {}
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
                            "\"" +
                            query +
                            "\"." +
                            "Enter \".help'\" for help"
                    );
            }
        }
    }

    public static void printOutputQuery(Terminal terminal, ResultSet rs)
        throws Exception {
        ResultSetMetaData metadata = rs.getMetaData();
        // `cols` start at 1
        int colSize = metadata.getColumnCount();

        List<String> columns = new ArrayList<>(colSize);

        for (int i = 1; i <= colSize; i++) {
            String colname = metadata.getColumnName(i);
            columns.add(colname);
        }

        OutputTable table = new OutputTable(columns);

        while (rs.next()) {
            List<String> row = new ArrayList<>(colSize);
            for (int i = 1; i <= colSize; i++) {
                String value = rs.getString(i);
                row.add(value);
            }
            table.addRow(row);
        }

        terminal.writer().println(table.toString());
        terminal.flush();
    }

    public static void runDynamicQuery(
        Terminal terminal,
        Statement stmt,
        String query
    ) throws Exception {
        String trimmedQuery = query.trim();
        boolean hasResultSet = stmt.execute(trimmedQuery);
        if (!hasResultSet) {
            // this must be an INSERT, UPDATE or DELETE statement
            terminal
                .writer()
                .println("Successfully executed statement " + trimmedQuery);
            terminal.flush();
        } else {
            // this must be a SELECT statement
            ResultSet rs = stmt.getResultSet();
            printOutputQuery(terminal, rs);
        }
        terminal.flush();
    }

    public static void main(String[] args) {
        String path = args.length <= 0 ? "" : args[0];

        Completer completer = DbCompletion.getDotAutoComplete();

        try {
            Terminal terminal = TerminalBuilder.builder()
                .system(true)
                .provider("ffm")
                .dumb(false)
                .build();

            LineReader reader = LineReaderBuilder.builder()
                .terminal(terminal)
                .completer(completer)
                .build();

            Connection conn = DriverManager.getConnection(
                "jdbc:sqlite:" + path
            );
            Statement stmt = conn.createStatement();

            while (true) {
                String query = reader.readLine("dbcli> ");

                if (query.startsWith(".exit")) {
                    String exit = query.split(" ")[1];
                    int exitCode = Integer.parseInt(exit);
                    System.exit(exitCode);
                }

                if (query.equals(".quit")) {
                    System.exit(0);
                }

                if (query.startsWith(".")) {
                    handleDotCommands(terminal, query);
                } else {
                    runDynamicQuery(terminal, stmt, query);
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
