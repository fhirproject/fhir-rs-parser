use serde::{Deserialize, Serialize};

/// A container for a collection of resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Bundle {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for timestamp
  _timestamp: Element,

  /// If a set of search matches, this is the total number of entries of type 'match'
  /// across all pages in the search.  It does not include search.mode = 'include' or
  /// 'outcome' entries and it does not provide a count of the number of entries in
  /// the Bundle.
  total: unsignedInt,

  /// Extensions for language
  _language: Element,

  /// Extensions for total
  _total: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for type
  _type: Element,

  /// A series of links that provide context to this bundle.
  link: Vec<Bundle_Link>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A persistent identifier for the bundle that won't change as a bundle is copied
  /// from server to server.
  identifier: Identifier,

  /// The base language in which the resource is written.
  language: String,

  /// An entry in a bundle resource - will either contain a resource or information
  /// about a resource (transactions and history only).
  entry: Vec<Bundle_Entry>,

  /// Indicates the purpose of this bundle - how it is intended to be used.
  type: BundleType,

  /// Digital Signature - base64 encoded. XML-DSig or a JWT.
  signature: Signature,

  /// The date/time that the bundle was assembled - i.e. when the resources were
  /// placed in the bundle.
  timestamp: instant,

}

#[derive(Debug, Serialize, Deserialize)]
enum BundleType {
  #[serde(rename = "document")]
  Document,

  #[serde(rename = "message")]
  Message,

  #[serde(rename = "transaction")]
  Transaction,

  #[serde(rename = "transaction-response")]
  TransactionResponse,

  #[serde(rename = "batch")]
  Batch,

  #[serde(rename = "batch-response")]
  BatchResponse,

  #[serde(rename = "history")]
  History,

  #[serde(rename = "searchset")]
  Searchset,

  #[serde(rename = "collection")]
  Collection,

}
