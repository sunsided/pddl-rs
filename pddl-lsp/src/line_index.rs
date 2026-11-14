//! Converts between byte offsets and LSP positions (line and column).

use std::ops::Range;

use lsp_types::Position;

/// How the client counts columns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// UTF-16 code units, the LSP default.
    Utf16,
    /// Bytes.
    Utf8,
}

/// The byte offsets at which each line of a text starts.
#[derive(Debug, Clone)]
pub struct LineIndex {
    /// Byte offset of the start of each line; the first is always 0.
    starts: Vec<usize>,
    /// Whether each line is ASCII, so that its columns are byte offsets in any encoding.
    ascii: Vec<bool>,
    /// How columns are counted.
    encoding: Encoding,
}

impl LineIndex {
    /// Indexes the lines of `text`. Lines end at `\n`; a `\r` before it belongs to the line.
    #[must_use]
    pub fn new(text: &str, encoding: Encoding) -> Self {
        let starts: Vec<usize> = std::iter::once(0)
            .chain(text.match_indices('\n').map(|(i, _)| i + 1))
            .collect();
        let ends = starts.iter().skip(1).copied().chain([text.len()]);
        let ascii = starts
            .iter()
            .zip(ends)
            .map(|(&s, e)| text[s..e].is_ascii())
            .collect();
        Self {
            starts,
            ascii,
            encoding,
        }
    }

    /// The position of byte `offset` in `text`, which must be the indexed text.
    #[must_use]
    pub fn position(&self, text: &str, offset: usize) -> Position {
        let offset = floor_char_boundary(text, offset.min(text.len()));
        let line = self.starts.partition_point(|&start| start <= offset) - 1;
        let start = self.starts[line];
        let column = match self.encoding {
            Encoding::Utf16 if !self.ascii[line] => text[start..offset].encode_utf16().count(),
            _ => offset - start,
        };
        Position::new(to_u32(line), to_u32(column))
    }

    /// The LSP range of the bytes `range` in `text`.
    #[must_use]
    pub fn range(&self, text: &str, range: Range<usize>) -> lsp_types::Range {
        lsp_types::Range::new(
            self.position(text, range.start),
            self.position(text, range.end),
        )
    }

    /// The byte offset of `position` in `text`. A position past the end of its
    /// line or of the text is moved back to that end.
    #[must_use]
    pub fn offset(&self, text: &str, position: Position) -> usize {
        let Some(&start) = self.starts.get(position.line as usize) else {
            return text.len();
        };
        let line = &text[start..];
        let line = &line[..line.find('\n').unwrap_or(line.len())];
        let line = line.strip_suffix('\r').unwrap_or(line);
        let column = position.character as usize;
        let within = match self.encoding {
            _ if self.ascii[position.line as usize] => column.min(line.len()),
            Encoding::Utf8 => floor_char_boundary(line, column.min(line.len())),
            Encoding::Utf16 => {
                let mut units = 0;
                line.char_indices()
                    .find(|(_, c)| {
                        units += c.len_utf16();
                        units > column
                    })
                    .map_or(line.len(), |(i, _)| i)
            }
        };
        start + within
    }
}

/// The largest char boundary of `text` at or before `offset`.
fn floor_char_boundary(text: &str, mut offset: usize) -> usize {
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

/// Converts a line or column to the `u32` LSP uses; no editor holds a text that long.
fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn positions_round_trip_in_both_encodings() {
        let text = "ab\r\n(ä 𝄞 x)\nlast";
        let x = text.find('x').unwrap();
        let utf16 = LineIndex::new(text, Encoding::Utf16);
        check!(utf16.position(text, x) == Position::new(1, 6));
        check!(utf16.offset(text, Position::new(1, 6)) == x);
        let utf8 = LineIndex::new(text, Encoding::Utf8);
        check!(utf8.position(text, x) == Position::new(1, 9));
        check!(utf8.offset(text, Position::new(1, 9)) == x);
        check!(utf16.position(text, text.len()) == Position::new(2, 4));
    }

    #[test]
    fn positions_past_the_end_are_clamped() {
        let text = "ab\r\ncd";
        let index = LineIndex::new(text, Encoding::Utf16);
        check!(index.offset(text, Position::new(0, 99)) == 2);
        check!(index.offset(text, Position::new(9, 0)) == text.len());
    }
}
