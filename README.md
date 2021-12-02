# Rust FHIR Schema Generator

This is a dead-simple schema generator for the FHIR spec. I created it after seeing that most of the other json schema tools out there either could not handle the FHIR spec, or did not map well onto Rust's lack of inheritance.

This is a one-file schema generator that generates all types from the FHIR schema as simple deserializable structs and enums.

## Exemple of usage
For the moment the package is not published on cargo so you need to add it like this: `fhir-rs = { git = "https://github.com/mypl-io/fhir-rs-parser" }`

```rs
use fhir_rs::{fhir_parse, model};

if let Some(resource_list) = fhir_parse(&res_string) {
    match resource_list.resource() {
        Some(model::ResourceList::ResourceListEnum::ResourcePractitioner(practitioner)) => {
            println!("Practitioner: {:?}", practitioner);
        }
        _ => {}
    }
}

```

## Our models are FAST!

The codegenerated models are built around lazy access to the underlying json responses from a FHIR server. Our sparse memory layout combined with our lazy access and lack of copied data means that these models are **very fast** compared to other compiled language implementations of FHIR. 

## What's coming next?

Immediate Roadmap:

1. **Done**. Testing. We've got to get the standard json tests integrated in here to validate that our schema is correct.
2. **Partially complete**. Validation. We ignore the data validation patterns currently, but we should be checking the format of data we receive when requested to.
3. **Done**. Better support for Extensions. Right now the "_" extension properties are ugly and I'd like to find a more elegant way to handle them. Also, they're essentially a union of every possible type, and that means in practice they occupy huge amounts of memory when actually allocated.
4. Builders! Right now the library is read-only, it's useful for providing a type-safe parsing layer on top of FHIR responses.

Longer-term steps:
1. Integrate with Hyper and add support for managing the full REST SoF API.
2. Generate a GraphQL schema based on the FHIR schema, and serve that API through a Juniper server. This will allow clients to use GQL codegen, and work with an isolated part of the full FHIR schema in their clients, leaving interop with SoF to the Rust intermediary server.
3. Allow this GQL intermediate layer to operate in either client or server mode. Embed it in clients to provide an elegant and fast query interface for SoF integrations. Use it as a dedicated server tier to create shared caching layers and dramatically improve your SoF client performance.
