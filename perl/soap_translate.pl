use strict;
use warnings;
use SOAP::Lite;
use HTTP::Tiny;
use JSON;
use URI::Escape;

my $n = shift || 10;
my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
my $soap = SOAP::Lite->service($wsdl);

my $result = $soap->NumberToWords({ ubiNum => $n });
my $english = $result->{NumberToWordsResult};

my $client = HTTP::Tiny->new();
my $q = uri_escape($english);
my $url = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=$q";
my $response = $client->get($url);

if ($response->{success}) {
    my $data = decode_json($response->{content});
    my $spanish = $data->[0][0][0];
    print "$spanish\n";
} else {
    die "Translation failed: $response->{status} $response->{reason}\n";
}
