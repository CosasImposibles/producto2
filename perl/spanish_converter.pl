use strict;
use warnings;
use HTTP::Daemon;
use HTTP::Status;
use HTTP::Response;
use HTTP::Tiny;
use JSON;
use URI::Escape;

my $daemon = HTTP::Daemon->new(LocalAddr => '127.0.0.1', LocalPort => 4000) or die $!;
print "Server running at http://127.0.0.1:4000/\n";

sub soap_to_english {
    my ($n) = @_;
    my $client = HTTP::Tiny->new();
    my $soap_body = qq{<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <NumberToWords xmlns="http://www.dataaccess.com/webservicesserver/">
      <ubiNum>$n</ubiNum>
    </NumberToWords>
  </soap:Body>
</soap:Envelope>};

    my $response = $client->post(
        'https://www.dataaccess.com/webservicesserver/NumberConversion.wso',
        {
            headers => {
                'Content-Type' => 'text/xml; charset=utf-8',
                'SOAPAction' => 'http://www.dataaccess.com/webservicesserver/NumberToWords'
            },
            content => $soap_body
        }
    );

    return '' if !$response->{success};
    my $xml = $response->{content};

    if ($xml =~ m{<NumberToWordsResult>(.*?)</NumberToWordsResult>}s) {
        my $value = $1;
        $value =~ s/^\s+|\s+$//g;
        return $value;
    }
    if ($xml =~ m{<m:NumberToWordsResult>(.*?)</m:NumberToWordsResult>}s) {
        my $value = $1;
        $value =~ s/^\s+|\s+$//g;
        return $value;
    }
    return '';
}

sub translate_to_spanish {
    my ($text) = @_;
    return '' if !defined $text || $text eq '';
    my $client = HTTP::Tiny->new();
    my $q = uri_escape($text);
    my $url = "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl=es&dt=t&q=$q";
    my $response = $client->get($url);
    if ($response->{success}) {
        my $data = decode_json($response->{content});
        return $data->[0][0][0];
    }
    die "Translation failed: $response->{status} $response->{reason}\n";
}

sub get_number_from_query {
    my ($query) = @_;
    if ($query =~ /n=(\d+)/) {
        return $1;
    }
    return 10;
}

while (my $client_conn = $daemon->accept) {
    while (my $request = $client_conn->get_request) {
        if ($request->method eq 'GET') {
            my $query = $request->uri->query || '';
            my $n = get_number_from_query($query);

            my $english = soap_to_english($n);
            my $spanish = translate_to_spanish($english);

            if ($spanish eq '') {
                $client_conn->send_response(HTTP::Response->new(RC_INTERNAL_SERVER_ERROR, 'Error', ['Content-Type' => 'text/plain'], 'No se pudo traducir el resultado SOAP'));
                next;
            }

            $client_conn->send_response(HTTP::Response->new(RC_OK, 'OK', ['Content-Type' => 'text/plain'], $spanish));
        } else {
            $client_conn->send_error(RC_FORBIDDEN);
        }
    }
    $client_conn->close;
    undef $client_conn;
}
