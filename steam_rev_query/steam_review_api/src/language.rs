#[cfg(feature = "convenience_structs")]
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Languages as represented by the Steam API.
/// Source: https://partner.steamgames.com/doc/store/localization
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Language {
    All,
    Arabic,
    Bulgarian,
    SimplifiedChinese,
    TraditionalChinese,
    Czech,
    Danish,
    Dutch,
    English,
    Finnish,
    French,
    German,
    Greek,
    Hungarian,
    Italian,
    Japanese,
    Korean,
    Norwegian,
    Polish,
    Portuguese,
    PortugueseBrazilian,
    Romanian,
    Russian,
    SpanishSpain,
    SpanishLatAm,
    Swedish,
    Thai,
    Turkish,
    Ukrainian,
    Vietnamese,
}

#[derive(Debug, Clone, Copy)]
pub struct LangParseError;

impl std::error::Error for LangParseError {}

impl Display for LangParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tried to a parse an unlisted language. \
            Please report! Valve probably added in new languages since I last updated."
        )
    }
}

impl Language {
    /// String representation of how Language appears in queries.
    pub fn as_str(self) -> &'static str {
        use Language::*;
        match self {
            All => "all",
            Arabic => "arabic",
            Bulgarian => "bulgarian",
            SimplifiedChinese => "schinese",
            TraditionalChinese => "tchinese",
            Czech => "czech",
            Danish => "danish",
            Dutch => "dutch",
            English => "english",
            Finnish => "finnish",
            French => "french",
            German => "german",
            Greek => "greek",
            Hungarian => "hungarian",
            Italian => "italian",
            Japanese => "japanese",
            Korean => "koreana",
            Norwegian => "norwegian",
            Polish => "polish",
            Portuguese => "portuguese",
            PortugueseBrazilian => "brazilian",
            Romanian => "romanian",
            Russian => "russian",
            SpanishSpain => "spanish",
            SpanishLatAm => "latam",
            Swedish => "swedish",
            Thai => "thai",
            Turkish => "turkish",
            Ukrainian => "ukrainian",
            Vietnamese => "vietnamese",
        }
    }

    /// Shorthand language code as represented by the Steam web API.
    pub fn language_code(self) -> &'static str {
        use Language::*;
        match self {
            All => "all",
            Arabic => "ar",
            Bulgarian => "bg",
            SimplifiedChinese => "zh-CN",
            TraditionalChinese => "zh-TW",
            Czech => "cs",
            Danish => "da",
            Dutch => "nl",
            English => "en",
            Finnish => "fi",
            French => "fr",
            German => "de",
            Greek => "el el",
            Hungarian => "hu",
            Italian => "it",
            Japanese => "ja",
            Korean => "ko",
            Norwegian => "no",
            Polish => "pl",
            Portuguese => "pt",
            PortugueseBrazilian => "pt-BR",
            Romanian => "ro",
            Russian => "ru",
            SpanishSpain => "es",
            SpanishLatAm => "es-419",
            Swedish => "sv",
            Thai => "th",
            Turkish => "tr",
            Ukrainian => "uk",
            Vietnamese => "vn",
        }
    }

    /// Language's native name.
    pub fn native_name(self) -> &'static str {
        use Language::*;
        match self {
            All => "All",
            Arabic => "??????????????",
            Bulgarian => "?????????????????? ????????",
            SimplifiedChinese => "????????????",
            TraditionalChinese => "????????????",
            Czech => "??e??tina",
            Danish => "Dansk",
            Dutch => "Nederlands",
            English => "English",
            Finnish => "Suomi",
            French => "Fran??ais",
            German => "Deutsch",
            Greek => "????????????????",
            Hungarian => "Magyar",
            Italian => "Italiano",
            Japanese => "?????????",
            Korean => "?????????",
            Norwegian => "Norsk",
            Polish => "Polski",
            Portuguese => "Portugu??s",
            PortugueseBrazilian => "Portugu??s-Brasil",
            Romanian => "Rom??n??",
            Russian => "??????????????",
            SpanishSpain => "Espa??ol-Espa??a",
            SpanishLatAm => "Espa??ol-Latinoam??rica",
            Swedish => "Svenska",
            Thai => "?????????",
            Turkish => "T??rk??e",
            Ukrainian => "????????????????????",
            Vietnamese => "Ti???ng Vi???t",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Language {
    type Err = LangParseError;

    /// String slice to Language.
    /// Native names as well as shorthands are handled.
    ///
    /// ## Errors
    /// Returns [LangParseError] if an unsupported language is passed in.
    /// In other words, this function shouldn't fail until Valve adds in new
    /// languages...in which case you should let me know!
    fn from_str(s: &str) -> Result<Self, LangParseError> {
        use Language::*;
        match s {
            "all" => Ok(All),
            "arabic" | "??????????????" | "ar" => Ok(Arabic),
            "bulgarian" | "?????????????????? ????????" | "bg" => Ok(Bulgarian),
            "schinese" | "????????????" | "zh-CN" => Ok(SimplifiedChinese),
            "tchinese" | "????????????" | "zh-TW" => Ok(TraditionalChinese),
            "czech" | "??e??tina" | "cs" => Ok(Czech),
            "danish" | "Dansk" | "da" => Ok(Danish),
            "dutch" | "Nederlands" | "nl" => Ok(Dutch),
            "english" | "English" | "en" => Ok(English),
            "finnish" | "Suomi" | "fl" => Ok(Finnish),
            "french" | "Fran??ais" | "fr" => Ok(French),
            "german" | "Deutsch" | "de" => Ok(German),
            "greek" | "????????????????" | "el" => Ok(Greek),
            "hungarian" | "Magyar" | "hu" => Ok(Hungarian),
            "italian" | "Italiano" | "it" => Ok(Italian),
            "japanese" | "?????????" | "ja" => Ok(Japanese),
            "koreana" | "?????????" | "ko" => Ok(Korean),
            "norwegian" | "Norsk" | "no" => Ok(Norwegian),
            "polish" | "Polski" | "pl" => Ok(Polish),
            "portuguese" | "Portugu??s" | "pt" => Ok(Portuguese),
            "brazilian" | "Portugu??s-Brasil" | "pt-BR" => Ok(PortugueseBrazilian),
            "romanian" | "Rom??n??" | "ro" => Ok(Romanian),
            "russian" | "??????????????" | "ru" => Ok(Russian),
            "spanish" | "Espa??ol-Espa??a" | "es" => Ok(SpanishSpain),
            "latam" | "Espa??ol-Latinoam??rica" | "es-419" => Ok(SpanishLatAm),
            "swedish" | "Svenska" | "sv" => Ok(Swedish),
            "thai" | "?????????" | "th" => Ok(Thai),
            "turkish" | "T??rk??e" | "tr" => Ok(Turkish),
            "ukrainian" | "????????????????????" | "uk" => Ok(Ukrainian),
            "vietnamese" | "Ti???ng Vi???t" | "vn" => Ok(Vietnamese),
            _ => Err(LangParseError),
        }
    }
}

// Deserialize and Serialize
#[cfg(feature = "convenience_structs")]
impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Language, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<Language>().map_err(D::Error::custom)
    }
}

#[cfg(feature = "convenience_structs")]
impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(feature = "convenience_structs")]
#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::{
        value::{Error, StringDeserializer},
        IntoDeserializer,
    };

    #[test]
    fn good_parse() {
        let english: StringDeserializer<Error> = "english".to_owned().into_deserializer();
        assert_eq!(Language::deserialize(english), Ok(Language::English));
    }

    #[test]
    fn bad_parse() {
        let cat_lang: StringDeserializer<Error> = "meow talk".to_owned().into_deserializer();
        let _err = Language::deserialize(cat_lang).unwrap_err();
    }
}
