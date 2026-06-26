package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "net/url"
    "github.com/hooklift/gowsdl/soap"
)

func translateText(text string) (string, error) {
    url := "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" + url.QueryEscape(text)
    resp, err := http.Get(url)
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

func main() {
    wsdl := "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL"
    client, err := soap.NewClient(wsdl)
    if err != nil {
        log.Fatal(err)
    }

    n := 10
    result := ""
    req := map[string]interface{}{"ubiNum": n}
    var res struct{ NumberToWordsResult string `xml:"NumberToWordsResult"` }
    if err = client.Call("NumberToWords", req, &res); err != nil {
        log.Fatal(err)
    }
    result = res.NumberToWordsResult

    spanish, err := translateText(result)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(spanish)
}
