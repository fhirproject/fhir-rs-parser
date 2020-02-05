use serde::{Deserialize, Serialize};

/// This resource is a non-persisted resource used to pass information into and back
/// from an [operation](operations.html). It has no other use, and there is no
/// RESTful endpoint associated with it.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Parameters {
  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A parameter passed to or received from the operation.
  parameter: Vec<Parameters_Parameter>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for language
  _language: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

}
