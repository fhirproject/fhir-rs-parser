#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CapabilityStatement_Interaction::CapabilityStatement_Interaction;
use crate::model::Element::Element;
use crate::model::CapabilityStatement_SearchParam::CapabilityStatement_SearchParam;
use crate::model::Extension::Extension;
use crate::model::CapabilityStatement_Operation::CapabilityStatement_Operation;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_Resource {
  /// Extensions for searchRevInclude
  #[serde(rename = "_searchRevInclude")]
  _search_rev_include: Option<Vec<Element>>,

  /// Search parameters for implementations to support and/or make use of - either
  /// references to ones defined in the specification, or additional ones defined
  /// for/by the implementation.
  #[serde(rename = "searchParam")]
  search_param: Option<Vec<CapabilityStatement_SearchParam>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for versioning
  #[serde(rename = "_versioning")]
  _versioning: Option<Element>,

  /// A flag to indicate that the server allows or needs to allow the client to create
  /// new identities on the server (that is, the client PUTs to a location where there
  /// is no existing resource). Allowing this operation means that the server allows
  /// the client to create new identities on the server.
  #[serde(rename = "updateCreate")]
  update_create: Option<bool>,

  /// Extensions for updateCreate
  #[serde(rename = "_updateCreate")]
  _update_create: Option<Element>,

  /// A list of profiles that represent different use cases supported by the system.
  /// For a server, "supported by the system" means the system hosts/produces a set of
  /// resources that are conformant to a particular profile, and allows clients that
  /// use its services to search using this profile and to find appropriate data. For
  /// a client, it means the system will search by this profile and process data
  /// according to the guidance implicit in the profile. See further discussion in
  /// [Using Profiles](profiling.html#profile-uses).
  #[serde(rename = "supportedProfile")]
  supported_profile: Option<Vec<String>>,

  /// Extensions for readHistory
  #[serde(rename = "_readHistory")]
  _read_history: Option<Element>,

  /// A specification of the profile that describes the solution's overall support for
  /// the resource, including any constraints on cardinality, bindings, lengths or
  /// other limitations. See further discussion in [Using
  /// Profiles](profiling.html#profile-uses).
  profile: Option<String>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Definition of an operation or a named query together with its parameters and
  /// their meaning and type. Consult the definition of the operation for details
  /// about how to invoke the operation, and the parameters.
  operation: Option<Vec<CapabilityStatement_Operation>>,

  /// Extensions for conditionalDelete
  #[serde(rename = "_conditionalDelete")]
  _conditional_delete: Option<Element>,

  /// Additional information about the resource type used by the system.
  documentation: Option<String>,

  /// Identifies a restful operation supported by the solution.
  interaction: Option<Vec<CapabilityStatement_Interaction>>,

  /// A type of resource exposed via the restful interface.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

  /// Extensions for conditionalCreate
  #[serde(rename = "_conditionalCreate")]
  _conditional_create: Option<Element>,

  /// A list of _revinclude (reverse include) values supported by the server.
  #[serde(rename = "searchRevInclude")]
  search_rev_include: Option<Vec<String>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A flag that indicates that the server supports conditional create.
  #[serde(rename = "conditionalCreate")]
  conditional_create: Option<bool>,

  /// Extensions for referencePolicy
  #[serde(rename = "_referencePolicy")]
  _reference_policy: Option<Vec<Element>>,

  /// A list of _include values supported by the server.
  #[serde(rename = "searchInclude")]
  search_include: Option<Vec<String>>,

  /// Extensions for searchInclude
  #[serde(rename = "_searchInclude")]
  _search_include: Option<Vec<Element>>,

  /// A flag for whether the server is able to return past versions as part of the
  /// vRead operation.
  #[serde(rename = "readHistory")]
  read_history: Option<bool>,

  /// A flag that indicates that the server supports conditional update.
  #[serde(rename = "conditionalUpdate")]
  conditional_update: Option<bool>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// Extensions for conditionalRead
  #[serde(rename = "_conditionalRead")]
  _conditional_read: Option<Element>,

  /// This field is set to no-version to specify that the system does not support
  /// (server) or use (client) versioning for this resource type. If this has some
  /// other value, the server must at least correctly track and populate the versionId
  /// meta-property on resources. If the value is 'versioned-update', then the server
  /// supports all the versioning features, including using e-tags for version
  /// integrity in the API.
  versioning: Option<CapabilityStatement_ResourceVersioning>,

  /// A code that indicates how the server supports conditional read.
  #[serde(rename = "conditionalRead")]
  conditional_read: Option<CapabilityStatement_ResourceConditionalRead>,

  /// Extensions for conditionalUpdate
  #[serde(rename = "_conditionalUpdate")]
  _conditional_update: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// A code that indicates how the server supports conditional delete.
  #[serde(rename = "conditionalDelete")]
  conditional_delete: Option<CapabilityStatement_ResourceConditionalDelete>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_ResourceVersioning {
  #[serde(rename = "no-version")]
  NoVersion,

  #[serde(rename = "versioned")]
  Versioned,

  #[serde(rename = "versioned-update")]
  VersionedUpdate,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_ResourceConditionalRead {
  #[serde(rename = "not-supported")]
  NotSupported,

  #[serde(rename = "modified-since")]
  ModifiedSince,

  #[serde(rename = "not-match")]
  NotMatch,

  #[serde(rename = "full-support")]
  FullSupport,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_ResourceConditionalDelete {
  #[serde(rename = "not-supported")]
  NotSupported,

  #[serde(rename = "single")]
  Single,

  #[serde(rename = "multiple")]
  Multiple,

}
