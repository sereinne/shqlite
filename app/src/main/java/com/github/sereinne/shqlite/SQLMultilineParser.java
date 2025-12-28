package com.github.sereinne.shqlite;

import org.jline.reader.EOFError;
import org.jline.reader.ParsedLine;
import org.jline.reader.Parser;
import org.jline.reader.SyntaxError;
import org.jline.reader.impl.DefaultParser;

// https://github.com/jline/jline3/issues/36#issuecomment-652522724
public final class SQLMultilineParser implements Parser {

    private static final Parser DEFAULT_PARSER = new DefaultParser();

    @Override
    public ParsedLine parse(
        String line,
        int cursor,
        Parser.ParseContext context
    ) throws SyntaxError {
        if (
            (Parser.ParseContext.UNSPECIFIED.equals(context) ||
                Parser.ParseContext.ACCEPT_LINE.equals(context))
        ) {
            if (line.trim().startsWith(".")) {
                return DEFAULT_PARSER.parse(line, cursor, context);
            }
            if (!line.trim().endsWith(";")) {
                throw new EOFError(-1, cursor, "Missing semicolon (;)");
            }
        }

        return DEFAULT_PARSER.parse(line, cursor, context);
    }

    public SQLMultilineParser() {}
}
