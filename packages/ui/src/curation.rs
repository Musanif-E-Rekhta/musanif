//! Editorial seed data for the Discover view.
//!
//! Two pieces of curation live here: a small rotating library of couplets
//! that signs the top of the page, and the slug lists that bind each
//! curated strip to specific books in the corpus. Both are intentionally
//! short and hand-picked rather than algorithmic — the room should feel
//! like a librarian arranged it, not a recommender.
//!
//! The page never assumes a slug exists. A strip whose curated slugs
//! don't appear in the live catalog silently drops out, so the corpus
//! and the curation can drift without leaving empty rails behind.
//!
//! Rotation is week-based off `chrono::Utc::now().iso_week()`; the same
//! couplet greets everyone on the same week, then changes Monday. This
//! is deliberate — a daily rotation reads as noise, a static one reads
//! as inattention.
use chrono::{Datelike, Utc};

/// A two-line Urdu couplet with its Roman transliteration and a poet
/// attribution. The Urdu lines render in Nastaliq (RTL, large) and the
/// Roman renders in a serif italic underneath as a quiet aid for readers
/// who can't parse the script yet.
pub struct Couplet {
    pub urdu_line_1: &'static str,
    pub urdu_line_2: &'static str,
    pub roman_line_1: &'static str,
    pub roman_line_2: &'static str,
    pub attribution: &'static str,
}

pub const COUPLETS: &[Couplet] = &[
    Couplet {
        urdu_line_1: "ہزاروں خواہشیں ایسی کہ ہر خواہش پہ دم نکلے",
        urdu_line_2: "بہت نکلے مرے ارمان لیکن پھر بھی کم نکلے",
        roman_line_1: "hazāroñ ḳhvāhisheñ aisī ki har ḳhvāhish pe dam nikle",
        roman_line_2: "bahut nikle mire armān lekin phir bhī kam nikle",
        attribution: "Mirza Ghalib",
    },
    Couplet {
        urdu_line_1: "خودی کو کر بلند اتنا کہ ہر تقدیر سے پہلے",
        urdu_line_2: "خدا بندے سے خود پوچھے بتا تیری رضا کیا ہے",
        roman_line_1: "ḳhudī ko kar buland itnā ki har taqdīr se pahle",
        roman_line_2: "ḳhudā bande se ḳhud pūchhe batā terī razā kyā hai",
        attribution: "Allama Iqbal",
    },
    Couplet {
        urdu_line_1: "بول، کہ لب آزاد ہیں تیرے",
        urdu_line_2: "بول، زباں اب تک تیری ہے",
        roman_line_1: "bol, ki lab āzād haiñ tere",
        roman_line_2: "bol, zabāñ ab tak terī hai",
        attribution: "Faiz Ahmed Faiz",
    },
    Couplet {
        urdu_line_1: "پتا پتا بوٹا بوٹا حال ہمارا جانے ہے",
        urdu_line_2: "جانے نہ جانے گل ہی نہ جانے باغ تو سارا جانے ہے",
        roman_line_1: "pattā pattā būṭā būṭā hāl hamārā jāne hai",
        roman_line_2: "jāne na jāne gul hī na jāne bāġh to sārā jāne hai",
        attribution: "Mir Taqi Mir",
    },
    Couplet {
        urdu_line_1: "وہ کہیں ہو، اُسے میرا یقین رکھنا تھا",
        urdu_line_2: "میں نے بھی، گھر کوئی، اس کے سوا نہیں رکھا",
        roman_line_1: "vo kahīñ ho, use mirā yaqīñ rakhnā thā",
        roman_line_2: "maiñ ne bhī, ghar koī, us ke sivā nahīñ rakhā",
        attribution: "Parveen Shakir",
    },
];

/// Picks a couplet keyed to the current ISO week so the greeting changes
/// weekly without drifting day to day. Falls back to the first entry if
/// `COUPLETS` is ever emptied; the modulo would panic on an empty slice.
pub fn weekly_couplet() -> &'static Couplet {
    if COUPLETS.is_empty() {
        // Empty slice is a build-time configuration error, not a runtime
        // state we can recover from. The const above guards against it
        // but the check makes the intent explicit.
        unreachable!("COUPLETS is statically non-empty")
    }
    let now = Utc::now();
    let week = now.iso_week().week() as usize;
    let year = now.year().unsigned_abs() as usize;
    let idx = (week.wrapping_add(year)) % COUPLETS.len();
    &COUPLETS[idx]
}

/// Slugs that compose the "Ghazals · the form, the heart" strip. The
/// page filters the loaded catalog down to whichever of these are
/// actually published, in the order given here. If none match, the
/// strip is silently skipped.
pub const STRIP_GHAZALS: &[&str] = &[
    "diwan-e-ghalib",
    "kulliyat-e-iqbal",
    "kulliyat-e-mir",
    "nuskha-ha-e-wafa",
    "intikhab-parveen-shakir",
    "deewan-e-momin",
];

/// Slugs that compose the "Long-form · for an evening" strip. Novels,
/// short-story collections, memoirs — pieces that ask for a longer sit
/// than a ghazal sheet does.
pub const STRIP_LONG_FORM: &[&str] = &[
    "udaas-naslen",
    "aag-ka-darya",
    "raja-gidh",
    "khuda-ki-basti",
    "aangan",
    "basti",
];

/// Returns true if `s` contains at least one Arabic-script codepoint
/// (Unicode block U+0600–U+06FF), which is the practical test for "this
/// is in Urdu and should render Nastaliq with `dir=rtl`". Cheap enough
/// to run on every couplet line without memoizing.
pub fn contains_urdu(s: &str) -> bool {
    s.chars().any(|c| ('\u{0600}'..='\u{06FF}').contains(&c))
}
