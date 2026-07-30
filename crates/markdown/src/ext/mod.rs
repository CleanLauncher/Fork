use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn extract_plain_text(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut plain = String::new();
    for event in parser {
        match event {
            Event::Text(text) => plain.push_str(&text),
            Event::Code(text) => plain.push_str(&text),
            Event::SoftBreak | Event::HardBreak => plain.push(' '),
            _ => {}
        }
    }
    plain
}

pub fn extract_headings(markdown: &str) -> Vec<(u32, String)> {
    let parser = Parser::new(markdown);
    let mut headings = Vec::new();
    let mut heading_level = 0;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading_level = level as u32;
            }
            Event::End(TagEnd::Heading(..)) => {
                heading_level = 0;
            }
            Event::Text(text) if heading_level > 0 => {
                headings.push((heading_level, text.to_string()));
            }
            _ => {}
        }
    }
    headings
}

pub fn get_word_count(markdown: &str) -> usize {
    let plain = extract_plain_text(markdown);
    plain.split_whitespace().count()
}

pub fn get_reading_time(markdown: &str, words_per_minute: usize) -> String {
    let word_count = get_word_count(markdown);
    let minutes = (word_count as f64 / words_per_minute as f64).ceil() as u64;
    if minutes == 0 {
        "< 1 min read".to_string()
    } else if minutes == 1 {
        "1 min read".to_string()
    } else {
        format!("{} min read", minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_plain_text() {
        let md = "# Hello\n**bold** and *italic*";
        let plain = extract_plain_text(md);
        assert!(plain.contains("Hello"));
        assert!(plain.contains("bold"));
        assert!(plain.contains("italic"));
    }

    #[test]
    fn test_extract_headings() {
        let md = "# Title\n## Section\n### Subsection";
        let headings = extract_headings(md);
        assert_eq!(headings.len(), 3);
        assert_eq!(headings[0], (1, "Title".to_string()));
        assert_eq!(headings[1], (2, "Section".to_string()));
    }

    #[test]
    fn test_word_count() {
        let md = "one two three four";
        assert_eq!(get_word_count(md), 4);
    }
}
