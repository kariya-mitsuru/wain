use std::fmt;
use wain_ast::source::Source;

pub(crate) fn describe_position(
    f: &mut fmt::Formatter<'_>,
    source: &[u8],
    pos: usize,
) -> fmt::Result {
    if pos == source.len() {
        write!(f, " caused at byte offset {} (end of input)", pos)
    } else {
        let source = &source[pos..];
        let source = if source.len() > 25 {
            &source[..25]
        } else {
            source
        };
        write!(f, " caused at byte offset {}\n\n ...", pos)?;
        for b in source {
            write!(f, " {:2x}", b)?;
        }
        f.write_str("\n     ^\n     starts from here")
    }
}

#[derive(Clone)]
pub struct BinarySource<'a>(pub(crate) &'a [u8]);

impl<'a> Source for BinarySource<'a> {
    type Raw = &'a [u8];

    fn describe(&self, f: &mut fmt::Formatter<'_>, offset: usize) -> fmt::Result {
        describe_position(f, self.0, offset)
    }

    fn raw(&self) -> Self::Raw {
        self.0
    }
}
