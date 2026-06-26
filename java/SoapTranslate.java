import java.io.*;
import java.net.*;
import java.nio.charset.StandardCharsets;

public class SoapTranslate {
    private static final String WSDL_URL = "https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL";

    public static String postSoap(String xml) throws IOException {
        URL url = new URL("https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
        HttpURLConnection conn = (HttpURLConnection) url.openConnection();
        conn.setDoOutput(true);
        conn.setRequestMethod("POST");
        conn.setRequestProperty("Content-Type", "text/xml; charset=utf-8");
        conn.setRequestProperty("SOAPAction", "http://www.dataaccess.com/webservicesserver/NumberToWords");

        try (OutputStream os = conn.getOutputStream()) {
            os.write(xml.getBytes(StandardCharsets.UTF_8));
        }

        try (InputStream is = conn.getInputStream();
             BufferedReader reader = new BufferedReader(new InputStreamReader(is, StandardCharsets.UTF_8))) {
            StringBuilder response = new StringBuilder();
            String line;
            while ((line = reader.readLine()) != null) {
                response.append(line);
            }
            return response.toString();
        }
    }

    public static String extractTag(String xml, String tag) {
        String open = "<" + tag + ">";
        String close = "</" + tag + ">";
        int start = xml.indexOf(open);
        if (start == -1) {
            open = "<m:" + tag + ">";
            close = "</m:" + tag + ">";
            start = xml.indexOf(open);
        }
        if (start == -1) return "";
        int end = xml.indexOf(close, start);
        if (end == -1) return "";
        return xml.substring(start + open.length(), end).trim();
    }

    public static String translateText(String text) throws IOException {
        String endpoint = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=" +
                URLEncoder.encode(text, StandardCharsets.UTF_8);
        URL url = new URL(endpoint);
        HttpURLConnection conn = (HttpURLConnection) url.openConnection();
        conn.setRequestMethod("GET");

        try (InputStream is = conn.getInputStream();
             BufferedReader reader = new BufferedReader(new InputStreamReader(is, StandardCharsets.UTF_8))) {
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
        int n = args.length > 0 ? Integer.parseInt(args[0]) : 10;
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

        String response = postSoap(soapRequest);
        String english = extractTag(response, "NumberToWordsResult");
        String spanish = translateText(english);
        System.out.println(spanish);
    }
}
