use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        // Read from file
        let filename = &args[1];
        fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("Error reading file {}: {}", filename, err);
            std::process::exit(1);
        })
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .unwrap_or_else(|err| {
                eprintln!("Error reading from stdin: {}", err);
                std::process::exit(1);
            });
        buffer
    };

    let formatted = format_markdown_sentences(&input);
    print!("{}", formatted);
}

/// Format markdown text so each sentence is on its own line
fn format_markdown_sentences(text: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;
    let mut code_fence = String::new();

    for line in text.lines() {
        // Check for code block fences
        if line.trim_start().starts_with("```") || line.trim_start().starts_with("~~~") {
            if !in_code_block {
                // Starting a code block
                in_code_block = true;
                code_fence = if line.trim_start().starts_with("```") {
                    "```".to_string()
                } else {
                    "~~~".to_string()
                };
                result.push_str(line);
                result.push('\n');
            } else if line.trim_start().starts_with(&code_fence) {
                // Ending a code block
                in_code_block = false;
                result.push_str(line);
                result.push('\n');
            } else {
                // Inside code block, different fence
                result.push_str(line);
                result.push('\n');
            }
            continue;
        }

        // Inside code block - preserve as is
        if in_code_block {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check for indented code blocks (4 spaces or tab)
        if line.starts_with("    ") || line.starts_with("\t") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check for table rows (contain pipes)
        if line.trim().contains('|') && !line.trim().is_empty() {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check for empty lines - preserve them
        if line.trim().is_empty() {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Check for headers, lists, blockquotes, horizontal rules
        let trimmed = line.trim_start();
        if trimmed.starts_with('#')
            || trimmed.starts_with('*')
            || trimmed.starts_with('-')
            || trimmed.starts_with('+')
            || trimmed.starts_with('>')
            || trimmed.starts_with("---")
            || trimmed.starts_with("***")
            || trimmed.starts_with("___")
            || is_list_item(trimmed)
        {
            // Check if it's a list item or header - these might contain sentences
            if trimmed.starts_with('#') || is_list_item(trimmed) {
                // Process the content after the marker
                let (prefix, content) = extract_prefix_and_content(line);
                if content.trim().is_empty() {
                    result.push_str(line);
                    result.push('\n');
                } else {
                    let formatted_content = format_paragraph(&content);
                    for (i, sentence) in formatted_content.lines().enumerate() {
                        if i == 0 {
                            result.push_str(&prefix);
                            result.push_str(sentence);
                        } else {
                            // Continuation lines should be indented to align with the content
                            result.push_str(&" ".repeat(prefix.len()));
                            result.push_str(sentence);
                        }
                        result.push('\n');
                    }
                }
            } else {
                // Blockquote or horizontal rule - preserve as is
                result.push_str(line);
                result.push('\n');
            }
            continue;
        }

        // Regular paragraph text - format sentences
        let formatted = format_paragraph(line);
        result.push_str(&formatted);
    }

    result
}

/// Check if a line is a list item
fn is_list_item(trimmed: &str) -> bool {
    // Unordered list: *, -, +
    if trimmed.starts_with("* ") || trimmed.starts_with("- ") || trimmed.starts_with("+ ") {
        return true;
    }

    // Ordered list: number followed by . or )
    if let Some(pos) = trimmed.find(['.', ')']) {
        if pos > 0 {
            let num_part = &trimmed[..pos];
            return num_part.chars().all(|c| c.is_ascii_digit());
        }
    }

    false
}

/// Extract the prefix (indentation + marker) and content from a line
fn extract_prefix_and_content(line: &str) -> (String, String) {
    // Find where the actual content starts
    let indent_len = line.len() - line.trim_start().len();
    let after_indent = &line[indent_len..];

    // Handle headers
    if after_indent.starts_with('#') {
        if let Some(space_pos) = after_indent.find(' ') {
            let prefix = &line[..indent_len + space_pos + 1];
            let content = &line[indent_len + space_pos + 1..];
            return (prefix.to_string(), content.to_string());
        }
    }

    // Handle list items
    if is_list_item(after_indent) {
        // For unordered lists (*, -, +), find the space after the marker
        if after_indent.starts_with("* ")
            || after_indent.starts_with("- ")
            || after_indent.starts_with("+ ")
        {
            let prefix = &line[..indent_len + 2];
            let content = &line[indent_len + 2..];
            return (prefix.to_string(), content.to_string());
        }

        // For ordered lists, find the period/paren and space after the number
        if let Some(marker_end) = after_indent.find(['.', ')']) {
            // The content starts after the marker and the space
            let content_start = indent_len + marker_end + 1;
            if content_start < line.len() && line.chars().nth(content_start) == Some(' ') {
                let prefix = &line[..content_start + 1];
                let content = &line[content_start + 1..];
                return (prefix.to_string(), content.to_string());
            }
        }
    }

    (String::new(), line.to_string())
}

/// Format a paragraph by splitting sentences
fn format_paragraph(text: &str) -> String {
    if text.trim().is_empty() {
        return format!("{}\n", text);
    }

    let mut result = String::new();
    let mut current_sentence = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        current_sentence.push(ch);

        // Check for sentence delimiters
        if ch == '.' || ch == '!' || ch == '?' {
            // Look ahead to see if this is really the end of a sentence
            let mut j = i + 1;

            // Skip whitespace after the delimiter
            while j < chars.len() && chars[j].is_whitespace() && chars[j] != '\n' {
                current_sentence.push(chars[j]);
                j += 1;
            }

            // If we're at the end or the next character is uppercase/start of new sentence
            if j >= chars.len()
                || chars[j].is_uppercase()
                || chars[j].is_numeric()
                || chars[j] == '['
                || chars[j] == '('
            {
                // This is a sentence boundary
                result.push_str(current_sentence.trim());
                result.push('\n');
                current_sentence.clear();
                i = j - 1; // -1 because we'll increment at the end of the loop
            }
        }

        i += 1;
    }

    // Add any remaining text
    if !current_sentence.trim().is_empty() {
        result.push_str(current_sentence.trim());
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_sentences() {
        let input = "This is a sentence. This is another sentence.";
        let output = format_paragraph(input);
        assert_eq!(output, "This is a sentence.\nThis is another sentence.\n");
    }

    #[test]
    fn test_multiple_delimiters() {
        let input = "Is this a question? Yes! This is an answer.";
        let output = format_paragraph(input);
        assert_eq!(output, "Is this a question?\nYes!\nThis is an answer.\n");
    }

    #[test]
    fn test_code_blocks_preserved() {
        let input =
            "This is text.\n```rust\nfn main() {\n    println!(\"Hello.\");\n}\n```\nMore text.";
        let output = format_markdown_sentences(input);
        assert!(output.contains("```rust"));
        assert!(output.contains("println!(\"Hello.\");"));
    }

    #[test]
    fn test_headers_preserved() {
        let input = "# This is a header. With a sentence.";
        let output = format_markdown_sentences(input);
        assert!(output.starts_with("# This is a header."));
    }

    #[test]
    fn test_empty_lines_preserved() {
        let input = "Sentence one.\n\nSentence two.";
        let output = format_markdown_sentences(input);
        assert!(output.contains("\n\n"));
    }

    #[test]
    fn test_list_items() {
        let input = "* First item. Second sentence.";
        let output = format_markdown_sentences(input);
        assert!(output.starts_with("* First item."));
        assert!(output.contains("  Second sentence."));
    }

    #[test]
    fn test_indented_code_preserved() {
        let input = "Text here.\n    code line\n    another code line\nMore text.";
        let output = format_markdown_sentences(input);
        assert!(output.contains("    code line"));
        assert!(output.contains("    another code line"));
    }

    #[test]
    fn test_tables_preserved() {
        let input = "| Column 1 | Column 2 |\n|----------|----------|\n| Data | More data |";
        let output = format_markdown_sentences(input);
        assert!(output.contains("| Column 1 | Column 2 |"));
        assert!(output.contains("|----------|----------|"));
    }

    #[test]
    fn test_ordered_lists() {
        let input = "1. First item. Second sentence.\n2. Second item.";
        let output = format_markdown_sentences(input);
        assert!(output.starts_with("1. First item."));
        assert!(output.contains("   Second sentence."));
        assert!(output.contains("2. Second item."));
    }
}
