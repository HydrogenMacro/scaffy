use std::{borrow::Cow, cell::LazyCell};

use aho_corasick::AhoCorasick;

#[derive(Clone, Copy, Debug)]
pub enum WordCasing {
    Preserve,
    Upper,
    Lower,
    Title,
}
pub fn string_to_casing(
    input: impl AsRef<str>,
    joiner: &'static str,
    word_casing: WordCasing,
    first_word_casing: Option<WordCasing>,
) -> String {
    let mut string_with_casing = String::new();
    for (i, word) in input.as_ref().split_ascii_whitespace().enumerate() {
        let word = Cow::Borrowed(word);

        let current_word_casing = if i == 0
            && let Some(casing) = first_word_casing
        {
            casing
        } else {
            word_casing
        };
        let conv_word = match current_word_casing {
            WordCasing::Preserve => word,
            WordCasing::Lower => Cow::Owned(word.to_lowercase()),
            WordCasing::Title => {
                let mut converted_word = word.to_lowercase();
                if let Some(first_char) = converted_word.chars().next() {
                    converted_word.replace_range(0..1, &String::from(first_char).to_uppercase());
                }
                Cow::Owned(converted_word)
            }
            WordCasing::Upper => Cow::Owned(word.to_uppercase()),
        };

        if i != 0 {
            string_with_casing.push_str(joiner);
        }
        string_with_casing.push_str(&conv_word);
    }
    return string_with_casing;
}

pub static SCAFFY_PATTERNS: &'static [&'static str] = &[
    "@@SCAFFY_PROJECT_NAME@@",
    "@@SCAFFY_PROJECT_NAME_TITLECASE@@",
    "@@SCAFFY_PROJECT_NAME_UPPERCASE@@",
    "@@SCAFFY_PROJECT_NAME_LOWERCASE@@",
    "@@SCAFFY_PROJECT_NAME_SNAKECASE@@",
    "@@SCAFFY_PROJECT_NAME_UPPERSNAKECASE@@",
    "@@SCAFFY_PROJECT_NAME_LOWERSNAKECASE@@",
    "@@SCAFFY_PROJECT_NAME_LOWERCAMELCASE@@",
    "@@SCAFFY_PROJECT_NAME_UPPERCAMELCASE@@",
    "@@SCAFFY_PROJECT_NAME_KEBABCASE@@",
    "@@SCAFFY_PROJECT_NAME_LOWERKEBABCASE@@",
    "@@SCAFFY_PROJECT_NAME_UPPERKEBABCASE@@",
];
pub static SCAFFY_CASING_ARGS: &'static [(&'static str, WordCasing, Option<WordCasing>)] = &[
    (" ", WordCasing::Preserve, None),
    (" ", WordCasing::Title, None),
    (" ", WordCasing::Title, None),
    (" ", WordCasing::Title, None),
    ("_", WordCasing::Title, None),
    ("_", WordCasing::Title, None),
    ("_", WordCasing::Title, None),
    ("", WordCasing::Title, Some(WordCasing::Lower)),
    ("", WordCasing::Title, None),
    ("-", WordCasing::Title, None),
    ("-", WordCasing::Title, None),
    ("-", WordCasing::Title, None),
];
thread_local! {
    pub static SCAFFY_STRING_SEARCHER: LazyCell<AhoCorasick> = LazyCell::new(|| {
        let aho_corasick = AhoCorasick::new(SCAFFY_PATTERNS).unwrap();
        return aho_corasick;
    })
}

pub fn scaffy_string_replacement(input: impl AsRef<str>, project_name: impl AsRef<str>) -> String {
    let input = input.as_ref();
    let mut output = Vec::with_capacity(input.len());
    let replacement_strings: Vec<String> = SCAFFY_CASING_ARGS.iter().map(|&(joiner, word_casing, first_word_casing)| string_to_casing(project_name.as_ref(), joiner, word_casing, first_word_casing)).collect();
    SCAFFY_STRING_SEARCHER.with(|scaffy_string_searcher| scaffy_string_searcher.try_stream_replace_all(input.as_bytes(), &mut output, &replacement_strings).unwrap());
    return String::try_from(output).unwrap();
}
