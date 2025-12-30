package com.github.sereinne.shqlite;

import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.jline.reader.Highlighter;
import org.jline.reader.LineReader;
import org.jline.utils.AttributedString;
import org.jline.utils.AttributedStringBuilder;
import org.jline.utils.AttributedStyle;

public class SyntaxHighlight implements Highlighter {

    public final Pattern SQLITE_KEYWORD_REGEX_PATTERN = Pattern.compile(
        "\\b(ABORT|ACTION|ADD|AFTER|ALL|ALTER|ALWAYS|ANALYZE|AND|AS|ASC|ATTACH|AUTOINCREMENT|BEFORE|BEGIN|BETWEEN|BY|CASCADE|CASE|CAST|CHECK|COLLATE|COLUMN|COMMIT|CONFLICT|CONSTRAINT|CREATE|CROSS|CURRENT|CURRENT_DATE|CURRENT_TIME|CURRENT_TIMESTAMP|DATABASE|DEFAULT|DEFERRABLE|DEFERRED|DELETE|DESC|DETACH|DISTINCT|DO|DROP|EACH|ELSE|END|ESCAPE|EXCEPT|EXCLUDE|EXCLUSIVE|EXISTS|EXPLAIN|FAIL|FILTER|FIRST|FOLLOWING|FOR|FOREIGN|FROM|FULL|GENERATED|GLOB|GROUP|GROUPS|HAVING|IF|IGNORE|IMMEDIATE|IN|INDEX|INDEXED|INITIALLY|INNER|INSERT|INSTEAD|INTERSECT|INTO|IS|ISNULL|JOIN|KEY|LAST|LEFT|LIKE|LIMIT|MATCH|MATERIALIZED|NATURAL|NO|NOT|NOTHING|NOTNULL|NULL|NULLS|OF|OFFSET|ON|OR|ORDER|OTHERS|OUTER|OVER|PARTITION|PLAN|PRAGMA|PRECEDING|PRIMARY|QUERY|RAISE|RANGE|RECURSIVE|REFERENCES|REGEXP|REINDEX|RELEASE|RENAME|REPLACE|RESTRICT|RETURNING|RIGHT|ROLLBACK|ROW|ROWS|SAVEPOINT|SELECT|SET|TABLE|TEMP|TEMPORARY|THEN|TIES|TO|TRANSACTION|TRIGGER|UNBOUNDED|UNION|UNIQUE|UPDATE|USING|VACUUM|VALUES|VIEW|VIRTUAL|WHEN|WHERE|WINDOW|WITH|WITHOUT)"
    );

    @Override
    public AttributedString highlight(LineReader reader, String buffer) {
        AttributedStringBuilder builder = new AttributedStringBuilder();

        // Find all SQL keywords in the buffer
        Matcher matcher = SQLITE_KEYWORD_REGEX_PATTERN.matcher(buffer);
        int lastEnd = 0;

        while (matcher.find()) {
            // Add text before the keyword with default style
            builder.styled(
                AttributedStyle.DEFAULT.foreground(250, 189, 47),
                buffer.substring(lastEnd, matcher.start())
            );

            // Add the keyword with gruvbox red style
            builder.styled(
                AttributedStyle.DEFAULT.foreground(204, 36, 29),
                buffer.substring(matcher.start(), matcher.end())
            );

            lastEnd = matcher.end();
        }

        // Add any remaining text
        if (lastEnd < buffer.length()) {
            builder.append(buffer.substring(lastEnd));
        }

        return builder.toAttributedString();
    }
}
