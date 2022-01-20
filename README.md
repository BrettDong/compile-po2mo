# `compile-po2mo`

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Actions](https://github.com/BrettDong/compile-po2mo/actions/workflows/release.yml/badge.svg)

A blazing fast CLI program to compile GNU gettext .po file to binary .mo format.

## Usage

You can either specify a concrete pair of input and output file paths:

`$ compile-po2mo input.po output.mo`

Or you can also give a pair of path patterns denoted by `<lang>`:

`$ compile-po2mo ./i18n/translations/<lang>.po ./build/i18n/<lang>/LC_MESSAGES/app.mo`

In the latter case, all `.po` files matching the pattern will be compiled to the destination path:

```
./i18n/translations/ja.po => ./build/i18n/ja/LC_MESSAGES/app.mo
./i18n/translations/es_ES.po => ./build/i18n/es_ES/LC_MESSAGES/app.mo
...
```

## Limitation / Caveat

Just compiles a single `.po` file in UTF-8 encoding to a `.mo` file in UTF-8 encoding and nothing else. No fancy features, no rigorous input sanity check, all for maximum performance. The program may panic or silently produce nonsense output when input data is ill-formed. You should only run this when all input data is trusted, and you want to cut down project build times and CI/CD costs.

## Benchmark

Detailed logs can be viewed at https://github.com/BrettDong/compile-po2mo/actions/runs/1723363718 .

Time to compile [all translations to 35 languages](https://github.com/CleverRaven/Cataclysm-DDA/tree/372311faa019666a9015e7e8254a53bae98190e2/lang/po) on GitHub Actions runners:

| GitHub Actions Host | Method | Time |
|---|---|---|
| Windows Server 2019 | `msgfmt` | 47.168s |
| Windows Server 2019 | `compile-po2mo` | 6.328s |
| Ubuntu Linux 20.04 | `msgfmt` | 35.642s |
| Ubuntu Linux 20.04 | `compile-po2mo` | 4.132s |
| macOS 10.15 | `msgfmt` | 25.736s |
| macOS 10.15 | `compile-po2mo` | 5.990s |
