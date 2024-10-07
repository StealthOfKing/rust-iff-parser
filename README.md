# iff-parser

![GitHub Package Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FStealthOfKing%2Frust-iff-parser%2Frefs%2Fheads%2Fmaster%2FCargo.toml&query=%24.package.version&prefix=v&label=Rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/StealthOfKing/rust-iff-parser/rust.yml)
![GitHub License](https://img.shields.io/github/license/StealthOfKing/rust-iff-parser)

[Interchange File Format] parser implemented using the `chunk-parser` pattern.

[Interchange File Format]: https://en.wikipedia.org/wiki/Interchange_File_Format

## Usage

`iff-parser` implements a small heuristic function capable of parsing any
arbitrary IFF format:

```
$ iff-guess /path/to/noise.aiff

00000000 FORM -> AIFC                   135222 bytes
00000012   FVER                              4 bytes
00000024   COMM                             24 bytes
00000056   SSND                         135166 bytes
00135230 EOF  
```