use serde::{Deserialize, Serialize};

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityStatement_Resource {
  /// Extensions for referencePolicy
  #[serde(rename = "_referencePolicy")]
  _reference_policy: Vec<Element>,

  /// A list of _include values supported by the server.
  #[serde(rename = "searchInclude")]
  search_include: Vec<String>,

  /// This field is set to no-version to specify that the system does not support
  /// (server) or use (client) versioning for this resource type. If this has some
  /// other value, the server must at least correctly track and populate the versionId
  /// meta-property on resources. If the value is 'versioned-update', then the server
  /// supports all the versioning features, including using e-tags for version
  /// integrity in the API.
  versioning: CapabilityStatement_ResourceVersioning,

  /// A specification of the profile that describes the solution's overall support for
  /// the resource, including any constraints on cardinality, bindings, lengths or
  /// other limitations. See further discussion in [Using
  /// Profiles](profiling.html#profile-uses).
  profile: canonical,

  /// Identifies a restful operation supported by the solution.
  interaction: Vec<CapabilityStatement_Interaction>,

  /// Search parameters for implementations to support and/or make use of - either
  /// references to ones defined in the specification, or additional ones defined
  /// for/by the implementation.
  #[serde(rename = "searchParam")]
  search_param: Vec<CapabilityStatement_SearchParam>,

  /// Extensions for updateCreate
  #[serde(rename = "_updateCreate")]
  _update_create: Element,

  /// Extensions for searchInclude
  #[serde(rename = "_searchInclude")]
  _search_include: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for conditionalDelete
  #[serde(rename = "_conditionalDelete")]
  _conditional_delete: Element,

  /// Additional information about the resource type used by the system.
  documentation: markdown,

  /// Extensions for type
  _type: Element,

  /// A code that indicates how the server supports conditional delete.
  #[serde(rename = "conditionalDelete")]
  conditional_delete: CapabilityStatement_ResourceConditionalDelete,

  /// A flag that indicates that the server supports conditional create.
  #[serde(rename = "conditionalCreate")]
  conditional_create: bool,

  /// Extensions for readHistory
  #[serde(rename = "_readHistory")]
  _read_history: Element,

  /// Extensions for conditionalCreate
  #[serde(rename = "_conditionalCreate")]
  _conditional_create: Element,

  /// Extensions for conditionalRead
  #[serde(rename = "_conditionalRead")]
  _conditional_read: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A list of profiles that represent different use cases supported by the system.
  /// For a server, "supported by the system" means the system hosts/produces a set of
  /// resources that are conformant to a particular profile, and allows clients that
  /// use its services to search using this profile and to find appropriate data. For
  /// a client, it means the system will search by this profile and process data
  /// according to the guidance implicit in the profile. See further discussion in
  /// [Using Profiles](profiling.html#profile-uses).
  #[serde(rename = "supportedProfile")]
  supported_profile: Vec<canonical>,

  /// A code that indicates how the server supports conditional read.
  #[serde(rename = "conditionalRead")]
  conditional_read: CapabilityStatement_ResourceConditionalRead,

  /// Extensions for conditionalUpdate
  #[serde(rename = "_conditionalUpdate")]
  _conditional_update: Element,

  /// A flag to indicate that the server allows or needs to allow the client to create
  /// new identities on the server (that is, the client PUTs to a location where there
  /// is no existing resource). Allowing this operation means that the server allows
  /// the client to create new identities on the server.
  #[serde(rename = "updateCreate")]
  update_create: bool,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for searchRevInclude
  #[serde(rename = "_searchRevInclude")]
  _search_rev_include: Vec<Element>,

  /// Extensions for versioning
  _versioning: Element,

  /// Extensions for documentation
  _documentation: Element,

  /// A flag for whether the server is able to return past versions as part of the
  /// vRead operation.
  #[serde(rename = "readHistory")]
  read_history: bool,

  /// A list of _revinclude (reverse include) values supported by the server.
  #[serde(rename = "searchRevInclude")]
  search_rev_include: Vec<String>,

  /// Definition of an operation or a named query together with its parameters and
  /// their meaning and type. Consult the definition of the operation for details
  /// about how to invoke the operation, and the parameters.
  operation: Vec<CapabilityStatement_Operation>,

  /// A flag that indicates that the server supports conditional update.
  #[serde(rename = "conditionalUpdate")]
  conditional_update: bool,

  /// A type of resource exposed via the restful interface.
  type: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatement_ResourceVersioning {
  #[serde(rename = "no-version")]
  NoVersion,

  #[serde(rename = "versioned")]
  Versioned,

  #[serde(rename = "versioned-update")]
  VersionedUpdate,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatement_ResourceConditionalDelete {
  #[serde(rename = "not-supported")]
  NotSupported,

  #[serde(rename = "single")]
  Single,

  #[serde(rename = "multiple")]
  Multiple,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatement_ResourceConditionalRead {
  #[serde(rename = "not-supported")]
  NotSupported,

  #[serde(rename = "modified-since")]
  ModifiedSince,

  #[serde(rename = "not-match")]
  NotMatch,

  #[serde(rename = "full-support")]
  FullSupport,

}
