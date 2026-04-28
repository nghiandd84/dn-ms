use shared_shared_auth::{
    define_resource_perms,
    permission::{CREATE, DELETE, READ, UPDATE},
    ResourcePermission,
};

// EMAIL_TEMPLATE Permission
const EMAIL_TEMPLATE_RESOURCE: &str = "EMAIL_TEMPLATE";

define_resource_perms! {
    CanCreateEmailTemplate => (CREATE, EMAIL_TEMPLATE_RESOURCE),
    CanReadEmailTemplate => (READ, EMAIL_TEMPLATE_RESOURCE),
    CanUpdateEmailTemplate => (UPDATE, EMAIL_TEMPLATE_RESOURCE),
    CanDeleteEmailTemplate => (DELETE, EMAIL_TEMPLATE_RESOURCE)
}

// TEMPLATE_TRANSLATION Permission
const TEMPLATE_TRANSLATION_RESOURCE: &str = "TEMPLATE_TRANSLATION";

define_resource_perms! {
    CanCreateTemplateTranslation => (CREATE, TEMPLATE_TRANSLATION_RESOURCE),
    CanReadTemplateTranslation => (READ, TEMPLATE_TRANSLATION_RESOURCE),
    CanUpdateTemplateTranslation => (UPDATE, TEMPLATE_TRANSLATION_RESOURCE),
    CanDeleteTemplateTranslation => (DELETE, TEMPLATE_TRANSLATION_RESOURCE)
}

// TEMPLATE_PLACEHOLDER Permission
const TEMPLATE_PLACEHOLDER_RESOURCE: &str = "TEMPLATE_PLACEHOLDER";

define_resource_perms! {
    CanCreateTemplatePlaceholder => (CREATE, TEMPLATE_PLACEHOLDER_RESOURCE),
    CanReadTemplatePlaceholder => (READ, TEMPLATE_PLACEHOLDER_RESOURCE),
    CanUpdateTemplatePlaceholder => (UPDATE, TEMPLATE_PLACEHOLDER_RESOURCE),
    CanDeleteTemplatePlaceholder => (DELETE, TEMPLATE_PLACEHOLDER_RESOURCE)
}
