import soap from 'soap';

const wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
const n = process.argv[2] || '10';

soap.createClient(wsdl, (err, client) => {
  if (err) throw err;
  client.NumberToWords({ ubiNum: parseInt(n, 10) }, (err, result) => {
    if (err) throw err;
    console.log(result.NumberToWordsResult);
  });
});
