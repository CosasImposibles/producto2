import soap from 'soap';
import translate from '@vitalets/google-translate-api';

const wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
const n = process.argv[2] || '10';

soap.createClient(wsdl, (err, client) => {
  if (err) throw err;
  client.NumberToWords({ ubiNum: parseInt(n, 10) }, async (err, result) => {
    if (err) throw err;
    const english = result.NumberToWordsResult;
    const res = await translate(english, { to: 'es' });
    console.log(res.text);
  });
});
