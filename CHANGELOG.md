# Changelog

All notable changes to the facet project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.42.1](https://github.com/romac/facet/compare/facet-html-v0.42.0...facet-html-v0.42.1) - 2026-01-09

### Fixed

- *(html)* preserve whitespace-only text nodes in parser
- *(html)* preserve whitespace in text content per HTML spec

### Other

- explain XML vs HTML data model differences
- Add second ws preservation test
- Add link for HTML because rustdoc is smart enough to warn us about it but not enough to just link it I guess.
- Fails to preserve line breaks

## [0.42.1](https://github.com/romac/facet/compare/facet-asn1-v0.42.0...facet-asn1-v0.42.1) - 2026-01-09

### Added

- *(format-suite)* implement net type tests for all formats

## [0.42.1](https://github.com/romac/facet/compare/facet-shapelike-v0.42.0...facet-shapelike-v0.42.1) - 2026-01-09

### Fixed

- *(soundness)* make shape field private on Ox* types

## [0.42.1](https://github.com/romac/facet/compare/facet-yaml-v0.42.0...facet-yaml-v0.42.1) - 2026-01-09

### Added

- *(format-suite)* implement net type tests for all formats

## [0.42.1](https://github.com/romac/facet/compare/facet-toml-v0.42.0...facet-toml-v0.42.1) - 2026-01-09

### Added

- *(format-suite)* implement net type tests for all formats

## [0.42.1](https://github.com/romac/facet/compare/facet-msgpack-v0.42.0...facet-msgpack-v0.42.1) - 2026-01-09

### Added

- *(format-suite)* implement net type tests for all formats

## [0.42.1](https://github.com/romac/facet/compare/facet-kdl-v0.42.0...facet-kdl-v0.42.1) - 2026-01-09

### Fixed

- *(facet-kdl)* implement transparent document model for roundtripping

## [0.43.0](https://github.com/romac/facet/compare/facet-args-v0.42.0...facet-args-v0.43.0) - 2026-01-09

### Added

- *(facet-args)* add counted flag support

## [0.42.1](https://github.com/romac/facet/compare/facet-format-v0.42.0...facet-format-v0.42.1) - 2026-01-09

### Fixed

- *(facet-kdl)* implement transparent document model for roundtripping
- accept all numeric scalar tags for numeric types
- *(jit)* validate scalar tags before reading payload to prevent type confusion
- *(html)* preserve whitespace-only text nodes in parser

## [0.43.0](https://github.com/romac/facet/compare/facet-core-v0.42.0...facet-core-v0.43.0) - 2026-01-09

### Added

- *(shape)* add DeclId for identifying type declarations
- *(facet-core)* add module_path to foreign type implementations
- *(shape)* add module_path and source location fields

### Fixed

- *(variance)* &'a mut T is covariant in 'a when T is bivariant
- prevent exponential variance computation for recursive types
- *(soundness)* propagate variance correctly for reference types
- resolve rustdoc links and clippy auto-deref warnings
- *(soundness)* introduce TryFromOutcome enum for explicit ownership semantics
- *(soundness)* return Invariant when variance depth limit is hit
- *(soundness)* make shape field private on Ox* types

### Other

- *(decl_id)* auto-compute from module_path + kind + type_identifier
- *(decl_id)* auto-compute for non-generic types
- *(variance)* replace function-based variance with declarative model

## [0.42.0](https://github.com/facet-rs/facet/compare/facet-core-v0.41.0...facet-core-v0.42.0) - 2026-01-06

### Added

- implement Facet for core::convert::Infallible

### Fixed

- mark function pointers as invariant to prevent lifetime UB
- *(soundness)* make OxRef::new and OxMut::new unsafe

### Other

- *(bytestring)* simplify ByteString impl with vtable_direct! macro
- Fix #1629: Preserve custom HTML elements during parse/serialize roundtrip
- Add facet-validate crate for field validation during deserialization
- Add rust_decimal::Decimal support + fix XML type inference
- Add rust_decimal::Decimal support to facet-core
- Add Facet implementation for smol_str::SmolStr
- Set up release-plz with synchronized versions and trusted publishing
- Add `facet_no_doc` cfg for global doc string stripping
- Fix facet-pretty to respect skip_serializing_if and add HTML roundtrip tests
- Add html::text attribute for enum variants and comprehensive roundtrip test
- Fix inconsistent Shape hash (issue #1574)
- Fix soundness issue: Attr can contain non-Sync data
- Require 'static for Opaque Facet impl
- *(facet-core)* simplify Ox API by requiring T: Facet
- fix broken intra-doc link to Peek in facet-core
- Improve AGENTS.md, closes #1551
