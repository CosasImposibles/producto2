require 'soap/wsdlDriver'

wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL'
client = SOAP::WSDLDriverFactory.new(wsdl).create_rpc_driver

n = ARGV[0] || '10'
response = client.NumberToWords('ubiNum' => n.to_i)
puts response.NumberToWordsResult
