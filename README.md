# doc-tools

Tools for working with markdown documentation

## Tools

### md-sentence-format

A tool that reformats markdown files so that each sentence appears on its own line. This makes it easier to track changes in version control and review documentation.

**Features:**
- Splits sentences at periods (.), exclamation marks (!), and question marks (?)
- Preserves markdown formatting (headers, bold, italic, links, etc.)
- Leaves code blocks untouched (both fenced and indented)
- Preserves tables
- Handles lists (both ordered and unordered) with proper indentation
- Preserves blockquotes and other markdown elements

**Usage:**

```bash
# Format a file
cargo run --bin md-sentence-format input.md > output.md

# Or read from stdin
echo "This is a sentence. This is another." | cargo run --bin md-sentence-format

# Build and install
cargo build --release
# The binary will be in target/release/md-sentence-format
```

**Example:**

Input:
```markdown
This is a paragraph with multiple sentences. This is the second sentence. And this is the third!
```

Output:
```markdown
This is a paragraph with multiple sentences.
This is the second sentence.
And this is the third!
```

See `example.md` and `example_formatted.md` for a comprehensive demonstration.

**Known Limitations:**
- Abbreviations like "Dr.", "Mr.", etc. may be treated as sentence endings
- Decimal numbers like "3.14" may be split incorrectly
- These edge cases can be improved in future versions

## Building

```bash
cargo build --release
```

## Testing

```bash
cargo test
```

## Development

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

