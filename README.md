# tomlrs-php

Use Rust to provide fast TOML parsing to PHP.

This is somewhat experimental at the moment, but happy to help anyone take it to the next stage.

## Building, extending PHP

```sh
$> cargo build
# in php.ini set extension=/path-to-project/target/debug/libtomlrs_php.dylib or:
$> php -dextension=path-to-project/target/debug/libtomlrs_php.dylib <your php file>
```

# PHP

```php
<?php

$parsed = parse_toml(__DIR__ . "/test.toml");
var_dump($parsed); // key-value array of TOML values
```
