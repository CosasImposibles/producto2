using System;
using System.ServiceModel;
using System.ServiceModel.Channels;

namespace SoapClientApp
{
    [ServiceContract(Namespace = "http://www.dataaccess.com/webservicesserver/")]
    public interface INumberConversionSoap
    {
        [OperationContract(Action = "http://www.dataaccess.com/webservicesserver/NumberConversion.wso/NumberToWords", ReplyAction = "*")]
        string NumberToWords(string ubiNum);
    }

    class Program
    {
        static void Main(string[] args)
        {
            int n = args.Length > 0 && int.TryParse(args[0], out var parsed) ? parsed : 10;
            var binding = new BasicHttpBinding(BasicHttpSecurityMode.None);
            var endpoint = new EndpointAddress("https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
            var factory = new ChannelFactory<INumberConversionSoap>(binding, endpoint);
            var client = factory.CreateChannel();
            string result = client.NumberToWords(n.ToString());
            Console.WriteLine(result);
        }
    }
}
