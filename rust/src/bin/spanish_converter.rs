use reqwest::blocking::Client;
use serde_json::Value;
use tiny_http::{Response, Server};
use std::error::Error;

fn soap_to_english(n: i32) -> Result<String, Box<dyn Error>> {
    let wsdl_url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    let body = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>{}</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>"#, n);

    let client = Client::new();
    let resp = client.post(wsdl_url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords")
        .body(body)
        .send()?;

    let text = resp.text()?;
    if let Some(start) = text.find("<NumberToWordsResult>") {
        let start = start + "<NumberToWordsResult>".len();
        let end = text.find("</NumberToWordsResult>").ok_or("missing end tag")?;
        return Ok(text[start..end].trim().to_string());
    }

    if let Some(start) = text.find("<m:NumberToWordsResult>") {
        let start = start + "<m:NumberToWordsResult>".len();
        let end = text.find("</m:NumberToWordsResult>").ok_or("missing end tag")?;
        return Ok(text[start..end].trim().to_string());
    }

    Ok(local_number_to_words(n))
}

fn translate(text: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q={}",
        urlencoding::encode(text)
    );
    let resp = Client::new().get(&url).send()?.text()?;
    let parsed: Value = serde_json::from_str(&resp)?;
    let translated = parsed[0][0][0].as_str().unwrap_or("");
    if translated.is_empty() {
        Ok(english_to_spanish(text))
    } else {
        Ok(translated.to_string())
    }
}

fn local_number_to_words(n: i32) -> String {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
    ];

    if (0..=20).contains(&n) {
        return words[n as usize].to_string();
    }

    if n < 30 {
        return format!("twenty {}", words[(n - 20) as usize]);
    }

    if n < 100 {
        let tens_words = ["zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
        let tens = (n / 10) as usize;
        let units = (n % 10) as usize;
        return if units == 0 {
            tens_words[tens].to_string()
        } else {
            format!("{} {}", tens_words[tens], words[units])
        };
    }

    "number out of range".to_string()
}

fn english_to_spanish(text: &str) -> String {
    match text.trim().to_lowercase().as_str() {
        "zero" => "cero".to_string(),
        "one" => "uno".to_string(),
        "two" => "dos".to_string(),
        "three" => "tres".to_string(),
        "four" => "cuatro".to_string(),
        "five" => "cinco".to_string(),
        "six" => "seis".to_string(),
        "seven" => "siete".to_string(),
        "eight" => "ocho".to_string(),
        "nine" => "nueve".to_string(),
        "ten" => "diez".to_string(),
        "eleven" => "once".to_string(),
        "twelve" => "doce".to_string(),
        "thirteen" => "trece".to_string(),
        "fourteen" => "catorce".to_string(),
        "fifteen" => "quince".to_string(),
        "sixteen" => "dieciséis".to_string(),
        "seventeen" => "diecisiete".to_string(),
        "eighteen" => "dieciocho".to_string(),
        "nineteen" => "diecinueve".to_string(),
        "twenty" => "veinte".to_string(),
        other if other.starts_with("twenty ") => {
            let suffix = &other[7..];
            format!("veinti{}", english_to_spanish(suffix))
        }
        other if other.starts_with("thirty ") => format!("treinta y {}", english_to_spanish(&other[7..])),
        other if other.starts_with("forty ") => format!("cuarenta y {}", english_to_spanish(&other[6..])),
        other if other.starts_with("fifty ") => format!("cincuenta y {}", english_to_spanish(&other[6..])),
        other if other.starts_with("sixty ") => format!("sesenta y {}", english_to_spanish(&other[6..])),
        other if other.starts_with("seventy ") => format!("setenta y {}", english_to_spanish(&other[8..])),
        other if other.starts_with("eighty ") => format!("ochenta y {}", english_to_spanish(&other[7..])),
        other if other.starts_with("ninety ") => format!("noventa y {}", english_to_spanish(&other[7..])),
        _ => text.to_string(),
    }
}

fn extract_query_n(query: &str) -> i32 {
    for part in query.split('&') {
        let mut kv = part.splitn(2, '=');
        if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
            if key == "n" {
                if let Ok(parsed) = value.parse::<i32>() {
                    return parsed;
                }
            }
        }
    }
    10
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server = Server::http("0.0.0.0:8080")?;
    println!("Servidor iniciado en http://localhost:8080");

    for request in server.incoming_requests() {
        let query = request.url().split('?').nth(1).unwrap_or("");
        let n = extract_query_n(query);

        let english = soap_to_english(n).unwrap_or_else(|_| local_number_to_words(n));
        let spanish = translate(&english).unwrap_or_else(|_| english_to_spanish(&english));

        let response = Response::from_string(spanish);
        let _ = request.respond(response);
    }

    Ok(())
}
