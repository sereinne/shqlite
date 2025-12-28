package com.github.sereinne.shqlite;

import com.github.sereinne.shqlite.OutputTable.Format;
import java.lang.Runnable;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;
import org.jline.reader.Completer;
import org.jline.reader.LineReader;
import org.jline.reader.LineReaderBuilder;
import org.jline.terminal.Terminal;
import org.jline.terminal.TerminalBuilder;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Parameters;

@Command(
    name = "shqlite",
    version = "shqlite 1.0.0",
    mixinStandardHelpOptions = true,
    description = "terminal `sqlite` client in Java"
)
public class Shqlite implements Runnable {

    @Parameters(
        index = "0",
        description = "Database file to connect to",
        defaultValue = ""
    )
    String path;

    Connection dbConn;
    Terminal terminal;
    LineReader reader;

    public static void main(String[] args) {
        new CommandLine(new Shqlite()).execute(args);
    }

    @Override
    public void run() {
        Completer completer = DbCompletion.getDotAutoComplete();
        Statement stmt = null;
        try {
            this.dbConn = DriverManager.getConnection("jdbc:sqlite:" + path);
            this.terminal = TerminalBuilder.builder()
                .system(true)
                .provider("ffm")
                .dumb(false)
                .build();
            this.reader = LineReaderBuilder.builder()
                .terminal(this.terminal)
                // make this `LineReader` parse multiline sql query
                .parser(new SQLMultilineParser())
                .completer(completer)
                .build();

            stmt = dbConn.createStatement();

            while (true) {
                String query = reader.readLine("dbcli> ");

                try {
                    if (query.startsWith(".open")) {
                        dbConn.close();
                        this.dbConn = DotCommands.dotOpen(query);
                        stmt = dbConn.createStatement();
                    } else if (query.startsWith(".")) {
                        new DotCommands(stmt, terminal).handleDotCommands(
                            dbConn,
                            query
                        );
                    } else {
                        String oneLined = query.replace("\n", " ");
                        runDynamicQuery(terminal, stmt, oneLined);
                    }
                } catch (SQLException sqle) {
                    sqle.printStackTrace();
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
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

        OutputTable table = new OutputTable(Format.RIGHT, columns);

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
}
