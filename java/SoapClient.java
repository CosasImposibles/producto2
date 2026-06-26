import javax.xml.namespace.QName;
import javax.xml.ws.Service;
import java.net.URL;

public class SoapClient {
    public static void main(String[] args) throws Exception {
        URL wsdlURL = new URL("https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL");
        QName SERVICE_NAME = new QName("http://www.dataaccess.com/webservicesserver/", "NumberConversion" );
        Service service = Service.create(wsdlURL, SERVICE_NAME);
        NumberConversionSoapType soap = service.getPort(NumberConversionSoapType.class);

        int n = args.length > 0 ? Integer.parseInt(args[0]) : 10;
        String result = soap.numberToWords(n);
        System.out.println(result);
    }
}
