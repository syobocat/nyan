use wasm_bindgen::prelude::*;
use regex::Regex;

#[wasm_bindgen]
pub fn translate(text: &str) -> String {
    let re1 = Regex::new(r"(.+?)(である|だ|です)(。|\(|\s|$)").unwrap();
    let re2 = Regex::new(r"([えけせてねへめれ])ます(。|\(|\s|$)").unwrap();
    let re3 = Regex::new(r"(\S+?[^。\)\s])(。|\(|\s|$)").unwrap();
    let re4 = Regex::new(r"、にゃ").unwrap();
    let re5 = Regex::new(r"ないにゃ").unwrap();
    let re6 = Regex::new(r"ない").unwrap();

    let mut new: String;
    new = text.to_string();
    new = re1.replace_all(&new, "$1$3").to_mut().to_string();
    new = re2.replace_all(&new, "$1る$2").to_mut().to_string();
    new = re3.replace_all(&new, r"$1にゃ$2").to_mut().to_string();
    new = re4.replace_all(&new, "、").to_mut().to_string();
    new = re5.replace_all(&new, "にゃい").to_mut().to_string();
    new = re6.replace_all(&new, "にゃい").to_mut().to_string();
    return new;
}