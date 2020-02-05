# Rust FHIR Schema Generator

This is a dead-simple schema generator for the FHIR spec. I created it after seeing that most of the other json schema tools out there either could not handle the FHIR spec, or did not map well onto Rust's lack of inheritance.

This is a one-line schema generator that generates all types from the FHIR schema as simple serializable and deserializable structs and enums. We use derived Serde to map to and from JSON, so the mapping operations are generally pretty quick.

## What's coming next?

Immediate Roadmap:

1. Testing. We've got to get the standard json tests integrated in here to validate that our schema is correct.
2. Validation. We ignore the data validation patterns currently, but we should use something like https://github.com/Keats/validator to validate string data matches expected formats.
3. Better support for Extensions. Right now the "_" extension properties are ugly and I'd like to find a more elegant way to handle them. Also, they're essentially a union of every possible type, and that means in practice they occupy huge amounts of memory when actually allocated.

Longer-term steps:
1. Integrate with Hyper and add support for managing the full REST SoF API.
2. Generate a GraphQL schema based on the FHIR schema, and serve that API through a Juniper server. This will allow clients to use GQL codegen, and work with an isolated part of the full FHIR schema in their clients, leaving interop with SoF to the Rust intermediary server.
3. Allow this GQL intermediate layer to operate in either client or server mode. Embed it in clients to provide an elegant and fast query interface for SoF integrations. Use it as a dedicated server tier to create shared caching layers and dramatically improve your SoF client performance.
