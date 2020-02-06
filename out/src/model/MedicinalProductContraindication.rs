#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Population::Population;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductContraindication_OtherTherapy::MedicinalProductContraindication_OtherTherapy;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;


/// The clinical particulars - indications, contraindications etc. of a medicinal
/// product, including for regulatory purposes.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductContraindication {
  /// Information about the use of the medicinal product in relation to other
  /// therapies described as part of the indication.
  #[serde(rename = "otherTherapy")]
  other_therapy: Option<Vec<MedicinalProductContraindication_OtherTherapy>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The disease, symptom or procedure for the contraindication.
  disease: Option<CodeableConcept>,

  /// The status of the disease or symptom for the contraindication.
  #[serde(rename = "diseaseStatus")]
  disease_status: Option<CodeableConcept>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A comorbidity (concurrent condition) or coinfection.
  comorbidity: Option<Vec<CodeableConcept>>,

  /// The population group to which this applies.
  population: Option<Vec<Population>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Information about the use of the medicinal product in relation to other
  /// therapies as part of the indication.
  #[serde(rename = "therapeuticIndication")]
  therapeutic_indication: Option<Vec<Box<Reference>>>,

  /// The medication for which this is an indication.
  subject: Option<Vec<Box<Reference>>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

}
