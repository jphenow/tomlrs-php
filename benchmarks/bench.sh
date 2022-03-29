#!/usr/bin/env bash

set -euo pipefail

ONLY="${ONLY:-}"

benches=(
    "jamesmoss-toml"
    "leonelquinteros-php-toml"
    "yosymfony-toml"
    "tomlrs-php"
)

export TIMES=1000
benchDir="$PWD"

for benchProject in "${benches[@]}"; do
    # TODO: output device info

    if [ "$ONLY" != "" ] && [ "$ONLY" != "$benchProject" ]; then
        continue
    fi

    cd $benchProject > /dev/null

    if [ "$benchProject" == "tomlrs-php" ]; then
        echo "$benchProject: $(php -dextension=/Users/jphenow/Code/tomlrs-php/target/release/libtomlrs_php.dylib index.php)ms"
    else
        echo "$benchProject: $(php index.php)ms"
    fi
    cd .. > /dev/null
done

