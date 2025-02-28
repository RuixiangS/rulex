use pomsky_syntax::{
    diagnose::{
        CharClassError, CharStringError, DeprecationError, ParseErrorKind, RepetitionError,
    },
    Span,
};

use super::CompileErrorKind;

pub(super) fn get_parser_help(
    kind: &ParseErrorKind,
    slice: &str,
    span: &mut Span,
) -> Option<String> {
    match kind {
        ParseErrorKind::LexErrorWithMessage(msg) => msg.get_help(slice),
        ParseErrorKind::RangeIsNotIncreasing => {
            let dash_pos = slice.find('-').unwrap();
            let (part1, part2) = slice.split_at(dash_pos);
            let part2 = part2.trim_start_matches('-');
            Some(format!("Switch the numbers: {}-{}", part2.trim(), part1.trim()))
        }
        #[cfg(feature = "suggestions")]
        ParseErrorKind::CharClass(CharClassError::UnknownNamedClass {
            similar: Some(ref similar),
            ..
        }) => Some(format!("Perhaps you meant `{similar}`")),
        ParseErrorKind::CharClass(CharClassError::DescendingRange(..)) => {
            let dash_pos = slice.find('-').unwrap();
            let (part1, part2) = slice.split_at(dash_pos);
            let part2 = part2.trim_start_matches('-');
            Some(format!("Switch the characters: {}-{}", part2.trim(), part1.trim()))
        }
        ParseErrorKind::CharClass(CharClassError::CaretInGroup) => {
            Some("Use `![...]` to negate a character class".into())
        }
        ParseErrorKind::CharString(CharStringError::TooManyCodePoints)
            if slice.trim_matches(&['"', '\''][..]).chars().all(|c| c.is_ascii_digit()) =>
        {
            Some(
                "Try a `range` expression instead:\n\
                https://pomsky-lang.org/docs/language-tour/ranges/"
                    .into(),
            )
        }
        ParseErrorKind::KeywordAfterLet(_) => Some("Use a different variable name".into()),
        ParseErrorKind::UnallowedMultiNot(n) => Some(if n % 2 == 0 {
            "The number of exclamation marks is even, so you can remove all of them".into()
        } else {
            "The number of exclamation marks is odd, so you can remove all of them but one".into()
        }),
        ParseErrorKind::LetBindingExists => Some("Use a different name".into()),
        ParseErrorKind::MissingLetKeyword => Some(format!("Try `let {slice} ...`")),
        ParseErrorKind::Repetition(RepetitionError::QmSuffix) => Some(
            "If you meant to make the repetition lazy, append the `lazy` keyword instead.\n\
                If this is intentional, consider adding parentheses around the inner repetition."
                .into(),
        ),
        ParseErrorKind::Repetition(RepetitionError::Multi) => {
            Some("Add parentheses around the first repetition.".into())
        }
        ParseErrorKind::InvalidEscapeInStringAt(offset) => {
            let span_start = span.range_unchecked().start;
            *span = Span::new(span_start + offset - 1, span_start + offset + 1);
            None
        }
        ParseErrorKind::RecursionLimit => Some(
            "Try a less nested expression. It helps to refactor it using variables:\n\
                https://pomsky-lang.org/docs/language-tour/variables/"
                .into(),
        ),
        ParseErrorKind::Deprecated(DeprecationError::DotInSet) => {
            Some("Use `.` without brackets instead".into())
        }
        _ => None,
    }
}

pub(super) fn get_compiler_help(
    kind: &CompileErrorKind,
    slice: &str,
    _span: Span,
) -> Option<String> {
    match kind {
        CompileErrorKind::UnknownVariable { found, .. }
            if found.starts_with('U') && found[1..].chars().all(|c| c.is_ascii_hexdigit()) =>
        {
            Some(format!("Perhaps you meant a code point: `U+{cp}`", cp = &found[1..]))
        }

        #[cfg(feature = "suggestions")]
        CompileErrorKind::UnknownVariable { similar: Some(ref similar), .. }
        | CompileErrorKind::UnknownReferenceName { similar: Some(ref similar), .. } => {
            Some(format!("Perhaps you meant `{similar}`"))
        }

        CompileErrorKind::EmptyClassNegated { group1, group2 } => Some(format!(
            "The group is empty because it contains both `{group1:?}` and `{group2:?}`, \
            which together match every code point",
        )),

        CompileErrorKind::NameUsedMultipleTimes(_) => {
            Some("Give this group a different name".into())
        }

        CompileErrorKind::UnknownReferenceNumber(0) => {
            Some("Capturing group numbers start with 1".into())
        }
        CompileErrorKind::RelativeRefZero => Some(
            "Perhaps you meant `::-1` to refer to the previous or surrounding capturing group"
                .into(),
        ),

        CompileErrorKind::JsWordBoundaryInUnicodeMode => {
            Some(format!("Disable Unicode, e.g. `(disable unicode; {slice})`"))
        }
        CompileErrorKind::NegativeShorthandInAsciiMode | CompileErrorKind::UnicodeInAsciiMode => {
            Some(format!("Enable Unicode, e.g. `(enable unicode; {slice})`"))
        }

        _ => None,
    }
}
