import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.io.OutputStream;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.InputStream;
import java.net.HttpURLConnection;
import java.net.InetSocketAddress;
import java.net.URI;
import java.net.URL;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;

public class SpanishConverter {
    private static String callSoap(int n) throws Exception {
        String soapRequest = "<?xml version=\"1.0\" encoding=\"utf-8\"?>" +
                "<soap:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" " +
                "xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" " +
                "xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">" +
                "<soap:Body>" +
                "<NumberToWords xmlns=\"http://www.dataaccess.com/webservicesserver/\">" +
                "<ubiNum>" + n + "</ubiNum>" +
                "</NumberToWords>" +
                "</soap:Body>" +
                "</soap:Envelope>";

        URL url = new URL("https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
        HttpURLConnection conn = (HttpURLConnection) url.openConnection();
        conn.setDoOutput(true);
        conn.setRequestMethod("POST");
        conn.setRequestProperty("Content-Type", "text/xml; charset=utf-8");
        conn.setRequestProperty("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords");

        conn.getOutputStream().write(soapRequest.getBytes(StandardCharsets.UTF_8));

        InputStream stream = conn.getResponseCode() >= 400 ? conn.getErrorStream() : conn.getInputStream();
        if (stream == null) {
            throw new IOException("Empty SOAP response");
        }

        StringBuilder response = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(stream, StandardCharsets.UTF_8))) {
            String line;
            while ((line = reader.readLine()) != null) {
                response.append(line);
            }
        }

        String xml = response.toString();
        String result = extractTag(xml, "NumberToWordsResult");
        if (result.isEmpty()) {
            throw new IOException("SOAP parse error: " + xml);
        }
        return result;
    }

    private static String extractTag(String xml, String tag) {
        String open = "<" + tag + ">";
        String close = "</" + tag + ">";
        int start = xml.indexOf(open);
        if (start == -1) {
            open = "<m:" + tag + ">";
            close = "</m:" + tag + ">";
            start = xml.indexOf(open);
        }
        if (start == -1) {
            return "";
        }
        int end = xml.indexOf(close, start);
        if (end == -1) {
            return "";
        }
        return xml.substring(start + open.length(), end).trim();
    }

    private static String translateText(String text) throws IOException {
        String endpoint = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" + URLEncoder.encode(text, StandardCharsets.UTF_8);
        URL url = new URL(endpoint);
        HttpURLConnection conn = (HttpURLConnection) url.openConnection();
        conn.setRequestMethod("GET");

        try (BufferedReader reader = new BufferedReader(new InputStreamReader(conn.getInputStream(), StandardCharsets.UTF_8))) {
            StringBuilder response = new StringBuilder();
            String line;
            while ((line = reader.readLine()) != null) {
                response.append(line);
            }
            String json = response.toString();
            int firstQuote = json.indexOf('"');
            int secondQuote = json.indexOf('"', firstQuote + 1);
            if (firstQuote >= 0 && secondQuote > firstQuote) {
                return json.substring(firstQuote + 1, secondQuote);
            }
            return "";
        }
    }

    public static void main(String[] args) throws Exception {
        HttpServer server = HttpServer.create(new InetSocketAddress(8080), 0);
        server.createContext("/", new HttpHandler() {
            @Override
            public void handle(HttpExchange exchange) throws IOException {
                URI requestURI = exchange.getRequestURI();
                String query = requestURI.getQuery();
                int n = 10;
                if (query != null && query.contains("n=")) {
                    try {
                        String value = query.split("n=")[1].split("&")[0];
                        n = Integer.parseInt(value);
                    } catch (Exception ignored) {
                    }
                }

                String english;
                String spanish;
                try {
                    english = callSoap(n);
                    spanish = translateText(english);
                } catch (Exception ex) {
                    String error = "Error: " + ex.getMessage();
                    exchange.sendResponseHeaders(500, error.getBytes(StandardCharsets.UTF_8).length);
                    try (OutputStream os = exchange.getResponseBody()) {
                        os.write(error.getBytes(StandardCharsets.UTF_8));
                    }
                    return;
                }

                exchange.sendResponseHeaders(200, spanish.getBytes(StandardCharsets.UTF_8).length);
                try (OutputStream os = exchange.getResponseBody()) {
                    os.write(spanish.getBytes(StandardCharsets.UTF_8));
                }
            }
        });
        server.start();
        System.out.println("Servidor escuchando en http://localhost:8080/?n=10");
    }
}
