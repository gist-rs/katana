use anyhow::bail;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Kana, KanaType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KanaCard {
    english: String,
    kana: String,
    kanji: String,
    src: String,
}

impl KanaCard {
    fn default() -> Self {
        KanaCard {
            english: "n/a".to_owned(),
            kana: "n/a".to_owned(),
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
    let kana_key = props.kana_key;
    let kana = props.kana;

    let kana_key = kana_key.clone();
    let current_type_string = current_type.to_string();
    let future = use_resource(use_reactive!(
        |(kana_key, current_type_string)| async move {
            reqwest::get(format!(
                "http://localhost:8081/{current_type_string}/{kana_key}.json"
            ))
            .await
            .unwrap()
            .json::<Vec<KanaCard>>()
            .await
        }
    ));

    match future.read_unchecked().as_ref() {
        Some(Ok(response)) => {
            log::info!("{response:?}");

            rsx! {
                style { {include_str!("../public/card.css")} }
                div { class: "card",
                    div { class: "card-left",
                        {
                            match current_type {
                                KanaType::Hiragana=>rsx! { "{kana.hiragana}" },
                                KanaType::Katakana=>rsx! { "{kana.katakana}" },
                            }
                        },
                        br {}
                        small { class: "card-left-romaji",
                            "{kana.romaji}"
                        }
                    }
                    {
                        response.iter().map(|kana_card: &KanaCard| {
                            let kana = kana_card.kana.clone();

                            rsx! {
                                div {
                                    class: "card-right",
                                    img {
                                        style: "padding: 8px;",
                                        max_width: "64px",
                                        max_height: "64px",
                                        src: "{kana_card.src}"
                                    }
                                    div {
                                        "{kana}"
                                    }
                                    div {
                                        "{kana_card.english}"
                                    }
                                }
                            }
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
