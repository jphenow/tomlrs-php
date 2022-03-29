# tomlrs-php

Use Rust to provide fast TOML parsing to PHP.

This is somewhat experimental at the moment, but happy to help anyone take it to the next stage.

This was written after leaving a company that used PHP and I had some performance issues in particular cases.
I will not be using PHP professionally for the foreseeable future, so if you'd like to take this project and turn it into a published library please do so.

## Building, extending PHP

```sh
$> cargo build --release
# in php.ini set extension=/path-to-project/target/debug/libtomlrs_php.dylib or:
$> php -dextension=path-to-project/target/release/libtomlrs_php.dylib <your php file>
```

# PHP

```php
<?php

$parsed = parse_toml(__DIR__ . "/test.toml");
var_dump($parsed); // key-value array of TOML values
```

## Benchmark

Run on my Macbook Pro:

```
Processor Name:	8-Core Intel Core i9
Processor Speed:	2.4 GHz
Number of Processors:	1
Total Number of Cores:	8
L2 Cache (per Core):	256 KB
L3 Cache:	16 MB
Hyper-Threading Technology:	Enabled
Memory:	64 GB
```

Running each parser of [an example toml file](./benchmarks/test.toml) 1000 times and collecting the average time to parse:

| Library | Time (average in milliseconds) |
| ------ | -------- |
| `jamesmoss/toml` | 0.381ms |
| `leonelquinteros/php-toml` | 0.847ms |
| `yosymfony/toml` | 2.298ms |
| `tomlrs-php` | 0.129ms |

For more information, see the [benchmarks dir](./benchmarks/).
