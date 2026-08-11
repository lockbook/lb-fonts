// Noto
// Sourced from Google Fonts
// This subset of Noto fonts is chosen to cover many people in few bytes; it's
// missing the large CJK fonts and fonts for some niche languages like Khmer.
pub const NOTO_SANS_REGULAR: &[u8] = include_bytes!("../fonts/NotoSans-Regular.ttf");
pub const NOTO_SANS_BOLD: &[u8] = include_bytes!("../fonts/NotoSans-Bold.ttf");
pub const NOTO_SANS_ITALIC: &[u8] = include_bytes!("../fonts/NotoSans-Italic.ttf");
pub const NOTO_SANS_MONO_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansMono-Regular.ttf");

pub const NOTO_SANS_ARABIC_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansArabic-Regular.ttf");
pub const NOTO_SANS_BENGALI_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansBengali-Regular.ttf");
pub const NOTO_SANS_DEVANAGARI_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansDevanagari-Regular.ttf");
pub const NOTO_SANS_GUJARATI_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansGujarati-Regular.ttf");
pub const NOTO_SANS_GURMUKHI_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansGurmukhi-Regular.ttf");
pub const NOTO_SANS_HEBREW_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansHebrew-Regular.ttf");
pub const NOTO_SANS_KANNADA_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansKannada-Regular.ttf");
pub const NOTO_SANS_MALAYALAM_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansMalayalam-Regular.ttf");
pub const NOTO_SANS_MYANMAR_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansMyanmar-Regular.ttf");
pub const NOTO_SANS_TAMIL_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansTamil-Regular.ttf");
pub const NOTO_SANS_TELUGU_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansTelugu-Regular.ttf");
pub const NOTO_SANS_THAI_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansThai-Regular.ttf");

pub const NOTO_SANS_ARABIC_BOLD: &[u8] = include_bytes!("../fonts/NotoSansArabic-Bold.ttf");
pub const NOTO_SANS_BENGALI_BOLD: &[u8] = include_bytes!("../fonts/NotoSansBengali-Bold.ttf");
pub const NOTO_SANS_DEVANAGARI_BOLD: &[u8] = include_bytes!("../fonts/NotoSansDevanagari-Bold.ttf");
pub const NOTO_SANS_GUJARATI_BOLD: &[u8] = include_bytes!("../fonts/NotoSansGujarati-Bold.ttf");
pub const NOTO_SANS_GURMUKHI_BOLD: &[u8] = include_bytes!("../fonts/NotoSansGurmukhi-Bold.ttf");
pub const NOTO_SANS_HEBREW_BOLD: &[u8] = include_bytes!("../fonts/NotoSansHebrew-Bold.ttf");
pub const NOTO_SANS_KANNADA_BOLD: &[u8] = include_bytes!("../fonts/NotoSansKannada-Bold.ttf");
pub const NOTO_SANS_MALAYALAM_BOLD: &[u8] = include_bytes!("../fonts/NotoSansMalayalam-Bold.ttf");
pub const NOTO_SANS_MYANMAR_BOLD: &[u8] = include_bytes!("../fonts/NotoSansMyanmar-Bold.ttf");
pub const NOTO_SANS_TAMIL_BOLD: &[u8] = include_bytes!("../fonts/NotoSansTamil-Bold.ttf");
pub const NOTO_SANS_TELUGU_BOLD: &[u8] = include_bytes!("../fonts/NotoSansTelugu-Bold.ttf");
pub const NOTO_SANS_THAI_BOLD: &[u8] = include_bytes!("../fonts/NotoSansThai-Bold.ttf");

pub const NOTO: [&[u8]; 28] = [
    NOTO_SANS_REGULAR,
    NOTO_SANS_BOLD,
    NOTO_SANS_ITALIC,
    NOTO_SANS_MONO_REGULAR,
    NOTO_SANS_ARABIC_REGULAR,
    NOTO_SANS_BENGALI_REGULAR,
    NOTO_SANS_DEVANAGARI_REGULAR,
    NOTO_SANS_GUJARATI_REGULAR,
    NOTO_SANS_GURMUKHI_REGULAR,
    NOTO_SANS_HEBREW_REGULAR,
    NOTO_SANS_KANNADA_REGULAR,
    NOTO_SANS_MALAYALAM_REGULAR,
    NOTO_SANS_MYANMAR_REGULAR,
    NOTO_SANS_TAMIL_REGULAR,
    NOTO_SANS_TELUGU_REGULAR,
    NOTO_SANS_THAI_REGULAR,
    NOTO_SANS_ARABIC_BOLD,
    NOTO_SANS_BENGALI_BOLD,
    NOTO_SANS_DEVANAGARI_BOLD,
    NOTO_SANS_GUJARATI_BOLD,
    NOTO_SANS_GURMUKHI_BOLD,
    NOTO_SANS_HEBREW_BOLD,
    NOTO_SANS_KANNADA_BOLD,
    NOTO_SANS_MALAYALAM_BOLD,
    NOTO_SANS_MYANMAR_BOLD,
    NOTO_SANS_TAMIL_BOLD,
    NOTO_SANS_TELUGU_BOLD,
    NOTO_SANS_THAI_BOLD,
];

// SF
// sourced from https://github.com/sahibjotsaggu/San-Francisco-Pro-Fonts
pub const SF_PRO_TEXT_REGULAR: &[u8] = include_bytes!("../fonts/SF-Pro-Text-Regular.otf");
pub const SF_PRO_TEXT_BOLD: &[u8] = include_bytes!("../fonts/SF-Pro-Text-Bold.otf");
pub const SF_PRO_TEXT_ITALIC: &[u8] = include_bytes!("../fonts/SF-Pro-Text-RegularItalic.otf");
pub const SF_MONO_REGULAR: &[u8] = include_bytes!("../fonts/SF-Mono-Regular.otf");

pub const SF: [&[u8]; 4] = [
    SF_PRO_TEXT_REGULAR,
    SF_PRO_TEXT_BOLD,
    SF_PRO_TEXT_ITALIC,
    SF_MONO_REGULAR,
];

// symbols
pub const NERD_FONTS_MONO_SYMBOLS: &[u8] = include_bytes!("../fonts/SymbolsNerdFontMono-Regular.ttf");
pub const TWEMOJI_MOZILLA: &[u8] = include_bytes!("../fonts/TwemojiMozilla.ttf");
pub const PHOSPHOR: &[u8] = include_bytes!("../fonts/Phosphor.ttf");

pub const SYMBOLS: [&[u8]; 3] = [NERD_FONTS_MONO_SYMBOLS, TWEMOJI_MOZILLA, PHOSPHOR];
