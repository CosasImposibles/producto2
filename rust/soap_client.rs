use reqwest::blocking::Client;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NumberToWordsResponse {
    #[serde(rename = "NumberToWordsResult")]
    pub NumberToWordsResult: String,
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
    let resp = client.post(wsdl_url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords")
        .body(body)
        .send()?;

    let text = resp.text()?;
    let start = text.find("<NumberToWordsResult>").unwrap_or(0) + "<NumberToWordsResult>".len();
    let end = text.find("</NumberToWordsResult>").unwrap_or(text.len());
    let result = &text[start..end];
    println!("{}", result);
    Ok(())
}
