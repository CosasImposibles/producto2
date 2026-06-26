import express from 'express';
import soap from 'soap';
import translate from '@vitalets/google-translate-api';

const app = express();
const port = process.env.PORT || 3000;
const wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';

app.get('/', async (req, res) => {
  const n = parseInt(req.query.n || '10', 10);

  try {
    const client = await soap.createClientAsync(wsdl);
    const result = await client.NumberToWordsAsync({ ubiNum: n });
    const english = result[0].NumberToWordsResult;
    const translation = await translate(english, { to: 'es' });
    res.send(translation.text);
  } catch (err) {
    res.status(500).send(`Error: ${err.message}`);
  }
});

app.listen(port, () => {
  console.log(`Servidor iniciado en http://localhost:${port}`);
});
