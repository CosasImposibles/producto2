package main

import (
    "fmt"
    "log"
    "net/http"
    "github.com/hooklift/gowsdl/soap"
)

func main() {
    wsdl := "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL"
    client, err := soap.NewClient(wsdl)
    if err != nil {
        log.Fatal(err)
    }

    n := 10
    req := map[string]interface{}{"ubiNum": n}
    var res struct { NumberToWordsResult string `xml:"NumberToWordsResult"` }

    err = client.Call("NumberToWords", req, &res)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(res.NumberToWordsResult)
}
