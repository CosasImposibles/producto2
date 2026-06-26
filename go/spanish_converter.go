package main

import (
    "encoding/json"
    "fmt"
    "io"
    "log"
    "net/http"
    "net/url"
    "strconv"
    "strings"
)

func translateText(text string) (string, error) {
    endpoint := "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" + url.QueryEscape(text)
    resp, err := http.Get(endpoint)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()

    var result any
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return "", err
    }

    data, ok := result.([]any)
    if !ok || len(data) < 1 {
        return "", fmt.Errorf("unexpected response format")
    }

    first, ok := data[0].([]any)
    if !ok || len(first) < 1 {
        return "", fmt.Errorf("unexpected response format")
    }

    firstEntry, ok := first[0].([]any)
    if !ok || len(firstEntry) < 1 {
        return "", fmt.Errorf("unexpected response format")
    }

    translated, ok := firstEntry[0].(string)
    if !ok {
        return "", fmt.Errorf("unexpected response format")
    }
    return translated, nil
}

func callSoapNumberToWords(n int) (string, error) {
    soapURL := "https://www.dataaccess.com/webservicesserver/NumberConversion.wso"
    soapBody := fmt.Sprintf(`<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>%d</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>`, n)

    req, err := http.NewRequest(http.MethodPost, soapURL, strings.NewReader(soapBody))
    if err != nil {
        return "", err
    }
    req.Header.Set("Content-Type", "text/xml; charset=utf-8")
    req.Header.Set("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords")

    resp, err := http.DefaultClient.Do(req)
    if err != nil {
        return "", err
    }
    defer resp.Body.Close()

    raw, err := io.ReadAll(resp.Body)
    if err != nil {
        return "", err
    }

    if resp.StatusCode < 200 || resp.StatusCode >= 300 {
        return "", fmt.Errorf("HTTP Status %d: %s", resp.StatusCode, string(raw))
    }

    body := string(raw)
    startTag := "<NumberToWordsResult>"
    endTag := "</NumberToWordsResult>"
    start := strings.Index(body, startTag)
    if start == -1 {
        startTag = "<m:NumberToWordsResult>"
        endTag = "</m:NumberToWordsResult>"
        start = strings.Index(body, startTag)
    }
    if start == -1 {
        return "", fmt.Errorf("SOAP response parse error: missing NumberToWordsResult")
    }
    start += len(startTag)
    end := strings.Index(body[start:], endTag)
    if end == -1 {
        return "", fmt.Errorf("SOAP response parse error: missing NumberToWordsResult closing tag")
    }

    return strings.TrimSpace(body[start : start+end]), nil
}

func handler(w http.ResponseWriter, r *http.Request) {
    n := 10
    if qs := r.URL.Query().Get("n"); qs != "" {
        if parsed, err := strconv.Atoi(qs); err == nil {
            n = parsed
        }
    }

    english, err := callSoapNumberToWords(n)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }

    spanish, err := translateText(english)
    if err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }

    fmt.Fprint(w, spanish)
}

func main() {
    http.HandleFunc("/", handler)
    fmt.Println("Servidor iniciado en http://localhost:8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}
