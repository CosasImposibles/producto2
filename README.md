# SOAP Case Study

Repositorio de ejemplo para el caso de estudio: consumir y procesar el servicio SOAP público de conversión de números a texto.

## Objetivo

Por cada lenguaje de servidor se incluyen tres versiones:

1. Cliente SOAP que consume el servicio público `NumberToWords`.
2. Cliente SOAP que consume el servicio y luego traduce el resultado inglés→español usando una librería.
3. Aplicación web local que convierte un número a su forma en letras en español sin llamar al servicio SOAP.

## Servicios usados

- SOAP WSDL: `https://www.dataaccess.com/webservicesserver/NumberConversion.wso?WSDL`
- Método SOAP usado: `NumberToWords`

## Estructura

- `ruby/`
- `perl/`
- `node/`
- `dotnet10/`
- `go/`
- `java/`
- `cpp/`
- `rust/`

Cada carpeta contiene los tres casos de ejemplo.

## Ejecución general

### Ruby

```bash
cd ruby
ruby soap_client.rb
ruby soap_translate.rb
ruby spanish_converter.rb
```

### Perl

```bash
cd perl
perl soap_client.pl
perl soap_translate.pl
perl spanish_converter.pl
```

### Node

```bash
cd node
npm install
node soap_client.js
node soap_translate.js
node spanish_converter.js
```

### .NET 10

```bash
cd dotnet10/soap-client
dotnet run
```

### Go

```bash
go run go/soap_client.go
```

### Java

```bash
javac java/SoapClient.java
java java.SoapClient
```

### C++

```bash
g++ cpp/soap_client.cpp -o cpp/soap_client
./cpp/soap_client
```

### Rust

```bash
cd rust
cargo run --bin soap_client
```

## Notas

- Algunos ejemplos dependen de paquetes externos que deben instalarse con el gestor de paquetes de cada lenguaje.
- En todos los casos el número `n` se puede pasar por URL como `?n=10` si el ejemplo arranca un servidor web.
