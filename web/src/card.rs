use std::clone;

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

    let current_type_string = current_type.to_string();
    let future = use_resource(use_reactive!(|(current_type_string)| async move {
        reqwest::get(format!(
            "http://localhost:8081/{current_type_string}/words.json"
        ))
        .await
        .unwrap()
        .json::<Vec<KanaCard>>()
        .await
    }));

    let mut index = use_signal(|| 0);
    let kana = kana.clone();

    match future.read_unchecked().as_ref() {
        Some(Ok(response)) => {
            log::info!("{response:?}");

            let filtered_response = &response
                .iter()
                .filter(|v| match current_type {
                    KanaType::Hiragana => v.kana.contains(&kana.hiragana),
                    KanaType::Katakana => v.kana.contains(&kana.katakana),
                })
                .collect::<Vec<_>>();

            log::info!("{filtered_response:?}");

            // Not exist
            if filtered_response.is_empty() {
                return rsx! {  };
            }

            let total = filtered_response.len();

            rsx! {
                style { {include_str!("../public/card.css")} }
                div { class: "card", id: "{kana_key}-card",
                    div { class: "card-left",
                        {
                            match current_type {
                                KanaType::Hiragana=>rsx! { "{kana.hiragana}" },
                                KanaType::Katakana=>rsx! { "{kana.katakana}" },
                            }
                        },
                        br {}
                        small { class: "card-left-romaji", "{kana.romaji}" }
                    }
                    div { class: "card-right",
                        {
                            if index() > total - 1 { index.set(total - 1)};
                            let kana_card = filtered_response[index()].clone();
                            let kana = kana_card.kana.clone();
                        
                            rsx! {
                                div {
                                    class: "card-right-item",
                                    img {
                                        src: "{kana_card.src}"
                                    }
                                    div {
                                        div {
                                            "{kana}"
                                        }
                                        div {
                                            "{kana_card.english}"
                                        }
                                    }
                                }
                            }
                        },
                        div { class: "card-right-nav",
                            button { onclick: move |_| if index() > 0 { index -= 1 } else { index.set(total - 1) },
                                "⇦"
                            }
                            span { "{index() + 1}/{total}" }
                            button { onclick: move |_| if index() < total - 1 { index += 1 } else { index.set(0) },
                                "⇨"
                            }
                        }
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
