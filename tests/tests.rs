/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/swamp
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use source_map_cache::SourceMap;

#[cfg(test)]
mod tests {
    use super::*;
    use seq_map::SeqMap;
    use source_map_node::Span;
    use std::path::PathBuf;

    // Helper function to create a source map for testing without path verification
    fn create_test_source_map() -> SourceMap {
        SourceMap {
            mounts: SeqMap::new(),
            cache: SeqMap::new(),
            file_cache: SeqMap::new(),
            next_file_id: 1,
        }
    }

    #[test]
    fn test_basic_line_offsets() {
        let mut source_map = create_test_source_map();

        let file_content = "line 1\nline 2\nline 3\n";
        let file_id = 1;
        source_map.add_manual(
            file_id,
            "test",
            &PathBuf::from("test.txt"),
            file_content
        );

        assert_eq!(source_map.get_source_line(file_id, 1), Some("line 1"));
        assert_eq!(source_map.get_source_line(file_id, 2), Some("line 2"));
        assert_eq!(source_map.get_source_line(file_id, 3), Some("line 3"));
        assert_eq!(source_map.get_source_line(file_id, 4), None);

        assert_eq!(source_map.get_span_location_utf8(file_id, 0), (1, 1)); // Start of file
        assert_eq!(source_map.get_span_location_utf8(file_id, 7), (2, 1)); // Start of line 2
    }

    #[test]
    fn test_file_without_trailing_newline() {
        let mut source_map = create_test_source_map();

				let file_content = "first line\nsecond line";
        let file_id = 1;
        source_map.add_manual(
            file_id,
            "test",
            &PathBuf::from("no_newline.txt"),
            file_content
        );

        assert_eq!(source_map.get_source_line(file_id, 1), Some("first line"));
        assert_eq!(source_map.get_source_line(file_id, 2), Some("second line"));
        assert_eq!(source_map.get_source_line(file_id, 3), None);

        let span = Span {
            file_id,
            offset: 11, // Start of "second line"
            length: 11, // Length of "second line"
        };

        assert_eq!(source_map.get_text_span(&span), "second line");
    }

    #[test]
    fn test_empty_file_and_edge_cases() {
        let mut source_map = create_test_source_map();

        let empty_id = 1;
        source_map.add_manual(
            empty_id,
            "test",
            &PathBuf::from("empty.txt"),
            ""
        );

        let single_id = 2;
        source_map.add_manual(
            single_id,
            "test",
            &PathBuf::from("single.txt"),
            "just one line"
        );

        let newlines_id = 3;
        source_map.add_manual(
            newlines_id,
            "test",
            &PathBuf::from("newlines.txt"),
            "\n\n\n"
        );

        // Test empty file
        assert_eq!(source_map.get_source_line(empty_id, 1), None);

        // Test single line file
        assert_eq!(source_map.get_source_line(single_id, 1), Some("just one line"));
        assert_eq!(source_map.get_source_line(single_id, 2), None);

        // Test file with only newlines
        assert_eq!(source_map.get_source_line(newlines_id, 1), Some(""));
        assert_eq!(source_map.get_source_line(newlines_id, 2), Some(""));
        assert_eq!(source_map.get_source_line(newlines_id, 3), Some(""));
        assert_eq!(source_map.get_source_line(newlines_id, 4), None);
    }
}