//! Contains structs and methods related to generating feedback strings
//! for providing help for the user to generate stronger passwords.

use crate::frequency_lists::DictionaryType;
use crate::matching::patterns::*;
use crate::matching::Match;
use std::fmt;

/// A warning explains what's wrong with the password.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[allow(missing_docs)]
pub enum Warning {
    StraightRowsOfKeysAreEasyToGuess,
    ShortKeyboardPatternsAreEasyToGuess,
    RepeatsLikeAaaAreEasyToGuess,
    RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess,
    ThisIsATop10Password,
    ThisIsATop100Password,
    ThisIsACommonPassword,
    ThisIsSimilarToACommonlyUsedPassword,
    SequencesLikeAbcAreEasyToGuess,
    RecentYearsAreEasyToGuess,
    AWordByItselfIsEasyToGuess,
    DatesAreOftenEasyToGuess,
    NamesAndSurnamesByThemselvesAreEasyToGuess,
    CommonNamesAndSurnamesAreEasyToGuess,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::StraightRowsOfKeysAreEasyToGuess => {
                // write!(f, "Straight rows of keys are easy to guess.")
                write!(f, "一排一排的密码很容易猜出来.")
            }
            Warning::ShortKeyboardPatternsAreEasyToGuess => {
                // write!(f, "Short keyboard patterns are easy to guess.")
                write!(f, "短键盘图案很容易猜测.")
            }
            Warning::RepeatsLikeAaaAreEasyToGuess => {
                // write!(f, "Repeats like \"aaa\" are easy to guess.")
                write!(f, "像 \"aaa\" 这样的重复很容易猜测.")
            }
            Warning::RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess => write!(
                f,
                // "Repeats like \"abcabcabc\" are only slightly harder to guess than \"abc\"."
                "像 \"abcabcabc\" 这样的重复只比 \"abc\" 稍微难猜一点."
            ),
            // Warning::ThisIsATop10Password => write!(f, "This is a top-10 common password."),
            Warning::ThisIsATop10Password => write!(f, "这是前10个常用密码."),
            // Warning::ThisIsATop100Password => write!(f, "This is a top-100 common password."),
            Warning::ThisIsATop100Password => write!(f, "这是前100个常用密码."),
            // Warning::ThisIsACommonPassword => write!(f, "This is a very common password."),
            Warning::ThisIsACommonPassword => write!(f, "这是一个非常常见的密码."),
            Warning::ThisIsSimilarToACommonlyUsedPassword => {
                // write!(f, "This is similar to a commonly used password.")
                write!(f, "这非常类似于常用的密码.")
            }
            Warning::SequencesLikeAbcAreEasyToGuess => {
                // write!(f, "Sequences like abc or 6543 are easy to guess.")
                write!(f, "像 abc 或 6543 这样的序列很容易猜测.")
            }
            // Warning::RecentYearsAreEasyToGuess => write!(f, "Recent years are easy to guess."),
            Warning::RecentYearsAreEasyToGuess => write!(f, "最近年份很容易猜测."),
            // Warning::AWordByItselfIsEasyToGuess => write!(f, "A word by itself is easy to guess."),
            Warning::AWordByItselfIsEasyToGuess => write!(f, "一个词本身很容易猜出来."),
            // Warning::DatesAreOftenEasyToGuess => write!(f, "Dates are often easy to guess."),
            Warning::DatesAreOftenEasyToGuess => write!(f, "日期通常很容易猜测."),
            Warning::NamesAndSurnamesByThemselvesAreEasyToGuess => {
                // write!(f, "Names and surnames by themselves are easy to guess.")
                write!(f, "名字和姓氏本身很容易猜测.")
            }
            Warning::CommonNamesAndSurnamesAreEasyToGuess => {
                // write!(f, "Common names and surnames are easy to guess.")
                write!(f, "常见的名字和姓氏很容易猜测.")
            }
        }
    }
}

/// A suggestion helps to choose a better password.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[allow(missing_docs)]
pub enum Suggestion {
    UseAFewWordsAvoidCommonPhrases,
    NoNeedForSymbolsDigitsOrUppercaseLetters,
    AddAnotherWordOrTwo,
    CapitalizationDoesntHelpVeryMuch,
    AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase,
    ReversedWordsArentMuchHarderToGuess,
    PredictableSubstitutionsDontHelpVeryMuch,
    UseALongerKeyboardPatternWithMoreTurns,
    AvoidRepeatedWordsAndCharacters,
    AvoidSequences,
    AvoidRecentYears,
    AvoidYearsThatAreAssociatedWithYou,
    AvoidDatesAndYearsThatAreAssociatedWithYou,
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suggestion::UseAFewWordsAvoidCommonPhrases => {
                // write!(f, "Use a few words, avoid common phrases.")
                write!(f, "使用几个单词, 避免使用常用短语.")
            }
            Suggestion::NoNeedForSymbolsDigitsOrUppercaseLetters => {
                // write!(f, "No need for symbols, digits, or uppercase letters.")
                write!(f, "不需要符号、数字或大写字母.")
            }
            Suggestion::AddAnotherWordOrTwo => {
                // write!(f, "Add another word or two. Uncommon words are better.")
                write!(f, "再加一两个字, 不常用的更好.")
            }
            Suggestion::CapitalizationDoesntHelpVeryMuch => {
                // write!(f, "Capitalization doesn't help very much.")
                write!(f, "大写没有什么用.")
            }
            Suggestion::AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase => write!(
                f,
                // "All-uppercase is almost as easy to guess as all-lowercase."
                "全大写几乎和全小写一样容易猜测."
            ),
            Suggestion::ReversedWordsArentMuchHarderToGuess => {
                // write!(f, "Reversed words aren't much harder to guess.")
                write!(f, "颠倒的单词并不难猜.")
            }
            Suggestion::PredictableSubstitutionsDontHelpVeryMuch => write!(
                f,
                // "Predictable substitutions like '@' instead of 'a' don't help very much."
                "像把 \"a\" 替换成 \"@\", 这样可预测的替换并没有什么用."
            ),
            Suggestion::UseALongerKeyboardPatternWithMoreTurns => {
                // write!(f, "Use a longer keyboard pattern with more turns.")
                write!(f, "使用较长的键盘模式，猜测次数可能稍多.")
            }
            Suggestion::AvoidRepeatedWordsAndCharacters => {
                // write!(f, "Avoid repeated words and characters.")
                write!(f, "避免重复的单词和字符.")
            }
            // Suggestion::AvoidSequences => write!(f, "Avoid sequences."),
            Suggestion::AvoidSequences => write!(f, "避免序列."),
            // Suggestion::AvoidRecentYears => write!(f, "Avoid recent years."),
            Suggestion::AvoidRecentYears => write!(f, "避免最近的年份."),
            Suggestion::AvoidYearsThatAreAssociatedWithYou => {
                // write!(f, "Avoid years that are associated with you.")
                write!(f, "避免与你相关的年份.")
            }
            Suggestion::AvoidDatesAndYearsThatAreAssociatedWithYou => {
                // write!(f, "Avoid dates and years that are associated with you.")
                write!(f, "避免与你相关的日期和年份.")
            }
        }
    }
}

/// Verbal feedback to help choose better passwords
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ser", derive(Serialize))]
pub struct Feedback {
    /// Explains what's wrong, e.g. "This is a top-10 common password". Not always set.
    warning: Option<Warning>,
    /// A possibly-empty list of suggestions to help choose a less guessable password.
    /// E.g. "Add another word or two".
    suggestions: Vec<Suggestion>,
}

impl Feedback {
    /// Explains what's wrong, e.g. "This is a top-10 common password". Not always set.
    pub fn warning(&self) -> Option<Warning> {
        self.warning
    }

    /// A possibly-empty list of suggestions to help choose a less guessable password.
    /// E.g. "Add another word or two".
    pub fn suggestions(&self) -> &[Suggestion] {
        &self.suggestions
    }
}

pub(crate) fn get_feedback(score: u8, sequence: &[Match]) -> Option<Feedback> {
    if sequence.is_empty() {
        // default feedback
        return Some(Feedback {
            warning: None,
            suggestions: vec![
                Suggestion::UseAFewWordsAvoidCommonPhrases,
                Suggestion::NoNeedForSymbolsDigitsOrUppercaseLetters,
            ],
        });
    }
    if score >= 3 {
        return None;
    }

    let longest_match = sequence
        .iter()
        .max_by_key(|x| x.token.chars().count())
        .unwrap();
    let mut feedback = get_match_feedback(longest_match, sequence.len() == 1);
    let extra_feedback = Suggestion::AddAnotherWordOrTwo;

    feedback.suggestions.insert(0, extra_feedback);
    Some(feedback)
}

fn get_match_feedback(cur_match: &Match, is_sole_match: bool) -> Feedback {
    match cur_match.pattern {
        MatchPattern::Dictionary(ref pattern) => {
            get_dictionary_match_feedback(cur_match, pattern, is_sole_match)
        }
        MatchPattern::Spatial(ref pattern) => Feedback {
            warning: Some(if pattern.turns == 1 {
                Warning::StraightRowsOfKeysAreEasyToGuess
            } else {
                Warning::ShortKeyboardPatternsAreEasyToGuess
            }),
            suggestions: vec![Suggestion::UseALongerKeyboardPatternWithMoreTurns],
        },
        MatchPattern::Repeat(ref pattern) => Feedback {
            warning: Some(if pattern.base_token.chars().count() == 1 {
                Warning::RepeatsLikeAaaAreEasyToGuess
            } else {
                Warning::RepeatsLikeAbcAbcAreOnlySlightlyHarderToGuess
            }),
            suggestions: vec![Suggestion::AvoidRepeatedWordsAndCharacters],
        },
        MatchPattern::Sequence(_) => Feedback {
            warning: Some(Warning::SequencesLikeAbcAreEasyToGuess),
            suggestions: vec![Suggestion::AvoidSequences],
        },
        MatchPattern::Regex(ref pattern) => {
            if pattern.regex_name == "recent_year" {
                Feedback {
                    warning: Some(Warning::RecentYearsAreEasyToGuess),
                    suggestions: vec![
                        Suggestion::AvoidRecentYears,
                        Suggestion::AvoidYearsThatAreAssociatedWithYou,
                    ],
                }
            } else {
                Feedback::default()
            }
        }
        MatchPattern::Date(_) => Feedback {
            warning: Some(Warning::DatesAreOftenEasyToGuess),
            suggestions: vec![Suggestion::AvoidDatesAndYearsThatAreAssociatedWithYou],
        },
        _ => Feedback {
            warning: None,
            suggestions: vec![],
        },
    }
}

fn get_dictionary_match_feedback(
    cur_match: &Match,
    pattern: &DictionaryPattern,
    is_sole_match: bool,
) -> Feedback {
    let warning: Option<Warning> = match pattern.dictionary_name {
        DictionaryType::Passwords => Some(if is_sole_match && !pattern.l33t && !pattern.reversed {
            let rank = pattern.rank;
            if rank <= 10 {
                Warning::ThisIsATop10Password
            } else if rank <= 100 {
                Warning::ThisIsATop100Password
            } else {
                Warning::ThisIsACommonPassword
            }
        } else {
            Warning::ThisIsSimilarToACommonlyUsedPassword
        }),
        DictionaryType::English => {
            if is_sole_match {
                Some(Warning::AWordByItselfIsEasyToGuess)
            } else {
                None
            }
        }
        DictionaryType::Surnames | DictionaryType::FemaleNames | DictionaryType::MaleNames => {
            Some(if is_sole_match {
                Warning::NamesAndSurnamesByThemselvesAreEasyToGuess
            } else {
                Warning::CommonNamesAndSurnamesAreEasyToGuess
            })
        }
        _ => None,
    };

    let mut suggestions: Vec<Suggestion> = Vec::new();
    let word = &cur_match.token;
    if word.is_empty() {
        return Feedback::default();
    }

    if word.chars().next().unwrap().is_uppercase() {
        suggestions.push(Suggestion::CapitalizationDoesntHelpVeryMuch);
    } else if word.chars().all(char::is_uppercase) {
        suggestions.push(Suggestion::AllUppercaseIsAlmostAsEasyToGuessAsAllLowercase);
    }

    if pattern.reversed && word.chars().count() >= 4 {
        suggestions.push(Suggestion::ReversedWordsArentMuchHarderToGuess);
    }
    if pattern.l33t {
        suggestions.push(Suggestion::PredictableSubstitutionsDontHelpVeryMuch);
    }

    Feedback {
        warning,
        suggestions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_top_password_feedback() {
        use crate::zxcvbn;

        let password = "password";
        let entropy = zxcvbn(password, &[]).unwrap();
        assert_eq!(
            entropy.feedback.unwrap().warning,
            Some(Warning::ThisIsATop10Password)
        );

        let password = "test";
        let entropy = zxcvbn(password, &[]).unwrap();
        assert_eq!(
            entropy.feedback.unwrap().warning,
            Some(Warning::ThisIsATop100Password)
        );

        let password = "p4ssw0rd";
        let entropy = zxcvbn(password, &[]).unwrap();
        assert_eq!(
            entropy.feedback.unwrap().warning,
            Some(Warning::ThisIsSimilarToACommonlyUsedPassword)
        );
    }
}
