use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Kana, KanaType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KanaCard {
    romaji: String,
    meaning: String,
    kana: String,
    kanji: String,
    src: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct KanaCardComponentProps {
    pub current_type: KanaType,
    pub kana_key: String,
    pub kana: Kana,
}

pub fn KanaCardComponent(props: KanaCardComponentProps) -> Element {
    let current_type = props.current_type;
    let kana_key = props.kana_key.clone();
    let kana = props.kana.clone();
    let mut index = use_signal(|| 0);

    let future = use_resource(use_reactive!(|(current_type)| async move {
        reqwest::get(format!("http://localhost:8081/{current_type}/words.json"))
            .await
            .unwrap()
            .json::<Vec<KanaCard>>()
            .await
    }));

    match future.read_unchecked().as_ref() {
        Some(Ok(response)) => {
            let mut filtered_response = response
                .iter()
                .filter(|v| match current_type {
                    KanaType::Hiragana => v.kana.contains(&kana.hiragana),
                    KanaType::Katakana => v.kana.contains(&kana.katakana),
                })
                .collect::<Vec<_>>();

            filtered_response.sort_by_key(|v| v.kana.len());

            match current_type {
                KanaType::Hiragana => {
                    filtered_response.sort_by_key(|v| !v.kana.starts_with(kana.hiragana.as_str()))
                }
                KanaType::Katakana => {
                    filtered_response.sort_by_key(|v| !v.kana.starts_with(kana.katakana.as_str()))
                }
            }

            // Not exist
            if filtered_response.is_empty() {
                return rsx! {  };
            }

            let total = filtered_response.len() as u8;

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
                            let kana_card = filtered_response[index() as usize].clone();
                            let kana = kana_card.kana.clone();
                        
                            rsx! {
                                div { class: "card-right-item",
                                    img {
                                        src: "{kana_card.src}"
                                    }
                                    div { class: "card-detail",
                                        div {
                                            "{kana}"
                                        }
                                        div {
                                            "{kana_card.romaji}"
                                        }
                                        div {
                                            class: "card-meaning",
                                            "{kana_card.meaning}"
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
