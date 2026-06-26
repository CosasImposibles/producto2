using Google.Cloud.Translate.V3;
using System;
using System.ServiceModel;
using System.ServiceModel.Channels;

namespace SoapTranslateApp
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
            string english = client.NumberToWords(n.ToString());

            TranslationServiceClient translateClient = TranslationServiceClient.Create();
            var response = translateClient.TranslateText(
                parent: new LocationName("YOUR_PROJECT_ID", "global"),
                contents: { english },
                targetLanguageCode: "es");
            Console.WriteLine(response.Translations[0].TranslatedText);
        }
    }
}
