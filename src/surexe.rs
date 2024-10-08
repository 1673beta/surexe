use anyhow::Result;
use curl::easy::{Easy, List};
use serde::Deserialize;
use termimad::MadSkin;

#[derive(Deserialize)]
struct Response {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Option<Content>, // contentフィールドが存在しない場合に備えてOptionに変更
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

pub fn post_gemini(parts: Vec<&str>, api_key: &str) -> Result<String> {
    let mut easy = Easy::new();
    easy.url(&format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key))?;
    easy.post(true)?;
    
    let mut list = List::new();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list)?;

    let data = format!(r#"{{"contents":[{{"parts":[{{"text":"explain this command. answer in japanese. {}"}}]}}]}}"#, parts.join(" "));
    easy.post_fields_copy(data.as_bytes())?;

    let mut res = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            res.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }

    let res_str = String::from_utf8(res)?;
    Ok(res_str)
}

pub fn display_response(res: &str) -> Result<()> {
    let parsed: Response = serde_json::from_str(res)?;
    let skin = MadSkin::default();
    for candidate in parsed.candidates {
        if let Some(content) = candidate.content {
            for part in content.parts {
                skin.print_text(&part.text);
            }
        } else {
            eprintln!("No content found in candidate");
        }
    }
    Ok(())
}