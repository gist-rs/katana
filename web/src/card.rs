use std::collections::HashMap;

use anyhow::bail;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Kana, KanaType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KanaCard {
    english: String,
    hiragana: Option<String>,
    katakana: Option<String>,
    kanji: String,
    src: String,
}

impl KanaCard {
    fn default() -> Self {
        KanaCard {
            english: "n/a".to_owned(),
            hiragana: Some("n/a".to_owned()),
            katakana: Some("n/a".to_owned()),
            kanji: "n/a".to_owned(),
            src: "n/a".to_owned(),
        }
    }
}

pub async fn get_kana_card_data(kana_key: String) -> anyhow::Result<Vec<KanaCard>> {
    // Get metadata
    match reqwest::get(format!("http://localhost:8081/{kana_key}.json"))
        .await
        .unwrap()
        .json::<Vec<KanaCard>>()
        .await
    {
        Ok(response) => Ok(response),
        Err(error) => bail!("Failed to load:{}, Error: {error}", kana_key),
    }
}

pub struct KanaCardComponentProps<'a> {
    pub current_type: &'a KanaType,
    pub kana_key: String,
    pub kana: Kana,
}

pub fn KanaCardComponent(props: KanaCardComponentProps) -> Element {
    let current_type = props.current_type;
    let kana = props.kana;

    let card_style: &str = r#"
        display: flex;
        width: 100%;
    "#;

    let text_expand_style = r#"
        transition: all .3s;
        display: block;
        width: 50%;
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

    let text_expand_example_style = r#"
        display: flex;
        flex-direction: column;
        width: 50%;
        align-items: center;
        justify-content: center;
        background-color: #eeeeee;
    "#;

    log::info!("{kana:?}");

    // let kana_key = kana_key.as_str();
    let future = use_resource(|| async move {
        log::info!("reqwest.....");
        // reqwest::get(format!("http://localhost:8081/{kana_key}.json"))
        reqwest::get(format!("http://localhost:8081/ka.json"))
            // reqwest::get("http://localhost:8081/kana.json")
            .await
            .unwrap()
            .json::<Vec<KanaCard>>()
            .await
    });

    match future.read_unchecked().as_ref() {
        Some(Ok(response)) => {
            log::info!("{response:?}");

            rsx! {
                div { style: "{card_style}",
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
                    {
                        response.iter().filter(|v| {
                            match current_type {
                                KanaType::Hiragana=>v.hiragana.is_some(),
                                KanaType::Katakana=>v.katakana.is_some(),
                            }
                        }).map(|kana_card: &KanaCard| {
                            let hiragana = kana_card.hiragana.clone().unwrap_or("".to_owned());
                            let katakana = kana_card.katakana.clone().unwrap_or("".to_owned());
                    
                            rsx! {
                            div {
                                style: "{text_expand_example_style}",
                                img {
                                    style: "padding: 8px;",
                                    max_width: "64px",
                                    max_height: "64px",
                                    src: "{kana_card.src}"
                                }
                                div {
                                    "{hiragana}"
                                }
                                div {
                                    "{katakana}"
                                }
                                div {
                                    "{kana_card.english}"
                                }
                            }}
                        })
                    }
                }
            }
        }
        _ => {
            log::info!("hmmmm");
            rsx! {
                div { "" }
            }
        }
    }
}
