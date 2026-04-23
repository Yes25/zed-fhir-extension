; highlights.scm
(pair value: (_) @string)

((pair
    key: (string) @_ (#not-eq? @_ "\"resourceType\""))
   @number)

((pair
    key: (string) @_ (#eq? @_ "\"resourceType\""))
  (pair) @property)

 (pair
      key: (string) @fhir.resource.type.key
      value: (string) @function
      (#eq? @fhir.resource.type.key "\"resourceType\"")
  ) @keyword
