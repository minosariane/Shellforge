# Shellforge
Terminal application to generate reverse shells. Great for CTFs.

## Example usage
The following command generates a reverse shell with "bash -i" encoded in base64
``` console
shellforge generate --ip 192.168.50.1 --port 7777 --format bash-i --b64
```

Some values are set by default. If you just enter your IP, shellforge will generate the top 10 shells on port 7777.
```console
shellforge generate -i 192.168.50.1
```
