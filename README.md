# `compile-po2mo`

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![GitHub Actions](https://github.com/BrettDong/compile-po2mo/actions/workflows/build.yml/badge.svg)

A blazing fast CLI program to compile GNU gettext `.po` file to binary `.mo` format.

## Usage

### Compile a single `.po` file to `.mo`

`$ compile-po2mo input.po output.mo`

### Compile a batch of `.po` files in parallel

Use path patterns denoted by `<lang>`:

`$ compile-po2mo "./i18n/translations/<lang>.po" "./build/i18n/<lang>/LC_MESSAGES/app.mo"`

All `.po` files matching the pattern will be compiled to the destination path:

```
./i18n/translations/ja.po => ./build/i18n/ja/LC_MESSAGES/app.mo
./i18n/translations/es_ES.po => ./build/i18n/es_ES/LC_MESSAGES/app.mo
...
```

Be noted that on UNIX shell the path pattern has to be quoted, otherwise `<` and `>` symbols are interpreted as redirection by the shell.

### Use on GitHub Actions

I have prepared an action to use this tool to compile translations in GNU gettext format as a step in your CI/CD workflow on GitHub Actions:

```yml
  - name: Compile translations
    uses: BrettDong/compile-po2mo-action@v1
    with:
        input-path: 'lang/po/<lang>.po'
        output-path: 'lang/mo/<lang>/LC_MESSAGES/app.mo'
```


Visit http://github.com/BrettDong/compile-po2mo-action for details.

## Limitation / Caveat

* Only supports UTF-8 encoding

* Does not check input `.po` data on C format string correctness, duplicate entries, etc.

The program may panic or silently produce nonsense output when input data is ill-formed. You should only use this tool when all input data is trusted and valid, and you want to cut down project build times and CI/CD costs.

## Benchmark

Detailed logs can be viewed at https://github.com/BrettDong/compile-po2mo/actions/runs/1757459199 .

Time to compile [all translations to 36 languages](https://github.com/CleverRaven/Cataclysm-DDA/tree/761fb517639f30146b382cf629aa5b3b47733011/lang/po) on GitHub Actions runners:

| GitHub Actions Host | Method | Parallelism | Time |
|---|---|---|---|
| Windows Server 2019 | `msgfmt` | dual-core | 30.157s |
| Windows Server 2019 | `compile-po2mo` | dual-core | 1.908s |
| Ubuntu Linux 20.04 | `msgfmt` | dual-core | 16.950s |
| Ubuntu Linux 20.04 | `compile-po2mo` | dual-core | 1.050s |
| macOS 10.15 | `msgfmt` | tri-core | 11.270s |
| macOS 10.15 | `compile-po2mo` | tri-core | 1.231s |
