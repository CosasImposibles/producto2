use reqwest::blocking::Client;
use serde_json::Value;

fn translate(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q={}",
        urlencoding::encode(text)
    );
    let resp = Client::new().get(&url).send()?.text()?;
    let parsed: Value = serde_json::from_str(&resp)?;
    let translated = parsed[0][0][0].as_str().unwrap_or("");
    Ok(translated.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wsdl_url = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    let body = r#"<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <soap:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">
            <soap:Body>
                <NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">
                    <ubiNum>10</ubiNum>
                </NumberToWords>
            </soap:Body>
        </soap:Envelope>"#;

    let client = Client::new();
    let resp = client
        .post(wsdl_url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords")
        .body(body)
        .send()?;

    let text = resp.text()?;
    let start = text.find("<NumberToWordsResult>").unwrap_or(0) + "<NumberToWordsResult>".len();
    let end = text.find("</NumberToWordsResult>").unwrap_or(text.len());
    let english = text[start..end].trim();

    let spanish = translate(english)?;
    println!("{}", spanish);
    Ok(())
}
