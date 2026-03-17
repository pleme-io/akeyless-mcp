use crate::api::types::*;
use std::fmt::Write;

pub fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

pub fn format_account_custom_field_create(data: &AccountCustomFieldCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    out
}

pub fn format_account_custom_field_delete(data: &AccountCustomFieldDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_account_custom_field_get(data: &AccountCustomFieldGetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.account_id {
        let _ = writeln!(out, "Account Id: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.deletion_date {
        let _ = writeln!(out, "Deletion Date: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.object {
        let _ = writeln!(out, "Object: {v}");
    }
    if let Some(ref v) = data.object_type {
        let _ = writeln!(out, "Object Type: {v}");
    }
    if let Some(ref v) = data.required {
        let _ = writeln!(out, "Required: {v}");
    }
    out
}

pub fn format_account_custom_field_list(data: &AccountCustomFieldListOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_account_custom_field_update(data: &AccountCustomFieldUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_alias_details(data: &DeleteRoleOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_assoc_role_auth_method(data: &CreateRoleAuthMethodAssocOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.assoc_id {
        let _ = writeln!(out, "Assoc Id: {v}");
    }
    out
}

pub fn format_assoc_target_item(data: &CreateTargetItemAssocOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.assoc_id {
        let _ = writeln!(out, "Assoc Id: {v}");
    }
    out
}

pub fn format_auth(data: &AuthOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.complete_auth_link {
        let _ = writeln!(out, "Complete Auth Link: {v}");
    }
    if let Some(ref v) = data.creds {
        let _ = writeln!(out, "Creds: {v}");
    }
    if let Some(ref v) = data.expiration {
        let _ = writeln!(out, "Expiration: {v}");
    }
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_auth_method_create_api_key(data: &AuthMethodCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.access_key {
        let _ = writeln!(out, "Access Key: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_auth_method_create_aws_iam(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_azure_ad(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_cert(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_email(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_gcp(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_k8s(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_kerberos(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_ldap(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_oauth2(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_oci(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_oidc(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_saml(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_create_universal_identity(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_delete(data: &AuthMethodDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_auth_method_get(data: &AuthMethod) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_date {
        let _ = writeln!(out, "Access Date: {v}");
    }
    if let Some(ref v) = data.access_date_display {
        let _ = writeln!(out, "Access Date Display: {v}");
    }
    if let Some(ref v) = data.access_info {
        let _ = writeln!(out, "Access Info: {v}");
    }
    if let Some(ref v) = data.account_id {
        let _ = writeln!(out, "Account Id: {v}");
    }
    if let Some(ref v) = data.associated_gw_ids {
        let _ = writeln!(out, "Associated Gw Ids: {v}");
    }
    if let Some(ref v) = data.auth_method_access_id {
        let _ = writeln!(out, "Auth Method Access Id: {v}");
    }
    if let Some(ref v) = data.auth_method_additional_data {
        let _ = writeln!(out, "Auth Method Additional Data: {v}");
    }
    if let Some(ref v) = data.auth_method_id {
        let _ = writeln!(out, "Auth Method Id: {v}");
    }
    if let Some(ref v) = data.auth_method_name {
        let _ = writeln!(out, "Auth Method Name: {v}");
    }
    if let Some(ref v) = data.auth_method_roles_assoc {
        let _ = writeln!(out, "Auth Method Roles Assoc: {v}");
    }
    if let Some(ref v) = data.client_permissions {
        let _ = writeln!(out, "Client Permissions: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.delete_protection {
        let _ = writeln!(out, "Delete Protection: {v}");
    }
    if let Some(ref v) = data.description {
        let _ = writeln!(out, "Description: {v}");
    }
    if let Some(ref v) = data.expiration_events {
        let _ = writeln!(out, "Expiration Events: {v}");
    }
    if let Some(ref v) = data.is_approved {
        let _ = writeln!(out, "Is Approved: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    out
}

pub fn format_auth_method_list(data: &ListAuthMethodsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.auth_methods {
        let _ = writeln!(out, "Auth Methods: {v}");
    }
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    out
}

pub fn format_auth_method_update_api_key(data: &AuthMethodUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_auth_method_update_aws_iam(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_azure_ad(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_cert(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_email(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_gcp(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_k8s(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_kerberos(data: &AuthMethodCreateOutput) -> String {
    format_auth_method_create_api_key(data)
}

pub fn format_auth_method_update_ldap(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_oauth2(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_oci(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_oidc(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_saml(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_auth_method_update_universal_identity(data: &AuthMethodUpdateOutput) -> String {
    format_auth_method_update_api_key(data)
}

pub fn format_calc_password_security_info(data: &PasswordSecurityInfo) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.breach_info {
        let _ = writeln!(out, "Breach Info: {v}");
    }
    if let Some(ref v) = data.score_info {
        let _ = writeln!(out, "Score Info: {v}");
    }
    out
}

pub fn format_certificate_discovery(data: &CertificateDiscoveryOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.results {
        let _ = writeln!(out, "Results: {v}");
    }
    out
}

pub fn format_change_admin_account_password(data: &serde_json::Value) -> String {
    format!("{:#?}", data)
}

pub fn format_configure(data: &ConfigureOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.profile {
        let _ = writeln!(out, "Profile: {v}");
    }
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_connect(data: &ConnectOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_create_artifactory_target(data: &CreateArtifactoryTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_auth_method(data: &CreateAuthMethodOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.access_key {
        let _ = writeln!(out, "Access Key: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_auth_method_awsiam(data: &CreateAuthMethodAwsiamOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_azure_ad(data: &CreateAuthMethodAzureAdOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_cert(data: &CreateAuthMethodCertOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_email(data: &CreateAuthMethodEmailOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_gcp(data: &CreateAuthMethodGcpOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_huawei(data: &CreateAuthMethodHuaweiOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_k8s(data: &CreateAuthMethodK8sOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_create_auth_method_ldap(data: &CreateAuthMethodLdapOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_create_auth_method_o_auth2(data: &CreateAuthMethodOAuth2Output) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_oci(data: &CreateAuthMethodOciOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_oidc(data: &CreateAuthMethodOidcOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_saml(data: &CreateAuthMethodSamlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_auth_method_universal_identity(data: &CreateAuthMethodUniversalIdentityOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_create_aws_target(data: &CreateAwsTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_azure_target(data: &CreateAzureTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_certificate(data: &CreateCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_classic_key(data: &CreateClassicKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.classic_key_id {
        let _ = writeln!(out, "Classic Key Id: {v}");
    }
    if let Some(ref v) = data.classic_key_name {
        let _ = writeln!(out, "Classic Key Name: {v}");
    }
    if let Some(ref v) = data.classic_key_type {
        let _ = writeln!(out, "Classic Key Type: {v}");
    }
    if let Some(ref v) = data.public_key {
        let _ = writeln!(out, "Public Key: {v}");
    }
    out
}

pub fn format_create_db_target(data: &CreateDbTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_dfc_key(data: &CreateDfcKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.fragment_results {
        let _ = writeln!(out, "Fragment Results: {v}");
    }
    out
}

pub fn format_create_dockerhub_target(data: &CreateDockerhubTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_dynamic_secret(data: &CreateDynamicSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_create_eks_target(data: &CreateEksTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_esm(data: &CreateEsmOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.external_secret_manager_id {
        let _ = writeln!(out, "External Secret Manager Id: {v}");
    }
    if let Some(ref v) = data.external_secret_manager_name {
        let _ = writeln!(out, "External Secret Manager Name: {v}");
    }
    out
}

pub fn format_create_event_forwarder(data: &CreateEventForwarderOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.event_forwarder_id {
        let _ = writeln!(out, "Event Forwarder Id: {v}");
    }
    out
}

pub fn format_create_gcp_target(data: &CreateGcpTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_github_target(data: &CreateGithubTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_gitlab_target(data: &CreateGitlabTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_gke_target(data: &CreateGkeTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_global_sign_atlas_target(data: &CreateGlobalSignAtlasTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_global_sign_target(data: &CreateGlobalSignTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_godaddy_target(data: &CreateGodaddyTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_group(data: &CreateGroupOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.group_alias {
        let _ = writeln!(out, "Group Alias: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_hashi_vault_target(data: &CreateHashiVaultTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_native_k8s_target(data: &CreateNativeK8sTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_key(data: &CreateKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.display_id {
        let _ = writeln!(out, "Display Id: {v}");
    }
    if let Some(ref v) = data.fragment_results {
        let _ = writeln!(out, "Fragment Results: {v}");
    }
    if let Some(ref v) = data.item_id {
        let _ = writeln!(out, "Item Id: {v}");
    }
    out
}

pub fn format_createldap_target(data: &CreateLdapTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_linked_target(data: &CreateLinkedTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_oidc_app(data: &CreateOidcAppOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.client_id {
        let _ = writeln!(out, "Client Id: {v}");
    }
    if let Some(ref v) = data.client_secret {
        let _ = writeln!(out, "Client Secret: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_passkey(data: &CreatePasskeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.classic_key_id {
        let _ = writeln!(out, "Classic Key Id: {v}");
    }
    if let Some(ref v) = data.classic_key_name {
        let _ = writeln!(out, "Classic Key Name: {v}");
    }
    if let Some(ref v) = data.classic_key_type {
        let _ = writeln!(out, "Classic Key Type: {v}");
    }
    if let Some(ref v) = data.public_key {
        let _ = writeln!(out, "Public Key: {v}");
    }
    out
}

pub fn format_create_ping_target(data: &CreatePingTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_pki_cert_issuer(data: &CreatePkiCertIssuerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_rabbit_mq_target(data: &CreateRabbitMqTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_role(data: &CreateRoleOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_create_rotated_secret(data: &CreateRotatedSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_salesforce_target(data: &CreateSalesforceTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_secret(data: &CreateSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_ssh_cert_issuer(data: &CreateSshCertIssuerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_ssh_target(data: &CreateSshTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_tokenizer(data: &CreateTokenizerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_create_usc(data: &CreateUscOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.universal_secrets_connector_id {
        let _ = writeln!(out, "Universal Secrets Connector Id: {v}");
    }
    if let Some(ref v) = data.universal_secrets_connector_name {
        let _ = writeln!(out, "Universal Secrets Connector Name: {v}");
    }
    out
}

pub fn format_create_user_event(data: &CreateUserEventOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.response {
        let _ = writeln!(out, "Response: {v}");
    }
    out
}

pub fn format_create_web_target(data: &CreateWebTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_windows_target(data: &CreateWindowsTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_create_zero_ssl_target(data: &CreateZeroSslTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_deactivate_acme_account(data: &DeactivateAcmeAccountOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_decrypt(data: &DecryptOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_decrypt_batch(data: &DecryptOutput) -> String {
    format_decrypt(data)
}

pub fn format_decrypt_gpg(data: &DecryptGpgOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_decrypt_pkcs1(data: &DecryptPkcs1Output) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.plaintext {
        let _ = writeln!(out, "Plaintext: {v}");
    }
    out
}

pub fn format_decrypt_with_classic_key(data: &DecryptWithClassicKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_derive_key(data: &DeriveKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.key {
        let _ = writeln!(out, "Key: {v}");
    }
    if let Some(ref v) = data.salt {
        let _ = writeln!(out, "Salt: {v}");
    }
    out
}

pub fn format_describe_item(data: &Item) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_date {
        let _ = writeln!(out, "Access Date: {v}");
    }
    if let Some(ref v) = data.access_date_display {
        let _ = writeln!(out, "Access Date Display: {v}");
    }
    if let Some(ref v) = data.access_request_status {
        let _ = writeln!(out, "Access Request Status: {v}");
    }
    if let Some(ref v) = data.auto_rotate {
        let _ = writeln!(out, "Auto Rotate: {v}");
    }
    if let Some(ref v) = data.bastion_details {
        let _ = writeln!(out, "Bastion Details: {v}");
    }
    if let Some(ref v) = data.cert_issuer_signer_key_name {
        let _ = writeln!(out, "Cert Issuer Signer Key Name: {v}");
    }
    if let Some(ref v) = data.certificate_issue_details {
        let _ = writeln!(out, "Certificate Issue Details: {v}");
    }
    if let Some(ref v) = data.certificates {
        let _ = writeln!(out, "Certificates: {v}");
    }
    if let Some(ref v) = data.client_permissions {
        let _ = writeln!(out, "Client Permissions: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.customer_fragment_id {
        let _ = writeln!(out, "Customer Fragment Id: {v}");
    }
    if let Some(ref v) = data.delete_protection {
        let _ = writeln!(out, "Delete Protection: {v}");
    }
    if let Some(ref v) = data.deletion_date {
        let _ = writeln!(out, "Deletion Date: {v}");
    }
    if let Some(ref v) = data.display_id {
        let _ = writeln!(out, "Display Id: {v}");
    }
    if let Some(ref v) = data.gateway_details {
        let _ = writeln!(out, "Gateway Details: {v}");
    }
    if let Some(ref v) = data.is_access_request_enabled {
        let _ = writeln!(out, "Is Access Request Enabled: {v}");
    }
    if let Some(ref v) = data.is_enabled {
        let _ = writeln!(out, "Is Enabled: {v}");
    }
    if let Some(ref v) = data.item_accessibility {
        let _ = writeln!(out, "Item Accessibility: {v}");
    }
    if let Some(ref v) = data.item_custom_fields_details {
        let _ = writeln!(out, "Item Custom Fields Details: {v}");
    }
    if let Some(ref v) = data.item_general_info {
        let _ = writeln!(out, "Item General Info: {v}");
    }
    if let Some(ref v) = data.item_id {
        let _ = writeln!(out, "Item Id: {v}");
    }
    if let Some(ref v) = data.item_metadata {
        let _ = writeln!(out, "Item Metadata: {v}");
    }
    if let Some(ref v) = data.item_name {
        let _ = writeln!(out, "Item Name: {v}");
    }
    if let Some(ref v) = data.item_size {
        let _ = writeln!(out, "Item Size: {v}");
    }
    if let Some(ref v) = data.item_state {
        let _ = writeln!(out, "Item State: {v}");
    }
    if let Some(ref v) = data.item_sub_type {
        let _ = writeln!(out, "Item Sub Type: {v}");
    }
    if let Some(ref v) = data.item_tags {
        let _ = writeln!(out, "Item Tags: {v}");
    }
    if let Some(ref v) = data.item_targets_assoc {
        let _ = writeln!(out, "Item Targets Assoc: {v}");
    }
    if let Some(ref v) = data.item_type {
        let _ = writeln!(out, "Item Type: {v}");
    }
    if let Some(ref v) = data.item_versions {
        let _ = writeln!(out, "Item Versions: {v}");
    }
    if let Some(ref v) = data.last_rotation_date {
        let _ = writeln!(out, "Last Rotation Date: {v}");
    }
    if let Some(ref v) = data.last_version {
        let _ = writeln!(out, "Last Version: {v}");
    }
    if let Some(ref v) = data.linked_details {
        let _ = writeln!(out, "Linked Details: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    if let Some(ref v) = data.next_rotation_date {
        let _ = writeln!(out, "Next Rotation Date: {v}");
    }
    if let Some(ref v) = data.protection_key_name {
        let _ = writeln!(out, "Protection Key Name: {v}");
    }
    if let Some(ref v) = data.protection_key_type {
        let _ = writeln!(out, "Protection Key Type: {v}");
    }
    if let Some(ref v) = data.public_value {
        let _ = writeln!(out, "Public Value: {v}");
    }
    if let Some(ref v) = data.rotation_interval {
        let _ = writeln!(out, "Rotation Interval: {v}");
    }
    if let Some(ref v) = data.shared_by {
        let _ = writeln!(out, "Shared By: {v}");
    }
    if let Some(ref v) = data.target_versions {
        let _ = writeln!(out, "Target Versions: {v}");
    }
    if let Some(ref v) = data.usc_sync_associated_items {
        let _ = writeln!(out, "Usc Sync Associated Items: {v}");
    }
    if let Some(ref v) = data.with_customer_fragment {
        let _ = writeln!(out, "With Customer Fragment: {v}");
    }
    out
}

pub fn format_describe_permissions(data: &DescribePermissionsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.client_permissions {
        let _ = writeln!(out, "Client Permissions: {v}");
    }
    out
}

pub fn format_describe_assoc(data: &RoleAssociationDetails) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.assoc_id {
        let _ = writeln!(out, "Assoc Id: {v}");
    }
    if let Some(ref v) = data.auth_method_name {
        let _ = writeln!(out, "Auth Method Name: {v}");
    }
    if let Some(ref v) = data.auth_method_sub_claims {
        let _ = writeln!(out, "Auth Method Sub Claims: {v}");
    }
    if let Some(ref v) = data.is_subclaims_with_operator {
        let _ = writeln!(out, "Is Subclaims With Operator: {v}");
    }
    if let Some(ref v) = data.role_name {
        let _ = writeln!(out, "Role Name: {v}");
    }
    if let Some(ref v) = data.sub_claims_case_sensitive {
        let _ = writeln!(out, "Sub Claims Case Sensitive: {v}");
    }
    out
}

pub fn format_describe_sub_claims(data: &DescribeSubClaimsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.sub_claims {
        let _ = writeln!(out, "Sub Claims: {v}");
    }
    out
}

pub fn format_detokenize(data: &DetokenizeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_detokenize_batch(data: &DetokenizeOutput) -> String {
    format_detokenize(data)
}

pub fn format_dynamic_secret_create_artifactory(data: &DynamicSecretCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.dynamic_secret_details {
        let _ = writeln!(out, "Dynamic Secret Details: {v}");
    }
    out
}

pub fn format_dynamic_secret_create_aws(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_azure(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_cassandra(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_custom(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_dockerhub(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_eks(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_gcp(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_github(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_gitlab(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_gke(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_google_workspace(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_hana_db(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_k8s(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_ldap(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_mongo_db(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_ms_sql(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_my_sql(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_open_ai(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_oracle_db(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_ping(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_postgre_sql(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_rabbit_mq(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_rdp(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_redis(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_redshift(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_snowflake(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_create_venafi(data: &DynamicSecretCreateOutput) -> String {
    format_dynamic_secret_create_artifactory(data)
}

pub fn format_dynamic_secret_delete(data: &DynamicSecretDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.dynamic_secret_name {
        let _ = writeln!(out, "Dynamic Secret Name: {v}");
    }
    out
}

pub fn format_dynamic_secret_get(data: &DsProducerDetails) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_token_manager_id {
        let _ = writeln!(out, "Access Token Manager Id: {v}");
    }
    if let Some(ref v) = data.acl_rules {
        let _ = writeln!(out, "Acl Rules: {v}");
    }
    if let Some(ref v) = data.active {
        let _ = writeln!(out, "Active: {v}");
    }
    if let Some(ref v) = data.admin_name {
        let _ = writeln!(out, "Admin Name: {v}");
    }
    if let Some(ref v) = data.admin_pwd {
        let _ = writeln!(out, "Admin Pwd: {v}");
    }
    if let Some(ref v) = data.admin_rotation_interval_days {
        let _ = writeln!(out, "Admin Rotation Interval Days: {v}");
    }
    if let Some(ref v) = data.administrative_port {
        let _ = writeln!(out, "Administrative Port: {v}");
    }
    if let Some(ref v) = data.api_key {
        let _ = writeln!(out, "Api Key: {v}");
    }
    if let Some(ref v) = data.api_key_id {
        let _ = writeln!(out, "Api Key Id: {v}");
    }
    if let Some(ref v) = data.artifactory_admin_apikey {
        let _ = writeln!(out, "Artifactory Admin Apikey: {v}");
    }
    if let Some(ref v) = data.artifactory_admin_username {
        let _ = writeln!(out, "Artifactory Admin Username: {v}");
    }
    if let Some(ref v) = data.artifactory_base_url {
        let _ = writeln!(out, "Artifactory Base Url: {v}");
    }
    if let Some(ref v) = data.artifactory_token_audience {
        let _ = writeln!(out, "Artifactory Token Audience: {v}");
    }
    if let Some(ref v) = data.artifactory_token_scope {
        let _ = writeln!(out, "Artifactory Token Scope: {v}");
    }
    if let Some(ref v) = data.authorization_port {
        let _ = writeln!(out, "Authorization Port: {v}");
    }
    if let Some(ref v) = data.aws_access_key_id {
        let _ = writeln!(out, "Aws Access Key Id: {v}");
    }
    if let Some(ref v) = data.aws_access_mode {
        let _ = writeln!(out, "Aws Access Mode: {v}");
    }
    if let Some(ref v) = data.aws_external_id {
        let _ = writeln!(out, "Aws External Id: {v}");
    }
    if let Some(ref v) = data.aws_region {
        let _ = writeln!(out, "Aws Region: {v}");
    }
    if let Some(ref v) = data.aws_role_arns {
        let _ = writeln!(out, "Aws Role Arns: {v}");
    }
    if let Some(ref v) = data.aws_secret_access_key {
        let _ = writeln!(out, "Aws Secret Access Key: {v}");
    }
    if let Some(ref v) = data.aws_session_tags {
        let _ = writeln!(out, "Aws Session Tags: {v}");
    }
    if let Some(ref v) = data.aws_session_token {
        let _ = writeln!(out, "Aws Session Token: {v}");
    }
    if let Some(ref v) = data.aws_transitive_tag_keys {
        let _ = writeln!(out, "Aws Transitive Tag Keys: {v}");
    }
    if let Some(ref v) = data.aws_user_console_access {
        let _ = writeln!(out, "Aws User Console Access: {v}");
    }
    if let Some(ref v) = data.aws_user_groups {
        let _ = writeln!(out, "Aws User Groups: {v}");
    }
    if let Some(ref v) = data.aws_user_policies {
        let _ = writeln!(out, "Aws User Policies: {v}");
    }
    if let Some(ref v) = data.aws_user_programmatic_access {
        let _ = writeln!(out, "Aws User Programmatic Access: {v}");
    }
    if let Some(ref v) = data.azure_administrative_unit {
        let _ = writeln!(out, "Azure Administrative Unit: {v}");
    }
    if let Some(ref v) = data.azure_app_object_id {
        let _ = writeln!(out, "Azure App Object Id: {v}");
    }
    if let Some(ref v) = data.azure_client_id {
        let _ = writeln!(out, "Azure Client Id: {v}");
    }
    if let Some(ref v) = data.azure_client_secret {
        let _ = writeln!(out, "Azure Client Secret: {v}");
    }
    if let Some(ref v) = data.azure_cloud {
        let _ = writeln!(out, "Azure Cloud: {v}");
    }
    if let Some(ref v) = data.azure_fixed_user_name_sub_claim_key {
        let _ = writeln!(out, "Azure Fixed User Name Sub Claim Key: {v}");
    }
    if let Some(ref v) = data.azure_fixed_user_only {
        let _ = writeln!(out, "Azure Fixed User Only: {v}");
    }
    if let Some(ref v) = data.azure_resource_group_name {
        let _ = writeln!(out, "Azure Resource Group Name: {v}");
    }
    if let Some(ref v) = data.azure_resource_name {
        let _ = writeln!(out, "Azure Resource Name: {v}");
    }
    if let Some(ref v) = data.azure_subscription_id {
        let _ = writeln!(out, "Azure Subscription Id: {v}");
    }
    if let Some(ref v) = data.azure_tenant_id {
        let _ = writeln!(out, "Azure Tenant Id: {v}");
    }
    if let Some(ref v) = data.azure_user_groups_obj_id {
        let _ = writeln!(out, "Azure User Groups Obj Id: {v}");
    }
    if let Some(ref v) = data.azure_user_portal_access {
        let _ = writeln!(out, "Azure User Portal Access: {v}");
    }
    if let Some(ref v) = data.azure_user_programmatic_access {
        let _ = writeln!(out, "Azure User Programmatic Access: {v}");
    }
    if let Some(ref v) = data.azure_user_roles_template_id {
        let _ = writeln!(out, "Azure User Roles Template Id: {v}");
    }
    if let Some(ref v) = data.azure_username {
        let _ = writeln!(out, "Azure Username: {v}");
    }
    if let Some(ref v) = data.cassandra_creation_statements {
        let _ = writeln!(out, "Cassandra Creation Statements: {v}");
    }
    if let Some(ref v) = data.chef_organizations {
        let _ = writeln!(out, "Chef Organizations: {v}");
    }
    if let Some(ref v) = data.chef_server_access_mode {
        let _ = writeln!(out, "Chef Server Access Mode: {v}");
    }
    if let Some(ref v) = data.chef_server_host_name {
        let _ = writeln!(out, "Chef Server Host Name: {v}");
    }
    if let Some(ref v) = data.chef_server_key {
        let _ = writeln!(out, "Chef Server Key: {v}");
    }
    if let Some(ref v) = data.chef_server_port {
        let _ = writeln!(out, "Chef Server Port: {v}");
    }
    if let Some(ref v) = data.chef_server_url {
        let _ = writeln!(out, "Chef Server Url: {v}");
    }
    if let Some(ref v) = data.chef_server_username {
        let _ = writeln!(out, "Chef Server Username: {v}");
    }
    if let Some(ref v) = data.chef_skip_ssl {
        let _ = writeln!(out, "Chef Skip Ssl: {v}");
    }
    if let Some(ref v) = data.client_authentication_type {
        let _ = writeln!(out, "Client Authentication Type: {v}");
    }
    if let Some(ref v) = data.cloud_service_provider {
        let _ = writeln!(out, "Cloud Service Provider: {v}");
    }
    if let Some(ref v) = data.cluster_mode {
        let _ = writeln!(out, "Cluster Mode: {v}");
    }
    if let Some(ref v) = data.connection_type {
        let _ = writeln!(out, "Connection Type: {v}");
    }
    if let Some(ref v) = data.create_sync_url {
        let _ = writeln!(out, "Create Sync Url: {v}");
    }
    if let Some(ref v) = data.db_client_id {
        let _ = writeln!(out, "Db Client Id: {v}");
    }
    if let Some(ref v) = data.db_client_secret {
        let _ = writeln!(out, "Db Client Secret: {v}");
    }
    if let Some(ref v) = data.db_host_name {
        let _ = writeln!(out, "Db Host Name: {v}");
    }
    if let Some(ref v) = data.db_isolation_level {
        let _ = writeln!(out, "Db Isolation Level: {v}");
    }
    if let Some(ref v) = data.db_max_idle_conns {
        let _ = writeln!(out, "Db Max Idle Conns: {v}");
    }
    if let Some(ref v) = data.db_max_open_conns {
        let _ = writeln!(out, "Db Max Open Conns: {v}");
    }
    if let Some(ref v) = data.db_name {
        let _ = writeln!(out, "Db Name: {v}");
    }
    if let Some(ref v) = data.db_port {
        let _ = writeln!(out, "Db Port: {v}");
    }
    if let Some(ref v) = data.db_private_key {
        let _ = writeln!(out, "Db Private Key: {v}");
    }
    if let Some(ref v) = data.db_private_key_passphrase {
        let _ = writeln!(out, "Db Private Key Passphrase: {v}");
    }
    if let Some(ref v) = data.db_pwd {
        let _ = writeln!(out, "Db Pwd: {v}");
    }
    if let Some(ref v) = data.db_server_certificates {
        let _ = writeln!(out, "Db Server Certificates: {v}");
    }
    if let Some(ref v) = data.db_server_name {
        let _ = writeln!(out, "Db Server Name: {v}");
    }
    if let Some(ref v) = data.db_tenant_id {
        let _ = writeln!(out, "Db Tenant Id: {v}");
    }
    if let Some(ref v) = data.db_user_name {
        let _ = writeln!(out, "Db User Name: {v}");
    }
    if let Some(ref v) = data.delete_protection {
        let _ = writeln!(out, "Delete Protection: {v}");
    }
    if let Some(ref v) = data.dynamic_secret_id {
        let _ = writeln!(out, "Dynamic Secret Id: {v}");
    }
    if let Some(ref v) = data.dynamic_secret_key {
        let _ = writeln!(out, "Dynamic Secret Key: {v}");
    }
    if let Some(ref v) = data.dynamic_secret_name {
        let _ = writeln!(out, "Dynamic Secret Name: {v}");
    }
    if let Some(ref v) = data.dynamic_secret_type {
        let _ = writeln!(out, "Dynamic Secret Type: {v}");
    }
    if let Some(ref v) = data.eks_access_key_id {
        let _ = writeln!(out, "Eks Access Key Id: {v}");
    }
    if let Some(ref v) = data.eks_assume_role {
        let _ = writeln!(out, "Eks Assume Role: {v}");
    }
    if let Some(ref v) = data.eks_cluster_ca_certificate {
        let _ = writeln!(out, "Eks Cluster Ca Certificate: {v}");
    }
    if let Some(ref v) = data.eks_cluster_endpoint {
        let _ = writeln!(out, "Eks Cluster Endpoint: {v}");
    }
    if let Some(ref v) = data.eks_cluster_name {
        let _ = writeln!(out, "Eks Cluster Name: {v}");
    }
    if let Some(ref v) = data.eks_region {
        let _ = writeln!(out, "Eks Region: {v}");
    }
    if let Some(ref v) = data.eks_secret_access_key {
        let _ = writeln!(out, "Eks Secret Access Key: {v}");
    }
    if let Some(ref v) = data.enable_admin_rotation {
        let _ = writeln!(out, "Enable Admin Rotation: {v}");
    }
    if let Some(ref v) = data.enforce_replay_prevention {
        let _ = writeln!(out, "Enforce Replay Prevention: {v}");
    }
    if let Some(ref v) = data.expiration_date {
        let _ = writeln!(out, "Expiration Date: {v}");
    }
    if let Some(ref v) = data.externally_provided_user {
        let _ = writeln!(out, "Externally Provided User: {v}");
    }
    if let Some(ref v) = data.failure_message {
        let _ = writeln!(out, "Failure Message: {v}");
    }
    if let Some(ref v) = data.fixed_user_only {
        let _ = writeln!(out, "Fixed User Only: {v}");
    }
    if let Some(ref v) = data.gcp_access_type {
        let _ = writeln!(out, "Gcp Access Type: {v}");
    }
    if let Some(ref v) = data.gcp_fixed_user_claim_keyname {
        let _ = writeln!(out, "Gcp Fixed User Claim Keyname: {v}");
    }
    if let Some(ref v) = data.gcp_key_algo {
        let _ = writeln!(out, "Gcp Key Algo: {v}");
    }
    if let Some(ref v) = data.gcp_project_id {
        let _ = writeln!(out, "Gcp Project Id: {v}");
    }
    if let Some(ref v) = data.gcp_role_bindings {
        let _ = writeln!(out, "Gcp Role Bindings: {v}");
    }
    if let Some(ref v) = data.gcp_role_names {
        let _ = writeln!(out, "Gcp Role Names: {v}");
    }
    if let Some(ref v) = data.gcp_service_account_email {
        let _ = writeln!(out, "Gcp Service Account Email: {v}");
    }
    if let Some(ref v) = data.gcp_service_account_key {
        let _ = writeln!(out, "Gcp Service Account Key: {v}");
    }
    if let Some(ref v) = data.gcp_service_account_key_base64 {
        let _ = writeln!(out, "Gcp Service Account Key Base64: {v}");
    }
    if let Some(ref v) = data.gcp_service_account_key_id {
        let _ = writeln!(out, "Gcp Service Account Key Id: {v}");
    }
    if let Some(ref v) = data.gcp_service_account_type {
        let _ = writeln!(out, "Gcp Service Account Type: {v}");
    }
    if let Some(ref v) = data.gcp_tmp_service_account_name {
        let _ = writeln!(out, "Gcp Tmp Service Account Name: {v}");
    }
    if let Some(ref v) = data.gcp_token_lifetime {
        let _ = writeln!(out, "Gcp Token Lifetime: {v}");
    }
    if let Some(ref v) = data.gcp_token_scope {
        let _ = writeln!(out, "Gcp Token Scope: {v}");
    }
    if let Some(ref v) = data.gcp_token_type {
        let _ = writeln!(out, "Gcp Token Type: {v}");
    }
    if let Some(ref v) = data.github_app_id {
        let _ = writeln!(out, "Github App Id: {v}");
    }
    if let Some(ref v) = data.github_app_private_key {
        let _ = writeln!(out, "Github App Private Key: {v}");
    }
    if let Some(ref v) = data.github_base_url {
        let _ = writeln!(out, "Github Base Url: {v}");
    }
    if let Some(ref v) = data.github_installation_id {
        let _ = writeln!(out, "Github Installation Id: {v}");
    }
    if let Some(ref v) = data.github_installation_token_permissions {
        let _ = writeln!(out, "Github Installation Token Permissions: {v}");
    }
    if let Some(ref v) = data.github_installation_token_repositories {
        let _ = writeln!(out, "Github Installation Token Repositories: {v}");
    }
    if let Some(ref v) = data.github_installation_token_repositories_ids {
        let _ = writeln!(out, "Github Installation Token Repositories Ids: {v}");
    }
    if let Some(ref v) = data.github_organization_name {
        let _ = writeln!(out, "Github Organization Name: {v}");
    }
    if let Some(ref v) = data.github_repository_path {
        let _ = writeln!(out, "Github Repository Path: {v}");
    }
    if let Some(ref v) = data.gitlab_access_token {
        let _ = writeln!(out, "Gitlab Access Token: {v}");
    }
    if let Some(ref v) = data.gitlab_access_type {
        let _ = writeln!(out, "Gitlab Access Type: {v}");
    }
    if let Some(ref v) = data.gitlab_certificate {
        let _ = writeln!(out, "Gitlab Certificate: {v}");
    }
    if let Some(ref v) = data.gitlab_group_name {
        let _ = writeln!(out, "Gitlab Group Name: {v}");
    }
    if let Some(ref v) = data.gitlab_project_name {
        let _ = writeln!(out, "Gitlab Project Name: {v}");
    }
    if let Some(ref v) = data.gitlab_role {
        let _ = writeln!(out, "Gitlab Role: {v}");
    }
    if let Some(ref v) = data.gitlab_token_scope {
        let _ = writeln!(out, "Gitlab Token Scope: {v}");
    }
    if let Some(ref v) = data.gitlab_url {
        let _ = writeln!(out, "Gitlab Url: {v}");
    }
    if let Some(ref v) = data.gke_cluster_ca_certificate {
        let _ = writeln!(out, "Gke Cluster Ca Certificate: {v}");
    }
    if let Some(ref v) = data.gke_cluster_endpoint {
        let _ = writeln!(out, "Gke Cluster Endpoint: {v}");
    }
    if let Some(ref v) = data.gke_cluster_name {
        let _ = writeln!(out, "Gke Cluster Name: {v}");
    }
    if let Some(ref v) = data.gke_service_account_key {
        let _ = writeln!(out, "Gke Service Account Key: {v}");
    }
    if let Some(ref v) = data.gke_service_account_name {
        let _ = writeln!(out, "Gke Service Account Name: {v}");
    }
    if let Some(ref v) = data.google_workspace_access_mode {
        let _ = writeln!(out, "Google Workspace Access Mode: {v}");
    }
    if let Some(ref v) = data.google_workspace_admin_name {
        let _ = writeln!(out, "Google Workspace Admin Name: {v}");
    }
    if let Some(ref v) = data.google_workspace_fixed_user_name_sub_claim_key {
        let _ = writeln!(out, "Google Workspace Fixed User Name Sub Claim Key: {v}");
    }
    if let Some(ref v) = data.google_workspace_group_name {
        let _ = writeln!(out, "Google Workspace Group Name: {v}");
    }
    if let Some(ref v) = data.google_workspace_group_role {
        let _ = writeln!(out, "Google Workspace Group Role: {v}");
    }
    if let Some(ref v) = data.google_workspace_role_name {
        let _ = writeln!(out, "Google Workspace Role Name: {v}");
    }
    if let Some(ref v) = data.google_workspace_role_scope {
        let _ = writeln!(out, "Google Workspace Role Scope: {v}");
    }
    if let Some(ref v) = data.grace_rotated_secret_key {
        let _ = writeln!(out, "Grace Rotated Secret Key: {v}");
    }
    if let Some(ref v) = data.grant_types {
        let _ = writeln!(out, "Grant Types: {v}");
    }
    if let Some(ref v) = data.groups {
        let _ = writeln!(out, "Groups: {v}");
    }
    if let Some(ref v) = data.gw_cloud_identity_external_id_opt {
        let _ = writeln!(out, "Gw Cloud Identity External Id Opt: {v}");
    }
    if let Some(ref v) = data.hanadb_creation_statements {
        let _ = writeln!(out, "Hanadb Creation Statements: {v}");
    }
    if let Some(ref v) = data.hanadb_revocation_statements {
        let _ = writeln!(out, "Hanadb Revocation Statements: {v}");
    }
    if let Some(ref v) = data.host_name {
        let _ = writeln!(out, "Host Name: {v}");
    }
    if let Some(ref v) = data.host_port {
        let _ = writeln!(out, "Host Port: {v}");
    }
    if let Some(ref v) = data.implementation_type {
        let _ = writeln!(out, "Implementation Type: {v}");
    }
    if let Some(ref v) = data.is_fixed_user {
        let _ = writeln!(out, "Is Fixed User: {v}");
    }
    if let Some(ref v) = data.issuer {
        let _ = writeln!(out, "Issuer: {v}");
    }
    if let Some(ref v) = data.item_custom_fields_details {
        let _ = writeln!(out, "Item Custom Fields Details: {v}");
    }
    if let Some(ref v) = data.item_targets_assoc {
        let _ = writeln!(out, "Item Targets Assoc: {v}");
    }
    if let Some(ref v) = data.jwks {
        let _ = writeln!(out, "Jwks: {v}");
    }
    if let Some(ref v) = data.jwks_url {
        let _ = writeln!(out, "Jwks Url: {v}");
    }
    if let Some(ref v) = data.k8s_allowed_namespaces {
        let _ = writeln!(out, "K8s Allowed Namespaces: {v}");
    }
    if let Some(ref v) = data.k8s_auth_type {
        let _ = writeln!(out, "K8s Auth Type: {v}");
    }
    if let Some(ref v) = data.k8s_bearer_token {
        let _ = writeln!(out, "K8s Bearer Token: {v}");
    }
    if let Some(ref v) = data.k8s_client_cert_data {
        let _ = writeln!(out, "K8s Client Cert Data: {v}");
    }
    if let Some(ref v) = data.k8s_client_key_data {
        let _ = writeln!(out, "K8s Client Key Data: {v}");
    }
    if let Some(ref v) = data.k8s_cluster_ca_certificate {
        let _ = writeln!(out, "K8s Cluster Ca Certificate: {v}");
    }
    if let Some(ref v) = data.k8s_cluster_endpoint {
        let _ = writeln!(out, "K8s Cluster Endpoint: {v}");
    }
    if let Some(ref v) = data.k8s_cluster_name {
        let _ = writeln!(out, "K8s Cluster Name: {v}");
    }
    if let Some(ref v) = data.k8s_dynamic_mode {
        let _ = writeln!(out, "K8s Dynamic Mode: {v}");
    }
    if let Some(ref v) = data.k8s_multiple_doc_yaml_temp_definition {
        let _ = writeln!(out, "K8s Multiple Doc Yaml Temp Definition: {v}");
    }
    if let Some(ref v) = data.k8s_namespace {
        let _ = writeln!(out, "K8s Namespace: {v}");
    }
    if let Some(ref v) = data.k8s_role_name {
        let _ = writeln!(out, "K8s Role Name: {v}");
    }
    if let Some(ref v) = data.k8s_role_type {
        let _ = writeln!(out, "K8s Role Type: {v}");
    }
    if let Some(ref v) = data.k8s_service_account {
        let _ = writeln!(out, "K8s Service Account: {v}");
    }
    if let Some(ref v) = data.last_admin_rotation {
        let _ = writeln!(out, "Last Admin Rotation: {v}");
    }
    if let Some(ref v) = data.ldap_audience {
        let _ = writeln!(out, "Ldap Audience: {v}");
    }
    if let Some(ref v) = data.ldap_bind_dn {
        let _ = writeln!(out, "Ldap Bind Dn: {v}");
    }
    if let Some(ref v) = data.ldap_bind_password {
        let _ = writeln!(out, "Ldap Bind Password: {v}");
    }
    if let Some(ref v) = data.ldap_certificate {
        let _ = writeln!(out, "Ldap Certificate: {v}");
    }
    if let Some(ref v) = data.ldap_fixed_user_name_sub_claim_key {
        let _ = writeln!(out, "Ldap Fixed User Name Sub Claim Key: {v}");
    }
    if let Some(ref v) = data.ldap_fixed_user_type {
        let _ = writeln!(out, "Ldap Fixed User Type: {v}");
    }
    if let Some(ref v) = data.ldap_group_dn {
        let _ = writeln!(out, "Ldap Group Dn: {v}");
    }
    if let Some(ref v) = data.ldap_token_expiration {
        let _ = writeln!(out, "Ldap Token Expiration: {v}");
    }
    if let Some(ref v) = data.ldap_url {
        let _ = writeln!(out, "Ldap Url: {v}");
    }
    if let Some(ref v) = data.ldap_user_attr {
        let _ = writeln!(out, "Ldap User Attr: {v}");
    }
    if let Some(ref v) = data.ldap_user_dn {
        let _ = writeln!(out, "Ldap User Dn: {v}");
    }
    if let Some(ref v) = data.metadata {
        let _ = writeln!(out, "Metadata: {v}");
    }
    if let Some(ref v) = data.mongodb_atlas_api_private_key {
        let _ = writeln!(out, "Mongodb Atlas Api Private Key: {v}");
    }
    if let Some(ref v) = data.mongodb_atlas_api_public_key {
        let _ = writeln!(out, "Mongodb Atlas Api Public Key: {v}");
    }
    if let Some(ref v) = data.mongodb_atlas_project_id {
        let _ = writeln!(out, "Mongodb Atlas Project Id: {v}");
    }
    if let Some(ref v) = data.mongodb_custom_data {
        let _ = writeln!(out, "Mongodb Custom Data: {v}");
    }
    if let Some(ref v) = data.mongodb_db_name {
        let _ = writeln!(out, "Mongodb Db Name: {v}");
    }
    if let Some(ref v) = data.mongodb_default_auth_db {
        let _ = writeln!(out, "Mongodb Default Auth Db: {v}");
    }
    if let Some(ref v) = data.mongodb_host_port {
        let _ = writeln!(out, "Mongodb Host Port: {v}");
    }
    if let Some(ref v) = data.mongodb_is_atlas {
        let _ = writeln!(out, "Mongodb Is Atlas: {v}");
    }
    if let Some(ref v) = data.mongodb_password {
        let _ = writeln!(out, "Mongodb Password: {v}");
    }
    if let Some(ref v) = data.mongodb_roles {
        let _ = writeln!(out, "Mongodb Roles: {v}");
    }
    if let Some(ref v) = data.mongodb_scopes {
        let _ = writeln!(out, "Mongodb Scopes: {v}");
    }
    if let Some(ref v) = data.mongodb_uri_connection {
        let _ = writeln!(out, "Mongodb Uri Connection: {v}");
    }
    if let Some(ref v) = data.mongodb_uri_options {
        let _ = writeln!(out, "Mongodb Uri Options: {v}");
    }
    if let Some(ref v) = data.mongodb_username {
        let _ = writeln!(out, "Mongodb Username: {v}");
    }
    if let Some(ref v) = data.mssql_allowed_db_names {
        let _ = writeln!(out, "Mssql Allowed Db Names: {v}");
    }
    if let Some(ref v) = data.mssql_creation_statements {
        let _ = writeln!(out, "Mssql Creation Statements: {v}");
    }
    if let Some(ref v) = data.mssql_revocation_statements {
        let _ = writeln!(out, "Mssql Revocation Statements: {v}");
    }
    if let Some(ref v) = data.mysql_creation_statements {
        let _ = writeln!(out, "Mysql Creation Statements: {v}");
    }
    if let Some(ref v) = data.mysql_revocation_statements {
        let _ = writeln!(out, "Mysql Revocation Statements: {v}");
    }
    if let Some(ref v) = data.openai_url {
        let _ = writeln!(out, "Openai Url: {v}");
    }
    if let Some(ref v) = data.oracle_creation_statements {
        let _ = writeln!(out, "Oracle Creation Statements: {v}");
    }
    if let Some(ref v) = data.oracle_revocation_statements {
        let _ = writeln!(out, "Oracle Revocation Statements: {v}");
    }
    if let Some(ref v) = data.oracle_wallet_details {
        let _ = writeln!(out, "Oracle Wallet Details: {v}");
    }
    if let Some(ref v) = data.organization_id {
        let _ = writeln!(out, "Organization Id: {v}");
    }
    if let Some(ref v) = data.password {
        let _ = writeln!(out, "Password: {v}");
    }
    if let Some(ref v) = data.password_length {
        let _ = writeln!(out, "Password Length: {v}");
    }
    if let Some(ref v) = data.password_policy {
        let _ = writeln!(out, "Password Policy: {v}");
    }
    if let Some(ref v) = data.payload {
        let _ = writeln!(out, "Payload: {v}");
    }
    if let Some(ref v) = data.ping_url {
        let _ = writeln!(out, "Ping Url: {v}");
    }
    if let Some(ref v) = data.postgres_creation_statements {
        let _ = writeln!(out, "Postgres Creation Statements: {v}");
    }
    if let Some(ref v) = data.postgres_revocation_statements {
        let _ = writeln!(out, "Postgres Revocation Statements: {v}");
    }
    if let Some(ref v) = data.privileged_user {
        let _ = writeln!(out, "Privileged User: {v}");
    }
    if let Some(ref v) = data.project_id {
        let _ = writeln!(out, "Project Id: {v}");
    }
    if let Some(ref v) = data.rabbitmq_server_password {
        let _ = writeln!(out, "Rabbitmq Server Password: {v}");
    }
    if let Some(ref v) = data.rabbitmq_server_uri {
        let _ = writeln!(out, "Rabbitmq Server Uri: {v}");
    }
    if let Some(ref v) = data.rabbitmq_server_user {
        let _ = writeln!(out, "Rabbitmq Server User: {v}");
    }
    if let Some(ref v) = data.rabbitmq_user_conf_permission {
        let _ = writeln!(out, "Rabbitmq User Conf Permission: {v}");
    }
    if let Some(ref v) = data.rabbitmq_user_read_permission {
        let _ = writeln!(out, "Rabbitmq User Read Permission: {v}");
    }
    if let Some(ref v) = data.rabbitmq_user_tags {
        let _ = writeln!(out, "Rabbitmq User Tags: {v}");
    }
    if let Some(ref v) = data.rabbitmq_user_vhost {
        let _ = writeln!(out, "Rabbitmq User Vhost: {v}");
    }
    if let Some(ref v) = data.rabbitmq_user_write_permission {
        let _ = writeln!(out, "Rabbitmq User Write Permission: {v}");
    }
    if let Some(ref v) = data.rdp_fixed_user_name_sub_claim_key {
        let _ = writeln!(out, "Rdp Fixed User Name Sub Claim Key: {v}");
    }
    if let Some(ref v) = data.redirect_uris {
        let _ = writeln!(out, "Redirect Uris: {v}");
    }
    if let Some(ref v) = data.redshift_creation_statements {
        let _ = writeln!(out, "Redshift Creation Statements: {v}");
    }
    if let Some(ref v) = data.restricted_scopes {
        let _ = writeln!(out, "Restricted Scopes: {v}");
    }
    if let Some(ref v) = data.revoke_sync_url {
        let _ = writeln!(out, "Revoke Sync Url: {v}");
    }
    if let Some(ref v) = data.rotate_sync_url {
        let _ = writeln!(out, "Rotate Sync Url: {v}");
    }
    if let Some(ref v) = data.scopes {
        let _ = writeln!(out, "Scopes: {v}");
    }
    if let Some(ref v) = data.secure_remote_access_details {
        let _ = writeln!(out, "Secure Remote Access Details: {v}");
    }
    if let Some(ref v) = data.session_extension_warn_interval_min {
        let _ = writeln!(out, "Session Extension Warn Interval Min: {v}");
    }
    if let Some(ref v) = data.sf_account {
        let _ = writeln!(out, "Sf Account: {v}");
    }
    if let Some(ref v) = data.sf_auth_mode {
        let _ = writeln!(out, "Sf Auth Mode: {v}");
    }
    if let Some(ref v) = data.sf_key_algo {
        let _ = writeln!(out, "Sf Key Algo: {v}");
    }
    if let Some(ref v) = data.sf_user_role {
        let _ = writeln!(out, "Sf User Role: {v}");
    }
    if let Some(ref v) = data.sf_warehouse_name {
        let _ = writeln!(out, "Sf Warehouse Name: {v}");
    }
    if let Some(ref v) = data.should_stop {
        let _ = writeln!(out, "Should Stop: {v}");
    }
    if let Some(ref v) = data.signing_algorithm {
        let _ = writeln!(out, "Signing Algorithm: {v}");
    }
    if let Some(ref v) = data.ssl_connection_certificate {
        let _ = writeln!(out, "Ssl Connection Certificate: {v}");
    }
    if let Some(ref v) = data.ssl_connection_mode {
        let _ = writeln!(out, "Ssl Connection Mode: {v}");
    }
    if let Some(ref v) = data.subject_dn {
        let _ = writeln!(out, "Subject Dn: {v}");
    }
    if let Some(ref v) = data.tags {
        let _ = writeln!(out, "Tags: {v}");
    }
    if let Some(ref v) = data.timeout_seconds {
        let _ = writeln!(out, "Timeout Seconds: {v}");
    }
    if let Some(ref v) = data.use_gw_cloud_identity {
        let _ = writeln!(out, "Use Gw Cloud Identity: {v}");
    }
    if let Some(ref v) = data.use_gw_service_account {
        let _ = writeln!(out, "Use Gw Service Account: {v}");
    }
    if let Some(ref v) = data.user_name {
        let _ = writeln!(out, "User Name: {v}");
    }
    if let Some(ref v) = data.user_password {
        let _ = writeln!(out, "User Password: {v}");
    }
    if let Some(ref v) = data.user_principal_name {
        let _ = writeln!(out, "User Principal Name: {v}");
    }
    if let Some(ref v) = data.user_ttl {
        let _ = writeln!(out, "User Ttl: {v}");
    }
    if let Some(ref v) = data.username_length {
        let _ = writeln!(out, "Username Length: {v}");
    }
    if let Some(ref v) = data.username_policy {
        let _ = writeln!(out, "Username Policy: {v}");
    }
    if let Some(ref v) = data.username_template {
        let _ = writeln!(out, "Username Template: {v}");
    }
    if let Some(ref v) = data.venafi_allow_subdomains {
        let _ = writeln!(out, "Venafi Allow Subdomains: {v}");
    }
    if let Some(ref v) = data.venafi_allowed_domains {
        let _ = writeln!(out, "Venafi Allowed Domains: {v}");
    }
    if let Some(ref v) = data.venafi_api_key {
        let _ = writeln!(out, "Venafi Api Key: {v}");
    }
    if let Some(ref v) = data.venafi_auto_generated_folder {
        let _ = writeln!(out, "Venafi Auto Generated Folder: {v}");
    }
    if let Some(ref v) = data.venafi_base_url {
        let _ = writeln!(out, "Venafi Base Url: {v}");
    }
    if let Some(ref v) = data.venafi_root_first_in_chain {
        let _ = writeln!(out, "Venafi Root First In Chain: {v}");
    }
    if let Some(ref v) = data.venafi_sign_using_akeyless_pki {
        let _ = writeln!(out, "Venafi Sign Using Akeyless Pki: {v}");
    }
    if let Some(ref v) = data.venafi_signer_key_name {
        let _ = writeln!(out, "Venafi Signer Key Name: {v}");
    }
    if let Some(ref v) = data.venafi_store_private_key {
        let _ = writeln!(out, "Venafi Store Private Key: {v}");
    }
    if let Some(ref v) = data.venafi_tpp_access_token {
        let _ = writeln!(out, "Venafi Tpp Access Token: {v}");
    }
    if let Some(ref v) = data.venafi_tpp_client_id {
        let _ = writeln!(out, "Venafi Tpp Client Id: {v}");
    }
    if let Some(ref v) = data.venafi_tpp_password {
        let _ = writeln!(out, "Venafi Tpp Password: {v}");
    }
    if let Some(ref v) = data.venafi_tpp_refresh_token {
        let _ = writeln!(out, "Venafi Tpp Refresh Token: {v}");
    }
    if let Some(ref v) = data.venafi_tpp_username {
        let _ = writeln!(out, "Venafi Tpp Username: {v}");
    }
    if let Some(ref v) = data.venafi_use_tpp {
        let _ = writeln!(out, "Venafi Use Tpp: {v}");
    }
    if let Some(ref v) = data.venafi_zone {
        let _ = writeln!(out, "Venafi Zone: {v}");
    }
    if let Some(ref v) = data.warn_before_user_expiration_min {
        let _ = writeln!(out, "Warn Before User Expiration Min: {v}");
    }
    out
}

pub fn format_dynamic_secret_get_value(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_dynamic_secret_list(data: &GetProducersListReplyObj) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producers {
        let _ = writeln!(out, "Producers: {v}");
    }
    if let Some(ref v) = data.producers_errors {
        let _ = writeln!(out, "Producers Errors: {v}");
    }
    out
}

pub fn format_dynamic_secret_tmp_creds_get(data: &Vec<TmpUserData>) -> String {
    format!("{:#?}", data)
}

pub fn format_dynamic_secret_tmp_creds_delete(data: &JsonError) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.error {
        let _ = writeln!(out, "Error: {v}");
    }
    out
}

pub fn format_dynamic_secret_tmp_creds_update(data: &JsonError) -> String {
    format_dynamic_secret_tmp_creds_delete(data)
}

pub fn format_dynamic_secret_update_artifactory(data: &DynamicSecretUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.dynamic_secret_details {
        let _ = writeln!(out, "Dynamic Secret Details: {v}");
    }
    out
}

pub fn format_dynamic_secret_update_aws(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_azure(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_cassandra(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_custom(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_dockerhub(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_eks(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_gcp(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_github(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_gitlab(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_gke(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_google_workspace(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_hana_db(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_k8s(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_ldap(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_mongo_db(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_ms_sql(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_my_sql(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_open_ai(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_oracle_db(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_ping(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_postgre_sql(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_rabbit_mq(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_rdp(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_redis(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_redshift(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_snowflake(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_dynamic_secret_update_venafi(data: &DynamicSecretUpdateOutput) -> String {
    format_dynamic_secret_update_artifactory(data)
}

pub fn format_encrypt(data: &EncryptOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_encrypt_batch(data: &EncryptOutput) -> String {
    format_encrypt(data)
}

pub fn format_encrypt_gpg(data: &EncryptGpgOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_encrypt_with_classic_key(data: &EncryptOutput) -> String {
    format_encrypt(data)
}

pub fn format_esm_create(data: &EsmCreateSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secret_id {
        let _ = writeln!(out, "Secret Id: {v}");
    }
    if let Some(ref v) = data.version_id {
        let _ = writeln!(out, "Version Id: {v}");
    }
    out
}

pub fn format_esm_delete(data: &EsmDeleteSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_esm_get(data: &EsmGetSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.binary_value {
        let _ = writeln!(out, "Binary Value: {v}");
    }
    if let Some(ref v) = data.encryption_key {
        let _ = writeln!(out, "Encryption Key: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.metadata {
        let _ = writeln!(out, "Metadata: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.value {
        let _ = writeln!(out, "Value: {v}");
    }
    out
}

pub fn format_esm_list(data: &EsmListSecretsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secrets_list {
        let _ = writeln!(out, "Secrets List: {v}");
    }
    out
}

pub fn format_esm_update(data: &EsmUpdateSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.secret_id {
        let _ = writeln!(out, "Secret Id: {v}");
    }
    if let Some(ref v) = data.version_id {
        let _ = writeln!(out, "Version Id: {v}");
    }
    out
}

pub fn format_event_action(data: &EventActionOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_event_forwarder_create_email(data: &EventForwarderCreateUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.event_forwarder_id {
        let _ = writeln!(out, "Event Forwarder Id: {v}");
    }
    if let Some(ref v) = data.event_forwarder_name {
        let _ = writeln!(out, "Event Forwarder Name: {v}");
    }
    out
}

pub fn format_event_forwarder_create_service_now(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_create_slack(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_create_teams(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_create_webhook(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_delete(data: &EventForwarderDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.event_forwarder_name {
        let _ = writeln!(out, "Event Forwarder Name: {v}");
    }
    out
}

pub fn format_event_forwarder_get(data: &EventForwarderGetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.event_forwarder {
        let _ = writeln!(out, "Event Forwarder: {v}");
    }
    if let Some(ref v) = data.event_forwarder_details {
        let _ = writeln!(out, "Event Forwarder Details: {v}");
    }
    out
}

pub fn format_event_forwarder_update_email(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_update_service_now(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_update_slack(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_update_teams(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_event_forwarder_update_webhook(data: &EventForwarderCreateUpdateOutput) -> String {
    format_event_forwarder_create_email(data)
}

pub fn format_export_classic_key(data: &ExportClassicKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificate_pem {
        let _ = writeln!(out, "Certificate Pem: {v}");
    }
    if let Some(ref v) = data.key {
        let _ = writeln!(out, "Key: {v}");
    }
    if let Some(ref v) = data.ssh {
        let _ = writeln!(out, "Ssh: {v}");
    }
    if let Some(ref v) = data.wrapping_iv {
        let _ = writeln!(out, "Wrapping Iv: {v}");
    }
    out
}

pub fn format_folder_create(data: &FolderCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.folder_id {
        let _ = writeln!(out, "Folder Id: {v}");
    }
    out
}

pub fn format_folder_delete(data: &FolderDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_folder_get(data: &FolderGetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.folder {
        let _ = writeln!(out, "Folder: {v}");
    }
    out
}

pub fn format_folder_update(data: &FolderUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_gateway_create_allowed_access(data: &AllowedAccess) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.access_type {
        let _ = writeln!(out, "Access Type: {v}");
    }
    if let Some(ref v) = data.cluster_id {
        let _ = writeln!(out, "Cluster Id: {v}");
    }
    if let Some(ref v) = data.created_at {
        let _ = writeln!(out, "Created At: {v}");
    }
    if let Some(ref v) = data.description {
        let _ = writeln!(out, "Description: {v}");
    }
    if let Some(ref v) = data.editable {
        let _ = writeln!(out, "Editable: {v}");
    }
    if let Some(ref v) = data.error {
        let _ = writeln!(out, "Error: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.is_valid {
        let _ = writeln!(out, "Is Valid: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.permissions {
        let _ = writeln!(out, "Permissions: {v}");
    }
    if let Some(ref v) = data.sub_claims {
        let _ = writeln!(out, "Sub Claims: {v}");
    }
    if let Some(ref v) = data.sub_claims_case_insensitive {
        let _ = writeln!(out, "Sub Claims Case Insensitive: {v}");
    }
    if let Some(ref v) = data.updated_at {
        let _ = writeln!(out, "Updated At: {v}");
    }
    out
}

pub fn format_gateway_create_k8s_auth_config(data: &GatewayCreateK8sAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cluster_id {
        let _ = writeln!(out, "Cluster Id: {v}");
    }
    if let Some(ref v) = data.parts_change {
        let _ = writeln!(out, "Parts Change: {v}");
    }
    if let Some(ref v) = data.total_hash {
        let _ = writeln!(out, "Total Hash: {v}");
    }
    out
}

pub fn format_gateway_create_migration(data: &GatewayMigrationCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.migration_name {
        let _ = writeln!(out, "Migration Name: {v}");
    }
    out
}

pub fn format_gateway_create_producer_redis(data: &GatewayCreateProducerRedisOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_artifactory(data: &GatewayCreateProducerArtifactoryOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_aws(data: &GatewayCreateProducerAwsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_azure(data: &GatewayCreateProducerAzureOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_cassandra(data: &GatewayCreateProducerCassandraOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_venafi(data: &GatewayCreateProducerVenafiOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_chef(data: &GatewayCreateProducerChefOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_custom(data: &GatewayCreateProducerCustomOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_dockerhub(data: &GatewayCreateProducerDockerhubOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_eks(data: &GatewayCreateProducerEksOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_gcp(data: &GatewayCreateProducerGcpOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_github(data: &GatewayCreateProducerGithubOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_gke(data: &GatewayCreateProducerGkeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_hana_db(data: &GatewayCreateProducerHanaDbOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_native_k8s(data: &GatewayCreateProducerNativeK8sOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_ldap(data: &GatewayCreateProducerLdapOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_mongo(data: &GatewayCreateProducerMongoOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_mssql(data: &GatewayCreateProducerMssqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_my_sql(data: &GatewayCreateProducerMySqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_oracle_db(data: &GatewayCreateProducerOracleDbOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_ping(data: &GatewayCreateProducerPingOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_postgre_sql(data: &GatewayCreateProducerPostgreSqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_rabbit_mq(data: &GatewayCreateProducerRabbitMqOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_rdp(data: &GatewayCreateProducerRdpOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_redshift(data: &GatewayCreateProducerRedshiftOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_create_producer_snowflake(data: &GatewayCreateProducerSnowflakeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_delete_allowed_access(data: &GatewayDeleteAllowedAccessOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.allowed_access_name {
        let _ = writeln!(out, "Allowed Access Name: {v}");
    }
    out
}

pub fn format_gateway_delete_k8s_auth_config(data: &GatewayDeleteK8sAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cluster_id {
        let _ = writeln!(out, "Cluster Id: {v}");
    }
    if let Some(ref v) = data.parts_change {
        let _ = writeln!(out, "Parts Change: {v}");
    }
    if let Some(ref v) = data.total_hash {
        let _ = writeln!(out, "Total Hash: {v}");
    }
    out
}

pub fn format_gateway_delete_migration(data: &GatewayMigrationDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.migration_id {
        let _ = writeln!(out, "Migration Id: {v}");
    }
    out
}

pub fn format_gateway_delete_producer(data: &GatewayDeleteProducerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_name {
        let _ = writeln!(out, "Producer Name: {v}");
    }
    out
}

pub fn format_gateway_download_customer_fragments(data: &GatewayDownloadCustomerFragmentsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cf_json {
        let _ = writeln!(out, "Cf Json: {v}");
    }
    out
}

pub fn format_gateway_get_allowed_access(data: &AllowedAccess) -> String {
    format_gateway_create_allowed_access(data)
}

pub fn format_gateway_get_cache(data: &CacheConfigPart) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cache_enable {
        let _ = writeln!(out, "Cache Enable: {v}");
    }
    if let Some(ref v) = data.cache_encryption_key {
        let _ = writeln!(out, "Cache Encryption Key: {v}");
    }
    if let Some(ref v) = data.cache_ttl {
        let _ = writeln!(out, "Cache Ttl: {v}");
    }
    if let Some(ref v) = data.new_proactive_cache_enable {
        let _ = writeln!(out, "New Proactive Cache Enable: {v}");
    }
    if let Some(ref v) = data.proactive_cache_dump_interval {
        let _ = writeln!(out, "Proactive Cache Dump Interval: {v}");
    }
    if let Some(ref v) = data.proactive_cache_enable {
        let _ = writeln!(out, "Proactive Cache Enable: {v}");
    }
    if let Some(ref v) = data.proactive_cache_minimum_fetching_time {
        let _ = writeln!(out, "Proactive Cache Minimum Fetching Time: {v}");
    }
    out
}

pub fn format_gateway_get_config(data: &AkeylessGatewayConfig) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.admins {
        let _ = writeln!(out, "Admins: {v}");
    }
    if let Some(ref v) = data.ai_insights {
        let _ = writeln!(out, "Ai Insights: {v}");
    }
    if let Some(ref v) = data.ca_certificates {
        let _ = writeln!(out, "Ca Certificates: {v}");
    }
    if let Some(ref v) = data.cache {
        let _ = writeln!(out, "Cache: {v}");
    }
    if let Some(ref v) = data.cf {
        let _ = writeln!(out, "Cf: {v}");
    }
    if let Some(ref v) = data.config_protection_key_name {
        let _ = writeln!(out, "Config Protection Key Name: {v}");
    }
    if let Some(ref v) = data.general {
        let _ = writeln!(out, "General: {v}");
    }
    if let Some(ref v) = data.k8s_auths {
        let _ = writeln!(out, "K8s Auths: {v}");
    }
    if let Some(ref v) = data.kerberos {
        let _ = writeln!(out, "Kerberos: {v}");
    }
    if let Some(ref v) = data.kmip_clients {
        let _ = writeln!(out, "Kmip Clients: {v}");
    }
    if let Some(ref v) = data.ldap {
        let _ = writeln!(out, "Ldap: {v}");
    }
    if let Some(ref v) = data.leadership {
        let _ = writeln!(out, "Leadership: {v}");
    }
    if let Some(ref v) = data.log_forwarding {
        let _ = writeln!(out, "Log Forwarding: {v}");
    }
    if let Some(ref v) = data.message_queue_info {
        let _ = writeln!(out, "Message Queue Info: {v}");
    }
    if let Some(ref v) = data.migrations {
        let _ = writeln!(out, "Migrations: {v}");
    }
    if let Some(ref v) = data.producers {
        let _ = writeln!(out, "Producers: {v}");
    }
    if let Some(ref v) = data.rotators {
        let _ = writeln!(out, "Rotators: {v}");
    }
    if let Some(ref v) = data.saml {
        let _ = writeln!(out, "Saml: {v}");
    }
    if let Some(ref v) = data.version {
        let _ = writeln!(out, "Version: {v}");
    }
    out
}

pub fn format_gateway_get_defaults(data: &GatewayGetDefaultsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificate_access_id {
        let _ = writeln!(out, "Certificate Access Id: {v}");
    }
    if let Some(ref v) = data.default_protection_key_id {
        let _ = writeln!(out, "Default Protection Key Id: {v}");
    }
    if let Some(ref v) = data.hvp_route_version {
        let _ = writeln!(out, "Hvp Route Version: {v}");
    }
    if let Some(ref v) = data.notify_on_status_change {
        let _ = writeln!(out, "Notify On Status Change: {v}");
    }
    if let Some(ref v) = data.oidc_access_id {
        let _ = writeln!(out, "Oidc Access Id: {v}");
    }
    if let Some(ref v) = data.saml_access_id {
        let _ = writeln!(out, "Saml Access Id: {v}");
    }
    out
}

pub fn format_gateway_get_k8s_auth_config(data: &GatewayGetK8sAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.am_token_expiration {
        let _ = writeln!(out, "Am Token Expiration: {v}");
    }
    if let Some(ref v) = data.auth_method_access_id {
        let _ = writeln!(out, "Auth Method Access Id: {v}");
    }
    if let Some(ref v) = data.auth_method_prv_key_pem {
        let _ = writeln!(out, "Auth Method Prv Key Pem: {v}");
    }
    if let Some(ref v) = data.cluster_api_type {
        let _ = writeln!(out, "Cluster Api Type: {v}");
    }
    if let Some(ref v) = data.disable_iss_validation {
        let _ = writeln!(out, "Disable Iss Validation: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.k8s_auth_type {
        let _ = writeln!(out, "K8s Auth Type: {v}");
    }
    if let Some(ref v) = data.k8s_ca_cert {
        let _ = writeln!(out, "K8s Ca Cert: {v}");
    }
    if let Some(ref v) = data.k8s_client_cert_data {
        let _ = writeln!(out, "K8s Client Cert Data: {v}");
    }
    if let Some(ref v) = data.k8s_client_key_data {
        let _ = writeln!(out, "K8s Client Key Data: {v}");
    }
    if let Some(ref v) = data.k8s_host {
        let _ = writeln!(out, "K8s Host: {v}");
    }
    if let Some(ref v) = data.k8s_issuer {
        let _ = writeln!(out, "K8s Issuer: {v}");
    }
    if let Some(ref v) = data.k8s_pub_keys_pem {
        let _ = writeln!(out, "K8s Pub Keys Pem: {v}");
    }
    if let Some(ref v) = data.k8s_token_reviewer_jwt {
        let _ = writeln!(out, "K8s Token Reviewer Jwt: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.protection_key {
        let _ = writeln!(out, "Protection Key: {v}");
    }
    if let Some(ref v) = data.rancher_api_key {
        let _ = writeln!(out, "Rancher Api Key: {v}");
    }
    if let Some(ref v) = data.rancher_cluster_id {
        let _ = writeln!(out, "Rancher Cluster Id: {v}");
    }
    if let Some(ref v) = data.use_local_ca_jwt {
        let _ = writeln!(out, "Use Local Ca Jwt: {v}");
    }
    out
}

pub fn format_gateway_get_ldap_auth_config(data: &GatewayGetLdapAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.ldap_access_id {
        let _ = writeln!(out, "Ldap Access Id: {v}");
    }
    if let Some(ref v) = data.ldap_anonymous_search {
        let _ = writeln!(out, "Ldap Anonymous Search: {v}");
    }
    if let Some(ref v) = data.ldap_bind_dn {
        let _ = writeln!(out, "Ldap Bind Dn: {v}");
    }
    if let Some(ref v) = data.ldap_bind_password {
        let _ = writeln!(out, "Ldap Bind Password: {v}");
    }
    if let Some(ref v) = data.ldap_cert {
        let _ = writeln!(out, "Ldap Cert: {v}");
    }
    if let Some(ref v) = data.ldap_enable {
        let _ = writeln!(out, "Ldap Enable: {v}");
    }
    if let Some(ref v) = data.ldap_group_attr {
        let _ = writeln!(out, "Ldap Group Attr: {v}");
    }
    if let Some(ref v) = data.ldap_group_dn {
        let _ = writeln!(out, "Ldap Group Dn: {v}");
    }
    if let Some(ref v) = data.ldap_group_filter {
        let _ = writeln!(out, "Ldap Group Filter: {v}");
    }
    if let Some(ref v) = data.ldap_private_key {
        let _ = writeln!(out, "Ldap Private Key: {v}");
    }
    if let Some(ref v) = data.ldap_token_expiration {
        let _ = writeln!(out, "Ldap Token Expiration: {v}");
    }
    if let Some(ref v) = data.ldap_url {
        let _ = writeln!(out, "Ldap Url: {v}");
    }
    if let Some(ref v) = data.ldap_user_attr {
        let _ = writeln!(out, "Ldap User Attr: {v}");
    }
    if let Some(ref v) = data.ldap_user_dn {
        let _ = writeln!(out, "Ldap User Dn: {v}");
    }
    out
}

pub fn format_gateway_get_log_forwarding(data: &LogForwardingConfigPart) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.aws_s3_config {
        let _ = writeln!(out, "Aws S3 Config: {v}");
    }
    if let Some(ref v) = data.azure_analytics_config {
        let _ = writeln!(out, "Azure Analytics Config: {v}");
    }
    if let Some(ref v) = data.datadog_config {
        let _ = writeln!(out, "Datadog Config: {v}");
    }
    if let Some(ref v) = data.elasticsearch_config {
        let _ = writeln!(out, "Elasticsearch Config: {v}");
    }
    if let Some(ref v) = data.google_chronicle_config {
        let _ = writeln!(out, "Google Chronicle Config: {v}");
    }
    if let Some(ref v) = data.json_output {
        let _ = writeln!(out, "Json Output: {v}");
    }
    if let Some(ref v) = data.logan_enable {
        let _ = writeln!(out, "Logan Enable: {v}");
    }
    if let Some(ref v) = data.logan_url {
        let _ = writeln!(out, "Logan Url: {v}");
    }
    if let Some(ref v) = data.logstash_config {
        let _ = writeln!(out, "Logstash Config: {v}");
    }
    if let Some(ref v) = data.logz_io_config {
        let _ = writeln!(out, "Logz Io Config: {v}");
    }
    if let Some(ref v) = data.pull_interval_sec {
        let _ = writeln!(out, "Pull Interval Sec: {v}");
    }
    if let Some(ref v) = data.splunk_config {
        let _ = writeln!(out, "Splunk Config: {v}");
    }
    if let Some(ref v) = data.sumo_logic_config {
        let _ = writeln!(out, "Sumo Logic Config: {v}");
    }
    if let Some(ref v) = data.syslog_config {
        let _ = writeln!(out, "Syslog Config: {v}");
    }
    if let Some(ref v) = data.target_log_type {
        let _ = writeln!(out, "Target Log Type: {v}");
    }
    out
}

pub fn format_gateway_get_migration(data: &GatewayMigrationGetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.body {
        let _ = writeln!(out, "Body: {v}");
    }
    out
}

pub fn format_gateway_get_producer(data: &DsProducerDetails) -> String {
    format_dynamic_secret_get(data)
}

pub fn format_gateway_get_tmp_users(data: &Vec<TmpUserData>) -> String {
    format_dynamic_secret_tmp_creds_get(data)
}

pub fn format_gateway_get_remote_access(data: &BastionConfigReplyObj) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cluster_id {
        let _ = writeln!(out, "Cluster Id: {v}");
    }
    if let Some(ref v) = data.desktop_app {
        let _ = writeln!(out, "Desktop App: {v}");
    }
    if let Some(ref v) = data.gator_cluster_id {
        let _ = writeln!(out, "Gator Cluster Id: {v}");
    }
    if let Some(ref v) = data.global {
        let _ = writeln!(out, "Global: {v}");
    }
    if let Some(ref v) = data.ssh_bastion {
        let _ = writeln!(out, "Ssh Bastion: {v}");
    }
    if let Some(ref v) = data.web_bastion {
        let _ = writeln!(out, "Web Bastion: {v}");
    }
    out
}

pub fn format_gateway_list_customer_fragments(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_gateway_list_migration(data: &GatewayMigrationListOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.body {
        let _ = writeln!(out, "Body: {v}");
    }
    out
}

pub fn format_gateway_list_producers(data: &GetProducersListReplyObj) -> String {
    format_dynamic_secret_list(data)
}

pub fn format_gateway_list_rotated_secrets(data: &ListItemsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.has_next {
        let _ = writeln!(out, "Has Next: {v}");
    }
    if let Some(ref v) = data.items {
        let _ = writeln!(out, "Items: {v}");
    }
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    out
}

pub fn format_gateway_migrate_personal_items(data: &GatewayMigratePersonalItemsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.migration_items {
        let _ = writeln!(out, "Migration Items: {v}");
    }
    out
}

pub fn format_gateway_status_migration(data: &MigrationStatusReplyObj) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificates {
        let _ = writeln!(out, "Certificates: {v}");
    }
    if let Some(ref v) = data.computers {
        let _ = writeln!(out, "Computers: {v}");
    }
    if let Some(ref v) = data.duration_time {
        let _ = writeln!(out, "Duration Time: {v}");
    }
    if let Some(ref v) = data.error {
        let _ = writeln!(out, "Error: {v}");
    }
    if let Some(ref v) = data.last_status_message {
        let _ = writeln!(out, "Last Status Message: {v}");
    }
    if let Some(ref v) = data.max_name_length {
        let _ = writeln!(out, "Max Name Length: {v}");
    }
    if let Some(ref v) = data.max_value_length {
        let _ = writeln!(out, "Max Value Length: {v}");
    }
    if let Some(ref v) = data.migration_id {
        let _ = writeln!(out, "Migration Id: {v}");
    }
    if let Some(ref v) = data.migration_items {
        let _ = writeln!(out, "Migration Items: {v}");
    }
    if let Some(ref v) = data.migration_name {
        let _ = writeln!(out, "Migration Name: {v}");
    }
    if let Some(ref v) = data.migration_state {
        let _ = writeln!(out, "Migration State: {v}");
    }
    if let Some(ref v) = data.migration_type {
        let _ = writeln!(out, "Migration Type: {v}");
    }
    if let Some(ref v) = data.migration_type_name {
        let _ = writeln!(out, "Migration Type Name: {v}");
    }
    if let Some(ref v) = data.rotated_secrets {
        let _ = writeln!(out, "Rotated Secrets: {v}");
    }
    if let Some(ref v) = data.start_time {
        let _ = writeln!(out, "Start Time: {v}");
    }
    if let Some(ref v) = data.targets {
        let _ = writeln!(out, "Targets: {v}");
    }
    out
}

pub fn format_gateway_revoke_tmp_users(data: &JsonError) -> String {
    format_dynamic_secret_tmp_creds_delete(data)
}

pub fn format_rotate_secret(data: &RotatedSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_gateway_start_producer(data: &GatewayStartProducerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_name {
        let _ = writeln!(out, "Producer Name: {v}");
    }
    out
}

pub fn format_gateway_stop_producer(data: &GatewayStopProducerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_name {
        let _ = writeln!(out, "Producer Name: {v}");
    }
    out
}

pub fn format_gateway_sync_migration(data: &GatewayMigrationSyncOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.migration_name {
        let _ = writeln!(out, "Migration Name: {v}");
    }
    out
}

pub fn format_gateway_update_allowed_access(data: &AllowedAccess) -> String {
    format_gateway_create_allowed_access(data)
}

pub fn format_gateway_update_cache(data: &GatewayUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_gateway_update_defaults(data: &GatewayUpdateOutput) -> String {
    format_gateway_update_cache(data)
}

pub fn format_gateway_update_item(data: &GatewayUpdateItemOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_gateway_update_k8s_auth_config(data: &GatewayUpdateK8sAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cluster_id {
        let _ = writeln!(out, "Cluster Id: {v}");
    }
    if let Some(ref v) = data.parts_change {
        let _ = writeln!(out, "Parts Change: {v}");
    }
    if let Some(ref v) = data.total_hash {
        let _ = writeln!(out, "Total Hash: {v}");
    }
    out
}

pub fn format_gateway_update_ldap_auth_config(data: &GatewayUpdateLdapAuthConfigOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_gateway_update_log_forwarding_aws_s3(data: &GatewayUpdateLogForwardingOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_gateway_update_log_forwarding_azure_analytics(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_datadog(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_elasticsearch(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_google_chronicle(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_logstash(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_logz_io(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_splunk(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_stdout(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_sumologic(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_log_forwarding_syslog(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_migration(data: &GatewayMigrationUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.migration_name {
        let _ = writeln!(out, "Migration Name: {v}");
    }
    out
}

pub fn format_gateway_update_producer_artifactory(data: &GatewayUpdateProducerArtifactoryOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_aws(data: &GatewayUpdateProducerAwsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_azure(data: &GatewayUpdateProducerAzureOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_cassandra(data: &GatewayUpdateProducerCassandraOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_venafi(data: &GatewayUpdateProducerVenafiOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_chef(data: &GatewayUpdateProducerChefOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_custom(data: &GatewayUpdateProducerCustomOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_dockerhub(data: &GatewayUpdateProducerDockerhubOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_eks(data: &GatewayUpdateProducerEksOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_gcp(data: &GatewayUpdateProducerGcpOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_github(data: &GatewayUpdateProducerGithubOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_gke(data: &GatewayUpdateProducerGkeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_hana_db(data: &GatewayUpdateProducerHanaDbOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_native_k8s(data: &GatewayUpdateProducerNativeK8sOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_ldap(data: &GatewayUpdateProducerLdapOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_mongo(data: &GatewayUpdateProducerMongoOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_mssql(data: &GatewayUpdateProducerMssqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_my_sql(data: &GatewayUpdateProducerMySqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_oracle_db(data: &GatewayUpdateProducerOracleDbOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_ping(data: &GatewayUpdateProducerPingOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_postgre_sql(data: &GatewayUpdateProducerPostgreSqlOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_rabbit_mq(data: &GatewayUpdateProducerRabbitMqOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_rdp(data: &GatewayUpdateProducerRdpOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_redis(data: &GatewayUpdateProducerRedisOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_redshift(data: &GatewayUpdateProducerRedshiftOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_producer_snowflake(data: &GatewayUpdateProducerSnowflakeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.producer_details {
        let _ = writeln!(out, "Producer Details: {v}");
    }
    out
}

pub fn format_gateway_update_tmp_users(data: &JsonError) -> String {
    format_dynamic_secret_tmp_creds_delete(data)
}

pub fn format_gateway_update_remote_access(data: &GatewayUpdateRemoteAccessOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_gateway_update_remote_access_desktop_app(data: &GatewayUpdateRemoteAccessDesktopAppOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_gateway_update_remote_access_rdp_recordings(data: &GatewayUpdateRemoteAccessRdpRecordingsOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_gw_update_remote_access_session_logs_aws_s3(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_azure_analytics(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_datadog(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_elasticsearch(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_google_chronicle(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_logstash(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_logz_io(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_splunk(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_stdout(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_sumologic(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gw_update_remote_access_session_logs_syslog(data: &GatewayUpdateLogForwardingOutput) -> String {
    format_gateway_update_log_forwarding_aws_s3(data)
}

pub fn format_gateway_update_tls_cert(data: &GatewayUpdateTlsCertOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_generate_acme_eab(data: &GenerateAcmeEabOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.expires_at {
        let _ = writeln!(out, "Expires At: {v}");
    }
    if let Some(ref v) = data.kid {
        let _ = writeln!(out, "Kid: {v}");
    }
    if let Some(ref v) = data.mac_key {
        let _ = writeln!(out, "Mac Key: {v}");
    }
    out
}

pub fn format_generate_ca(data: &GenerateCaOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.intermediate_certificate_name {
        let _ = writeln!(out, "Intermediate Certificate Name: {v}");
    }
    if let Some(ref v) = data.intermediate_issuer_name {
        let _ = writeln!(out, "Intermediate Issuer Name: {v}");
    }
    if let Some(ref v) = data.intermediate_key_name {
        let _ = writeln!(out, "Intermediate Key Name: {v}");
    }
    if let Some(ref v) = data.root_certificate_name {
        let _ = writeln!(out, "Root Certificate Name: {v}");
    }
    if let Some(ref v) = data.root_issuer_name {
        let _ = writeln!(out, "Root Issuer Name: {v}");
    }
    if let Some(ref v) = data.root_key_name {
        let _ = writeln!(out, "Root Key Name: {v}");
    }
    out
}

pub fn format_generate_csr(data: &GenerateCsrOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.data {
        let _ = writeln!(out, "Data: {v}");
    }
    out
}

pub fn format_get_account_logo(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_get_account_settings(data: &GetAccountSettingsCommandOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.account_id {
        let _ = writeln!(out, "Account Id: {v}");
    }
    if let Some(ref v) = data.address {
        let _ = writeln!(out, "Address: {v}");
    }
    if let Some(ref v) = data.company_name {
        let _ = writeln!(out, "Company Name: {v}");
    }
    if let Some(ref v) = data.email {
        let _ = writeln!(out, "Email: {v}");
    }
    if let Some(ref v) = data.general_settings {
        let _ = writeln!(out, "General Settings: {v}");
    }
    if let Some(ref v) = data.object_version_settings {
        let _ = writeln!(out, "Object Version Settings: {v}");
    }
    if let Some(ref v) = data.phone {
        let _ = writeln!(out, "Phone: {v}");
    }
    if let Some(ref v) = data.secret_management {
        let _ = writeln!(out, "Secret Management: {v}");
    }
    if let Some(ref v) = data.secure_remote_access {
        let _ = writeln!(out, "Secure Remote Access: {v}");
    }
    if let Some(ref v) = data.system_access_creds_settings {
        let _ = writeln!(out, "System Access Creds Settings: {v}");
    }
    out
}

pub fn format_get_analytics_data(data: &AllAnalyticsData) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.analytics_data {
        let _ = writeln!(out, "Analytics Data: {v}");
    }
    if let Some(ref v) = data.certificates_expiry_data {
        let _ = writeln!(out, "Certificates Expiry Data: {v}");
    }
    if let Some(ref v) = data.clients_usage_reports {
        let _ = writeln!(out, "Clients Usage Reports: {v}");
    }
    if let Some(ref v) = data.date_updated {
        let _ = writeln!(out, "Date Updated: {v}");
    }
    if let Some(ref v) = data.usage_reports {
        let _ = writeln!(out, "Usage Reports: {v}");
    }
    out
}

pub fn format_get_auth_method(data: &AuthMethod) -> String {
    format_auth_method_get(data)
}

pub fn format_get_cert_challenge(data: &GetCertChallengeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.challenge {
        let _ = writeln!(out, "Challenge: {v}");
    }
    out
}

pub fn format_get_certificate_value(data: &GetCertificateValueOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificate_pem {
        let _ = writeln!(out, "Certificate Pem: {v}");
    }
    if let Some(ref v) = data.private_key_pem {
        let _ = writeln!(out, "Private Key Pem: {v}");
    }
    out
}

pub fn format_get_dynamic_secret_value(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_get_event_forwarder(data: &GetEventForwarderOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.event_forwarder {
        let _ = writeln!(out, "Event Forwarder: {v}");
    }
    out
}

pub fn format_get_group(data: &GetGroupOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.account_id {
        let _ = writeln!(out, "Account Id: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.description {
        let _ = writeln!(out, "Description: {v}");
    }
    if let Some(ref v) = data.group_alias {
        let _ = writeln!(out, "Group Alias: {v}");
    }
    if let Some(ref v) = data.group_id {
        let _ = writeln!(out, "Group Id: {v}");
    }
    if let Some(ref v) = data.group_name {
        let _ = writeln!(out, "Group Name: {v}");
    }
    if let Some(ref v) = data.is_subclaims_with_operator {
        let _ = writeln!(out, "Is Subclaims With Operator: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    if let Some(ref v) = data.user_assignments {
        let _ = writeln!(out, "User Assignments: {v}");
    }
    out
}

pub fn format_get_kube_exec_creds(data: &GetKubeExecCredsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.api_version {
        let _ = writeln!(out, "Api Version: {v}");
    }
    if let Some(ref v) = data.kind {
        let _ = writeln!(out, "Kind: {v}");
    }
    if let Some(ref v) = data.status {
        let _ = writeln!(out, "Status: {v}");
    }
    out
}

pub fn format_get_pki_certificate(data: &GetPkiCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cert_display_id {
        let _ = writeln!(out, "Cert Display Id: {v}");
    }
    if let Some(ref v) = data.cert_item_id {
        let _ = writeln!(out, "Cert Item Id: {v}");
    }
    if let Some(ref v) = data.data {
        let _ = writeln!(out, "Data: {v}");
    }
    if let Some(ref v) = data.http_challenge_info {
        let _ = writeln!(out, "Http Challenge Info: {v}");
    }
    if let Some(ref v) = data.parent_cert {
        let _ = writeln!(out, "Parent Cert: {v}");
    }
    if let Some(ref v) = data.path {
        let _ = writeln!(out, "Path: {v}");
    }
    if let Some(ref v) = data.reading_token {
        let _ = writeln!(out, "Reading Token: {v}");
    }
    out
}

pub fn format_get_role(data: &Role) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_date {
        let _ = writeln!(out, "Access Date: {v}");
    }
    if let Some(ref v) = data.access_date_display {
        let _ = writeln!(out, "Access Date Display: {v}");
    }
    if let Some(ref v) = data.client_permissions {
        let _ = writeln!(out, "Client Permissions: {v}");
    }
    if let Some(ref v) = data.comment {
        let _ = writeln!(out, "Comment: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.delete_protection {
        let _ = writeln!(out, "Delete Protection: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    if let Some(ref v) = data.role_auth_methods_assoc {
        let _ = writeln!(out, "Role Auth Methods Assoc: {v}");
    }
    if let Some(ref v) = data.role_id {
        let _ = writeln!(out, "Role Id: {v}");
    }
    if let Some(ref v) = data.role_name {
        let _ = writeln!(out, "Role Name: {v}");
    }
    if let Some(ref v) = data.rules {
        let _ = writeln!(out, "Rules: {v}");
    }
    out
}

pub fn format_get_rotated_secret_value(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_get_rsa_public(data: &GetRsaPublicOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.pem {
        let _ = writeln!(out, "Pem: {v}");
    }
    if let Some(ref v) = data.raw {
        let _ = writeln!(out, "Raw: {v}");
    }
    if let Some(ref v) = data.ssh {
        let _ = writeln!(out, "Ssh: {v}");
    }
    out
}

pub fn format_get_secret_value(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_get_ssh_certificate(data: &GetSshCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.data {
        let _ = writeln!(out, "Data: {v}");
    }
    if let Some(ref v) = data.path {
        let _ = writeln!(out, "Path: {v}");
    }
    out
}

pub fn format_get_tags(data: &Vec<String>) -> String {
    format!("{:#?}", data)
}

pub fn format_get_target(data: &Target) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_date {
        let _ = writeln!(out, "Access Date: {v}");
    }
    if let Some(ref v) = data.access_date_display {
        let _ = writeln!(out, "Access Date Display: {v}");
    }
    if let Some(ref v) = data.access_request_status {
        let _ = writeln!(out, "Access Request Status: {v}");
    }
    if let Some(ref v) = data.attributes {
        let _ = writeln!(out, "Attributes: {v}");
    }
    if let Some(ref v) = data.client_permissions {
        let _ = writeln!(out, "Client Permissions: {v}");
    }
    if let Some(ref v) = data.comment {
        let _ = writeln!(out, "Comment: {v}");
    }
    if let Some(ref v) = data.creation_date {
        let _ = writeln!(out, "Creation Date: {v}");
    }
    if let Some(ref v) = data.is_access_request_enabled {
        let _ = writeln!(out, "Is Access Request Enabled: {v}");
    }
    if let Some(ref v) = data.last_version {
        let _ = writeln!(out, "Last Version: {v}");
    }
    if let Some(ref v) = data.modification_date {
        let _ = writeln!(out, "Modification Date: {v}");
    }
    if let Some(ref v) = data.parent_target_name {
        let _ = writeln!(out, "Parent Target Name: {v}");
    }
    if let Some(ref v) = data.protection_key_name {
        let _ = writeln!(out, "Protection Key Name: {v}");
    }
    if let Some(ref v) = data.target_details {
        let _ = writeln!(out, "Target Details: {v}");
    }
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    if let Some(ref v) = data.target_items_assoc {
        let _ = writeln!(out, "Target Items Assoc: {v}");
    }
    if let Some(ref v) = data.target_name {
        let _ = writeln!(out, "Target Name: {v}");
    }
    if let Some(ref v) = data.target_sub_type {
        let _ = writeln!(out, "Target Sub Type: {v}");
    }
    if let Some(ref v) = data.target_type {
        let _ = writeln!(out, "Target Type: {v}");
    }
    if let Some(ref v) = data.target_versions {
        let _ = writeln!(out, "Target Versions: {v}");
    }
    if let Some(ref v) = data.with_customer_fragment {
        let _ = writeln!(out, "With Customer Fragment: {v}");
    }
    out
}

pub fn format_get_target_details(data: &GetTargetDetailsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target {
        let _ = writeln!(out, "Target: {v}");
    }
    if let Some(ref v) = data.value {
        let _ = writeln!(out, "Value: {v}");
    }
    out
}

pub fn format_hmac(data: &HmacOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_import_passwords(data: &ImportPasswordsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.imported {
        let _ = writeln!(out, "Imported: {v}");
    }
    if let Some(ref v) = data.passwords_in_file {
        let _ = writeln!(out, "Passwords In File: {v}");
    }
    if let Some(ref v) = data.successfully_parsed {
        let _ = writeln!(out, "Successfully Parsed: {v}");
    }
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_kmip_client_delete_rule(data: &KmipClientUpdateResponse) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.client {
        let _ = writeln!(out, "Client: {v}");
    }
    out
}

pub fn format_kmip_client_set_rule(data: &KmipClientUpdateResponse) -> String {
    format_kmip_client_delete_rule(data)
}

pub fn format_kmip_create_client(data: &KmipCreateClientOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificate {
        let _ = writeln!(out, "Certificate: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.key {
        let _ = writeln!(out, "Key: {v}");
    }
    out
}

pub fn format_kmip_server_setup(data: &KmipEnvironmentCreateResponse) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.ca_cert {
        let _ = writeln!(out, "Ca Cert: {v}");
    }
    if let Some(ref v) = data.root {
        let _ = writeln!(out, "Root: {v}");
    }
    out
}

pub fn format_kmip_delete_client(data: &KmipDeleteClientOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_kmip_describe_client(data: &KmipClientGetResponse) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.client {
        let _ = writeln!(out, "Client: {v}");
    }
    out
}

pub fn format_kmip_describe_server(data: &KmipDescribeServerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.active {
        let _ = writeln!(out, "Active: {v}");
    }
    if let Some(ref v) = data.ca_cert {
        let _ = writeln!(out, "Ca Cert: {v}");
    }
    if let Some(ref v) = data.certificate_issue_date {
        let _ = writeln!(out, "Certificate Issue Date: {v}");
    }
    if let Some(ref v) = data.certificate_ttl_in_seconds {
        let _ = writeln!(out, "Certificate Ttl In Seconds: {v}");
    }
    if let Some(ref v) = data.hostname {
        let _ = writeln!(out, "Hostname: {v}");
    }
    if let Some(ref v) = data.root {
        let _ = writeln!(out, "Root: {v}");
    }
    out
}

pub fn format_kmip_list_clients(data: &KmipClientListResponse) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.clients {
        let _ = writeln!(out, "Clients: {v}");
    }
    out
}

pub fn format_kmip_move_server(data: &KmipMoveServerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.new_root {
        let _ = writeln!(out, "New Root: {v}");
    }
    if let Some(ref v) = data.old_root {
        let _ = writeln!(out, "Old Root: {v}");
    }
    out
}

pub fn format_kmip_renew_client_certificate(data: &KmipRenewClientCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.certificate {
        let _ = writeln!(out, "Certificate: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.key {
        let _ = writeln!(out, "Key: {v}");
    }
    out
}

pub fn format_kmip_renew_server_certificate(data: &KmipRenewServerCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.ca_cert {
        let _ = writeln!(out, "Ca Cert: {v}");
    }
    out
}

pub fn format_kmip_set_server_state(data: &KmipSetServerStateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.is_active {
        let _ = writeln!(out, "Is Active: {v}");
    }
    out
}

pub fn format_kubeconfig_generate(data: &KubeconfigGenerateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.conflicted_clusters_names {
        let _ = writeln!(out, "Conflicted Clusters Names: {v}");
    }
    if let Some(ref v) = data.data {
        let _ = writeln!(out, "Data: {v}");
    }
    if let Some(ref v) = data.errors {
        let _ = writeln!(out, "Errors: {v}");
    }
    out
}

pub fn format_list_acme_accounts(data: &ListAcmeAccountsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.accounts {
        let _ = writeln!(out, "Accounts: {v}");
    }
    out
}

pub fn format_list_auth_methods(data: &ListAuthMethodsOutput) -> String {
    format_auth_method_list(data)
}

pub fn format_list_gateways(data: &GatewaysListResponse) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.clusters {
        let _ = writeln!(out, "Clusters: {v}");
    }
    out
}

pub fn format_list_groups(data: &ListGroupsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.groups {
        let _ = writeln!(out, "Groups: {v}");
    }
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    out
}

pub fn format_list_items(data: &ListItemsInPathOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.folders {
        let _ = writeln!(out, "Folders: {v}");
    }
    if let Some(ref v) = data.has_next {
        let _ = writeln!(out, "Has Next: {v}");
    }
    if let Some(ref v) = data.items {
        let _ = writeln!(out, "Items: {v}");
    }
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    out
}

pub fn format_list_roles(data: &ListRolesOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    if let Some(ref v) = data.roles {
        let _ = writeln!(out, "Roles: {v}");
    }
    out
}

pub fn format_list_shared_items(data: &JsonError) -> String {
    format_dynamic_secret_tmp_creds_delete(data)
}

pub fn format_list_sra_bastions(data: &BastionsList) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.clusters {
        let _ = writeln!(out, "Clusters: {v}");
    }
    out
}

pub fn format_list_sra_sessions(data: &ListSraSessionsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.allowed_gateways {
        let _ = writeln!(out, "Allowed Gateways: {v}");
    }
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    if let Some(ref v) = data.sessions {
        let _ = writeln!(out, "Sessions: {v}");
    }
    out
}

pub fn format_list_targets(data: &ListTargetsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.next_page {
        let _ = writeln!(out, "Next Page: {v}");
    }
    if let Some(ref v) = data.targets {
        let _ = writeln!(out, "Targets: {v}");
    }
    out
}

pub fn format_move_objects(data: &MoveObjectsOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_policy_create_keys(data: &PoliciesCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    out
}

pub fn format_policies_delete(data: &PoliciesDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_policies_get(data: &PoliciesGetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.policy {
        let _ = writeln!(out, "Policy: {v}");
    }
    out
}

pub fn format_policies_list(data: &PoliciesListOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.policies {
        let _ = writeln!(out, "Policies: {v}");
    }
    out
}

pub fn format_policy_update_keys(data: &PoliciesUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_provision_certificate(data: &ProvisionCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.fail_message {
        let _ = writeln!(out, "Fail Message: {v}");
    }
    if let Some(ref v) = data.success_message {
        let _ = writeln!(out, "Success Message: {v}");
    }
    if let Some(ref v) = data.host_names {
        let _ = writeln!(out, "Host Names: {v}");
    }
    if let Some(ref v) = data.provisioned_at {
        let _ = writeln!(out, "Provisioned At: {v}");
    }
    out
}

pub fn format_raw_creds(data: &SystemAccessCredentialsReplyObj) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    if let Some(ref v) = data.auth_creds {
        let _ = writeln!(out, "Auth Creds: {v}");
    }
    if let Some(ref v) = data.expiry {
        let _ = writeln!(out, "Expiry: {v}");
    }
    if let Some(ref v) = data.kfm_creds {
        let _ = writeln!(out, "Kfm Creds: {v}");
    }
    if let Some(ref v) = data.need_mfa_app_first_config {
        let _ = writeln!(out, "Need Mfa App First Config: {v}");
    }
    if let Some(ref v) = data.required_mfa {
        let _ = writeln!(out, "Required Mfa: {v}");
    }
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    if let Some(ref v) = data.uam_creds {
        let _ = writeln!(out, "Uam Creds: {v}");
    }
    out
}

pub fn format_refresh_key(data: &RefreshKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_renew_certificate(data: &RenewCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cert {
        let _ = writeln!(out, "Cert: {v}");
    }
    if let Some(ref v) = data.cert_display_id {
        let _ = writeln!(out, "Cert Display Id: {v}");
    }
    if let Some(ref v) = data.item_id {
        let _ = writeln!(out, "Item Id: {v}");
    }
    if let Some(ref v) = data.parent_cert {
        let _ = writeln!(out, "Parent Cert: {v}");
    }
    if let Some(ref v) = data.private_key {
        let _ = writeln!(out, "Private Key: {v}");
    }
    if let Some(ref v) = data.reading_token {
        let _ = writeln!(out, "Reading Token: {v}");
    }
    out
}

pub fn format_request_access(data: &RequestAccessOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.response {
        let _ = writeln!(out, "Response: {v}");
    }
    out
}

pub fn format_reset_access_key(data: &ResetAuthMethodAccessKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_key {
        let _ = writeln!(out, "Access Key: {v}");
    }
    out
}

pub fn format_reverse_rbac(data: &ReverseRbacOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.clients {
        let _ = writeln!(out, "Clients: {v}");
    }
    out
}

pub fn format_revoke_certificate(data: &RevokeCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_revoke_creds(data: &RevokeCredsOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_rollback_secret(data: &RollbackSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.version {
        let _ = writeln!(out, "Version: {v}");
    }
    out
}

pub fn format_rotate_key(data: &RotateKeyOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.classic_key_gw_url {
        let _ = writeln!(out, "Classic Key Gw Url: {v}");
    }
    if let Some(ref v) = data.item_type {
        let _ = writeln!(out, "Item Type: {v}");
    }
    if let Some(ref v) = data.new_item_version {
        let _ = writeln!(out, "New Item Version: {v}");
    }
    if let Some(ref v) = data.next_rotation_date {
        let _ = writeln!(out, "Next Rotation Date: {v}");
    }
    out
}

pub fn format_rotate_oidc_client_secret(data: &RotateOidcClientOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.new_secret {
        let _ = writeln!(out, "New Secret: {v}");
    }
    out
}

pub fn format_rotated_secret_create_aws(data: &RotatedSecretCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_rotated_secret_create_azure(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_cassandra(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_custom(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_dockerhub(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_gcp(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_hanadb(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_ldap(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_mongodb(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_mssql(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_mysql(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_open_ai(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_oracledb(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_postgresql(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_redis(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_redshift(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_snowflake(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_splunk(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_ssh(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_create_windows(data: &RotatedSecretCreateOutput) -> String {
    format_rotated_secret_create_aws(data)
}

pub fn format_rotated_secret_delete(data: &DeleteItemOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.deletion_date {
        let _ = writeln!(out, "Deletion Date: {v}");
    }
    if let Some(ref v) = data.item_id {
        let _ = writeln!(out, "Item Id: {v}");
    }
    if let Some(ref v) = data.item_name {
        let _ = writeln!(out, "Item Name: {v}");
    }
    if let Some(ref v) = data.version_deleted {
        let _ = writeln!(out, "Version Deleted: {v}");
    }
    out
}

pub fn format_rotated_secret_delete_sync(data: &RotatedSecretDeleteSyncOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secret_name {
        let _ = writeln!(out, "Secret Name: {v}");
    }
    if let Some(ref v) = data.usc_name {
        let _ = writeln!(out, "Usc Name: {v}");
    }
    out
}

pub fn format_rotated_secret_get_value(data: &serde_json::Value) -> String {
    format_change_admin_account_password(data)
}

pub fn format_rotated_secret_list(data: &GetProducersListReplyObj) -> String {
    format_dynamic_secret_list(data)
}

pub fn format_rotated_secret_sync(data: &RotatedSecretSyncOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_rotated_secret_update_aws(data: &RotatedSecretUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_rotated_secret_update_azure(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_cassandra(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_custom(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_dockerhub(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_gcp(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_hanadb(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_ldap(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_mongodb(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_mssql(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_mysql(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_open_ai(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_oracledb(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_postgresql(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_redis(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_redshift(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_snowflake(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_splunk(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_ssh(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_rotated_secret_update_windows(data: &RotatedSecretUpdateOutput) -> String {
    format_rotated_secret_update_aws(data)
}

pub fn format_set_item_state(data: &SetItemStateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_set_role_rule(data: &SetRoleRuleOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_share_item(data: &ShareItemOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.email_error {
        let _ = writeln!(out, "Email Error: {v}");
    }
    if let Some(ref v) = data.items_error {
        let _ = writeln!(out, "Items Error: {v}");
    }
    if let Some(ref v) = data.s_token {
        let _ = writeln!(out, "S Token: {v}");
    }
    if let Some(ref v) = data.shared_users {
        let _ = writeln!(out, "Shared Users: {v}");
    }
    if let Some(ref v) = data.shared_users_full_info {
        let _ = writeln!(out, "Shared Users Full Info: {v}");
    }
    if let Some(ref v) = data.sharing_url {
        let _ = writeln!(out, "Sharing Url: {v}");
    }
    out
}

pub fn format_sign_data_with_classic_key(data: &SignOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_ec_dsa(data: &SignEcDsaOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_gpg(data: &SignGpgOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_jwt_with_classic_key(data: &SignJwtOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_pkcs1(data: &SignPkcs1Output) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_pki_cert_with_classic_key(data: &SignPkiCertOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_sign_rsa_ssa_pss(data: &SignRsaSsaPssOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_static_creds_auth(data: &StaticCredsAuthOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_static_secret_delete_sync(data: &StaticSecretDeleteSyncOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secret_name {
        let _ = writeln!(out, "Secret Name: {v}");
    }
    if let Some(ref v) = data.usc_name {
        let _ = writeln!(out, "Usc Name: {v}");
    }
    out
}

pub fn format_static_secret_sync(data: &SecretSyncOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.remote_secret_name {
        let _ = writeln!(out, "Remote Secret Name: {v}");
    }
    if let Some(ref v) = data.secret_name {
        let _ = writeln!(out, "Secret Name: {v}");
    }
    if let Some(ref v) = data.usc_name {
        let _ = writeln!(out, "Usc Name: {v}");
    }
    out
}

pub fn format_target_create_artifactory(data: &TargetCreateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_target_create_aws(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_azure(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_db(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_dockerhub(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_eks(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_gcp(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_gemini(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_github(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_gitlab(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_gke(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_global_sign(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_global_sign_atlas(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_godaddy(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_hashi_vault(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_k8s(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_ldap(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_lets_encrypt(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_linked(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_open_ai(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_ping(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_rabbit_mq(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_salesforce(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_sectigo(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_splunk(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_ssh(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_web(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_windows(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_create_zero_ssl(data: &TargetCreateOutput) -> String {
    format_target_create_artifactory(data)
}

pub fn format_target_delete(data: &TargetDeleteOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_target_get(data: &Target) -> String {
    format_get_target(data)
}

pub fn format_target_get_details(data: &GetTargetDetailsOutput) -> String {
    format_get_target_details(data)
}

pub fn format_target_list(data: &ListTargetsOutput) -> String {
    format_list_targets(data)
}

pub fn format_target_update_artifactory(data: &TargetUpdateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_target_update_aws(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_azure(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_db(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_dockerhub(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_eks(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_gcp(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_gemini(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_github(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_gitlab(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_gke(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_global_sign(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_global_sign_atlas(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_godaddy(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_hashi_vault(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_k8s(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_ldap(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_lets_encrypt(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_linked(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_open_ai(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_ping(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_rabbit_mq(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_salesforce(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_sectigo(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_ssh(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_web(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_windows(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_target_update_zero_ssl(data: &TargetUpdateOutput) -> String {
    format_target_update_artifactory(data)
}

pub fn format_tokenize(data: &TokenizeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    if let Some(ref v) = data.tweak {
        let _ = writeln!(out, "Tweak: {v}");
    }
    out
}

pub fn format_tokenize_batch(data: &TokenizeOutput) -> String {
    format_tokenize(data)
}

pub fn format_uid_create_child_token(data: &UidCreateChildTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_uid_generate_token(data: &UidGenerateTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_uid_list_children(data: &UniversalIdentityDetails) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.max_depth {
        let _ = writeln!(out, "Max Depth: {v}");
    }
    if let Some(ref v) = data.number_of_tokens {
        let _ = writeln!(out, "Number Of Tokens: {v}");
    }
    if let Some(ref v) = data.root {
        let _ = writeln!(out, "Root: {v}");
    }
    out
}

pub fn format_uid_revoke_token(data: &UidRevokeTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_uid_rotate_token(data: &UidRotateTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.token {
        let _ = writeln!(out, "Token: {v}");
    }
    out
}

pub fn format_unwrap_token(data: &UnwrapTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.value {
        let _ = writeln!(out, "Value: {v}");
    }
    out
}

pub fn format_update_account_settings(data: &UpdateAccountSettingsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_update_artifactory_target(data: &UpdateArtifactoryTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_assoc(data: &UpdateAssocOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method(data: &UpdateAuthMethodOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_auth_method_awsiam(data: &UpdateAuthMethodAwsiamOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_azure_ad(data: &UpdateAuthMethodAzureAdOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_cert(data: &UpdateAuthMethodCertOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_update_auth_method_gcp(data: &UpdateAuthMethodGcpOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_k8s(data: &UpdateAuthMethodK8sOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_update_auth_method_ldap(data: &UpdateAuthMethodLdapOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.prv_key {
        let _ = writeln!(out, "Prv Key: {v}");
    }
    out
}

pub fn format_update_auth_method_o_auth2(data: &UpdateAuthMethodOAuth2Output) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_oci(data: &UpdateAuthMethodOciOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_id {
        let _ = writeln!(out, "Access Id: {v}");
    }
    out
}

pub fn format_update_auth_method_oidc(data: &UpdateAuthMethodOidcOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_saml(data: &UpdateAuthMethodSamlOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_auth_method_universal_identity(data: &UpdateAuthMethodUniversalIdentityOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_aws_target(data: &UpdateAwsTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_aws_target_details(data: &UpdateTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_update_azure_target(data: &UpdateAzureTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_certificate_value(data: &UpdateCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_classic_key_certificate(data: &UpdateClassicKeyCertificateOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_db_target(data: &UpdateDbTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_db_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_dockerhub_target(data: &UpdateDockerhubTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_eks_target(data: &UpdateEksTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_event_forwarder(data: &UpdateEventForwarderOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_gcp_target(data: &UpdateGcpTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_github_target(data: &UpdateGithubTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_gitlab_target(data: &UpdateGitlabTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_gke_target(data: &UpdateGkeTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_global_sign_atlas_target(data: &UpdateGlobalSignAtlasTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_global_sign_target(data: &UpdateGlobalSignTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_godaddy_target(data: &UpdateGodaddyTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_group(data: &UpdateGroupOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.group_name {
        let _ = writeln!(out, "Group Name: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_hashi_vault_target(data: &UpdateHashiVaultTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_item(data: &UpdateItemOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_update_native_k8s_target(data: &UpdateNativeK8sTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_ldap_target(data: &UpdateLdapTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_ldap_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_linked_target(data: &UpdateLinkedTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_oidc_app(data: &UpdateOidcAppOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_ping_target(data: &UpdatePingTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_pki_cert_issuer(data: &UpdatePkiCertIssuerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_rabbit_mq_target(data: &UpdateRabbitMqTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_rabbit_mq_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_rdp_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_role(data: &UpdateRoleOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.updated {
        let _ = writeln!(out, "Updated: {v}");
    }
    out
}

pub fn format_update_rotated_secret(data: &UpdateRotatedSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_rotation_settings(data: &RotateKeyOutput) -> String {
    format_rotate_key(data)
}

pub fn format_update_salesforce_target(data: &UpdateSalesforceTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_secret_val(data: &UpdateSecretValOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_ssh_cert_issuer(data: &UpdateSshCertIssuerOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_update_ssh_target(data: &UpdateSshTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_ssh_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_target(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_web_target(data: &UpdateWebTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_update_web_target_details(data: &UpdateTargetOutput) -> String {
    format_update_aws_target_details(data)
}

pub fn format_update_windows_target(data: &UpdateWindowsTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_update_zero_ssl_target(data: &UpdateZeroSslTargetOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.target_id {
        let _ = writeln!(out, "Target Id: {v}");
    }
    out
}

pub fn format_upload_rsa(data: &UploadRsaOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_usc_create(data: &UscCreateSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secret_id {
        let _ = writeln!(out, "Secret Id: {v}");
    }
    if let Some(ref v) = data.version_id {
        let _ = writeln!(out, "Version Id: {v}");
    }
    out
}

pub fn format_usc_delete(data: &UscDeleteSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    out
}

pub fn format_usc_get(data: &UscGetSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.binary_value {
        let _ = writeln!(out, "Binary Value: {v}");
    }
    if let Some(ref v) = data.encryption_key {
        let _ = writeln!(out, "Encryption Key: {v}");
    }
    if let Some(ref v) = data.id {
        let _ = writeln!(out, "Id: {v}");
    }
    if let Some(ref v) = data.metadata {
        let _ = writeln!(out, "Metadata: {v}");
    }
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.value {
        let _ = writeln!(out, "Value: {v}");
    }
    out
}

pub fn format_usc_list(data: &UscListSecretsOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.secrets_list {
        let _ = writeln!(out, "Secrets List: {v}");
    }
    out
}

pub fn format_usc_update(data: &UscUpdateSecretOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.name {
        let _ = writeln!(out, "Name: {v}");
    }
    if let Some(ref v) = data.secret_id {
        let _ = writeln!(out, "Secret Id: {v}");
    }
    if let Some(ref v) = data.version_id {
        let _ = writeln!(out, "Version Id: {v}");
    }
    out
}

pub fn format_get_last_user_event_status(data: &GetUserEventStatusOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.access_status {
        let _ = writeln!(out, "Access Status: {v}");
    }
    if let Some(ref v) = data.event_created_at {
        let _ = writeln!(out, "Event Created At: {v}");
    }
    if let Some(ref v) = data.status {
        let _ = writeln!(out, "Status: {v}");
    }
    out
}

pub fn format_validate_certificate_challenge(data: &ValidateCertificateChallengeOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.cert {
        let _ = writeln!(out, "Cert: {v}");
    }
    if let Some(ref v) = data.cert_display_id {
        let _ = writeln!(out, "Cert Display Id: {v}");
    }
    if let Some(ref v) = data.cert_item_id {
        let _ = writeln!(out, "Cert Item Id: {v}");
    }
    if let Some(ref v) = data.status {
        let _ = writeln!(out, "Status: {v}");
    }
    out
}

pub fn format_validate_token(data: &ValidateTokenOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.expiration {
        let _ = writeln!(out, "Expiration: {v}");
    }
    if let Some(ref v) = data.is_valid {
        let _ = writeln!(out, "Is Valid: {v}");
    }
    if let Some(ref v) = data.last_rotate {
        let _ = writeln!(out, "Last Rotate: {v}");
    }
    if let Some(ref v) = data.reason {
        let _ = writeln!(out, "Reason: {v}");
    }
    if let Some(ref v) = data.ttl {
        let _ = writeln!(out, "Ttl: {v}");
    }
    out
}

pub fn format_vault_address(data: &VaultAddressOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.api_gw_url {
        let _ = writeln!(out, "Api Gw Url: {v}");
    }
    if let Some(ref v) = data.vault_url {
        let _ = writeln!(out, "Vault Url: {v}");
    }
    out
}

pub fn format_verify_data_with_classic_key(data: &VerifyPkiCertOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_verify_ec_dsa(data: &VerifyEcDsaOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_verify_gpg(data: &VerifyGpgOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_verify_jwt_with_classic_key(data: &VerifyJwtOutput) -> String {
    let mut out = String::with_capacity(512);
    if let Some(ref v) = data.result {
        let _ = writeln!(out, "Result: {v}");
    }
    out
}

pub fn format_verify_pkcs1(data: &VerifyPkcs1Output) -> String {
    let mut out = String::with_capacity(512);
    out
}

pub fn format_verify_pki_cert_with_classic_key(data: &VerifyPkiCertOutput) -> String {
    format_verify_data_with_classic_key(data)
}

pub fn format_verify_rsa_ssa_pss(data: &VerifyRsaSsaPssOutput) -> String {
    let mut out = String::with_capacity(512);
    out
}

