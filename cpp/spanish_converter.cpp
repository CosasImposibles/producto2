#include <windows.h>
#include <winhttp.h>
#include <httplib.h>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

static std::string urlEncode(const std::string& value) {
    std::ostringstream escaped;
    escaped.fill('0');
    escaped << std::hex;
    for (unsigned char c : value) {
        if (isalnum(c) || c == '-' || c == '_' || c == '.' || c == '~') {
            escaped << c;
        } else {
            escaped << '%' << std::uppercase << std::setw(2) << int(c) << std::nouppercase;
        }
    }
    return escaped.str();
}

static std::string readResponse(HINTERNET request) {
    std::string response;
    DWORD size = 0;
    do {
        DWORD bytesAvailable = 0;
        if (!WinHttpQueryDataAvailable(request, &bytesAvailable)) break;
        if (!bytesAvailable) break;
        std::string buffer(bytesAvailable, '\0');
        if (!WinHttpReadData(request, buffer.data(), bytesAvailable, &size)) break;
        buffer.resize(size);
        response += buffer;
    } while (size > 0);
    return response;
}

static std::string translateToSpanish(const std::string& text) {
    std::wstring host = L"translate.googleapis.com";
    std::string path = "/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" + urlEncode(text);

    HINTERNET hSession = WinHttpOpen(L"Translate Client/1.0",
        WINHTTP_ACCESS_TYPE_NO_PROXY,
        WINHTTP_NO_PROXY_NAME,
        WINHTTP_NO_PROXY_BYPASS, 0);
    if (!hSession) return "";

    HINTERNET hConnect = WinHttpConnect(hSession, host.c_str(), INTERNET_DEFAULT_HTTPS_PORT, 0);
    if (!hConnect) {
        WinHttpCloseHandle(hSession);
        return "";
    }

    HINTERNET hRequest = WinHttpOpenRequest(hConnect, L"GET", std::wstring(path.begin(), path.end()).c_str(), NULL,
        WINHTTP_NO_REFERER,
        WINHTTP_DEFAULT_ACCEPT_TYPES,
        WINHTTP_FLAG_SECURE);
    if (!hRequest) {
        WinHttpCloseHandle(hConnect);
        WinHttpCloseHandle(hSession);
        return "";
    }

    if (!WinHttpSendRequest(hRequest,
        WINHTTP_NO_ADDITIONAL_HEADERS,
        0,
        WINHTTP_NO_REQUEST_DATA,
        0,
        0,
        0) ||
        !WinHttpReceiveResponse(hRequest, NULL)) {
        WinHttpCloseHandle(hRequest);
        WinHttpCloseHandle(hConnect);
        WinHttpCloseHandle(hSession);
        return "";
    }

    std::string response = readResponse(hRequest);
    WinHttpCloseHandle(hRequest);
    WinHttpCloseHandle(hConnect);
    WinHttpCloseHandle(hSession);

    auto firstQuote = response.find('"');
    if (firstQuote == std::string::npos) return "";
    auto secondQuote = response.find('"', firstQuote + 1);
    if (secondQuote == std::string::npos) return "";
    return response.substr(firstQuote + 1, secondQuote - firstQuote - 1);
}

static std::string callSoapNumberToWords(int n) {
    std::wstring host = L"www.dataaccess.com";
    std::wstring path = L"/webservicesserver/NumberConversion.wso";
    std::string soapBody = R"(<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>)" + std::to_string(n) + R"(</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>)";

    HINTERNET hSession = WinHttpOpen(L"SOAP Client/1.0",
        WINHTTP_ACCESS_TYPE_NO_PROXY,
        WINHTTP_NO_PROXY_NAME,
        WINHTTP_NO_PROXY_BYPASS, 0);
    if (!hSession) return "";

    HINTERNET hConnect = WinHttpConnect(hSession, host.c_str(), INTERNET_DEFAULT_HTTPS_PORT, 0);
    if (!hConnect) {
        WinHttpCloseHandle(hSession);
        return "";
    }

    HINTERNET hRequest = WinHttpOpenRequest(hConnect, L"POST", path.c_str(), NULL,
        WINHTTP_NO_REFERER,
        WINHTTP_DEFAULT_ACCEPT_TYPES,
        WINHTTP_FLAG_SECURE);
    if (!hRequest) {
        WinHttpCloseHandle(hConnect);
        WinHttpCloseHandle(hSession);
        return "";
    }

    std::wstring headers = L"Content-Type: text/xml; charset=utf-8\r\nSOAPAction: \"http://www.dataaccess.com/webservicesserver/NumberToWords\"\r\n";
    WinHttpAddRequestHeaders(hRequest, headers.c_str(), -1L, WINHTTP_ADDREQ_FLAG_ADD);

    if (!WinHttpSendRequest(hRequest,
        WINHTTP_NO_ADDITIONAL_HEADERS,
        0,
        (LPVOID)soapBody.c_str(),
        (DWORD)soapBody.size(),
        (DWORD)soapBody.size(),
        0) ||
        !WinHttpReceiveResponse(hRequest, NULL)) {
        WinHttpCloseHandle(hRequest);
        WinHttpCloseHandle(hConnect);
        WinHttpCloseHandle(hSession);
        return "";
    }

    std::string response = readResponse(hRequest);
    WinHttpCloseHandle(hRequest);
    WinHttpCloseHandle(hConnect);
    WinHttpCloseHandle(hSession);

    std::string startTag = "<NumberToWordsResult>";
    std::string endTag = "</NumberToWordsResult>";
    auto start = response.find(startTag);
    if (start == std::string::npos) {
        startTag = "<m:NumberToWordsResult>";
        endTag = "</m:NumberToWordsResult>";
        start = response.find(startTag);
    }
    auto end = response.find(endTag);
    if (start != std::string::npos && end != std::string::npos && end > start) {
        start += startTag.length();
        return response.substr(start, end - start);
    }
    return "";
}

int main() {
    using namespace httplib;
    Server svr;

    svr.Get("/", [](const Request& req, Response& res) {
        int n = 10;
        if (req.has_param("n")) {
            n = std::stoi(req.get_param_value("n"));
        }

        std::string english = callSoapNumberToWords(n);
        std::string spanish = english.empty() ? "" : translateToSpanish(english);
        if (spanish.empty()) {
            res.status = 500;
            res.set_content("Error generating Spanish translation", "text/plain; charset=utf-8");
            return;
        }

        res.set_content(spanish, "text/plain; charset=utf-8");
    });

    std::cout << "Servidor iniciado en http://localhost:8080" << std::endl;
    svr.listen("0.0.0.0", 8080);
    return 0;
}
