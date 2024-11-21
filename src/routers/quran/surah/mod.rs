pub mod surah_add;
pub mod surah_delete;
pub mod surah_edit;
pub mod surah_list;
pub mod surah_view;

use std::hash::Hash;

use crate::{
    filter::{Filters, Order},
    models::QuranMushaf,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The quran text format Each word has its own uuid
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Text,
    Word,
}

impl Default for Format {
    fn default() -> Self {
        Self::Text
    }
}

// This is for Surah router
// which is faster than SimpleAyah in sorting
#[derive(Eq, Serialize, Clone, Debug)]
pub struct SimpleAyahSurah {
    pub number: u32,
    pub uuid: Uuid,
    pub sajdah: Option<String>,
}

// WARNING: Only hashing 'number' ?
// This can lead to collisions in hashmap if the number is not unique
impl Hash for SimpleAyahSurah {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl Ord for SimpleAyahSurah {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialEq for SimpleAyahSurah {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd for SimpleAyahSurah {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

/// The Ayah type that will return in the response
#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    pub number: u32,
    pub uuid: Uuid,
    pub sajdah: Option<String>,
}

/// it contains ayah info and the content
#[derive(Serialize, Clone, Debug)]
pub struct AyahWithText {
    #[serde(flatten)]
    pub ayah: SimpleAyah,
    pub text: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct AyahWithWords {
    #[serde(flatten)]
    pub ayah: SimpleAyah,
    pub words: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AyahTy {
    Text(AyahWithText),
    Words(AyahWithWords),
}

/// The final response body
#[derive(Serialize, Clone, Debug)]
pub struct QuranResponseData {
    #[serde(flatten)]
    surah: SingleSurahResponse,
    ayahs: Vec<AyahTy>,
}

/// the query for the /surah/{uuid}
/// example /surah/{uuid}?format=word
#[derive(Debug, Clone, Deserialize)]
pub struct GetSurahQuery {
    #[serde(default)]
    format: Format,

    lang_code: Option<String>,
}

/// The query needs the mushaf
/// for example /surah?mushaf=hafs
#[derive(Clone, Deserialize)]
pub struct SurahListQuery {
    lang_code: Option<String>,
    mushaf: String,
    sort: Option<String>,
    order: Option<Order>,

    from: Option<u64>,
    to: Option<u64>,
}

impl Filters for SurahListQuery {
    fn sort(&self) -> Option<String> {
        self.sort.clone()
    }

    fn order(&self) -> Option<Order> {
        self.order.clone()
    }

    fn from(&self) -> Option<u64> {
        self.from
    }

    fn to(&self) -> Option<u64> {
        self.to
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SurahName {
    pub arabic: String,
    pub pronunciation: Option<String>,
    pub translation_phrase: Option<String>,
    pub translation: Option<String>,
    pub transliteration: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahMushaf {
    pub uuid: Uuid,
    pub short_name: Option<String>,
    pub name: Option<String>,
    pub source: Option<String>,
}

impl From<QuranMushaf> for SingleSurahMushaf {
    fn from(value: QuranMushaf) -> Self {
        Self {
            uuid: value.uuid,
            short_name: value.short_name,
            name: value.name,
            source: value.source,
        }
    }
}

/// The response type for /surah/{id}
#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahResponse {
    pub uuid: Uuid,
    pub mushaf: SingleSurahMushaf,
    pub names: Vec<SurahName>,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub bismillah_text: Option<String>,
    pub number_of_ayahs: i64,
}

/// The response type for /surah
#[derive(Serialize, Clone, Debug)]
pub struct SurahListResponse {
    pub uuid: Uuid,
    pub number: i32,
    pub period: Option<String>,
    pub number_of_ayahs: i64,
    pub names: Vec<SurahName>,
}

// TODO: Remove number. number must be generated at api runtime
/// User request body type
#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SimpleSurah {
    pub name: String,
    pub name_pronunciation: Option<String>,
    pub name_translation_phrase: Option<String>,
    pub name_transliteration: Option<String>,
    pub period: Option<String>,
    pub number: i32,
    pub bismillah_status: bool,
    pub bismillah_as_first_ayah: bool,
    pub mushaf_uuid: Uuid,
}
