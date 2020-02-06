#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::EnrollmentResponse::EnrollmentResponse;
use crate::model::Medication::Medication;
use crate::model::MessageDefinition::MessageDefinition;
use crate::model::QuestionnaireResponse::QuestionnaireResponse;
use crate::model::MedicinalProductIngredient::MedicinalProductIngredient;
use crate::model::Binary::Binary;
use crate::model::Account::Account;
use crate::model::EnrollmentRequest::EnrollmentRequest;
use crate::model::Organization::Organization;
use crate::model::List::List;
use crate::model::DeviceRequest::DeviceRequest;
use crate::model::VisionPrescription::VisionPrescription;
use crate::model::BiologicallyDerivedProduct::BiologicallyDerivedProduct;
use crate::model::Condition::Condition;
use crate::model::SubstanceSpecification::SubstanceSpecification;
use crate::model::Consent::Consent;
use crate::model::Task::Task;
use crate::model::GraphDefinition::GraphDefinition;
use crate::model::DeviceDefinition::DeviceDefinition;
use crate::model::ServiceRequest::ServiceRequest;
use crate::model::BodyStructure::BodyStructure;
use crate::model::HealthcareService::HealthcareService;
use crate::model::Contract::Contract;
use crate::model::Substance::Substance;
use crate::model::Parameters::Parameters;
use crate::model::ResearchElementDefinition::ResearchElementDefinition;
use crate::model::MedicationDispense::MedicationDispense;
use crate::model::InsurancePlan::InsurancePlan;
use crate::model::RiskAssessment::RiskAssessment;
use crate::model::Questionnaire::Questionnaire;
use crate::model::CapabilityStatement::CapabilityStatement;
use crate::model::Endpoint::Endpoint;
use crate::model::TestScript::TestScript;
use crate::model::Flag::Flag;
use crate::model::Goal::Goal;
use crate::model::EvidenceVariable::EvidenceVariable;
use crate::model::CodeSystem::CodeSystem;
use crate::model::RiskEvidenceSynthesis::RiskEvidenceSynthesis;
use crate::model::CompartmentDefinition::CompartmentDefinition;
use crate::model::MedicinalProduct::MedicinalProduct;
use crate::model::MedicinalProductPharmaceutical::MedicinalProductPharmaceutical;
use crate::model::VerificationResult::VerificationResult;
use crate::model::Subscription::Subscription;
use crate::model::DeviceUseStatement::DeviceUseStatement;
use crate::model::MedicationKnowledge::MedicationKnowledge;
use crate::model::EpisodeOfCare::EpisodeOfCare;
use crate::model::GuidanceResponse::GuidanceResponse;
use crate::model::Observation::Observation;
use crate::model::FamilyMemberHistory::FamilyMemberHistory;
use crate::model::StructureDefinition::StructureDefinition;
use crate::model::AllergyIntolerance::AllergyIntolerance;
use crate::model::Bundle::Bundle;
use crate::model::ChargeItemDefinition::ChargeItemDefinition;
use crate::model::Communication::Communication;
use crate::model::ObservationDefinition::ObservationDefinition;
use crate::model::Immunization::Immunization;
use crate::model::MeasureReport::MeasureReport;
use crate::model::MedicinalProductContraindication::MedicinalProductContraindication;
use crate::model::ClinicalImpression::ClinicalImpression;
use crate::model::ActivityDefinition::ActivityDefinition;
use crate::model::EventDefinition::EventDefinition;
use crate::model::Person::Person;
use crate::model::DocumentReference::DocumentReference;
use crate::model::MedicinalProductInteraction::MedicinalProductInteraction;
use crate::model::DiagnosticReport::DiagnosticReport;
use crate::model::AdverseEvent::AdverseEvent;
use crate::model::StructureMap::StructureMap;
use crate::model::OperationDefinition::OperationDefinition;
use crate::model::Group::Group;
use crate::model::EffectEvidenceSynthesis::EffectEvidenceSynthesis;
use crate::model::PaymentNotice::PaymentNotice;
use crate::model::PractitionerRole::PractitionerRole;
use crate::model::Claim::Claim;
use crate::model::OperationOutcome::OperationOutcome;
use crate::model::RelatedPerson::RelatedPerson;
use crate::model::Linkage::Linkage;
use crate::model::PlanDefinition::PlanDefinition;
use crate::model::AuditEvent::AuditEvent;
use crate::model::Encounter::Encounter;
use crate::model::Invoice::Invoice;
use crate::model::NamingSystem::NamingSystem;
use crate::model::Provenance::Provenance;
use crate::model::SubstancePolymer::SubstancePolymer;
use crate::model::TerminologyCapabilities::TerminologyCapabilities;
use crate::model::SubstanceProtein::SubstanceProtein;
use crate::model::ResearchStudy::ResearchStudy;
use crate::model::MedicinalProductManufactured::MedicinalProductManufactured;
use crate::model::Device::Device;
use crate::model::MedicationRequest::MedicationRequest;
use crate::model::ClaimResponse::ClaimResponse;
use crate::model::CoverageEligibilityResponse::CoverageEligibilityResponse;
use crate::model::ExampleScenario::ExampleScenario;
use crate::model::Basic::Basic;
use crate::model::MedicinalProductAuthorization::MedicinalProductAuthorization;
use crate::model::MedicinalProductIndication::MedicinalProductIndication;
use crate::model::MedicinalProductUndesirableEffect::MedicinalProductUndesirableEffect;
use crate::model::CatalogEntry::CatalogEntry;
use crate::model::RequestGroup::RequestGroup;
use crate::model::MedicationStatement::MedicationStatement;
use crate::model::ChargeItem::ChargeItem;
use crate::model::PaymentReconciliation::PaymentReconciliation;
use crate::model::ResearchDefinition::ResearchDefinition;
use crate::model::CoverageEligibilityRequest::CoverageEligibilityRequest;
use crate::model::NutritionOrder::NutritionOrder;
use crate::model::AppointmentResponse::AppointmentResponse;
use crate::model::DetectedIssue::DetectedIssue;
use crate::model::Patient::Patient;
use crate::model::Slot::Slot;
use crate::model::SupplyRequest::SupplyRequest;
use crate::model::SubstanceNucleicAcid::SubstanceNucleicAcid;
use crate::model::MessageHeader::MessageHeader;
use crate::model::ConceptMap::ConceptMap;
use crate::model::ImmunizationRecommendation::ImmunizationRecommendation;
use crate::model::Measure::Measure;
use crate::model::ResearchSubject::ResearchSubject;
use crate::model::Procedure::Procedure;
use crate::model::Coverage::Coverage;
use crate::model::OrganizationAffiliation::OrganizationAffiliation;
use crate::model::MedicationAdministration::MedicationAdministration;
use crate::model::Practitioner::Practitioner;
use crate::model::CommunicationRequest::CommunicationRequest;
use crate::model::CareTeam::CareTeam;
use crate::model::TestReport::TestReport;
use crate::model::ImagingStudy::ImagingStudy;
use crate::model::Media::Media;
use crate::model::Location::Location;
use crate::model::MolecularSequence::MolecularSequence;
use crate::model::SearchParameter::SearchParameter;
use crate::model::ValueSet::ValueSet;
use crate::model::Composition::Composition;
use crate::model::Appointment::Appointment;
use crate::model::ImmunizationEvaluation::ImmunizationEvaluation;
use crate::model::SpecimenDefinition::SpecimenDefinition;
use crate::model::CarePlan::CarePlan;
use crate::model::DocumentManifest::DocumentManifest;
use crate::model::Schedule::Schedule;
use crate::model::SubstanceReferenceInformation::SubstanceReferenceInformation;
use crate::model::Specimen::Specimen;
use crate::model::SubstanceSourceMaterial::SubstanceSourceMaterial;
use crate::model::SupplyDelivery::SupplyDelivery;
use crate::model::ExplanationOfBenefit::ExplanationOfBenefit;
use crate::model::DeviceMetric::DeviceMetric;
use crate::model::Evidence::Evidence;
use crate::model::Library::Library;
use crate::model::MedicinalProductPackaged::MedicinalProductPackaged;
use crate::model::ImplementationGuide::ImplementationGuide;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum ResourceList {
  #[serde(rename = "Account")]
  ResourceAccount(Account),

  #[serde(rename = "ActivityDefinition")]
  ResourceActivityDefinition(ActivityDefinition),

  #[serde(rename = "AdverseEvent")]
  ResourceAdverseEvent(AdverseEvent),

  #[serde(rename = "AllergyIntolerance")]
  ResourceAllergyIntolerance(AllergyIntolerance),

  #[serde(rename = "Appointment")]
  ResourceAppointment(Appointment),

  #[serde(rename = "AppointmentResponse")]
  ResourceAppointmentResponse(AppointmentResponse),

  #[serde(rename = "AuditEvent")]
  ResourceAuditEvent(AuditEvent),

  #[serde(rename = "Basic")]
  ResourceBasic(Basic),

  #[serde(rename = "Binary")]
  ResourceBinary(Binary),

  #[serde(rename = "BiologicallyDerivedProduct")]
  ResourceBiologicallyDerivedProduct(BiologicallyDerivedProduct),

  #[serde(rename = "BodyStructure")]
  ResourceBodyStructure(BodyStructure),

  #[serde(rename = "Bundle")]
  ResourceBundle(Bundle),

  #[serde(rename = "CapabilityStatement")]
  ResourceCapabilityStatement(CapabilityStatement),

  #[serde(rename = "CarePlan")]
  ResourceCarePlan(CarePlan),

  #[serde(rename = "CareTeam")]
  ResourceCareTeam(CareTeam),

  #[serde(rename = "CatalogEntry")]
  ResourceCatalogEntry(CatalogEntry),

  #[serde(rename = "ChargeItem")]
  ResourceChargeItem(ChargeItem),

  #[serde(rename = "ChargeItemDefinition")]
  ResourceChargeItemDefinition(ChargeItemDefinition),

  #[serde(rename = "Claim")]
  ResourceClaim(Claim),

  #[serde(rename = "ClaimResponse")]
  ResourceClaimResponse(ClaimResponse),

  #[serde(rename = "ClinicalImpression")]
  ResourceClinicalImpression(ClinicalImpression),

  #[serde(rename = "CodeSystem")]
  ResourceCodeSystem(CodeSystem),

  #[serde(rename = "Communication")]
  ResourceCommunication(Communication),

  #[serde(rename = "CommunicationRequest")]
  ResourceCommunicationRequest(CommunicationRequest),

  #[serde(rename = "CompartmentDefinition")]
  ResourceCompartmentDefinition(CompartmentDefinition),

  #[serde(rename = "Composition")]
  ResourceComposition(Composition),

  #[serde(rename = "ConceptMap")]
  ResourceConceptMap(ConceptMap),

  #[serde(rename = "Condition")]
  ResourceCondition(Condition),

  #[serde(rename = "Consent")]
  ResourceConsent(Consent),

  #[serde(rename = "Contract")]
  ResourceContract(Contract),

  #[serde(rename = "Coverage")]
  ResourceCoverage(Coverage),

  #[serde(rename = "CoverageEligibilityRequest")]
  ResourceCoverageEligibilityRequest(CoverageEligibilityRequest),

  #[serde(rename = "CoverageEligibilityResponse")]
  ResourceCoverageEligibilityResponse(CoverageEligibilityResponse),

  #[serde(rename = "DetectedIssue")]
  ResourceDetectedIssue(DetectedIssue),

  #[serde(rename = "Device")]
  ResourceDevice(Device),

  #[serde(rename = "DeviceDefinition")]
  ResourceDeviceDefinition(DeviceDefinition),

  #[serde(rename = "DeviceMetric")]
  ResourceDeviceMetric(DeviceMetric),

  #[serde(rename = "DeviceRequest")]
  ResourceDeviceRequest(DeviceRequest),

  #[serde(rename = "DeviceUseStatement")]
  ResourceDeviceUseStatement(DeviceUseStatement),

  #[serde(rename = "DiagnosticReport")]
  ResourceDiagnosticReport(DiagnosticReport),

  #[serde(rename = "DocumentManifest")]
  ResourceDocumentManifest(DocumentManifest),

  #[serde(rename = "DocumentReference")]
  ResourceDocumentReference(DocumentReference),

  #[serde(rename = "EffectEvidenceSynthesis")]
  ResourceEffectEvidenceSynthesis(EffectEvidenceSynthesis),

  #[serde(rename = "Encounter")]
  ResourceEncounter(Encounter),

  #[serde(rename = "Endpoint")]
  ResourceEndpoint(Endpoint),

  #[serde(rename = "EnrollmentRequest")]
  ResourceEnrollmentRequest(EnrollmentRequest),

  #[serde(rename = "EnrollmentResponse")]
  ResourceEnrollmentResponse(EnrollmentResponse),

  #[serde(rename = "EpisodeOfCare")]
  ResourceEpisodeOfCare(EpisodeOfCare),

  #[serde(rename = "EventDefinition")]
  ResourceEventDefinition(EventDefinition),

  #[serde(rename = "Evidence")]
  ResourceEvidence(Evidence),

  #[serde(rename = "EvidenceVariable")]
  ResourceEvidenceVariable(EvidenceVariable),

  #[serde(rename = "ExampleScenario")]
  ResourceExampleScenario(ExampleScenario),

  #[serde(rename = "ExplanationOfBenefit")]
  ResourceExplanationOfBenefit(ExplanationOfBenefit),

  #[serde(rename = "FamilyMemberHistory")]
  ResourceFamilyMemberHistory(FamilyMemberHistory),

  #[serde(rename = "Flag")]
  ResourceFlag(Flag),

  #[serde(rename = "Goal")]
  ResourceGoal(Goal),

  #[serde(rename = "GraphDefinition")]
  ResourceGraphDefinition(GraphDefinition),

  #[serde(rename = "Group")]
  ResourceGroup(Group),

  #[serde(rename = "GuidanceResponse")]
  ResourceGuidanceResponse(GuidanceResponse),

  #[serde(rename = "HealthcareService")]
  ResourceHealthcareService(HealthcareService),

  #[serde(rename = "ImagingStudy")]
  ResourceImagingStudy(ImagingStudy),

  #[serde(rename = "Immunization")]
  ResourceImmunization(Immunization),

  #[serde(rename = "ImmunizationEvaluation")]
  ResourceImmunizationEvaluation(ImmunizationEvaluation),

  #[serde(rename = "ImmunizationRecommendation")]
  ResourceImmunizationRecommendation(ImmunizationRecommendation),

  #[serde(rename = "ImplementationGuide")]
  ResourceImplementationGuide(ImplementationGuide),

  #[serde(rename = "InsurancePlan")]
  ResourceInsurancePlan(InsurancePlan),

  #[serde(rename = "Invoice")]
  ResourceInvoice(Invoice),

  #[serde(rename = "Library")]
  ResourceLibrary(Library),

  #[serde(rename = "Linkage")]
  ResourceLinkage(Linkage),

  #[serde(rename = "List")]
  ResourceList(List),

  #[serde(rename = "Location")]
  ResourceLocation(Location),

  #[serde(rename = "Measure")]
  ResourceMeasure(Measure),

  #[serde(rename = "MeasureReport")]
  ResourceMeasureReport(MeasureReport),

  #[serde(rename = "Media")]
  ResourceMedia(Media),

  #[serde(rename = "Medication")]
  ResourceMedication(Medication),

  #[serde(rename = "MedicationAdministration")]
  ResourceMedicationAdministration(MedicationAdministration),

  #[serde(rename = "MedicationDispense")]
  ResourceMedicationDispense(MedicationDispense),

  #[serde(rename = "MedicationKnowledge")]
  ResourceMedicationKnowledge(MedicationKnowledge),

  #[serde(rename = "MedicationRequest")]
  ResourceMedicationRequest(MedicationRequest),

  #[serde(rename = "MedicationStatement")]
  ResourceMedicationStatement(MedicationStatement),

  #[serde(rename = "MedicinalProduct")]
  ResourceMedicinalProduct(MedicinalProduct),

  #[serde(rename = "MedicinalProductAuthorization")]
  ResourceMedicinalProductAuthorization(MedicinalProductAuthorization),

  #[serde(rename = "MedicinalProductContraindication")]
  ResourceMedicinalProductContraindication(MedicinalProductContraindication),

  #[serde(rename = "MedicinalProductIndication")]
  ResourceMedicinalProductIndication(MedicinalProductIndication),

  #[serde(rename = "MedicinalProductIngredient")]
  ResourceMedicinalProductIngredient(MedicinalProductIngredient),

  #[serde(rename = "MedicinalProductInteraction")]
  ResourceMedicinalProductInteraction(MedicinalProductInteraction),

  #[serde(rename = "MedicinalProductManufactured")]
  ResourceMedicinalProductManufactured(MedicinalProductManufactured),

  #[serde(rename = "MedicinalProductPackaged")]
  ResourceMedicinalProductPackaged(MedicinalProductPackaged),

  #[serde(rename = "MedicinalProductPharmaceutical")]
  ResourceMedicinalProductPharmaceutical(MedicinalProductPharmaceutical),

  #[serde(rename = "MedicinalProductUndesirableEffect")]
  ResourceMedicinalProductUndesirableEffect(MedicinalProductUndesirableEffect),

  #[serde(rename = "MessageDefinition")]
  ResourceMessageDefinition(MessageDefinition),

  #[serde(rename = "MessageHeader")]
  ResourceMessageHeader(MessageHeader),

  #[serde(rename = "MolecularSequence")]
  ResourceMolecularSequence(MolecularSequence),

  #[serde(rename = "NamingSystem")]
  ResourceNamingSystem(NamingSystem),

  #[serde(rename = "NutritionOrder")]
  ResourceNutritionOrder(NutritionOrder),

  #[serde(rename = "Observation")]
  ResourceObservation(Observation),

  #[serde(rename = "ObservationDefinition")]
  ResourceObservationDefinition(ObservationDefinition),

  #[serde(rename = "OperationDefinition")]
  ResourceOperationDefinition(OperationDefinition),

  #[serde(rename = "OperationOutcome")]
  ResourceOperationOutcome(OperationOutcome),

  #[serde(rename = "Organization")]
  ResourceOrganization(Organization),

  #[serde(rename = "OrganizationAffiliation")]
  ResourceOrganizationAffiliation(OrganizationAffiliation),

  #[serde(rename = "Parameters")]
  ResourceParameters(Parameters),

  #[serde(rename = "Patient")]
  ResourcePatient(Patient),

  #[serde(rename = "PaymentNotice")]
  ResourcePaymentNotice(PaymentNotice),

  #[serde(rename = "PaymentReconciliation")]
  ResourcePaymentReconciliation(PaymentReconciliation),

  #[serde(rename = "Person")]
  ResourcePerson(Person),

  #[serde(rename = "PlanDefinition")]
  ResourcePlanDefinition(PlanDefinition),

  #[serde(rename = "Practitioner")]
  ResourcePractitioner(Practitioner),

  #[serde(rename = "PractitionerRole")]
  ResourcePractitionerRole(PractitionerRole),

  #[serde(rename = "Procedure")]
  ResourceProcedure(Procedure),

  #[serde(rename = "Provenance")]
  ResourceProvenance(Provenance),

  #[serde(rename = "Questionnaire")]
  ResourceQuestionnaire(Questionnaire),

  #[serde(rename = "QuestionnaireResponse")]
  ResourceQuestionnaireResponse(QuestionnaireResponse),

  #[serde(rename = "RelatedPerson")]
  ResourceRelatedPerson(RelatedPerson),

  #[serde(rename = "RequestGroup")]
  ResourceRequestGroup(RequestGroup),

  #[serde(rename = "ResearchDefinition")]
  ResourceResearchDefinition(ResearchDefinition),

  #[serde(rename = "ResearchElementDefinition")]
  ResourceResearchElementDefinition(ResearchElementDefinition),

  #[serde(rename = "ResearchStudy")]
  ResourceResearchStudy(ResearchStudy),

  #[serde(rename = "ResearchSubject")]
  ResourceResearchSubject(ResearchSubject),

  #[serde(rename = "RiskAssessment")]
  ResourceRiskAssessment(RiskAssessment),

  #[serde(rename = "RiskEvidenceSynthesis")]
  ResourceRiskEvidenceSynthesis(RiskEvidenceSynthesis),

  #[serde(rename = "Schedule")]
  ResourceSchedule(Schedule),

  #[serde(rename = "SearchParameter")]
  ResourceSearchParameter(SearchParameter),

  #[serde(rename = "ServiceRequest")]
  ResourceServiceRequest(ServiceRequest),

  #[serde(rename = "Slot")]
  ResourceSlot(Slot),

  #[serde(rename = "Specimen")]
  ResourceSpecimen(Specimen),

  #[serde(rename = "SpecimenDefinition")]
  ResourceSpecimenDefinition(SpecimenDefinition),

  #[serde(rename = "StructureDefinition")]
  ResourceStructureDefinition(StructureDefinition),

  #[serde(rename = "StructureMap")]
  ResourceStructureMap(StructureMap),

  #[serde(rename = "Subscription")]
  ResourceSubscription(Subscription),

  #[serde(rename = "Substance")]
  ResourceSubstance(Substance),

  #[serde(rename = "SubstanceNucleicAcid")]
  ResourceSubstanceNucleicAcid(SubstanceNucleicAcid),

  #[serde(rename = "SubstancePolymer")]
  ResourceSubstancePolymer(SubstancePolymer),

  #[serde(rename = "SubstanceProtein")]
  ResourceSubstanceProtein(SubstanceProtein),

  #[serde(rename = "SubstanceReferenceInformation")]
  ResourceSubstanceReferenceInformation(SubstanceReferenceInformation),

  #[serde(rename = "SubstanceSourceMaterial")]
  ResourceSubstanceSourceMaterial(SubstanceSourceMaterial),

  #[serde(rename = "SubstanceSpecification")]
  ResourceSubstanceSpecification(SubstanceSpecification),

  #[serde(rename = "SupplyDelivery")]
  ResourceSupplyDelivery(SupplyDelivery),

  #[serde(rename = "SupplyRequest")]
  ResourceSupplyRequest(SupplyRequest),

  #[serde(rename = "Task")]
  ResourceTask(Task),

  #[serde(rename = "TerminologyCapabilities")]
  ResourceTerminologyCapabilities(TerminologyCapabilities),

  #[serde(rename = "TestReport")]
  ResourceTestReport(TestReport),

  #[serde(rename = "TestScript")]
  ResourceTestScript(TestScript),

  #[serde(rename = "ValueSet")]
  ResourceValueSet(ValueSet),

  #[serde(rename = "VerificationResult")]
  ResourceVerificationResult(VerificationResult),

  #[serde(rename = "VisionPrescription")]
  ResourceVisionPrescription(VisionPrescription),

}
