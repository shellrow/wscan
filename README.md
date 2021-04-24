[webscan-url]: https://github.com/shellrow/webscan
# wscan
Cross-platform web scan tool for content discovery and recon

## Basic Usage
```
wscan 0.1.0
shellrow <https://github.com/shellrow>
Cross-platform web scan tool for content discovery and recon

USAGE:
    wscan [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --uri <uri>               URI Scan - Ex: -u http://192.168.1.8/xvwa/ -w common.txt
    -d, --domain <domain_name>    Domain Scan - Ex: -d example.com -w subdomain.txt
    -t, --timeout <duration>      Set timeout in ms - Ex: -t 10000
    -w, --word <file_path>        Use word list - Ex: -w common.txt
    -m, --method <method>         Set HTTP request method for scanning
    -s, --save <file_path>        Save scan result to file - Ex: -s result.txt
```

## About webscan (lib)
Please check my [repository][webscan-url] for detail