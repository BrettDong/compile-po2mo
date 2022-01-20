# `compile-po2mo`

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Actions](https://github.com/BrettDong/compile-po2mo/actions/workflows/release.yml/badge.svg)

A blazing fast CLI program to compile GNU gettext .po file to binary .mo format.

## Usage

You can either specify a concrete pair of input and output file paths:

`$ compile-po2mo input.po output.mo`

Or you can also give a pair of path patterns denoted by `<lang>`:

`$ compile-po2mo "./i18n/translations/<lang>.po" "./build/i18n/<lang>/LC_MESSAGES/app.mo"`

In the latter case, all `.po` files matching the pattern will be compiled to the destination path:

```
./i18n/translations/ja.po => ./build/i18n/ja/LC_MESSAGES/app.mo
./i18n/translations/es_ES.po => ./build/i18n/es_ES/LC_MESSAGES/app.mo
...
```

On Windows, the path pattern has to be in backslash separator and do not contain current directory: `.\compile-po2mo.exe i18n\translations\<lang>.po build\i18n\<lang>\LC_MESSAGES\app.mo`. On UNIX shell, the path pattern has to be quoted.

## Limitation / Caveat

Just compiles a single `.po` file in UTF-8 encoding to a `.mo` file in UTF-8 encoding and nothing else. No fancy features, no rigorous input sanity check, all for maximum performance. The program may panic or silently produce nonsense output when input data is ill-formed. You should only run this when all input data is trusted, and you want to cut down project build times and CI/CD costs.

## Benchmark

Detailed logs can be viewed at https://github.com/BrettDong/compile-po2mo/actions/runs/1724533629 .

Time to compile [all translations to 35 languages](https://github.com/CleverRaven/Cataclysm-DDA/tree/372311faa019666a9015e7e8254a53bae98190e2/lang/po) on GitHub Actions runners:

| GitHub Actions Host | Method | Parallelism | Time |
|---|---|---|---|
| Windows Server 2019 | `msgfmt` | sequential | 47.168s |
| Windows Server 2019 | `msgfmt` | dual-core | ~30s *(1) |
| Windows Server 2019 | `compile-po2mo` | sequential | 6.328s |
| Windows Server 2019 | `compile-po2mo` | dual-core | 3.887s |
| Ubuntu Linux 20.04 | `msgfmt` | sequential | 35.642s |
| Ubuntu Linux 20.04 | `compile-po2mo` | sequential | 4.132s |
| Ubuntu Linux 20.04 | `compile-po2mo` | dual-core | 2.183s |
| macOS 10.15 | `msgfmt` | sequential | 25.736s |
| macOS 10.15 | `compile-po2mo` | sequential | 5.990s |
| macOS 10.15 | `compile-po2mo` | tri-core | 5.786s *(2) |

1. empirical data from https://github.com/CleverRaven/Cataclysm-DDA/runs/4881530176 .

2. it is weird that tri-core parallel compilation in macOS 10.15 virtual environment on GitHub Actions does not yield significant speedup.
