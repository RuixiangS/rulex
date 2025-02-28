//! Implements _boundaries_. The analogues in the regex world are
//! [word boundaries](https://www.regular-expressions.info/wordboundaries.html) and
//! [anchors](https://www.regular-expressions.info/anchors.html).

use crate::{error::ParseErrorKind, Span};

/// A [word boundary](https://www.regular-expressions.info/wordboundaries.html) or
/// [anchor](https://www.regular-expressions.info/anchors.html), which we combine under the term
/// _boundary_.
///
/// All boundaries use a variation of the `%` sigil, so they are easy to
/// remember.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Boundary {
    pub kind: BoundaryKind,
    pub span: Span,
}

impl Boundary {
    pub fn new(kind: BoundaryKind, span: Span) -> Self {
        Boundary { kind, span }
    }

    pub fn kind(&self) -> BoundaryKind {
        self.kind
    }

    pub(crate) fn negate(&mut self) -> Result<(), ParseErrorKind> {
        match self.kind {
            BoundaryKind::Start | BoundaryKind::End => Err(ParseErrorKind::UnallowedNot),
            BoundaryKind::NotWord => Err(ParseErrorKind::UnallowedMultiNot(2)),
            BoundaryKind::Word => {
                self.kind = BoundaryKind::NotWord;
                Ok(())
            }
        }
    }

    #[cfg(feature = "dbg")]
    pub(super) fn pretty_print(&self, buf: &mut crate::PrettyPrinter) {
        match self.kind {
            BoundaryKind::Start => buf.push('^'),
            BoundaryKind::Word => buf.push('%'),
            BoundaryKind::NotWord => buf.push_str("!%"),
            BoundaryKind::End => buf.push('$'),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "dbg", derive(Debug))]
pub enum BoundaryKind {
    /// `Start`, the start of the string (or start of line in single-line mode)
    Start,
    /// `%`, a word boundary
    Word,
    /// `!%`, not a word boundary
    NotWord,
    /// `End`, the end of the string (or end of line in single-line mode)
    End,
}
