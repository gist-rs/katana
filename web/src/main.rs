#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

pub fn TableSwitcher() -> Element {
    rsx! { div { style: "display: flex-grid" } }
}

#[derive(Debug, Eq, PartialEq, EnumString, Display, Clone, Copy)]
enum KanaType {
    #[strum(serialize = "hiragana", to_string = "hiragana")]
    Hiragana,
    #[strum(serialize = "katakana", to_string = "katakana")]
    Katakana,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct Kana {
    romaji: String,
    r#type: String,
    hiragana: String,
    katakana: String,
}

#[derive(PartialEq, Props, Clone, Copy)]
struct KanaSwitcherProps {
    kana_type: KanaType,
}

fn get_kana_display_name(current_type: &KanaType, kana_type: &KanaType) -> String {
    if current_type == kana_type {
        format!("✅ {}", current_type)
    } else {
        format!("{}", current_type)
    }
}

fn KanaSwitcher(props: KanaSwitcherProps) -> Element {
    // context -------------------------------------------

    let mut config_signal = consume_context::<Signal<AppConfig>>();

    // model -------------------------------------------

    let future = use_resource(|| async move {
        reqwest::get("http://localhost:8081/kana.json")
            .await
            .unwrap()
            .json::<Vec<Kana>>()
            .await
    });

    // css -------------------------------------------

    let header_top_style = r#"
        display: inline-block;
        padding: 0.1em;
        white-space: nowrap;
        width: 2.3em;
        line-height: 2.3em;
        text-align: center;
    "#;

    let header_left_style = &header_top_style;

    let container_style = r#"
        display: flex;
        gap: 10px;
        row-gap: 10px;
        width: 480px;
        flex-wrap: wrap;
    "#;

    let text_style = r#"
        background-color: #eeeeee;
        display: inline-block;
        padding: 0.1em;
        white-space: nowrap;
        width: 2.3em;
        height: 2.3em;
        text-align: center;
    "#;

    let text_romaji_style = r#"
        display: block;
        font-size: x-small;
        color: #aaaaaa;
    "#;

    // state -------------------------------------------

    let mut kana_hashmap_signal = use_signal(HashMap::<String, Kana>::new);
    let mut kana_focus_state = use_signal(|| "-".to_string());

    // render -------------------------------------------

    let current_type = &props.kana_type;
    let kana_hashmap = kana_hashmap_signal();

    rsx! {
        if kana_hashmap.is_empty() {
            match future.read_unchecked().as_ref() {
                Some(Ok(response)) => {
                    let mut kana_hashmap: HashMap<String, Kana> = HashMap::new();
                    response.iter().for_each(|kana: &Kana| {
                        let key = match kana.romaji.as_str() {
                            "shi" => "si".to_owned(),
                            "chi" => "ti".to_owned(),
                            "tsu" => "tu".to_owned(),
                            "sha" => "sya".to_owned(),
                            "shu" => "syu".to_owned(),
                            "sho" => "syo".to_owned(),
                            "cha" => "tya".to_owned(),
                            "chu" => "tyu".to_owned(),
                            "cho" => "tyo".to_owned(),
                            "n" => "Nu".to_owned(),
                            "ja" => "zya".to_owned(),
                            "ji" => "zi".to_owned(),
                            "ju" => "zyu".to_owned(),
                            "jo" => "zyo".to_owned(),
                            "ji (dji)" => "di".to_owned(),
                            "zu (dzu)" => "du".to_owned(),
                            "ja (dja)" => "dya".to_owned(),
                            "ju (dju)" => "dyu".to_owned(),
                            "jo (djo)" => "dyo".to_owned(),
                            romaji => romaji.to_owned(),
                        };
                        kana_hashmap.insert(key, Kana { ..kana.clone() });
                    });
            
                    kana_hashmap_signal.set(kana_hashmap);
            
                    rsx! { div { "ok" } }
                }
                Some(Err(error)) => rsx! { div { "Loading failed: {error}" } },
                None => rsx! { div { "Loading..." } },
            }
        }

        div {
            button {
                onclick: move |_| {
                    config_signal
                        .set(AppConfig {
                            kana_type: KanaType::Hiragana,
                        });
                },
                { get_kana_display_name(&KanaType::Hiragana, current_type) }
            }
            button {
                onclick: move |_| {
                    config_signal
                        .set(AppConfig {
                            kana_type: KanaType::Katakana,
                        });
                },
                { get_kana_display_name(&KanaType::Katakana, current_type) }
            }
        }
        div { style: "{container_style}",
            {
            
                ["", "k", "s", "t", "n", "h", "m", "y", "r", "w", "N", "g" , "z", "d", "b", "p"].iter().enumerate().map(|(j,f)| rsx! {
                    {
                        ["", "a", "i", "u", "e", "o", "ya", "yu", "yo"].iter().enumerate().map(|(i, e)| rsx! {
                            if i==0 {
                                div { style: "{header_left_style}", "{f}" }
                            } else if j==0 {
                                div { style: "{header_top_style}", "{e}" }
                            } else {
                                {
                                    let kana_key = format!("{f}{e}");
                                    let maybe_kana = kana_hashmap.get(&kana_key);
            
                                    match maybe_kana {
                                        Some(kana) => rsx!{
                                            div {
                                                // onclick: move |_| {
                                                //     kana_focus_state.set(kana.romaji.clone());
                                                // },
                                                style: "{text_style}",
                                                {
                                                    match current_type {
                                                        KanaType::Hiragana=>rsx! { "{kana.hiragana}"},
                                                        KanaType::Katakana=>rsx! { "{kana.katakana}"},
                                                    }
                                                },
                                                br {},
                                                small { style: "{text_romaji_style}", "{kana.romaji}"},
                                            }
                                        },
                                        None=> rsx!{
                                            div { style: "{text_style}", "" }
                                        }
                                    }
                                }
                            }
                        })
                    }
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
struct AppConfig {
    kana_type: KanaType,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            kana_type: KanaType::Hiragana,
        }
    }
}

fn App() -> Element {
    use_context_provider(|| Signal::new(AppConfig::default()));
    let config_signal = consume_context::<Signal<AppConfig>>();

    log::info!("{:#?}", config_signal);
    rsx! { KanaSwitcher { kana_type: config_signal().kana_type } }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("launch");
    launch(App);
}
