use std::collections::HashMap;
use std::sync::LazyLock;

type Blocks = HashMap<&'static str, &'static str>;

static UNICODE_BLOCKS: LazyLock<Blocks> = LazyLock::new(init_blocks);

fn init_blocks() -> Blocks {
    let mut map = HashMap::new();
    map.insert("IsBasicLatin", r#"[\u0000-\u007F]"#);
    map.insert("IsLatin-1Supplement", r#"[\u0080-\u00FF]"#);
    map.insert("IsLatinExtended-A", r#"[\u0100-\u017F]"#);
    map.insert("IsLatinExtended-B", r#"[\u0180-\u024F]"#);
    map.insert("IsIPAExtensions", r#"[\u0250-\u02AF]"#);
    map.insert("IsSpacingModifierLetters", r#"[\u02B0-\u02FF]"#);
    map.insert("IsCombiningDiacriticalMarks", r#"[\u0300-\u036F]"#);
    map.insert("IsGreek", r#"[\u0370-\u03FF]"#);
    map.insert("IsGreekandCoptic", r#"[\u0370-\u03FF]"#);
    map.insert("IsCyrillic", r#"[\u0400-\u04FF]"#);
    map.insert("IsCyrillicSupplement", r#"[\u0500-\u052F]"#);
    map.insert("IsArmenian", r#"[\u0530-\u058F]"#);
    map.insert("IsHebrew", r#"[\u0590-\u05FF]"#);
    map.insert("IsArabic", r#"[\u0600-\u06FF]"#);
    map.insert("IsSyriac", r#"[\u0700-\u074F]"#);
    map.insert("IsThaana", r#"[\u0780-\u07BF]"#);
    map.insert("IsDevanagari", r#"[\u0900-\u097F]"#);
    map.insert("IsBengali", r#"[\u0980-\u09FF]"#);
    map.insert("IsGurmukhi", r#"[\u0A00-\u0A7F]"#);
    map.insert("IsGujarati", r#"[\u0A80-\u0AFF]"#);
    map.insert("IsOriya", r#"[\u0B00-\u0B7F]"#);
    map.insert("IsTamil", r#"[\u0B80-\u0BFF]"#);
    map.insert("IsTelugu", r#"[\u0C00-\u0C7F]"#);
    map.insert("IsKannada", r#"[\u0C80-\u0CFF]"#);
    map.insert("IsMalayalam", r#"[\u0D00-\u0D7F]"#);
    map.insert("IsSinhala", r#"[\u0D80-\u0DFF]"#);
    map.insert("IsThai", r#"[\u0E00-\u0E7F]"#);
    map.insert("IsLao", r#"[\u0E80-\u0EFF]"#);
    map.insert("IsTibetan", r#"[\u0F00-\u0FFF]"#);
    map.insert("IsMyanmar", r#"[\u1000-\u109F]"#);
    map.insert("IsGeorgian", r#"[\u10A0-\u10FF]"#);
    map.insert("IsHangulJamo", r#"[\u1100-\u11FF]"#);
    map.insert("IsEthiopic", r#"[\u1200-\u137F]"#);
    map.insert("IsCherokee", r#"[\u13A0-\u13FF]"#);
    map.insert("IsUnifiedCanadianAboriginalSyllabics", r#"[\u1400-\u167F]"#);
    map.insert("IsOgham", r#"[\u1680-\u169F]"#);
    map.insert("IsRunic", r#"[\u16A0-\u16FF]"#);
    map.insert("IsTagalog", r#"[\u1700-\u171F]"#);
    map.insert("IsHanunoo", r#"[\u1720-\u173F]"#);
    map.insert("IsBuhid", r#"[\u1740-\u175F]"#);
    map.insert("IsTagbanwa", r#"[\u1760-\u177F]"#);
    map.insert("IsKhmer", r#"[\u1780-\u17FF]"#);
    map.insert("IsMongolian", r#"[\u1800-\u18AF]"#);
    map.insert("IsLimbu", r#"[\u1900-\u194F]"#);
    map.insert("IsTaiLe", r#"[\u1950-\u197F]"#);
    map.insert("IsKhmerSymbols", r#"[\u19E0-\u19FF]"#);
    map.insert("IsPhoneticExtensions", r#"[\u1D00-\u1D7F]"#);
    map.insert("IsLatinExtendedAdditional", r#"[\u1E00-\u1EFF]"#);
    map.insert("IsGreekExtended", r#"[\u1F00-\u1FFF]"#);
    map.insert("IsGeneralPunctuation", r#"[\u2000-\u206F]"#);
    map.insert("IsSuperscriptsandSubscripts", r#"[\u2070-\u209F]"#);
    map.insert("IsCurrencySymbols", r#"[\u20A0-\u20CF]"#);
    map.insert(
        "IsCombiningDiacriticalMarksforSymbols",
        r#"[\u20D0-\u20FF]"#,
    );
    map.insert("IsCombiningMarksforSymbols", r#"[\u20D0-\u20FF]"#);
    map.insert("IsLetterlikeSymbols", r#"[\u2100-\u214F]"#);
    map.insert("IsNumberForms", r#"[\u2150-\u218F]"#);
    map.insert("IsArrows", r#"[\u2190-\u21FF]"#);
    map.insert("IsMathematicalOperators", r#"[\u2200-\u22FF]"#);
    map.insert("IsMiscellaneousTechnical", r#"[\u2300-\u23FF]"#);
    map.insert("IsControlPictures", r#"[\u2400-\u243F]"#);
    map.insert("IsOpticalCharacterRecognition", r#"[\u2440-\u245F]"#);
    map.insert("IsEnclosedAlphanumerics", r#"[\u2460-\u24FF]"#);
    map.insert("IsBoxDrawing", r#"[\u2500-\u257F]"#);
    map.insert("IsBlockElements", r#"[\u2580-\u259F]"#);
    map.insert("IsGeometricShapes", r#"[\u25A0-\u25FF]"#);
    map.insert("IsMiscellaneousSymbols", r#"[\u2600-\u26FF]"#);
    map.insert("IsDingbats", r#"[\u2700-\u27BF]"#);
    map.insert("IsMiscellaneousMathematicalSymbols-A", r#"[\u27C0-\u27EF]"#);
    map.insert("IsSupplementalArrows-A", r#"[\u27F0-\u27FF]"#);
    map.insert("IsBraillePatterns", r#"[\u2800-\u28FF]"#);
    map.insert("IsSupplementalArrows-B", r#"[\u2900-\u297F]"#);
    map.insert("IsMiscellaneousMathematicalSymbols-B", r#"[\u2980-\u29FF]"#);
    map.insert("IsSupplementalMathematicalOperators", r#"[\u2A00-\u2AFF]"#);
    map.insert("IsMiscellaneousSymbolsandArrows", r#"[\u2B00-\u2BFF]"#);
    map.insert("IsCJKRadicalsSupplement", r#"[\u2E80-\u2EFF]"#);
    map.insert("IsKangxiRadicals", r#"[\u2F00-\u2FDF]"#);
    map.insert("IsIdeographicDescriptionCharacters", r#"[\u2FF0-\u2FFF]"#);
    map.insert("IsCJKSymbolsandPunctuation", r#"[\u3000-\u303F]"#);
    map.insert("IsHiragana", r#"[\u3040-\u309F]"#);
    map.insert("IsKatakana", r#"[\u30A0-\u30FF]"#);
    map.insert("IsBopomofo", r#"[\u3100-\u312F]"#);
    map.insert("IsHangulCompatibilityJamo", r#"[\u3130-\u318F]"#);
    map.insert("IsKanbun", r#"[\u3190-\u319F]"#);
    map.insert("IsBopomofoExtended", r#"[\u31A0-\u31BF]"#);
    map.insert("IsKatakanaPhoneticExtensions", r#"[\u31F0-\u31FF]"#);
    map.insert("IsEnclosedCJKLettersandMonths", r#"[\u3200-\u32FF]"#);
    map.insert("IsCJKCompatibility", r#"[\u3300-\u33FF]"#);
    map.insert("IsCJKUnifiedIdeographsExtensionA", r#"[\u3400-\u4DBF]"#);
    map.insert("IsYijingHexagramSymbols", r#"[\u4DC0-\u4DFF]"#);
    map.insert("IsCJKUnifiedIdeographs", r#"[\u4E00-\u9FFF]"#);
    map.insert("IsYiSyllables", r#"[\uA000-\uA48F]"#);
    map.insert("IsYiRadicals", r#"[\uA490-\uA4CF]"#);
    map.insert("IsHangulSyllables", r#"[\uAC00-\uD7AF]"#);
    map.insert("IsHighSurrogates", r#"[\uD800-\uDB7F]"#);
    map.insert("IsHighPrivateUseSurrogates", r#"[\uDB80-\uDBFF]"#);
    map.insert("IsLowSurrogates", r#"[\uDC00-\uDFFF]"#);
    map.insert("IsPrivateUse lub IsPrivateUseArea", r#"[\uE000-\uF8FF]"#);
    map.insert("IsCJKCompatibilityIdeographs", r#"[\uF900-\uFAFF]"#);
    map.insert("IsAlphabeticPresentationForms", r#"[\uFB00-\uFB4F]"#);
    map.insert("IsArabicPresentationForms-A", r#"[\uFB50-\uFDFF]"#);
    map.insert("IsVariationSelectors", r#"[\uFE00-\uFE0F]"#);
    map.insert("IsCombiningHalfMarks", r#"[\uFE20-\uFE2F]"#);
    map.insert("IsCJKCompatibilityForms", r#"[\uFE30-\uFE4F]"#);
    map.insert("IsSmallFormVariants", r#"[\uFE50-\uFE6F]"#);
    map.insert("IsArabicPresentationForms-B", r#"[\uFE70-\uFEFF]"#);
    map.insert("IsHalfwidthandFullwidthForms", r#"[\uFF00-\uFFEF]"#);
    map.insert("IsSpecials", r#"[\uFFF0-\uFFFF]"#);
    map
}

pub fn replace_block_names(input: &str, collapse_whitespaces: bool) -> String {
    let mut output = String::new();
    let mut buffer = String::new();
    let mut name = String::new();
    let mut state = 0;
    for ch in input.chars() {
        match state {
            0 => {
                if ch == '\\' {
                    buffer.push(ch);
                    state = 1;
                } else {
                    output.push(ch);
                    // State remains 0.
                }
            }
            1 => {
                if ch == 'p' {
                    buffer.push(ch);
                    state = 2;
                } else {
                    output.push_str(&buffer);
                    buffer.clear();
                    output.push(ch);
                    state = 0;
                }
            }
            2 => {
                if ch == '{' {
                    buffer.push(ch);
                    state = 3;
                } else {
                    output.push_str(&buffer);
                    buffer.clear();
                    output.push(ch);
                    state = 0;
                }
            }
            3 => {
                buffer.push(ch);
                match ch {
                    '}' => {
                        // Reached the end of the block name.
                        if let Some(block) = UNICODE_BLOCKS.get(name.as_str()) {
                            output.push_str(block);
                        } else {
                            output.push_str(&buffer);
                        }
                        buffer.clear();
                        name.clear();
                        state = 0;
                    }
                    ' ' if collapse_whitespaces => {
                        // Skip whitespace if needed.
                    }
                    other => {
                        // Collect the next character of the block name.
                        name.push(other);
                        // State remains 3.
                    }
                }
            }
            _ => break,
        }
    }
    if !buffer.is_empty() {
        output.push_str(&buffer);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replacing_block_names_should_work() {
        assert_eq!(
            r#"[\u0000-\u007F]+"#,
            replace_block_names(r#"\p{IsBasicLatin}+"#, false)
        );
        assert_eq!(
            r#"[\u0000-\u007F]+"#,
            replace_block_names(r#"\p{ I s   B a si   c La ti  n}+"#, true)
        );
        assert_eq!(
            r#"[\u0000-\u007F]+\s[\uFFF0-\uFFFF]+"#,
            replace_block_names(r#"\p{IsBasicLatin}+\s\p{IsSpecials}+"#, false)
        );
        assert_eq!(
            r#"[\u0000-\u007F]+\s"#,
            replace_block_names(r#"\p{IsBasicLatin}+\s"#, false)
        );
        assert_eq!(
            r#"\p{IsBasicLatin+"#,
            replace_block_names(r#"\p{IsBasicLatin+"#, false)
        );
        assert_eq!(r#"\p{}+"#, replace_block_names(r#"\p{}+"#, false));
        assert_eq!(
            r#"\p{NonExistingBlockName}+"#,
            replace_block_names(r#"\p{NonExistingBlockName}+"#, false)
        );
    }
}
