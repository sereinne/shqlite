package com.github.sereinne.shqlite;

import com.github.sereinne.shqlite.OutputTable.Format;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;
import org.jline.reader.LineReader;
import org.jline.reader.LineReaderBuilder;
import org.jline.terminal.Terminal;
import org.jline.terminal.TerminalBuilder;

public class Shqlite {

    Terminal terminal;
    LineReader reader;
    Connection conn;
    Statement stmt;

    public Shqlite() {
        try {
            this.conn = DriverManager.getConnection("jdbc:sqlite:");
            this.stmt = conn.createStatement();
            this.terminal = TerminalBuilder.builder()
                .system(true)
                .provider("ffm")
                .build();
            this.reader = LineReaderBuilder.builder()
                .terminal(this.terminal)
                .completer(new Completion(stmt))
                .variable("COMPLETION_MODE", "SQLKEYWORDS")
                .parser(new SQLMultilineParser())
                .build();
        } catch (Exception e) {
            System.err.println("[ERROR]: Could proceed because of");
            System.err.println(e.getMessage());
        }
    }

    public void setDbConnection(Connection newConn) {
        try {
            if (this.stmt != null && !this.stmt.isClosed()) {
                this.stmt.close();
            }
            this.conn = newConn;
            this.stmt = conn.createStatement();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        new Shqlite().start();
    }

    public void start() {
        try {
            while (true) {
                String userInput = reader.readLine("shqlite> ");
                if (userInput.startsWith(".open")) {
                    Connection newConnection = DotCommands.dotOpen(userInput);
                    this.setDbConnection(newConnection);
                } else if (userInput.startsWith(".quit")) {
                    break;
                } else if (userInput.startsWith(".")) {
                    new DotCommands(stmt, terminal).executeDotCommand(
                        userInput
                    );
                } else {
                    executeStandardQuery(userInput);
                }
            }
        } catch (Exception e) {
            System.err.println("[ERROR]: Could proceed because of");
            System.err.println(e.getMessage());
        }
    }

    public void printOutputQuery(ResultSet rs) throws Exception {
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

    public void executeStandardQuery(String query) throws Exception {
        String trimmedQuery = query.trim();
        boolean hasResultSet = stmt.execute(trimmedQuery);
        if (!hasResultSet) {
            // this must be a statement that has not output table to display
            terminal
                .writer()
                .println("Successfully executed statement " + trimmedQuery);
            terminal.flush();
        } else {
            ResultSet rs = stmt.getResultSet();
            printOutputQuery(rs);
        }
        terminal.flush();
    }
}
