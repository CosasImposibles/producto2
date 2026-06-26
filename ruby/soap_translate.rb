require 'soap/wsdlDriver'
require 'google/cloud/translate'

wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL'
client = SOAP::WSDLDriverFactory.new(wsdl).create_rpc_driver

n = ARGV[0] || '10'
response = client.NumberToWords('ubiNum' => n.to_i)
english = response.NumberToWordsResult

translate = Google::Cloud::Translate.translation_v2_service do |config|
  config.credentials = ENV['GOOGLE_APPLICATION_CREDENTIALS'] if ENV['GOOGLE_APPLICATION_CREDENTIALS']
end
spanish = translate.translate(english, to: 'es').text
puts spanish
