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

## Example
```
shellrow@MacBook-Pro wscan % wscan -u http://192.168.1.8/xvwa/ -w common.txt
wscan 0.1.0 macos
https://github.com/shellrow/wscan

Scan started at 2021-04-30 21:01:24.473873

-URI Scan Options---------------
    Base URI: http://192.168.1.8/xvwa/
    Word list: common.txt
--------------------------------

Scanning...Done

-Scan Reports-------------------
    http://192.168.1.8/xvwa/.hta 403 Forbidden
    http://192.168.1.8/xvwa/setup 500 Internal Server Error
    http://192.168.1.8/xvwa/.htaccess 403 Forbidden
    http://192.168.1.8/xvwa/fonts 200 OK
    http://192.168.1.8/xvwa/index.php 500 Internal Server Error
    http://192.168.1.8/xvwa/css 200 OK
    http://192.168.1.8/xvwa/img 200 OK
    http://192.168.1.8/xvwa/js 200 OK
    http://192.168.1.8/xvwa/LICENSE 200 OK
    http://192.168.1.8/xvwa/php.ini 200 OK
    http://192.168.1.8/xvwa/.git/HEAD 200 OK
    http://192.168.1.8/xvwa/.htpasswd 403 Forbidden
--------------------------------
Scan Time: 4.732139338s
```

## About webscan (lib)
Please check my [repository][webscan-url] for detail