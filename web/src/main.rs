#![allow(non_snake_case)]

use std::collections::HashMap;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

mod card;
use card::KanaCardComponent;

use crate::card::KanaCardComponentProps;

pub fn TableSwitcher() -> Element {
    rsx! {
        div { style: "display: flex-grid" }
    }
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
    kana_hashmap_signal: Signal<HashMap<String, Kana>>,
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

    // css -------------------------------------------

    let header_top_style = r#"
        display: inline-block;
        white-space: nowrap;
        width: 3em;
        line-height: 3em;
        text-align: center;
    "#;

    let header_left_style = &header_top_style;

    let container_style = format!(
        r#"
            display: flex;
            gap: 10px;
            row-gap: 10px;
            flex-wrap: wrap;
            width: 540px;
            width: {}px
        "#,
        48 * 11
    );

    let text_style = r#"
        transition: all .3s;
        background-color: #dddddd;
        display: inline-block;
        white-space: nowrap;
        width: 48px;
        height: 48px;
        text-align: center;
        line-height: 2em;
        cursor: pointer;
    "#;

    let text_style_focus = r#"
        transition: all .3s;
        display: inline-block;
        background-color: #eeffee;
        white-space: nowrap;
        width: 48px;
        height: 48px;
        text-align: center;
        line-height: 2em;
        cursor: pointer;
    "#;

    let text_style_none = r#"
        transition: all .3s;
        background-color: #eeeeee;
        display: inline-block;
        white-space: nowrap;
        width: 48px;
        height: 48px;
        text-align: center;
        line-height: 2em;
    "#;

    let text_romaji_style = r#"
        display: block;
        font-size: small;
        color: #999999;
        line-height: 0.5em;
    "#;

    // state -------------------------------------------

    // let mut kana_hashmap_signal = use_signal(HashMap::<String, Kana>::new);
    let mut kana_focus_signal = use_signal(|| "-".to_string());

    // render -------------------------------------------

    let current_type = &props.kana_type;
    let kana_hashmap = props.kana_hashmap_signal.read();

    rsx! {
        if kana_hashmap.is_empty() {
            div { "loading..." }
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
                ["", "", "k", "s", "t", "n", "h", "m", "y", "r", "w", "N", "g" , "z", "d", "b", "p"].iter().enumerate().map(|(j,f)| rsx! {
                    {
                        ["", "a", "i", "u", "e", "o", "ya", "yu", "yo"].iter().enumerate().map(|(i, e)| rsx! {
                            if i==0 {
                                div { style: "{header_left_style}", "{f}" }
                            } else if j==0 {
                                div { style: "{header_top_style}", "{e}" }
                            } else {
                                {
                                    let kana_key = format!("{f}{e}");
                                    let maybe_kana = if j==1 && i>5 { None } else { kana_hashmap.get(&kana_key) };
                                    let kana_style = if kana_focus_signal() != kana_key { text_style } else { text_style_focus };
            
                                    match maybe_kana {
                                        Some(kana) => rsx!{
                                            div {
                                                onclick: {
                                                    move |_| {
                                                        let current_kana_key = kana_focus_signal();
                                                        if current_kana_key != kana_key {
                                                            kana_focus_signal.set(kana_key.to_owned());
                                                        } else {
                                                            kana_focus_signal.set("-".to_owned());
                                                        }
                                                    }
                                                },
                                                style: "{kana_style}",
                                                {
                                                    match current_type {
                                                        KanaType::Hiragana=>rsx! { "{kana.hiragana}" },
                                                        KanaType::Katakana=>rsx! { "{kana.katakana}" },
                                                    }
                                                },
                                                br {},
                                                small { style: "{text_romaji_style}", "{kana.romaji}" },
                                            }
                                        },
                                        None=> rsx!{
                                            div { style: "{text_style_none}", "" }
                                        }
                                    }
                                }
                            }
                        })
                    },
                    {
                        rsx! {
                            if j > 0 {
                                {
                                    let kana_key = kana_focus_signal();
                                    rsx! {
                                        {
                                            let maybe_kana = kana_hashmap.get(&kana_key);
            
                                            match maybe_kana {
                                                Some(kana) => {
                                                    if (j == 1 && kana_key.len() == 1 && ["a", "i", "u", "e", "o"].contains(&kana_key.as_str())) || j > 1 && kana_key.starts_with(f) {
                                                        KanaCardComponent(KanaCardComponentProps { current_type: *current_type, kana_key, kana: kana.clone() })
                                                    } else {
                                                        rsx! {}
                                                    }
                                                },
                                                None => rsx! {},
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
    let mut kana_hashmap_signal = use_signal(HashMap::<String, Kana>::new);

    // model -------------------------------------------

    let future = use_resource(|| async move {
        reqwest::get("http://localhost:8081/kana.json")
            .await
            .unwrap()
            .json::<Vec<Kana>>()
            .await
    });

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
        }
        Some(Err(error)) => log::error!("Loading failed: {error}"),
        None => log::error!("None..."),
    }

    log::info!("{:#?}", config_signal);
    rsx! {
        KanaSwitcher { kana_type: config_signal().kana_type, kana_hashmap_signal }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("launch");
    launch(App);
}
