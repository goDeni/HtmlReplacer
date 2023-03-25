# HTML header replacement script

## Help
```
./html_replacer --help
Usage: html_replacer <HEADER_FILE> <HTML_FILES_DIR>

Arguments:
  <HEADER_FILE>     
  <HTML_FILES_DIR>  

Options:
  -h, --help  Print help
```
## Example

header.html 
```html
<head>
    <title>Replaced header!!!</title>
    <title>Replaced header!!!</title>
    <title>Replaced header!!!</title>
</head>
```

input/example.html 
```html
<!DOCTYPE html>
<html>
    <head>
        <title>Example</title>
    </head>
    <body>
        <p>This is an example of a simple HTML page with one paragraph.</p>
    </body>
</html>
```

Replacement
```
$ ./html_replacer header.html input
Backup created "/home/denis/html_replacer/backup/1679739098175"
Entering in directory input
Done "input/example.html"
```

Result input/example.html
```html
<!DOCTYPE html>
<html>
    <head>
    <title>Replaced header!!!</title>
    <title>Replaced header!!!</title>
    <title>Replaced header!!!</title>
</head>
    <body>
        <p>This is an example of a simple HTML page with one paragraph.</p>
    </body>
</html>
```