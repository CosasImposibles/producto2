require 'sinatra'
require 'net/http'
require 'json'
require 'uri'

set :bind, '127.0.0.1'
set :port, 4567

WSDL_URL = 'http://www.dataaccess.com/webservicesserver/NumberConversion.wso'

NUMBER_WORDS = {
  0 => 'zero', 1 => 'one', 2 => 'two', 3 => 'three', 4 => 'four',
  5 => 'five', 6 => 'six', 7 => 'seven', 8 => 'eight', 9 => 'nine',
  10 => 'ten', 11 => 'eleven', 12 => 'twelve', 13 => 'thirteen', 14 => 'fourteen',
  15 => 'fifteen', 16 => 'sixteen', 17 => 'seventeen', 18 => 'eighteen', 19 => 'nineteen',
  20 => 'twenty', 30 => 'thirty', 40 => 'forty', 50 => 'fifty',
  60 => 'sixty', 70 => 'seventy', 80 => 'eighty', 90 => 'ninety'
}.freeze

def local_number_to_words(n)
  return NUMBER_WORDS[n] if NUMBER_WORDS.key?(n)
  return "twenty-#{NUMBER_WORDS[n - 20]}" if n < 30

  if n < 100
    tens = (n / 10) * 10
    units = n % 10
    return units.zero? ? NUMBER_WORDS[tens] : "#{NUMBER_WORDS[tens]} #{NUMBER_WORDS[units]}"
  end

  'number out of range'
end

def soap_to_english(n)
  soap_body = <<~XML
    <?xml version="1.0" encoding="utf-8"?>
    <soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
      <soap:Body>
        <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
          <ubiNum>#{n}</ubiNum>
        </NumberToWords>
      </soap:Body>
    </soap:Envelope>
  XML

  uri = URI.parse(WSDL_URL)
  http = Net::HTTP.new(uri.host, uri.port)
  request = Net::HTTP::Post.new(uri.request_uri)
  request['Content-Type'] = 'text/xml; charset=utf-8'
  request['SOAPAction'] = 'http://www.dataaccess.com/webservicesserver/NumberToWords'
  request.body = soap_body

  response = http.request(request)
  return local_number_to_words(n) unless response.is_a?(Net::HTTPSuccess)

  if response.body =~ %r{<NumberToWordsResult>(.*?)</NumberToWordsResult>}m
    return Regexp.last_match(1).strip
  end

  if response.body =~ %r{<m:NumberToWordsResult>(.*?)</m:NumberToWordsResult>}m
    return Regexp.last_match(1).strip
  end

  local_number_to_words(n)
end

def translate_text(english)
  uri = URI.parse("https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=#{URI.encode_www_form_component(english)}")
  response = Net::HTTP.get_response(uri)
  return english if !response.is_a?(Net::HTTPSuccess)

  data = JSON.parse(response.body)
  data.dig(0, 0, 0)
end

get '/' do
  n = (params['n'] || '10').to_i
  english = begin
    soap_to_english(n)
  rescue Net::OpenTimeout, Net::ReadTimeout, SocketError, StandardError
    local_number_to_words(n)
  end

  translate_text(english) || english
end
