<?php

$time = microtime(true);
$parsed = parse_toml(__DIR__ . "/test.toml");
var_dump($parsed);
printf("Took %sms to parse toml with rust", (microtime(true) - $time) * 1000);
