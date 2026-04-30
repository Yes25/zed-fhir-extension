# FHIR Extension for Zed

Adds support for FHIR json messages in the [Zed editor](https://zed.dev). Syntax
highlighting is done via a [tree-sitter grammar](https://github.com/Yes25/tree-sitter-fhir).
Additionally a [fhir-lsp](https://github.com/Yes25/fhir-lsp)
is used to provide some convenience features like
- `go to defintion` and `go to reference` wihtin a Bundle or throghout a project.
- Provide Documation trough hover over fields
- Some diagnostics like missing mandatory fields
- Auto formating that sorts fields in the order as they appear in the [fhir documentation](https://www.hl7.org/fhir/overview.html)
