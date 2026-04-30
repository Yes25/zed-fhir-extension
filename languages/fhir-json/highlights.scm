; highlights.scm

; "resourceType" pair: whole pair → @keyword (colors the key + colon).
; Split into two separate patterns so @function on the value is a distinct pattern
; and correctly overrides the parent @keyword for the value's text span.
(pair
  key: (string) @fhir.resource.type.key
  (#eq? @fhir.resource.type.key "\"resourceType\"")
) @keyword

; "resourceType" value (e.g. "Patient") — separate pattern so it wins over the pair's @keyword.
(pair
  key: (string) @_
  value: (string) @function
  (#eq? @_ "\"resourceType\""))

; Any key whose parent object also has a "resourceType" sibling → @property.
; This covers top-level resources AND nested ones (Bundle entries, contained resources).
(object
  (pair key: (string) @_ (#eq? @_ "\"resourceType\""))
  (pair
    key: (string) @property
    (#not-eq? @property "\"resourceType\"")))

; Keys nested inside array → object values (e.g. name[0].family, entry wrappers).
; Intentionally only one level deep via arrays — avoids conflicting with the sibling
; @property rule for resource objects that happen to be values of named pairs (resource, contained, etc.).
(pair
  value: (array
    (object
      (pair
        key: (string) @variable
        (#not-eq? @variable "\"resourceType\"")))))

; String values in key-value pairs — explicitly excludes "resourceType" so @function wins there
(pair
  key: (string) @fhir.value.key
  value: (string) @string
  (#not-eq? @fhir.value.key "\"resourceType\""))

; String values directly inside arrays (e.g. ["official", "usual"])
(array (string) @string)
