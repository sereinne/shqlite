package com.github.sereinne.dbcli;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class OutputTable {

    final int PADDING = 2;
    List<String> columns = new ArrayList<>();
    List<List<String>> rows = new ArrayList<>();

    private static String centerString(String str, int width, String padstr) {
        if (str == null || width < str.length()) {
            return str; // Or handle the error as needed
        }

        int padding = width - str.length();
        int leftPadding = padding / 2;
        int rightPadding = padding - leftPadding;

        // Create the left padding string
        String left = padstr.repeat(leftPadding);

        // Create the right padding string
        String right = padstr.repeat(rightPadding);

        return left + str + right;
    }

    public OutputTable(String... columns) {
        this.columns.addAll(Arrays.asList(columns));
    }

    public void addRow(String... row) {
        this.rows.add(Arrays.asList(row));
    }

    // given a `column` that has n number of rows, return the longest row of that `column`
    public int getMaxColumnLength(int column) {
        int current = columns.get(column).length();
        for (List<String> row : rows) {
            int len = row.get(column).length();
            if (len > current) {
                current = len;
            }
        }
        return current;
    }

    public String formatColumnStart() {
        StringBuilder sb = new StringBuilder();
        sb.append("╭");
        int columnsLength = columns.size();
        for (int i = 0; i < columnsLength; i++) {
            for (int j = 0; j < getMaxColumnLength(i) + PADDING; j++) {
                sb.append("─");
            }
            if (i == columnsLength - 1) {
                sb.append("╮");
            } else {
                sb.append("┬");
            }
        }
        sb.append("\n");
        return sb.toString();
    }

    public String formatColumnEnd() {
        StringBuilder sb = new StringBuilder();
        sb.append("╰");
        int columnsLength = columns.size();
        for (int i = 0; i < columnsLength; i++) {
            for (int j = 0; j < getMaxColumnLength(i) + PADDING; j++) {
                sb.append("─");
            }
            if (i == columnsLength - 1) {
                sb.append("╯");
            } else {
                sb.append("┴");
            }
        }
        // sb.append("╮");
        sb.append("\n");
        return sb.toString();
    }

    public String formatRow(List<String> row) {
        StringBuilder sb = new StringBuilder();
        sb.append("│");
        for (int i = 0; i < row.size(); i++) {
            sb.append(
                centerString(row.get(i), getMaxColumnLength(i) + PADDING, " ")
            );
            sb.append("│");
        }
        sb.append("\n");
        return sb.toString();
    }

    public String stripes(List<String> row) {
        StringBuilder sb = new StringBuilder();
        sb.append("├");
        int rowLength = row.size();
        for (int i = 0; i < rowLength; i++) {
            sb.append(centerString("─", getMaxColumnLength(i) + PADDING, "─"));

            if (i == rowLength - 1) {
                sb.append("┤");
            } else {
                sb.append("┼");
            }
        }
        sb.append("\n");
        return sb.toString();
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(formatColumnStart());
        sb.append(formatRow(columns));
        sb.append(stripes(columns));
        for (List<String> row : rows) {
            sb.append(formatRow(row));
        }
        sb.append(formatColumnEnd());
        return sb.toString();
    }
}
