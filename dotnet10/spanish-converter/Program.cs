using System;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Hosting;

var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/", async (HttpRequest request) =>
{
    var nValue = request.Query["n"].ToString();
    if (!int.TryParse(nValue, out var value))
    {
        value = 10;
    }

    var english = await CallSoapNumberToWordsAsync(value);
    if (english == null)
    {
        return Results.Problem("Error al llamar al servicio SOAP.");
    }

    var spanish = await TranslateTextAsync(english);
    if (spanish == null)
    {
        return Results.Problem("Error al traducir el texto.");
    }

    return Results.Text(spanish);
});

app.Run();

static async Task<string?> CallSoapNumberToWordsAsync(int n)
{
    using var httpClient = new HttpClient();
    var soapUri = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso";
    var soapBody = $"<?xml version=\"1.0\" encoding=\"utf-8\"?>" +
        "<soap:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" " +
        "xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" " +
        "xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">" +
        "<soap:Body>" +
        "<NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">" +
        $"<ubiNum>{n}</ubiNum>" +
        "</NumberToWords>" +
        "</soap:Body>" +
        "</soap:Envelope>";

    using var request = new HttpRequestMessage(HttpMethod.Post, soapUri);
    request.Headers.Accept.Add(new MediaTypeWithQualityHeaderValue("text/xml"));
    request.Headers.Add("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords");
    request.Content = new StringContent(soapBody, Encoding.UTF8, "text/xml");

    try
    {
        using var response = await httpClient.SendAsync(request);
        response.EnsureSuccessStatusCode();
        var responseText = await response.Content.ReadAsStringAsync();
        return ExtractSoapResult(responseText, "NumberToWordsResult");
    }
    catch
    {
        return null;
    }
}

static async Task<string?> TranslateTextAsync(string text)
{
    using var httpClient = new HttpClient();
    var url = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" +
              Uri.EscapeDataString(text);

    try
    {
        using var response = await httpClient.GetAsync(url);
        response.EnsureSuccessStatusCode();
        var json = await response.Content.ReadAsStringAsync();
        return ExtractTranslationResult(json);
    }
    catch
    {
        return null;
    }
}

static string? ExtractSoapResult(string xml, string tagName)
{
    var openTag = $"<{tagName}>";
    var closeTag = $"</{tagName}>";
    var start = xml.IndexOf(openTag, StringComparison.Ordinal);
    if (start < 0)
    {
        openTag = $"<m:{tagName}>";
        closeTag = $"</m:{tagName}>";
        start = xml.IndexOf(openTag, StringComparison.Ordinal);
    }

    if (start < 0)
    {
        return null;
    }

    start += openTag.Length;
    var end = xml.IndexOf(closeTag, start, StringComparison.Ordinal);
    if (end < 0)
    {
        return null;
    }

    return xml[start..end].Trim();
}

static string? ExtractTranslationResult(string json)
{
    try
    {
        using var document = JsonDocument.Parse(json);
        var root = document.RootElement;
        if (root.ValueKind != JsonValueKind.Array || root.GetArrayLength() < 1)
            return null;

        var firstArray = root[0];
        if (firstArray.ValueKind != JsonValueKind.Array || firstArray.GetArrayLength() < 1)
            return null;

        var firstEntry = firstArray[0];
        if (firstEntry.ValueKind != JsonValueKind.Array || firstEntry.GetArrayLength() < 1)
            return null;

        return firstEntry[0].GetString();
    }
    catch
    {
        return null;
    }
}
