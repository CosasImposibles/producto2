use strict;
use warnings;
use SOAP::Lite;

my $n = shift || 10;
my $wsdl = 'https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL';
my $soap = SOAP::Lite->service($wsdl);

my $result = $soap->NumberToWords({ ubiNum => $n });
print $result->{NumberToWordsResult} . "\n";
