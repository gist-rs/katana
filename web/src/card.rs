use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Kana, KanaType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KanaCard {
    english: String,
    hiragana: String,
    kanji: String,
    src: String,
}

impl KanaCard {
    fn default() -> Self {
        KanaCard {
            english: "n/a".to_owned(),
            hiragana: "n/a".to_owned(),
            kanji: "n/a".to_owned(),
            src: "n/a".to_owned(),
        }
    }
}

pub async fn get_kana_card(kana_key: String) -> Vec<KanaCard> {
    // Get metadata
    reqwest::get(format!("http://localhost:8081/{kana_key}.json"))
        .await
        .unwrap()
        .json::<Vec<KanaCard>>()
        .await
        .unwrap_or_default()
}

pub struct KanaCardComponentProps<'a, 'b> {
    pub current_type: &'a KanaType,
    pub maybe_kana: Option<&'b Kana>,
}

#[component]
pub fn KanaCardComponent(props: KanaCardComponentProps) -> Element {
    let current_type = props.current_type;
    let maybe_kana = props.maybe_kana;

    let text_expand_style = r#"
        transition: all .3s;
        display: block;
        width: 100%;
        height: auto;
        text-align: center;
        line-height: 1.1em;
        font-size: 8em;
        background-color: #eeffee;
    "#;

    let text_expand_romaji_style = r#"
        display: block;
        font-size: medium;
        color: #aaaaaa;
        line-height: 2em;
    "#;

    match maybe_kana {
        Some(kana) => rsx! {
            div { style: "{text_expand_style}",
                {
                    match current_type {
                        KanaType::Hiragana=>rsx! { "{kana.hiragana}" },
                        KanaType::Katakana=>rsx! { "{kana.katakana}" },
                    }
                },
                br {}
                small { style: "{text_expand_romaji_style}", "{kana.romaji}" }
            }
        },
        None => rsx! { div { "" } },
    }
}
