# FHIR Extension for Zed

Adds support for FHIR json messages in the [Zed editor](https://zed.dev).
It provides syntaxhighlighting and additional convenience features through the [fhir-lsp](https://github.com/Yes25/fhir-lsp):
- `go to defintion` and `go to reference` wihtin a Bundle or throghout a project.
- Provide `documentation` on `hover` over fields
- Some `diagnostics` like missing mandatory fields
- `Auto formating` that sorts fields in the order they appear in the [fhir documentation](https://www.hl7.org/fhir/overview.html)
