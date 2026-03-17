use crate::api::types::*;
use crate::error::{AkeylessMcpError, Result};

/// HTTP client for the akeyless-mcp API.
///
/// The purpose of this application is to provide access to Akeyless API.
#[derive(Debug, Clone)]
pub struct AkeylessMcpClient {
    inner: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl AkeylessMcpClient {
    /// Create a new client.
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let inner = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .user_agent("pleme-io/akeyless_mcp 3.0")
            .build()
            .map_err(AkeylessMcpError::Request)?;

        Ok(Self {
            inner,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .inner
            .get(&self.url(path))
            
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .inner
            .post(&self.url(path))
            
            .json(body)
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn post_empty<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .inner
            .post(&self.url(path))
            
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn put<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .inner
            .put(&self.url(path))
            
            .json(body)
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn patch<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let resp = self
            .inner
            .patch(&self.url(path))
            
            .json(body)
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self
            .inner
            .delete(&self.url(path))
            
            .send()
            .await
            .map_err(AkeylessMcpError::Request)?;
        Self::handle_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T> {
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(AkeylessMcpError::Api { status, body });
        }
        let text = resp.text().await.map_err(AkeylessMcpError::Request)?;
        serde_json::from_str(&text).map_err(AkeylessMcpError::Json)
    }

    // -- Public API methods --

    /// POST /account-custom-field-create
    ///
    /// Create a new custom field.
    pub async fn account_custom_field_create(
        &self,
        req: &AccountCustomFieldCreate,
    ) -> Result<AccountCustomFieldCreateOutput> {
        self.post("/account-custom-field-create", req).await
    }

    /// POST /account-custom-field-delete
    ///
    /// Delete a custom field.
    pub async fn account_custom_field_delete(
        &self,
        req: &AccountCustomFieldDelete,
    ) -> Result<AccountCustomFieldDeleteOutput> {
        self.post("/account-custom-field-delete", req).await
    }

    /// POST /account-custom-field-get
    ///
    /// Get an account custom field by ID.
    pub async fn account_custom_field_get(
        &self,
        req: &AccountCustomFieldGet,
    ) -> Result<AccountCustomFieldGetOutput> {
        self.post("/account-custom-field-get", req).await
    }

    /// POST /account-custom-field-list
    ///
    /// List all account custom fields.
    pub async fn account_custom_field_list(
        &self,
        req: &AccountCustomFieldList,
    ) -> Result<AccountCustomFieldListOutput> {
        self.post("/account-custom-field-list", req).await
    }

    /// POST /account-custom-field-update
    ///
    /// Update a custom field.
    pub async fn account_custom_field_update(
        &self,
        req: &AccountCustomFieldUpdate,
    ) -> Result<AccountCustomFieldUpdateOutput> {
        self.post("/account-custom-field-update", req).await
    }

    /// POST /alias-details
    pub async fn alias_details(
        &self,
        req: &AliasDetails,
    ) -> Result<DeleteRoleOutput> {
        self.post("/alias-details", req).await
    }

    /// POST /assoc-role-am
    pub async fn assoc_role_auth_method(
        &self,
        req: &AssocRoleAuthMethod,
    ) -> Result<CreateRoleAuthMethodAssocOutput> {
        self.post("/assoc-role-am", req).await
    }

    /// POST /assoc-target-item
    pub async fn assoc_target_item(
        &self,
        req: &AssocTargetItem,
    ) -> Result<CreateTargetItemAssocOutput> {
        self.post("/assoc-target-item", req).await
    }

    /// POST /auth
    pub async fn auth(
        &self,
        req: &Auth,
    ) -> Result<AuthOutput> {
        self.post("/auth", req).await
    }

    /// POST /auth-method-create-api-key
    pub async fn auth_method_create_api_key(
        &self,
        req: &AuthMethodCreateApiKey,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-api-key", req).await
    }

    /// POST /auth-method-create-aws-iam
    pub async fn auth_method_create_aws_iam(
        &self,
        req: &AuthMethodCreateAwsIam,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-aws-iam", req).await
    }

    /// POST /auth-method-create-azure-ad
    pub async fn auth_method_create_azure_ad(
        &self,
        req: &AuthMethodCreateAzureAd,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-azure-ad", req).await
    }

    /// POST /auth-method-create-cert
    pub async fn auth_method_create_cert(
        &self,
        req: &AuthMethodCreateCert,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-cert", req).await
    }

    /// POST /auth-method-create-email
    pub async fn auth_method_create_email(
        &self,
        req: &AuthMethodCreateEmail,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-email", req).await
    }

    /// POST /auth-method-create-gcp
    pub async fn auth_method_create_gcp(
        &self,
        req: &AuthMethodCreateGcp,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-gcp", req).await
    }

    /// POST /auth-method-create-k8s
    pub async fn auth_method_create_k8s(
        &self,
        req: &AuthMethodCreateK8s,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-k8s", req).await
    }

    /// POST /auth-method-create-kerberos
    pub async fn auth_method_create_kerberos(
        &self,
        req: &AuthMethodCreateKerberos,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-kerberos", req).await
    }

    /// POST /auth-method-create-ldap
    pub async fn auth_method_create_ldap(
        &self,
        req: &AuthMethodCreateLdap,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-ldap", req).await
    }

    /// POST /auth-method-create-oauth2
    pub async fn auth_method_create_oauth2(
        &self,
        req: &AuthMethodCreateOauth2,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-oauth2", req).await
    }

    /// POST /auth-method-create-oci
    pub async fn auth_method_create_oci(
        &self,
        req: &AuthMethodCreateOci,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-oci", req).await
    }

    /// POST /auth-method-create-oidc
    pub async fn auth_method_create_oidc(
        &self,
        req: &AuthMethodCreateOidc,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-oidc", req).await
    }

    /// POST /auth-method-create-saml
    pub async fn auth_method_create_saml(
        &self,
        req: &AuthMethodCreateSaml,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-saml", req).await
    }

    /// POST /auth-method-create-universal-identity
    pub async fn auth_method_create_universal_identity(
        &self,
        req: &AuthMethodCreateUniversalIdentity,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-create-universal-identity", req).await
    }

    /// POST /auth-method-delete
    pub async fn auth_method_delete(
        &self,
        req: &AuthMethodDelete,
    ) -> Result<AuthMethodDeleteOutput> {
        self.post("/auth-method-delete", req).await
    }

    /// POST /auth-method-get
    pub async fn auth_method_get(
        &self,
        req: &AuthMethodGet,
    ) -> Result<AuthMethod> {
        self.post("/auth-method-get", req).await
    }

    /// POST /auth-method-list
    pub async fn auth_method_list(
        &self,
        req: &AuthMethodList,
    ) -> Result<ListAuthMethodsOutput> {
        self.post("/auth-method-list", req).await
    }

    /// POST /auth-method-update-api-key
    pub async fn auth_method_update_api_key(
        &self,
        req: &AuthMethodUpdateApiKey,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-api-key", req).await
    }

    /// POST /auth-method-update-aws-iam
    pub async fn auth_method_update_aws_iam(
        &self,
        req: &AuthMethodUpdateAwsIam,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-aws-iam", req).await
    }

    /// POST /auth-method-update-azure-ad
    pub async fn auth_method_update_azure_ad(
        &self,
        req: &AuthMethodUpdateAzureAd,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-azure-ad", req).await
    }

    /// POST /auth-method-update-cert
    pub async fn auth_method_update_cert(
        &self,
        req: &AuthMethodUpdateCert,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-cert", req).await
    }

    /// POST /auth-method-update-email
    pub async fn auth_method_update_email(
        &self,
        req: &AuthMethodUpdateEmail,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-email", req).await
    }

    /// POST /auth-method-update-gcp
    pub async fn auth_method_update_gcp(
        &self,
        req: &AuthMethodUpdateGcp,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-gcp", req).await
    }

    /// POST /auth-method-update-k8s
    pub async fn auth_method_update_k8s(
        &self,
        req: &AuthMethodUpdateK8s,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-k8s", req).await
    }

    /// POST /auth-method-update-kerberos
    pub async fn auth_method_update_kerberos(
        &self,
        req: &AuthMethodUpdateKerberos,
    ) -> Result<AuthMethodCreateOutput> {
        self.post("/auth-method-update-kerberos", req).await
    }

    /// POST /auth-method-update-ldap
    pub async fn auth_method_update_ldap(
        &self,
        req: &AuthMethodUpdateLdap,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-ldap", req).await
    }

    /// POST /auth-method-update-oauth2
    pub async fn auth_method_update_oauth2(
        &self,
        req: &AuthMethodUpdateOauth2,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-oauth2", req).await
    }

    /// POST /auth-method-update-oci
    pub async fn auth_method_update_oci(
        &self,
        req: &AuthMethodUpdateOci,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-oci", req).await
    }

    /// POST /auth-method-update-oidc
    pub async fn auth_method_update_oidc(
        &self,
        req: &AuthMethodUpdateOidc,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-oidc", req).await
    }

    /// POST /auth-method-update-saml
    pub async fn auth_method_update_saml(
        &self,
        req: &AuthMethodUpdateSaml,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-saml", req).await
    }

    /// POST /auth-method-update-universal-identity
    pub async fn auth_method_update_universal_identity(
        &self,
        req: &AuthMethodUpdateUniversalIdentity,
    ) -> Result<AuthMethodUpdateOutput> {
        self.post("/auth-method-update-universal-identity", req).await
    }

    /// POST /calc-password-security-info
    pub async fn calc_password_security_info(
        &self,
        req: &CalcPasswordSecurityInfo,
    ) -> Result<PasswordSecurityInfo> {
        self.post("/calc-password-security-info", req).await
    }

    /// POST /certificate-discovery
    pub async fn certificate_discovery(
        &self,
        req: &CertificateDiscovery,
    ) -> Result<CertificateDiscoveryOutput> {
        self.post("/certificate-discovery", req).await
    }

    /// POST /change-admin-account-password
    pub async fn change_admin_account_password(
        &self,
        req: &ChangeAdminAccountPassword,
    ) -> Result<serde_json::Value> {
        self.post("/change-admin-account-password", req).await
    }

    /// POST /configure
    pub async fn configure(
        &self,
        req: &Configure,
    ) -> Result<ConfigureOutput> {
        self.post("/configure", req).await
    }

    /// POST /connect
    pub async fn connect(
        &self,
        req: &Connect,
    ) -> Result<ConnectOutput> {
        self.post("/connect", req).await
    }

    /// POST /create-artifactory-target
    pub async fn create_artifactory_target(
        &self,
        req: &CreateArtifactoryTarget,
    ) -> Result<CreateArtifactoryTargetOutput> {
        self.post("/create-artifactory-target", req).await
    }

    /// POST /create-auth-method
    pub async fn create_auth_method(
        &self,
        req: &CreateAuthMethod,
    ) -> Result<CreateAuthMethodOutput> {
        self.post("/create-auth-method", req).await
    }

    /// POST /create-auth-method-aws-iam
    pub async fn create_auth_method_awsiam(
        &self,
        req: &CreateAuthMethodAwsiam,
    ) -> Result<CreateAuthMethodAwsiamOutput> {
        self.post("/create-auth-method-aws-iam", req).await
    }

    /// POST /create-auth-method-azure-ad
    pub async fn create_auth_method_azure_ad(
        &self,
        req: &CreateAuthMethodAzureAd,
    ) -> Result<CreateAuthMethodAzureAdOutput> {
        self.post("/create-auth-method-azure-ad", req).await
    }

    /// POST /create-auth-method-cert
    pub async fn create_auth_method_cert(
        &self,
        req: &CreateAuthMethodCert,
    ) -> Result<CreateAuthMethodCertOutput> {
        self.post("/create-auth-method-cert", req).await
    }

    /// POST /create-auth-method-email
    pub async fn create_auth_method_email(
        &self,
        req: &CreateAuthMethodEmail,
    ) -> Result<CreateAuthMethodEmailOutput> {
        self.post("/create-auth-method-email", req).await
    }

    /// POST /create-auth-method-gcp
    pub async fn create_auth_method_gcp(
        &self,
        req: &CreateAuthMethodGcp,
    ) -> Result<CreateAuthMethodGcpOutput> {
        self.post("/create-auth-method-gcp", req).await
    }

    /// POST /create-auth-method-huawei
    pub async fn create_auth_method_huawei(
        &self,
        req: &CreateAuthMethodHuawei,
    ) -> Result<CreateAuthMethodHuaweiOutput> {
        self.post("/create-auth-method-huawei", req).await
    }

    /// POST /create-auth-method-k8s
    pub async fn create_auth_method_k8s(
        &self,
        req: &CreateAuthMethodK8s,
    ) -> Result<CreateAuthMethodK8sOutput> {
        self.post("/create-auth-method-k8s", req).await
    }

    /// POST /create-auth-method-ldap
    pub async fn create_auth_method_ldap(
        &self,
        req: &CreateAuthMethodLdap,
    ) -> Result<CreateAuthMethodLdapOutput> {
        self.post("/create-auth-method-ldap", req).await
    }

    /// POST /create-auth-method-oauth2
    pub async fn create_auth_method_o_auth2(
        &self,
        req: &CreateAuthMethodOAuth2,
    ) -> Result<CreateAuthMethodOAuth2Output> {
        self.post("/create-auth-method-oauth2", req).await
    }

    /// POST /create-auth-method-oci
    pub async fn create_auth_method_oci(
        &self,
        req: &CreateAuthMethodOci,
    ) -> Result<CreateAuthMethodOciOutput> {
        self.post("/create-auth-method-oci", req).await
    }

    /// POST /create-auth-method-oidc
    pub async fn create_auth_method_oidc(
        &self,
        req: &CreateAuthMethodOidc,
    ) -> Result<CreateAuthMethodOidcOutput> {
        self.post("/create-auth-method-oidc", req).await
    }

    /// POST /create-auth-method-saml
    pub async fn create_auth_method_saml(
        &self,
        req: &CreateAuthMethodSaml,
    ) -> Result<CreateAuthMethodSamlOutput> {
        self.post("/create-auth-method-saml", req).await
    }

    /// POST /create-auth-method-universal-identity
    pub async fn create_auth_method_universal_identity(
        &self,
        req: &CreateAuthMethodUniversalIdentity,
    ) -> Result<CreateAuthMethodUniversalIdentityOutput> {
        self.post("/create-auth-method-universal-identity", req).await
    }

    /// POST /create-aws-target
    pub async fn create_aws_target(
        &self,
        req: &CreateAwsTarget,
    ) -> Result<CreateAwsTargetOutput> {
        self.post("/create-aws-target", req).await
    }

    /// POST /create-azure-target
    pub async fn create_azure_target(
        &self,
        req: &CreateAzureTarget,
    ) -> Result<CreateAzureTargetOutput> {
        self.post("/create-azure-target", req).await
    }

    /// POST /create-certificate
    pub async fn create_certificate(
        &self,
        req: &CreateCertificate,
    ) -> Result<CreateCertificateOutput> {
        self.post("/create-certificate", req).await
    }

    /// POST /create-classic-key
    pub async fn create_classic_key(
        &self,
        req: &CreateClassicKey,
    ) -> Result<CreateClassicKeyOutput> {
        self.post("/create-classic-key", req).await
    }

    /// POST /create-db-target
    pub async fn create_db_target(
        &self,
        req: &CreateDbTarget,
    ) -> Result<CreateDbTargetOutput> {
        self.post("/create-db-target", req).await
    }

    /// POST /create-dfc-key
    pub async fn create_dfc_key(
        &self,
        req: &CreateDfcKey,
    ) -> Result<CreateDfcKeyOutput> {
        self.post("/create-dfc-key", req).await
    }

    /// POST /create-dockerhub-target
    pub async fn create_dockerhub_target(
        &self,
        req: &CreateDockerhubTarget,
    ) -> Result<CreateDockerhubTargetOutput> {
        self.post("/create-dockerhub-target", req).await
    }

    /// POST /create-dynamic-secret
    pub async fn create_dynamic_secret(
        &self,
        req: &CreateDynamicSecret,
    ) -> Result<CreateDynamicSecretOutput> {
        self.post("/create-dynamic-secret", req).await
    }

    /// POST /create-eks-target
    pub async fn create_eks_target(
        &self,
        req: &CreateEksTarget,
    ) -> Result<CreateEksTargetOutput> {
        self.post("/create-eks-target", req).await
    }

    /// POST /create-esm
    pub async fn create_esm(
        &self,
        req: &CreateEsm,
    ) -> Result<CreateEsmOutput> {
        self.post("/create-esm", req).await
    }

    /// POST /create-event-forwarder
    pub async fn create_event_forwarder(
        &self,
        req: &CreateEventForwarder,
    ) -> Result<CreateEventForwarderOutput> {
        self.post("/create-event-forwarder", req).await
    }

    /// POST /create-gcp-target
    pub async fn create_gcp_target(
        &self,
        req: &CreateGcpTarget,
    ) -> Result<CreateGcpTargetOutput> {
        self.post("/create-gcp-target", req).await
    }

    /// POST /create-github-target
    pub async fn create_github_target(
        &self,
        req: &CreateGithubTarget,
    ) -> Result<CreateGithubTargetOutput> {
        self.post("/create-github-target", req).await
    }

    /// POST /create-gitlab-target
    pub async fn create_gitlab_target(
        &self,
        req: &CreateGitlabTarget,
    ) -> Result<CreateGitlabTargetOutput> {
        self.post("/create-gitlab-target", req).await
    }

    /// POST /create-gke-target
    pub async fn create_gke_target(
        &self,
        req: &CreateGkeTarget,
    ) -> Result<CreateGkeTargetOutput> {
        self.post("/create-gke-target", req).await
    }

    /// POST /create-globalsign-atlas-target
    pub async fn create_global_sign_atlas_target(
        &self,
        req: &CreateGlobalSignAtlasTarget,
    ) -> Result<CreateGlobalSignAtlasTargetOutput> {
        self.post("/create-globalsign-atlas-target", req).await
    }

    /// POST /create-globalsign-target
    pub async fn create_global_sign_target(
        &self,
        req: &CreateGlobalSignTarget,
    ) -> Result<CreateGlobalSignTargetOutput> {
        self.post("/create-globalsign-target", req).await
    }

    /// POST /create-godaddy-target
    pub async fn create_godaddy_target(
        &self,
        req: &CreateGodaddyTarget,
    ) -> Result<CreateGodaddyTargetOutput> {
        self.post("/create-godaddy-target", req).await
    }

    /// POST /create-group
    pub async fn create_group(
        &self,
        req: &CreateGroup,
    ) -> Result<CreateGroupOutput> {
        self.post("/create-group", req).await
    }

    /// POST /create-hashi-vault-target
    pub async fn create_hashi_vault_target(
        &self,
        req: &CreateHashiVaultTarget,
    ) -> Result<CreateHashiVaultTargetOutput> {
        self.post("/create-hashi-vault-target", req).await
    }

    /// POST /create-k8s-target
    pub async fn create_native_k8s_target(
        &self,
        req: &CreateNativeK8sTarget,
    ) -> Result<CreateNativeK8sTargetOutput> {
        self.post("/create-k8s-target", req).await
    }

    /// POST /create-key
    pub async fn create_key(
        &self,
        req: &CreateKey,
    ) -> Result<CreateKeyOutput> {
        self.post("/create-key", req).await
    }

    /// POST /create-ldap-target
    pub async fn createldap_target(
        &self,
        req: &CreateLdapTarget,
    ) -> Result<CreateLdapTargetOutput> {
        self.post("/create-ldap-target", req).await
    }

    /// POST /create-linked-target
    pub async fn create_linked_target(
        &self,
        req: &CreateLinkedTarget,
    ) -> Result<CreateLinkedTargetOutput> {
        self.post("/create-linked-target", req).await
    }

    /// POST /create-oidc-app
    pub async fn create_oidc_app(
        &self,
        req: &CreateOidcApp,
    ) -> Result<CreateOidcAppOutput> {
        self.post("/create-oidc-app", req).await
    }

    /// POST /create-passkey
    pub async fn create_passkey(
        &self,
        req: &CreatePasskey,
    ) -> Result<CreatePasskeyOutput> {
        self.post("/create-passkey", req).await
    }

    /// POST /create-ping-target
    pub async fn create_ping_target(
        &self,
        req: &CreatePingTarget,
    ) -> Result<CreatePingTargetOutput> {
        self.post("/create-ping-target", req).await
    }

    /// POST /create-pki-cert-issuer
    pub async fn create_pki_cert_issuer(
        &self,
        req: &CreatePkiCertIssuer,
    ) -> Result<CreatePkiCertIssuerOutput> {
        self.post("/create-pki-cert-issuer", req).await
    }

    /// POST /create-rabbitmq-target
    pub async fn create_rabbit_mq_target(
        &self,
        req: &CreateRabbitMqTarget,
    ) -> Result<CreateRabbitMqTargetOutput> {
        self.post("/create-rabbitmq-target", req).await
    }

    /// POST /create-role
    pub async fn create_role(
        &self,
        req: &CreateRole,
    ) -> Result<CreateRoleOutput> {
        self.post("/create-role", req).await
    }

    /// POST /create-rotated-secret
    pub async fn create_rotated_secret(
        &self,
        req: &CreateRotatedSecret,
    ) -> Result<CreateRotatedSecretOutput> {
        self.post("/create-rotated-secret", req).await
    }

    /// POST /create-salesforce-target
    pub async fn create_salesforce_target(
        &self,
        req: &CreateSalesforceTarget,
    ) -> Result<CreateSalesforceTargetOutput> {
        self.post("/create-salesforce-target", req).await
    }

    /// POST /create-secret
    pub async fn create_secret(
        &self,
        req: &CreateSecret,
    ) -> Result<CreateSecretOutput> {
        self.post("/create-secret", req).await
    }

    /// POST /create-ssh-cert-issuer
    pub async fn create_ssh_cert_issuer(
        &self,
        req: &CreateSshCertIssuer,
    ) -> Result<CreateSshCertIssuerOutput> {
        self.post("/create-ssh-cert-issuer", req).await
    }

    /// POST /create-ssh-target
    pub async fn create_ssh_target(
        &self,
        req: &CreateSshTarget,
    ) -> Result<CreateSshTargetOutput> {
        self.post("/create-ssh-target", req).await
    }

    /// POST /create-tokenizer
    pub async fn create_tokenizer(
        &self,
        req: &CreateTokenizer,
    ) -> Result<CreateTokenizerOutput> {
        self.post("/create-tokenizer", req).await
    }

    /// POST /create-usc
    pub async fn create_usc(
        &self,
        req: &CreateUsc,
    ) -> Result<CreateUscOutput> {
        self.post("/create-usc", req).await
    }

    /// POST /create-user-event
    pub async fn create_user_event(
        &self,
        req: &CreateUserEvent,
    ) -> Result<CreateUserEventOutput> {
        self.post("/create-user-event", req).await
    }

    /// POST /create-web-target
    pub async fn create_web_target(
        &self,
        req: &CreateWebTarget,
    ) -> Result<CreateWebTargetOutput> {
        self.post("/create-web-target", req).await
    }

    /// POST /create-windows-target
    pub async fn create_windows_target(
        &self,
        req: &CreateWindowsTarget,
    ) -> Result<CreateWindowsTargetOutput> {
        self.post("/create-windows-target", req).await
    }

    /// POST /create-zerossl-target
    pub async fn create_zero_ssl_target(
        &self,
        req: &CreateZeroSslTarget,
    ) -> Result<CreateZeroSslTargetOutput> {
        self.post("/create-zerossl-target", req).await
    }

    /// POST /deactivate-acme-account
    pub async fn deactivate_acme_account(
        &self,
        req: &DeactivateAcmeAccount,
    ) -> Result<DeactivateAcmeAccountOutput> {
        self.post("/deactivate-acme-account", req).await
    }

    /// POST /decrypt
    pub async fn decrypt(
        &self,
        req: &Decrypt,
    ) -> Result<DecryptOutput> {
        self.post("/decrypt", req).await
    }

    /// POST /decrypt-batch
    pub async fn decrypt_batch(
        &self,
        req: &BatchEncryptionRequest,
    ) -> Result<DecryptOutput> {
        self.post("/decrypt-batch", req).await
    }

    /// POST /decrypt-gpg
    pub async fn decrypt_gpg(
        &self,
        req: &DecryptGpg,
    ) -> Result<DecryptGpgOutput> {
        self.post("/decrypt-gpg", req).await
    }

    /// POST /decrypt-pkcs1
    pub async fn decrypt_pkcs1(
        &self,
        req: &DecryptPkcs1,
    ) -> Result<DecryptPkcs1Output> {
        self.post("/decrypt-pkcs1", req).await
    }

    /// POST /decrypt-with-classic-key
    pub async fn decrypt_with_classic_key(
        &self,
        req: &DecryptWithClassicKey,
    ) -> Result<DecryptWithClassicKeyOutput> {
        self.post("/decrypt-with-classic-key", req).await
    }

    /// POST /delete-assoc
    pub async fn delete_role_association(
        &self,
        req: &DeleteRoleAssociation,
    ) -> Result<DeleteRoleAssociationOutput> {
        self.post("/delete-assoc", req).await
    }

    /// POST /delete-assoc-target-item
    pub async fn delete_target_association(
        &self,
        req: &DeleteTargetAssociation,
    ) -> Result<DeleteTargetAssociationOutput> {
        self.post("/delete-assoc-target-item", req).await
    }

    /// POST /delete-auth-method
    pub async fn delete_auth_method(
        &self,
        req: &DeleteAuthMethod,
    ) -> Result<DeleteAuthMethodOutput> {
        self.post("/delete-auth-method", req).await
    }

    /// POST /delete-auth-methods
    pub async fn delete_auth_methods(
        &self,
        req: &DeleteAuthMethods,
    ) -> Result<DeleteAuthMethodsOutput> {
        self.post("/delete-auth-methods", req).await
    }

    /// POST /delete-event-forwarder
    pub async fn delete_event_forwarder(
        &self,
        req: &DeleteEventForwarder,
    ) -> Result<DeleteEventForwarderOutput> {
        self.post("/delete-event-forwarder", req).await
    }

    /// POST /delete-gateway-cluster
    pub async fn delete_gw_cluster(
        &self,
        req: &DeleteGwCluster,
    ) -> Result<DeleteGwClusterOutput> {
        self.post("/delete-gateway-cluster", req).await
    }

    /// POST /delete-group
    pub async fn delete_group(
        &self,
        req: &DeleteGroup,
    ) -> Result<DeleteGroupOutput> {
        self.post("/delete-group", req).await
    }

    /// POST /delete-item
    pub async fn delete_item(
        &self,
        req: &DeleteItem,
    ) -> Result<DeleteItemOutput> {
        self.post("/delete-item", req).await
    }

    /// POST /delete-items
    pub async fn delete_items(
        &self,
        req: &DeleteItems,
    ) -> Result<DeleteItemsOutput> {
        self.post("/delete-items", req).await
    }

    /// POST /delete-role
    pub async fn delete_role(
        &self,
        req: &DeleteRole,
    ) -> Result<DeleteRoleOutput> {
        self.post("/delete-role", req).await
    }

    /// POST /delete-role-rule
    pub async fn delete_role_rule(
        &self,
        req: &DeleteRoleRule,
    ) -> Result<DeleteRoleRuleOutput> {
        self.post("/delete-role-rule", req).await
    }

    /// POST /delete-roles
    pub async fn delete_roles(
        &self,
        req: &DeleteRoles,
    ) -> Result<DeleteRolesOutput> {
        self.post("/delete-roles", req).await
    }

    /// POST /delete-target
    pub async fn delete_target(
        &self,
        req: &DeleteTarget,
    ) -> Result<DeleteTargetOutput> {
        self.post("/delete-target", req).await
    }

    /// POST /delete-targets
    pub async fn delete_targets(
        &self,
        req: &DeleteTargets,
    ) -> Result<DeleteTargetsOutput> {
        self.post("/delete-targets", req).await
    }

    /// POST /derive-key
    pub async fn derive_key(
        &self,
        req: &DeriveKey,
    ) -> Result<DeriveKeyOutput> {
        self.post("/derive-key", req).await
    }

    /// POST /describe-item
    pub async fn describe_item(
        &self,
        req: &DescribeItem,
    ) -> Result<Item> {
        self.post("/describe-item", req).await
    }

    /// POST /describe-permissions
    pub async fn describe_permissions(
        &self,
        req: &DescribePermissions,
    ) -> Result<DescribePermissionsOutput> {
        self.post("/describe-permissions", req).await
    }

    /// POST /describe-role-am-assoc
    pub async fn describe_assoc(
        &self,
        req: &DescribeAssoc,
    ) -> Result<RoleAssociationDetails> {
        self.post("/describe-role-am-assoc", req).await
    }

    /// POST /describe-sub-claims
    pub async fn describe_sub_claims(
        &self,
        req: &DescribeSubClaims,
    ) -> Result<DescribeSubClaimsOutput> {
        self.post("/describe-sub-claims", req).await
    }

    /// POST /detokenize
    pub async fn detokenize(
        &self,
        req: &Detokenize,
    ) -> Result<DetokenizeOutput> {
        self.post("/detokenize", req).await
    }

    /// POST /detokenize-batch
    pub async fn detokenize_batch(
        &self,
        req: &BatchTokenizationRequest,
    ) -> Result<DetokenizeOutput> {
        self.post("/detokenize-batch", req).await
    }

    /// POST /dynamic-secret-create-artifactory
    pub async fn dynamic_secret_create_artifactory(
        &self,
        req: &DynamicSecretCreateArtifactory,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-artifactory", req).await
    }

    /// POST /dynamic-secret-create-aws
    pub async fn dynamic_secret_create_aws(
        &self,
        req: &DynamicSecretCreateAws,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-aws", req).await
    }

    /// POST /dynamic-secret-create-azure
    pub async fn dynamic_secret_create_azure(
        &self,
        req: &DynamicSecretCreateAzure,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-azure", req).await
    }

    /// POST /dynamic-secret-create-cassandra
    pub async fn dynamic_secret_create_cassandra(
        &self,
        req: &DynamicSecretCreateCassandra,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-cassandra", req).await
    }

    /// POST /dynamic-secret-create-custom
    pub async fn dynamic_secret_create_custom(
        &self,
        req: &DynamicSecretCreateCustom,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-custom", req).await
    }

    /// POST /dynamic-secret-create-dockerhub
    pub async fn dynamic_secret_create_dockerhub(
        &self,
        req: &DynamicSecretCreateDockerhub,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-dockerhub", req).await
    }

    /// POST /dynamic-secret-create-eks
    pub async fn dynamic_secret_create_eks(
        &self,
        req: &DynamicSecretCreateEks,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-eks", req).await
    }

    /// POST /dynamic-secret-create-gcp
    pub async fn dynamic_secret_create_gcp(
        &self,
        req: &DynamicSecretCreateGcp,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-gcp", req).await
    }

    /// POST /dynamic-secret-create-github
    pub async fn dynamic_secret_create_github(
        &self,
        req: &DynamicSecretCreateGithub,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-github", req).await
    }

    /// POST /dynamic-secret-create-gitlab
    pub async fn dynamic_secret_create_gitlab(
        &self,
        req: &DynamicSecretCreateGitlab,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-gitlab", req).await
    }

    /// POST /dynamic-secret-create-gke
    pub async fn dynamic_secret_create_gke(
        &self,
        req: &DynamicSecretCreateGke,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-gke", req).await
    }

    /// POST /dynamic-secret-create-google-workspace
    pub async fn dynamic_secret_create_google_workspace(
        &self,
        req: &DynamicSecretCreateGoogleWorkspace,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-google-workspace", req).await
    }

    /// POST /dynamic-secret-create-hanadb
    pub async fn dynamic_secret_create_hana_db(
        &self,
        req: &DynamicSecretCreateHanaDb,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-hanadb", req).await
    }

    /// POST /dynamic-secret-create-k8s
    pub async fn dynamic_secret_create_k8s(
        &self,
        req: &DynamicSecretCreateK8s,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-k8s", req).await
    }

    /// POST /dynamic-secret-create-ldap
    pub async fn dynamic_secret_create_ldap(
        &self,
        req: &DynamicSecretCreateLdap,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-ldap", req).await
    }

    /// POST /dynamic-secret-create-mongodb
    pub async fn dynamic_secret_create_mongo_db(
        &self,
        req: &DynamicSecretCreateMongoDb,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-mongodb", req).await
    }

    /// POST /dynamic-secret-create-mssql
    pub async fn dynamic_secret_create_ms_sql(
        &self,
        req: &DynamicSecretCreateMsSql,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-mssql", req).await
    }

    /// POST /dynamic-secret-create-mysql
    pub async fn dynamic_secret_create_my_sql(
        &self,
        req: &DynamicSecretCreateMySql,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-mysql", req).await
    }

    /// POST /dynamic-secret-create-openai
    pub async fn dynamic_secret_create_open_ai(
        &self,
        req: &DynamicSecretCreateOpenAi,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-openai", req).await
    }

    /// POST /dynamic-secret-create-oracle
    pub async fn dynamic_secret_create_oracle_db(
        &self,
        req: &DynamicSecretCreateOracleDb,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-oracle", req).await
    }

    /// POST /dynamic-secret-create-ping
    pub async fn dynamic_secret_create_ping(
        &self,
        req: &DynamicSecretCreatePing,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-ping", req).await
    }

    /// POST /dynamic-secret-create-postgresql
    pub async fn dynamic_secret_create_postgre_sql(
        &self,
        req: &DynamicSecretCreatePostgreSql,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-postgresql", req).await
    }

    /// POST /dynamic-secret-create-rabbitmq
    pub async fn dynamic_secret_create_rabbit_mq(
        &self,
        req: &DynamicSecretCreateRabbitMq,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-rabbitmq", req).await
    }

    /// POST /dynamic-secret-create-rdp
    pub async fn dynamic_secret_create_rdp(
        &self,
        req: &DynamicSecretCreateRdp,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-rdp", req).await
    }

    /// POST /dynamic-secret-create-redis
    pub async fn dynamic_secret_create_redis(
        &self,
        req: &DynamicSecretCreateRedis,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-redis", req).await
    }

    /// POST /dynamic-secret-create-redshift
    pub async fn dynamic_secret_create_redshift(
        &self,
        req: &DynamicSecretCreateRedshift,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-redshift", req).await
    }

    /// POST /dynamic-secret-create-snowflake
    pub async fn dynamic_secret_create_snowflake(
        &self,
        req: &DynamicSecretCreateSnowflake,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-snowflake", req).await
    }

    /// POST /dynamic-secret-create-venafi
    pub async fn dynamic_secret_create_venafi(
        &self,
        req: &DynamicSecretCreateVenafi,
    ) -> Result<DynamicSecretCreateOutput> {
        self.post("/dynamic-secret-create-venafi", req).await
    }

    /// POST /dynamic-secret-delete
    pub async fn dynamic_secret_delete(
        &self,
        req: &DynamicSecretDelete,
    ) -> Result<DynamicSecretDeleteOutput> {
        self.post("/dynamic-secret-delete", req).await
    }

    /// POST /dynamic-secret-get
    pub async fn dynamic_secret_get(
        &self,
        req: &DynamicSecretGet,
    ) -> Result<DsProducerDetails> {
        self.post("/dynamic-secret-get", req).await
    }

    /// POST /dynamic-secret-get-value
    pub async fn dynamic_secret_get_value(
        &self,
        req: &DynamicSecretGetValue,
    ) -> Result<serde_json::Value> {
        self.post("/dynamic-secret-get-value", req).await
    }

    /// POST /dynamic-secret-list
    pub async fn dynamic_secret_list(
        &self,
        req: &DynamicSecretList,
    ) -> Result<GetProducersListReplyObj> {
        self.post("/dynamic-secret-list", req).await
    }

    /// POST /dynamic-secret-tmp-creds-Get
    pub async fn dynamic_secret_tmp_creds_get(
        &self,
        req: &DynamicSecretTmpCredsGet,
    ) -> Result<Vec<TmpUserData>> {
        self.post("/dynamic-secret-tmp-creds-Get", req).await
    }

    /// POST /dynamic-secret-tmp-creds-delete
    pub async fn dynamic_secret_tmp_creds_delete(
        &self,
        req: &DynamicSecretTmpCredsDelete,
    ) -> Result<JsonError> {
        self.post("/dynamic-secret-tmp-creds-delete", req).await
    }

    /// POST /dynamic-secret-tmp-creds-update
    pub async fn dynamic_secret_tmp_creds_update(
        &self,
        req: &DynamicSecretTmpCredsUpdate,
    ) -> Result<JsonError> {
        self.post("/dynamic-secret-tmp-creds-update", req).await
    }

    /// POST /dynamic-secret-update-artifactory
    pub async fn dynamic_secret_update_artifactory(
        &self,
        req: &DynamicSecretUpdateArtifactory,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-artifactory", req).await
    }

    /// POST /dynamic-secret-update-aws
    pub async fn dynamic_secret_update_aws(
        &self,
        req: &DynamicSecretUpdateAws,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-aws", req).await
    }

    /// POST /dynamic-secret-update-azure
    pub async fn dynamic_secret_update_azure(
        &self,
        req: &DynamicSecretUpdateAzure,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-azure", req).await
    }

    /// POST /dynamic-secret-update-cassandra
    pub async fn dynamic_secret_update_cassandra(
        &self,
        req: &DynamicSecretUpdateCassandra,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-cassandra", req).await
    }

    /// POST /dynamic-secret-update-custom
    pub async fn dynamic_secret_update_custom(
        &self,
        req: &DynamicSecretUpdateCustom,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-custom", req).await
    }

    /// POST /dynamic-secret-update-dockerhub
    pub async fn dynamic_secret_update_dockerhub(
        &self,
        req: &DynamicSecretUpdateDockerhub,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-dockerhub", req).await
    }

    /// POST /dynamic-secret-update-eks
    pub async fn dynamic_secret_update_eks(
        &self,
        req: &DynamicSecretUpdateEks,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-eks", req).await
    }

    /// POST /dynamic-secret-update-gcp
    pub async fn dynamic_secret_update_gcp(
        &self,
        req: &DynamicSecretUpdateGcp,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-gcp", req).await
    }

    /// POST /dynamic-secret-update-github
    pub async fn dynamic_secret_update_github(
        &self,
        req: &DynamicSecretUpdateGithub,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-github", req).await
    }

    /// POST /dynamic-secret-update-gitlab
    pub async fn dynamic_secret_update_gitlab(
        &self,
        req: &DynamicSecretUpdateGitlab,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-gitlab", req).await
    }

    /// POST /dynamic-secret-update-gke
    pub async fn dynamic_secret_update_gke(
        &self,
        req: &DynamicSecretUpdateGke,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-gke", req).await
    }

    /// POST /dynamic-secret-update-google-workspace
    pub async fn dynamic_secret_update_google_workspace(
        &self,
        req: &DynamicSecretUpdateGoogleWorkspace,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-google-workspace", req).await
    }

    /// POST /dynamic-secret-update-hana
    pub async fn dynamic_secret_update_hana_db(
        &self,
        req: &DynamicSecretUpdateHanaDb,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-hana", req).await
    }

    /// POST /dynamic-secret-update-k8s
    pub async fn dynamic_secret_update_k8s(
        &self,
        req: &DynamicSecretUpdateK8s,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-k8s", req).await
    }

    /// POST /dynamic-secret-update-ldap
    pub async fn dynamic_secret_update_ldap(
        &self,
        req: &DynamicSecretUpdateLdap,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-ldap", req).await
    }

    /// POST /dynamic-secret-update-mongo
    pub async fn dynamic_secret_update_mongo_db(
        &self,
        req: &DynamicSecretUpdateMongoDb,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-mongo", req).await
    }

    /// POST /dynamic-secret-update-mssql
    pub async fn dynamic_secret_update_ms_sql(
        &self,
        req: &DynamicSecretUpdateMsSql,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-mssql", req).await
    }

    /// POST /dynamic-secret-update-mysql
    pub async fn dynamic_secret_update_my_sql(
        &self,
        req: &DynamicSecretUpdateMySql,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-mysql", req).await
    }

    /// POST /dynamic-secret-update-openai
    pub async fn dynamic_secret_update_open_ai(
        &self,
        req: &DynamicSecretUpdateOpenAi,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-openai", req).await
    }

    /// POST /dynamic-secret-update-oracle
    pub async fn dynamic_secret_update_oracle_db(
        &self,
        req: &DynamicSecretUpdateOracleDb,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-oracle", req).await
    }

    /// POST /dynamic-secret-update-ping
    pub async fn dynamic_secret_update_ping(
        &self,
        req: &DynamicSecretUpdatePing,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-ping", req).await
    }

    /// POST /dynamic-secret-update-postgresql
    pub async fn dynamic_secret_update_postgre_sql(
        &self,
        req: &DynamicSecretUpdatePostgreSql,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-postgresql", req).await
    }

    /// POST /dynamic-secret-update-rabbitmq
    pub async fn dynamic_secret_update_rabbit_mq(
        &self,
        req: &DynamicSecretUpdateRabbitMq,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-rabbitmq", req).await
    }

    /// POST /dynamic-secret-update-rdp
    pub async fn dynamic_secret_update_rdp(
        &self,
        req: &DynamicSecretUpdateRdp,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-rdp", req).await
    }

    /// POST /dynamic-secret-update-redis
    pub async fn dynamic_secret_update_redis(
        &self,
        req: &DynamicSecretUpdateRedis,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-redis", req).await
    }

    /// POST /dynamic-secret-update-redshift
    pub async fn dynamic_secret_update_redshift(
        &self,
        req: &DynamicSecretUpdateRedshift,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-redshift", req).await
    }

    /// POST /dynamic-secret-update-snowflake
    pub async fn dynamic_secret_update_snowflake(
        &self,
        req: &DynamicSecretUpdateSnowflake,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-snowflake", req).await
    }

    /// POST /dynamic-secret-update-venafi
    pub async fn dynamic_secret_update_venafi(
        &self,
        req: &DynamicSecretUpdateVenafi,
    ) -> Result<DynamicSecretUpdateOutput> {
        self.post("/dynamic-secret-update-venafi", req).await
    }

    /// POST /encrypt
    pub async fn encrypt(
        &self,
        req: &Encrypt,
    ) -> Result<EncryptOutput> {
        self.post("/encrypt", req).await
    }

    /// POST /encrypt-batch
    pub async fn encrypt_batch(
        &self,
        req: &BatchEncryptionRequest,
    ) -> Result<EncryptOutput> {
        self.post("/encrypt-batch", req).await
    }

    /// POST /encrypt-gpg
    pub async fn encrypt_gpg(
        &self,
        req: &EncryptGpg,
    ) -> Result<EncryptGpgOutput> {
        self.post("/encrypt-gpg", req).await
    }

    /// POST /encrypt-with-classic-key
    pub async fn encrypt_with_classic_key(
        &self,
        req: &EncryptWithClassicKey,
    ) -> Result<EncryptOutput> {
        self.post("/encrypt-with-classic-key", req).await
    }

    /// POST /esm-create
    pub async fn esm_create(
        &self,
        req: &EsmCreate,
    ) -> Result<EsmCreateSecretOutput> {
        self.post("/esm-create", req).await
    }

    /// POST /esm-delete
    pub async fn esm_delete(
        &self,
        req: &EsmDelete,
    ) -> Result<EsmDeleteSecretOutput> {
        self.post("/esm-delete", req).await
    }

    /// POST /esm-get
    pub async fn esm_get(
        &self,
        req: &EsmGet,
    ) -> Result<EsmGetSecretOutput> {
        self.post("/esm-get", req).await
    }

    /// POST /esm-list
    pub async fn esm_list(
        &self,
        req: &EsmList,
    ) -> Result<EsmListSecretsOutput> {
        self.post("/esm-list", req).await
    }

    /// POST /esm-update
    pub async fn esm_update(
        &self,
        req: &EsmUpdate,
    ) -> Result<EsmUpdateSecretOutput> {
        self.post("/esm-update", req).await
    }

    /// POST /event-action
    pub async fn event_action(
        &self,
        req: &EventAction,
    ) -> Result<EventActionOutput> {
        self.post("/event-action", req).await
    }

    /// POST /event-forwarder-create-email
    pub async fn event_forwarder_create_email(
        &self,
        req: &EventForwarderCreateEmail,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-create-email", req).await
    }

    /// POST /event-forwarder-create-servicenow
    pub async fn event_forwarder_create_service_now(
        &self,
        req: &EventForwarderCreateServiceNow,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-create-servicenow", req).await
    }

    /// POST /event-forwarder-create-slack
    pub async fn event_forwarder_create_slack(
        &self,
        req: &EventForwarderCreateSlack,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-create-slack", req).await
    }

    /// POST /event-forwarder-create-teams
    pub async fn event_forwarder_create_teams(
        &self,
        req: &EventForwarderCreateTeams,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-create-teams", req).await
    }

    /// POST /event-forwarder-create-webhook
    pub async fn event_forwarder_create_webhook(
        &self,
        req: &EventForwarderCreateWebhook,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-create-webhook", req).await
    }

    /// POST /event-forwarder-delete
    pub async fn event_forwarder_delete(
        &self,
        req: &EventForwarderDelete,
    ) -> Result<EventForwarderDeleteOutput> {
        self.post("/event-forwarder-delete", req).await
    }

    /// POST /event-forwarder-get
    pub async fn event_forwarder_get(
        &self,
        req: &EventForwarderGet,
    ) -> Result<EventForwarderGetOutput> {
        self.post("/event-forwarder-get", req).await
    }

    /// POST /event-forwarder-update-email
    pub async fn event_forwarder_update_email(
        &self,
        req: &EventForwarderUpdateEmail,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-update-email", req).await
    }

    /// POST /event-forwarder-update-servicenow
    pub async fn event_forwarder_update_service_now(
        &self,
        req: &EventForwarderUpdateServiceNow,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-update-servicenow", req).await
    }

    /// POST /event-forwarder-update-slack
    pub async fn event_forwarder_update_slack(
        &self,
        req: &EventForwarderUpdateSlack,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-update-slack", req).await
    }

    /// POST /event-forwarder-update-teams
    pub async fn event_forwarder_update_teams(
        &self,
        req: &EventForwarderUpdateTeams,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-update-teams", req).await
    }

    /// POST /event-forwarder-update-webhook
    pub async fn event_forwarder_update_webhook(
        &self,
        req: &EventForwarderUpdateWebhook,
    ) -> Result<EventForwarderCreateUpdateOutput> {
        self.post("/event-forwarder-update-webhook", req).await
    }

    /// POST /export-classic-key
    pub async fn export_classic_key(
        &self,
        req: &ExportClassicKey,
    ) -> Result<ExportClassicKeyOutput> {
        self.post("/export-classic-key", req).await
    }

    /// POST /folder-create
    pub async fn folder_create(
        &self,
        req: &FolderCreate,
    ) -> Result<FolderCreateOutput> {
        self.post("/folder-create", req).await
    }

    /// POST /folder-delete
    pub async fn folder_delete(
        &self,
        req: &FolderDelete,
    ) -> Result<FolderDeleteOutput> {
        self.post("/folder-delete", req).await
    }

    /// POST /folder-get
    pub async fn folder_get(
        &self,
        req: &FolderGet,
    ) -> Result<FolderGetOutput> {
        self.post("/folder-get", req).await
    }

    /// POST /folder-update
    pub async fn folder_update(
        &self,
        req: &FolderUpdate,
    ) -> Result<FolderUpdateOutput> {
        self.post("/folder-update", req).await
    }

    /// POST /gateway-create-allowed-access
    pub async fn gateway_create_allowed_access(
        &self,
        req: &GatewayCreateAllowedAccess,
    ) -> Result<AllowedAccess> {
        self.post("/gateway-create-allowed-access", req).await
    }

    /// POST /gateway-create-k8s-auth-config
    pub async fn gateway_create_k8s_auth_config(
        &self,
        req: &GatewayCreateK8sAuthConfig,
    ) -> Result<GatewayCreateK8sAuthConfigOutput> {
        self.post("/gateway-create-k8s-auth-config", req).await
    }

    /// POST /gateway-create-migration
    pub async fn gateway_create_migration(
        &self,
        req: &GatewayCreateMigration,
    ) -> Result<GatewayMigrationCreateOutput> {
        self.post("/gateway-create-migration", req).await
    }

    /// POST /gateway-create-producer-Redis
    pub async fn gateway_create_producer_redis(
        &self,
        req: &GatewayCreateProducerRedis,
    ) -> Result<GatewayCreateProducerRedisOutput> {
        self.post("/gateway-create-producer-Redis", req).await
    }

    /// POST /gateway-create-producer-artifactory
    pub async fn gateway_create_producer_artifactory(
        &self,
        req: &GatewayCreateProducerArtifactory,
    ) -> Result<GatewayCreateProducerArtifactoryOutput> {
        self.post("/gateway-create-producer-artifactory", req).await
    }

    /// POST /gateway-create-producer-aws
    pub async fn gateway_create_producer_aws(
        &self,
        req: &GatewayCreateProducerAws,
    ) -> Result<GatewayCreateProducerAwsOutput> {
        self.post("/gateway-create-producer-aws", req).await
    }

    /// POST /gateway-create-producer-azure
    pub async fn gateway_create_producer_azure(
        &self,
        req: &GatewayCreateProducerAzure,
    ) -> Result<GatewayCreateProducerAzureOutput> {
        self.post("/gateway-create-producer-azure", req).await
    }

    /// POST /gateway-create-producer-cassandra
    pub async fn gateway_create_producer_cassandra(
        &self,
        req: &GatewayCreateProducerCassandra,
    ) -> Result<GatewayCreateProducerCassandraOutput> {
        self.post("/gateway-create-producer-cassandra", req).await
    }

    /// POST /gateway-create-producer-certificate-automation
    pub async fn gateway_create_producer_venafi(
        &self,
        req: &GatewayCreateProducerVenafi,
    ) -> Result<GatewayCreateProducerVenafiOutput> {
        self.post("/gateway-create-producer-certificate-automation", req).await
    }

    /// POST /gateway-create-producer-chef
    pub async fn gateway_create_producer_chef(
        &self,
        req: &GatewayCreateProducerChef,
    ) -> Result<GatewayCreateProducerChefOutput> {
        self.post("/gateway-create-producer-chef", req).await
    }

    /// POST /gateway-create-producer-custom
    pub async fn gateway_create_producer_custom(
        &self,
        req: &GatewayCreateProducerCustom,
    ) -> Result<GatewayCreateProducerCustomOutput> {
        self.post("/gateway-create-producer-custom", req).await
    }

    /// POST /gateway-create-producer-dockerhub
    pub async fn gateway_create_producer_dockerhub(
        &self,
        req: &GatewayCreateProducerDockerhub,
    ) -> Result<GatewayCreateProducerDockerhubOutput> {
        self.post("/gateway-create-producer-dockerhub", req).await
    }

    /// POST /gateway-create-producer-eks
    pub async fn gateway_create_producer_eks(
        &self,
        req: &GatewayCreateProducerEks,
    ) -> Result<GatewayCreateProducerEksOutput> {
        self.post("/gateway-create-producer-eks", req).await
    }

    /// POST /gateway-create-producer-gcp
    pub async fn gateway_create_producer_gcp(
        &self,
        req: &GatewayCreateProducerGcp,
    ) -> Result<GatewayCreateProducerGcpOutput> {
        self.post("/gateway-create-producer-gcp", req).await
    }

    /// POST /gateway-create-producer-github
    pub async fn gateway_create_producer_github(
        &self,
        req: &GatewayCreateProducerGithub,
    ) -> Result<GatewayCreateProducerGithubOutput> {
        self.post("/gateway-create-producer-github", req).await
    }

    /// POST /gateway-create-producer-gke
    pub async fn gateway_create_producer_gke(
        &self,
        req: &GatewayCreateProducerGke,
    ) -> Result<GatewayCreateProducerGkeOutput> {
        self.post("/gateway-create-producer-gke", req).await
    }

    /// POST /gateway-create-producer-hanadb
    pub async fn gateway_create_producer_hana_db(
        &self,
        req: &GatewayCreateProducerHanaDb,
    ) -> Result<GatewayCreateProducerHanaDbOutput> {
        self.post("/gateway-create-producer-hanadb", req).await
    }

    /// POST /gateway-create-producer-k8s
    pub async fn gateway_create_producer_native_k8s(
        &self,
        req: &GatewayCreateProducerNativeK8s,
    ) -> Result<GatewayCreateProducerNativeK8sOutput> {
        self.post("/gateway-create-producer-k8s", req).await
    }

    /// POST /gateway-create-producer-ldap
    pub async fn gateway_create_producer_ldap(
        &self,
        req: &GatewayCreateProducerLdap,
    ) -> Result<GatewayCreateProducerLdapOutput> {
        self.post("/gateway-create-producer-ldap", req).await
    }

    /// POST /gateway-create-producer-mongo
    pub async fn gateway_create_producer_mongo(
        &self,
        req: &GatewayCreateProducerMongo,
    ) -> Result<GatewayCreateProducerMongoOutput> {
        self.post("/gateway-create-producer-mongo", req).await
    }

    /// POST /gateway-create-producer-mssql
    pub async fn gateway_create_producer_mssql(
        &self,
        req: &GatewayCreateProducerMssql,
    ) -> Result<GatewayCreateProducerMssqlOutput> {
        self.post("/gateway-create-producer-mssql", req).await
    }

    /// POST /gateway-create-producer-mysql
    pub async fn gateway_create_producer_my_sql(
        &self,
        req: &GatewayCreateProducerMySql,
    ) -> Result<GatewayCreateProducerMySqlOutput> {
        self.post("/gateway-create-producer-mysql", req).await
    }

    /// POST /gateway-create-producer-oracle
    pub async fn gateway_create_producer_oracle_db(
        &self,
        req: &GatewayCreateProducerOracleDb,
    ) -> Result<GatewayCreateProducerOracleDbOutput> {
        self.post("/gateway-create-producer-oracle", req).await
    }

    /// POST /gateway-create-producer-ping
    pub async fn gateway_create_producer_ping(
        &self,
        req: &GatewayCreateProducerPing,
    ) -> Result<GatewayCreateProducerPingOutput> {
        self.post("/gateway-create-producer-ping", req).await
    }

    /// POST /gateway-create-producer-postgresql
    pub async fn gateway_create_producer_postgre_sql(
        &self,
        req: &GatewayCreateProducerPostgreSql,
    ) -> Result<GatewayCreateProducerPostgreSqlOutput> {
        self.post("/gateway-create-producer-postgresql", req).await
    }

    /// POST /gateway-create-producer-rabbitmq
    pub async fn gateway_create_producer_rabbit_mq(
        &self,
        req: &GatewayCreateProducerRabbitMq,
    ) -> Result<GatewayCreateProducerRabbitMqOutput> {
        self.post("/gateway-create-producer-rabbitmq", req).await
    }

    /// POST /gateway-create-producer-rdp
    pub async fn gateway_create_producer_rdp(
        &self,
        req: &GatewayCreateProducerRdp,
    ) -> Result<GatewayCreateProducerRdpOutput> {
        self.post("/gateway-create-producer-rdp", req).await
    }

    /// POST /gateway-create-producer-redshift
    pub async fn gateway_create_producer_redshift(
        &self,
        req: &GatewayCreateProducerRedshift,
    ) -> Result<GatewayCreateProducerRedshiftOutput> {
        self.post("/gateway-create-producer-redshift", req).await
    }

    /// POST /gateway-create-producer-snowflake
    pub async fn gateway_create_producer_snowflake(
        &self,
        req: &GatewayCreateProducerSnowflake,
    ) -> Result<GatewayCreateProducerSnowflakeOutput> {
        self.post("/gateway-create-producer-snowflake", req).await
    }

    /// POST /gateway-delete-allowed-access
    pub async fn gateway_delete_allowed_access(
        &self,
        req: &GatewayDeleteAllowedAccess,
    ) -> Result<GatewayDeleteAllowedAccessOutput> {
        self.post("/gateway-delete-allowed-access", req).await
    }

    /// POST /gateway-delete-allowed-management-access
    pub async fn delete_gateway_allowed_access_id(
        &self,
        req: &DeleteGatewayAllowedAccessId,
    ) -> Result<DeleteGatewayAllowedAccessIdOutput> {
        self.post("/gateway-delete-allowed-management-access", req).await
    }

    /// POST /gateway-delete-k8s-auth-config
    pub async fn gateway_delete_k8s_auth_config(
        &self,
        req: &GatewayDeleteK8sAuthConfig,
    ) -> Result<GatewayDeleteK8sAuthConfigOutput> {
        self.post("/gateway-delete-k8s-auth-config", req).await
    }

    /// POST /gateway-delete-migration
    pub async fn gateway_delete_migration(
        &self,
        req: &GatewayDeleteMigration,
    ) -> Result<GatewayMigrationDeleteOutput> {
        self.post("/gateway-delete-migration", req).await
    }

    /// POST /gateway-delete-producer
    pub async fn gateway_delete_producer(
        &self,
        req: &GatewayDeleteProducer,
    ) -> Result<GatewayDeleteProducerOutput> {
        self.post("/gateway-delete-producer", req).await
    }

    /// POST /gateway-download-customer-fragments
    pub async fn gateway_download_customer_fragments(
        &self,
        req: &GatewayDownloadCustomerFragments,
    ) -> Result<GatewayDownloadCustomerFragmentsOutput> {
        self.post("/gateway-download-customer-fragments", req).await
    }

    /// POST /gateway-get-allowed-access
    pub async fn gateway_get_allowed_access(
        &self,
        req: &GatewayGetAllowedAccess,
    ) -> Result<AllowedAccess> {
        self.post("/gateway-get-allowed-access", req).await
    }

    /// POST /gateway-get-cache
    pub async fn gateway_get_cache(
        &self,
        req: &GatewayGetCache,
    ) -> Result<CacheConfigPart> {
        self.post("/gateway-get-cache", req).await
    }

    /// POST /gateway-get-config
    pub async fn gateway_get_config(
        &self,
        req: &GatewayGetConfig,
    ) -> Result<AkeylessGatewayConfig> {
        self.post("/gateway-get-config", req).await
    }

    /// POST /gateway-get-defaults
    pub async fn gateway_get_defaults(
        &self,
        req: &GatewayGetDefaults,
    ) -> Result<GatewayGetDefaultsOutput> {
        self.post("/gateway-get-defaults", req).await
    }

    /// POST /gateway-get-k8s-auth-config
    pub async fn gateway_get_k8s_auth_config(
        &self,
        req: &GatewayGetK8sAuthConfig,
    ) -> Result<GatewayGetK8sAuthConfigOutput> {
        self.post("/gateway-get-k8s-auth-config", req).await
    }

    /// POST /gateway-get-ldap-auth-config
    pub async fn gateway_get_ldap_auth_config(
        &self,
        req: &GatewayGetLdapAuthConfig,
    ) -> Result<GatewayGetLdapAuthConfigOutput> {
        self.post("/gateway-get-ldap-auth-config", req).await
    }

    /// POST /gateway-get-log-forwarding
    pub async fn gateway_get_log_forwarding(
        &self,
        req: &GatewayGetLogForwarding,
    ) -> Result<LogForwardingConfigPart> {
        self.post("/gateway-get-log-forwarding", req).await
    }

    /// POST /gateway-get-migration
    pub async fn gateway_get_migration(
        &self,
        req: &GatewayGetMigration,
    ) -> Result<GatewayMigrationGetOutput> {
        self.post("/gateway-get-migration", req).await
    }

    /// POST /gateway-get-producer
    pub async fn gateway_get_producer(
        &self,
        req: &GatewayGetProducer,
    ) -> Result<DsProducerDetails> {
        self.post("/gateway-get-producer", req).await
    }

    /// POST /gateway-get-producer-tmp-creds
    pub async fn gateway_get_tmp_users(
        &self,
        req: &GatewayGetTmpUsers,
    ) -> Result<Vec<TmpUserData>> {
        self.post("/gateway-get-producer-tmp-creds", req).await
    }

    /// POST /gateway-get-remote-access
    pub async fn gateway_get_remote_access(
        &self,
        req: &GatewayGetRemoteAccess,
    ) -> Result<BastionConfigReplyObj> {
        self.post("/gateway-get-remote-access", req).await
    }

    /// POST /gateway-list-customer-fragments
    pub async fn gateway_list_customer_fragments(
        &self,
        req: &GatewayListCustomerFragments,
    ) -> Result<serde_json::Value> {
        self.post("/gateway-list-customer-fragments", req).await
    }

    /// POST /gateway-list-migration
    pub async fn gateway_list_migration(
        &self,
        req: &GatewayListMigration,
    ) -> Result<GatewayMigrationListOutput> {
        self.post("/gateway-list-migration", req).await
    }

    /// POST /gateway-list-producers
    pub async fn gateway_list_producers(
        &self,
        req: &GatewayListProducers,
    ) -> Result<GetProducersListReplyObj> {
        self.post("/gateway-list-producers", req).await
    }

    /// POST /gateway-list-rotated-secrets
    pub async fn gateway_list_rotated_secrets(
        &self,
        req: &GatewayListRotatedSecrets,
    ) -> Result<ListItemsOutput> {
        self.post("/gateway-list-rotated-secrets", req).await
    }

    /// POST /gateway-migrate-personal-items
    pub async fn gateway_migrate_personal_items(
        &self,
        req: &GatewayMigratePersonalItems,
    ) -> Result<GatewayMigratePersonalItemsOutput> {
        self.post("/gateway-migrate-personal-items", req).await
    }

    /// POST /gateway-migration-status
    pub async fn gateway_status_migration(
        &self,
        req: &GatewayStatusMigration,
    ) -> Result<MigrationStatusReplyObj> {
        self.post("/gateway-migration-status", req).await
    }

    /// POST /gateway-revoke-producer-tmp-creds
    pub async fn gateway_revoke_tmp_users(
        &self,
        req: &GatewayRevokeTmpUsers,
    ) -> Result<JsonError> {
        self.post("/gateway-revoke-producer-tmp-creds", req).await
    }

    /// POST /gateway-rotate-secret
    pub async fn rotate_secret(
        &self,
        req: &RotateSecret,
    ) -> Result<RotatedSecretOutput> {
        self.post("/gateway-rotate-secret", req).await
    }

    /// POST /gateway-start-producer
    pub async fn gateway_start_producer(
        &self,
        req: &GatewayStartProducer,
    ) -> Result<GatewayStartProducerOutput> {
        self.post("/gateway-start-producer", req).await
    }

    /// POST /gateway-stop-producer
    pub async fn gateway_stop_producer(
        &self,
        req: &GatewayStopProducer,
    ) -> Result<GatewayStopProducerOutput> {
        self.post("/gateway-stop-producer", req).await
    }

    /// POST /gateway-sync-migration
    pub async fn gateway_sync_migration(
        &self,
        req: &GatewaySyncMigration,
    ) -> Result<GatewayMigrationSyncOutput> {
        self.post("/gateway-sync-migration", req).await
    }

    /// POST /gateway-update-allowed-access
    pub async fn gateway_update_allowed_access(
        &self,
        req: &GatewayUpdateAllowedAccess,
    ) -> Result<AllowedAccess> {
        self.post("/gateway-update-allowed-access", req).await
    }

    /// POST /gateway-update-cache
    pub async fn gateway_update_cache(
        &self,
        req: &GatewayUpdateCache,
    ) -> Result<GatewayUpdateOutput> {
        self.post("/gateway-update-cache", req).await
    }

    /// POST /gateway-update-defaults
    pub async fn gateway_update_defaults(
        &self,
        req: &GatewayUpdateDefaults,
    ) -> Result<GatewayUpdateOutput> {
        self.post("/gateway-update-defaults", req).await
    }

    /// POST /gateway-update-item
    pub async fn gateway_update_item(
        &self,
        req: &GatewayUpdateItem,
    ) -> Result<GatewayUpdateItemOutput> {
        self.post("/gateway-update-item", req).await
    }

    /// POST /gateway-update-k8s-auth-config
    pub async fn gateway_update_k8s_auth_config(
        &self,
        req: &GatewayUpdateK8sAuthConfig,
    ) -> Result<GatewayUpdateK8sAuthConfigOutput> {
        self.post("/gateway-update-k8s-auth-config", req).await
    }

    /// POST /gateway-update-ldap-auth-config
    pub async fn gateway_update_ldap_auth_config(
        &self,
        req: &GatewayUpdateLdapAuthConfig,
    ) -> Result<GatewayUpdateLdapAuthConfigOutput> {
        self.post("/gateway-update-ldap-auth-config", req).await
    }

    /// POST /gateway-update-log-forwarding-aws-s3
    pub async fn gateway_update_log_forwarding_aws_s3(
        &self,
        req: &GatewayUpdateLogForwardingAwsS3,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-aws-s3", req).await
    }

    /// POST /gateway-update-log-forwarding-azure-analytics
    pub async fn gateway_update_log_forwarding_azure_analytics(
        &self,
        req: &GatewayUpdateLogForwardingAzureAnalytics,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-azure-analytics", req).await
    }

    /// POST /gateway-update-log-forwarding-datadog
    pub async fn gateway_update_log_forwarding_datadog(
        &self,
        req: &GatewayUpdateLogForwardingDatadog,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-datadog", req).await
    }

    /// POST /gateway-update-log-forwarding-elasticsearch
    pub async fn gateway_update_log_forwarding_elasticsearch(
        &self,
        req: &GatewayUpdateLogForwardingElasticsearch,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-elasticsearch", req).await
    }

    /// POST /gateway-update-log-forwarding-google-chronicle
    pub async fn gateway_update_log_forwarding_google_chronicle(
        &self,
        req: &GatewayUpdateLogForwardingGoogleChronicle,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-google-chronicle", req).await
    }

    /// POST /gateway-update-log-forwarding-logstash
    pub async fn gateway_update_log_forwarding_logstash(
        &self,
        req: &GatewayUpdateLogForwardingLogstash,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-logstash", req).await
    }

    /// POST /gateway-update-log-forwarding-logz-io
    pub async fn gateway_update_log_forwarding_logz_io(
        &self,
        req: &GatewayUpdateLogForwardingLogzIo,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-logz-io", req).await
    }

    /// POST /gateway-update-log-forwarding-splunk
    pub async fn gateway_update_log_forwarding_splunk(
        &self,
        req: &GatewayUpdateLogForwardingSplunk,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-splunk", req).await
    }

    /// POST /gateway-update-log-forwarding-stdout
    pub async fn gateway_update_log_forwarding_stdout(
        &self,
        req: &GatewayUpdateLogForwardingStdout,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-stdout", req).await
    }

    /// POST /gateway-update-log-forwarding-sumologic
    pub async fn gateway_update_log_forwarding_sumologic(
        &self,
        req: &GatewayUpdateLogForwardingSumologic,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-sumologic", req).await
    }

    /// POST /gateway-update-log-forwarding-syslog
    pub async fn gateway_update_log_forwarding_syslog(
        &self,
        req: &GatewayUpdateLogForwardingSyslog,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-log-forwarding-syslog", req).await
    }

    /// POST /gateway-update-migration
    pub async fn gateway_update_migration(
        &self,
        req: &GatewayUpdateMigration,
    ) -> Result<GatewayMigrationUpdateOutput> {
        self.post("/gateway-update-migration", req).await
    }

    /// POST /gateway-update-producer-artifactory
    pub async fn gateway_update_producer_artifactory(
        &self,
        req: &GatewayUpdateProducerArtifactory,
    ) -> Result<GatewayUpdateProducerArtifactoryOutput> {
        self.post("/gateway-update-producer-artifactory", req).await
    }

    /// POST /gateway-update-producer-aws
    pub async fn gateway_update_producer_aws(
        &self,
        req: &GatewayUpdateProducerAws,
    ) -> Result<GatewayUpdateProducerAwsOutput> {
        self.post("/gateway-update-producer-aws", req).await
    }

    /// POST /gateway-update-producer-azure
    pub async fn gateway_update_producer_azure(
        &self,
        req: &GatewayUpdateProducerAzure,
    ) -> Result<GatewayUpdateProducerAzureOutput> {
        self.post("/gateway-update-producer-azure", req).await
    }

    /// POST /gateway-update-producer-cassandra
    pub async fn gateway_update_producer_cassandra(
        &self,
        req: &GatewayUpdateProducerCassandra,
    ) -> Result<GatewayUpdateProducerCassandraOutput> {
        self.post("/gateway-update-producer-cassandra", req).await
    }

    /// POST /gateway-update-producer-certificate-automation
    pub async fn gateway_update_producer_venafi(
        &self,
        req: &GatewayUpdateProducerVenafi,
    ) -> Result<GatewayUpdateProducerVenafiOutput> {
        self.post("/gateway-update-producer-certificate-automation", req).await
    }

    /// POST /gateway-update-producer-chef
    pub async fn gateway_update_producer_chef(
        &self,
        req: &GatewayUpdateProducerChef,
    ) -> Result<GatewayUpdateProducerChefOutput> {
        self.post("/gateway-update-producer-chef", req).await
    }

    /// POST /gateway-update-producer-custom
    pub async fn gateway_update_producer_custom(
        &self,
        req: &GatewayUpdateProducerCustom,
    ) -> Result<GatewayUpdateProducerCustomOutput> {
        self.post("/gateway-update-producer-custom", req).await
    }

    /// POST /gateway-update-producer-dockerhub
    pub async fn gateway_update_producer_dockerhub(
        &self,
        req: &GatewayUpdateProducerDockerhub,
    ) -> Result<GatewayUpdateProducerDockerhubOutput> {
        self.post("/gateway-update-producer-dockerhub", req).await
    }

    /// POST /gateway-update-producer-eks
    pub async fn gateway_update_producer_eks(
        &self,
        req: &GatewayUpdateProducerEks,
    ) -> Result<GatewayUpdateProducerEksOutput> {
        self.post("/gateway-update-producer-eks", req).await
    }

    /// POST /gateway-update-producer-gcp
    pub async fn gateway_update_producer_gcp(
        &self,
        req: &GatewayUpdateProducerGcp,
    ) -> Result<GatewayUpdateProducerGcpOutput> {
        self.post("/gateway-update-producer-gcp", req).await
    }

    /// POST /gateway-update-producer-github
    pub async fn gateway_update_producer_github(
        &self,
        req: &GatewayUpdateProducerGithub,
    ) -> Result<GatewayUpdateProducerGithubOutput> {
        self.post("/gateway-update-producer-github", req).await
    }

    /// POST /gateway-update-producer-gke
    pub async fn gateway_update_producer_gke(
        &self,
        req: &GatewayUpdateProducerGke,
    ) -> Result<GatewayUpdateProducerGkeOutput> {
        self.post("/gateway-update-producer-gke", req).await
    }

    /// POST /gateway-update-producer-hana
    pub async fn gateway_update_producer_hana_db(
        &self,
        req: &GatewayUpdateProducerHanaDb,
    ) -> Result<GatewayUpdateProducerHanaDbOutput> {
        self.post("/gateway-update-producer-hana", req).await
    }

    /// POST /gateway-update-producer-k8s
    pub async fn gateway_update_producer_native_k8s(
        &self,
        req: &GatewayUpdateProducerNativeK8s,
    ) -> Result<GatewayUpdateProducerNativeK8sOutput> {
        self.post("/gateway-update-producer-k8s", req).await
    }

    /// POST /gateway-update-producer-ldap
    pub async fn gateway_update_producer_ldap(
        &self,
        req: &GatewayUpdateProducerLdap,
    ) -> Result<GatewayUpdateProducerLdapOutput> {
        self.post("/gateway-update-producer-ldap", req).await
    }

    /// POST /gateway-update-producer-mongo
    pub async fn gateway_update_producer_mongo(
        &self,
        req: &GatewayUpdateProducerMongo,
    ) -> Result<GatewayUpdateProducerMongoOutput> {
        self.post("/gateway-update-producer-mongo", req).await
    }

    /// POST /gateway-update-producer-mssql
    pub async fn gateway_update_producer_mssql(
        &self,
        req: &GatewayUpdateProducerMssql,
    ) -> Result<GatewayUpdateProducerMssqlOutput> {
        self.post("/gateway-update-producer-mssql", req).await
    }

    /// POST /gateway-update-producer-mysql
    pub async fn gateway_update_producer_my_sql(
        &self,
        req: &GatewayUpdateProducerMySql,
    ) -> Result<GatewayUpdateProducerMySqlOutput> {
        self.post("/gateway-update-producer-mysql", req).await
    }

    /// POST /gateway-update-producer-oracle
    pub async fn gateway_update_producer_oracle_db(
        &self,
        req: &GatewayUpdateProducerOracleDb,
    ) -> Result<GatewayUpdateProducerOracleDbOutput> {
        self.post("/gateway-update-producer-oracle", req).await
    }

    /// POST /gateway-update-producer-ping
    pub async fn gateway_update_producer_ping(
        &self,
        req: &GatewayUpdateProducerPing,
    ) -> Result<GatewayUpdateProducerPingOutput> {
        self.post("/gateway-update-producer-ping", req).await
    }

    /// POST /gateway-update-producer-postgresql
    pub async fn gateway_update_producer_postgre_sql(
        &self,
        req: &GatewayUpdateProducerPostgreSql,
    ) -> Result<GatewayUpdateProducerPostgreSqlOutput> {
        self.post("/gateway-update-producer-postgresql", req).await
    }

    /// POST /gateway-update-producer-rabbitmq
    pub async fn gateway_update_producer_rabbit_mq(
        &self,
        req: &GatewayUpdateProducerRabbitMq,
    ) -> Result<GatewayUpdateProducerRabbitMqOutput> {
        self.post("/gateway-update-producer-rabbitmq", req).await
    }

    /// POST /gateway-update-producer-rdp
    pub async fn gateway_update_producer_rdp(
        &self,
        req: &GatewayUpdateProducerRdp,
    ) -> Result<GatewayUpdateProducerRdpOutput> {
        self.post("/gateway-update-producer-rdp", req).await
    }

    /// POST /gateway-update-producer-redis
    pub async fn gateway_update_producer_redis(
        &self,
        req: &GatewayUpdateProducerRedis,
    ) -> Result<GatewayUpdateProducerRedisOutput> {
        self.post("/gateway-update-producer-redis", req).await
    }

    /// POST /gateway-update-producer-redshift
    pub async fn gateway_update_producer_redshift(
        &self,
        req: &GatewayUpdateProducerRedshift,
    ) -> Result<GatewayUpdateProducerRedshiftOutput> {
        self.post("/gateway-update-producer-redshift", req).await
    }

    /// POST /gateway-update-producer-snowflake
    pub async fn gateway_update_producer_snowflake(
        &self,
        req: &GatewayUpdateProducerSnowflake,
    ) -> Result<GatewayUpdateProducerSnowflakeOutput> {
        self.post("/gateway-update-producer-snowflake", req).await
    }

    /// POST /gateway-update-producer-tmp-creds
    pub async fn gateway_update_tmp_users(
        &self,
        req: &GatewayUpdateTmpUsers,
    ) -> Result<JsonError> {
        self.post("/gateway-update-producer-tmp-creds", req).await
    }

    /// POST /gateway-update-remote-access
    pub async fn gateway_update_remote_access(
        &self,
        req: &GatewayUpdateRemoteAccess,
    ) -> Result<GatewayUpdateRemoteAccessOutput> {
        self.post("/gateway-update-remote-access", req).await
    }

    /// POST /gateway-update-remote-access-desktop-app
    pub async fn gateway_update_remote_access_desktop_app(
        &self,
        req: &GatewayUpdateRemoteAccessDesktopApp,
    ) -> Result<GatewayUpdateRemoteAccessDesktopAppOutput> {
        self.post("/gateway-update-remote-access-desktop-app", req).await
    }

    /// POST /gateway-update-remote-access-rdp-recording
    pub async fn gateway_update_remote_access_rdp_recordings(
        &self,
        req: &GatewayUpdateRemoteAccessRdpRecordings,
    ) -> Result<GatewayUpdateRemoteAccessRdpRecordingsOutput> {
        self.post("/gateway-update-remote-access-rdp-recording", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-aws-s3
    pub async fn gw_update_remote_access_session_logs_aws_s3(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsAwsS3,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-aws-s3", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-azure-analytics
    pub async fn gw_update_remote_access_session_logs_azure_analytics(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsAzureAnalytics,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-azure-analytics", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-datadog
    pub async fn gw_update_remote_access_session_logs_datadog(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsDatadog,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-datadog", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-elasticsearch
    pub async fn gw_update_remote_access_session_logs_elasticsearch(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsElasticsearch,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-elasticsearch", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-google-chronicle
    pub async fn gw_update_remote_access_session_logs_google_chronicle(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsGoogleChronicle,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-google-chronicle", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-logstash
    pub async fn gw_update_remote_access_session_logs_logstash(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsLogstash,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-logstash", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-logz-io
    pub async fn gw_update_remote_access_session_logs_logz_io(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsLogzIo,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-logz-io", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-splunk
    pub async fn gw_update_remote_access_session_logs_splunk(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsSplunk,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-splunk", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-stdout
    pub async fn gw_update_remote_access_session_logs_stdout(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsStdout,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-stdout", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-sumologic
    pub async fn gw_update_remote_access_session_logs_sumologic(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsSumologic,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-sumologic", req).await
    }

    /// POST /gateway-update-remote-access-session-forwarding-syslog
    pub async fn gw_update_remote_access_session_logs_syslog(
        &self,
        req: &GwUpdateRemoteAccessSessionLogsSyslog,
    ) -> Result<GatewayUpdateLogForwardingOutput> {
        self.post("/gateway-update-remote-access-session-forwarding-syslog", req).await
    }

    /// POST /gateway-update-tls-cert
    pub async fn gateway_update_tls_cert(
        &self,
        req: &GatewayUpdateTlsCert,
    ) -> Result<GatewayUpdateTlsCertOutput> {
        self.post("/gateway-update-tls-cert", req).await
    }

    /// POST /generate-acme-eab
    pub async fn generate_acme_eab(
        &self,
        req: &GenerateAcmeEab,
    ) -> Result<GenerateAcmeEabOutput> {
        self.post("/generate-acme-eab", req).await
    }

    /// POST /generate-ca
    pub async fn generate_ca(
        &self,
        req: &GenerateCa,
    ) -> Result<GenerateCaOutput> {
        self.post("/generate-ca", req).await
    }

    /// POST /generate-csr
    pub async fn generate_csr(
        &self,
        req: &GenerateCsr,
    ) -> Result<GenerateCsrOutput> {
        self.post("/generate-csr", req).await
    }

    /// POST /get-account-logo
    pub async fn get_account_logo(
        &self,
    ) -> Result<serde_json::Value> {
        self.post_empty("/get-account-logo").await
    }

    /// POST /get-account-settings
    pub async fn get_account_settings(
        &self,
        req: &GetAccountSettings,
    ) -> Result<GetAccountSettingsCommandOutput> {
        self.post("/get-account-settings", req).await
    }

    /// POST /get-analytics-data
    pub async fn get_analytics_data(
        &self,
        req: &GetAnalyticsData,
    ) -> Result<AllAnalyticsData> {
        self.post("/get-analytics-data", req).await
    }

    /// POST /get-auth-method
    pub async fn get_auth_method(
        &self,
        req: &GetAuthMethod,
    ) -> Result<AuthMethod> {
        self.post("/get-auth-method", req).await
    }

    /// POST /get-cert-challenge
    pub async fn get_cert_challenge(
        &self,
        req: &GetCertChallenge,
    ) -> Result<GetCertChallengeOutput> {
        self.post("/get-cert-challenge", req).await
    }

    /// POST /get-certificate-value
    pub async fn get_certificate_value(
        &self,
        req: &GetCertificateValue,
    ) -> Result<GetCertificateValueOutput> {
        self.post("/get-certificate-value", req).await
    }

    /// POST /get-dynamic-secret-value
    pub async fn get_dynamic_secret_value(
        &self,
        req: &GetDynamicSecretValue,
    ) -> Result<serde_json::Value> {
        self.post("/get-dynamic-secret-value", req).await
    }

    /// POST /get-event-forwarder
    pub async fn get_event_forwarder(
        &self,
        req: &GetEventForwarder,
    ) -> Result<GetEventForwarderOutput> {
        self.post("/get-event-forwarder", req).await
    }

    /// POST /get-group
    pub async fn get_group(
        &self,
        req: &GetGroup,
    ) -> Result<GetGroupOutput> {
        self.post("/get-group", req).await
    }

    /// POST /get-kube-exec-creds
    pub async fn get_kube_exec_creds(
        &self,
        req: &GetKubeExecCreds,
    ) -> Result<GetKubeExecCredsOutput> {
        self.post("/get-kube-exec-creds", req).await
    }

    /// POST /get-pki-certificate
    pub async fn get_pki_certificate(
        &self,
        req: &GetPkiCertificate,
    ) -> Result<GetPkiCertificateOutput> {
        self.post("/get-pki-certificate", req).await
    }

    /// POST /get-role
    pub async fn get_role(
        &self,
        req: &GetRole,
    ) -> Result<Role> {
        self.post("/get-role", req).await
    }

    /// POST /get-rotated-secret-value
    pub async fn get_rotated_secret_value(
        &self,
        req: &GetRotatedSecretValue,
    ) -> Result<serde_json::Value> {
        self.post("/get-rotated-secret-value", req).await
    }

    /// POST /get-rsa-public
    pub async fn get_rsa_public(
        &self,
        req: &GetRsaPublic,
    ) -> Result<GetRsaPublicOutput> {
        self.post("/get-rsa-public", req).await
    }

    /// POST /get-secret-value
    pub async fn get_secret_value(
        &self,
        req: &GetSecretValue,
    ) -> Result<serde_json::Value> {
        self.post("/get-secret-value", req).await
    }

    /// POST /get-ssh-certificate
    pub async fn get_ssh_certificate(
        &self,
        req: &GetSshCertificate,
    ) -> Result<GetSshCertificateOutput> {
        self.post("/get-ssh-certificate", req).await
    }

    /// POST /get-tags
    pub async fn get_tags(
        &self,
        req: &GetTags,
    ) -> Result<Vec<String>> {
        self.post("/get-tags", req).await
    }

    /// POST /get-target
    pub async fn get_target(
        &self,
        req: &GetTarget,
    ) -> Result<Target> {
        self.post("/get-target", req).await
    }

    /// POST /get-target-details
    pub async fn get_target_details(
        &self,
        req: &GetTargetDetails,
    ) -> Result<GetTargetDetailsOutput> {
        self.post("/get-target-details", req).await
    }

    /// POST /hmac
    pub async fn hmac(
        &self,
        req: &Hmac,
    ) -> Result<HmacOutput> {
        self.post("/hmac", req).await
    }

    /// POST /import-passwords
    pub async fn import_passwords(
        &self,
        req: &ImportPasswords,
    ) -> Result<ImportPasswordsOutput> {
        self.post("/import-passwords", req).await
    }

    /// POST /kmip-client-delete-rule
    pub async fn kmip_client_delete_rule(
        &self,
        req: &KmipClientDeleteRule,
    ) -> Result<KmipClientUpdateResponse> {
        self.post("/kmip-client-delete-rule", req).await
    }

    /// POST /kmip-client-set-rule
    pub async fn kmip_client_set_rule(
        &self,
        req: &KmipClientSetRule,
    ) -> Result<KmipClientUpdateResponse> {
        self.post("/kmip-client-set-rule", req).await
    }

    /// POST /kmip-create-client
    pub async fn kmip_create_client(
        &self,
        req: &KmipCreateClient,
    ) -> Result<KmipCreateClientOutput> {
        self.post("/kmip-create-client", req).await
    }

    /// POST /kmip-create-environment
    pub async fn kmip_server_setup(
        &self,
        req: &KmipServerSetup,
    ) -> Result<KmipEnvironmentCreateResponse> {
        self.post("/kmip-create-environment", req).await
    }

    /// POST /kmip-delete-client
    pub async fn kmip_delete_client(
        &self,
        req: &KmipDeleteClient,
    ) -> Result<KmipDeleteClientOutput> {
        self.post("/kmip-delete-client", req).await
    }

    /// DELETE /kmip-delete-environment
    pub async fn kmip_delete_server(
        &self,
        req: &KmipDeleteServer,
    ) -> Result<KmipDeleteServerOutput> {
        self.delete("/kmip-delete-environment", req).await
    }

    /// POST /kmip-get-client
    pub async fn kmip_describe_client(
        &self,
        req: &KmipDescribeClient,
    ) -> Result<KmipClientGetResponse> {
        self.post("/kmip-get-client", req).await
    }

    /// POST /kmip-get-environment
    pub async fn kmip_describe_server(
        &self,
        req: &KmipDescribeServer,
    ) -> Result<KmipDescribeServerOutput> {
        self.post("/kmip-get-environment", req).await
    }

    /// POST /kmip-list-clients
    pub async fn kmip_list_clients(
        &self,
        req: &KmipListClients,
    ) -> Result<KmipClientListResponse> {
        self.post("/kmip-list-clients", req).await
    }

    /// POST /kmip-move-environment
    pub async fn kmip_move_server(
        &self,
        req: &KmipMoveServer,
    ) -> Result<KmipMoveServerOutput> {
        self.post("/kmip-move-environment", req).await
    }

    /// POST /kmip-renew-client
    pub async fn kmip_renew_client_certificate(
        &self,
        req: &KmipRenewClientCertificate,
    ) -> Result<KmipRenewClientCertificateOutput> {
        self.post("/kmip-renew-client", req).await
    }

    /// POST /kmip-renew-environment
    pub async fn kmip_renew_server_certificate(
        &self,
        req: &KmipRenewServerCertificate,
    ) -> Result<KmipRenewServerCertificateOutput> {
        self.post("/kmip-renew-environment", req).await
    }

    /// POST /kmip-set-environment-state
    pub async fn kmip_set_server_state(
        &self,
        req: &KmipSetServerState,
    ) -> Result<KmipSetServerStateOutput> {
        self.post("/kmip-set-environment-state", req).await
    }

    /// POST /kubeconfig-generate
    pub async fn kubeconfig_generate(
        &self,
    ) -> Result<KubeconfigGenerateOutput> {
        self.post_empty("/kubeconfig-generate").await
    }

    /// POST /list-acme-accounts
    pub async fn list_acme_accounts(
        &self,
        req: &ListAcmeAccounts,
    ) -> Result<ListAcmeAccountsOutput> {
        self.post("/list-acme-accounts", req).await
    }

    /// POST /list-auth-methods
    pub async fn list_auth_methods(
        &self,
        req: &ListAuthMethods,
    ) -> Result<ListAuthMethodsOutput> {
        self.post("/list-auth-methods", req).await
    }

    /// POST /list-gateways
    pub async fn list_gateways(
        &self,
        req: &ListGateways,
    ) -> Result<GatewaysListResponse> {
        self.post("/list-gateways", req).await
    }

    /// POST /list-group
    pub async fn list_groups(
        &self,
        req: &ListGroups,
    ) -> Result<ListGroupsOutput> {
        self.post("/list-group", req).await
    }

    /// POST /list-items
    pub async fn list_items(
        &self,
        req: &ListItems,
    ) -> Result<ListItemsInPathOutput> {
        self.post("/list-items", req).await
    }

    /// POST /list-roles
    pub async fn list_roles(
        &self,
        req: &ListRoles,
    ) -> Result<ListRolesOutput> {
        self.post("/list-roles", req).await
    }

    /// POST /list-shared-items
    pub async fn list_shared_items(
        &self,
        req: &ListSharedItems,
    ) -> Result<JsonError> {
        self.post("/list-shared-items", req).await
    }

    /// POST /list-sra-bastions
    pub async fn list_sra_bastions(
        &self,
        req: &ListSraBastions,
    ) -> Result<BastionsList> {
        self.post("/list-sra-bastions", req).await
    }

    /// POST /list-sra-sessions
    pub async fn list_sra_sessions(
        &self,
        req: &ListSraSessions,
    ) -> Result<ListSraSessionsOutput> {
        self.post("/list-sra-sessions", req).await
    }

    /// POST /list-targets
    pub async fn list_targets(
        &self,
        req: &ListTargets,
    ) -> Result<ListTargetsOutput> {
        self.post("/list-targets", req).await
    }

    /// POST /move-objects
    pub async fn move_objects(
        &self,
        req: &MoveObjects,
    ) -> Result<MoveObjectsOutput> {
        self.post("/move-objects", req).await
    }

    /// POST /policy-create-keys
    pub async fn policy_create_keys(
        &self,
        req: &PolicyCreateKeys,
    ) -> Result<PoliciesCreateOutput> {
        self.post("/policy-create-keys", req).await
    }

    /// POST /policy-delete
    pub async fn policies_delete(
        &self,
        req: &PoliciesDelete,
    ) -> Result<PoliciesDeleteOutput> {
        self.post("/policy-delete", req).await
    }

    /// POST /policy-get
    pub async fn policies_get(
        &self,
        req: &PoliciesGet,
    ) -> Result<PoliciesGetOutput> {
        self.post("/policy-get", req).await
    }

    /// POST /policy-list
    pub async fn policies_list(
        &self,
        req: &PoliciesList,
    ) -> Result<PoliciesListOutput> {
        self.post("/policy-list", req).await
    }

    /// POST /policy-update-keys
    pub async fn policy_update_keys(
        &self,
        req: &PolicyUpdateKeys,
    ) -> Result<PoliciesUpdateOutput> {
        self.post("/policy-update-keys", req).await
    }

    /// POST /provision-certificate
    pub async fn provision_certificate(
        &self,
        req: &ProvisionCertificate,
    ) -> Result<ProvisionCertificateOutput> {
        self.post("/provision-certificate", req).await
    }

    /// POST /raw-creds
    pub async fn raw_creds(
        &self,
        req: &RawCreds,
    ) -> Result<SystemAccessCredentialsReplyObj> {
        self.post("/raw-creds", req).await
    }

    /// POST /refresh-key
    pub async fn refresh_key(
        &self,
        req: &RefreshKey,
    ) -> Result<RefreshKeyOutput> {
        self.post("/refresh-key", req).await
    }

    /// POST /renew-certificate
    pub async fn renew_certificate(
        &self,
        req: &RenewCertificate,
    ) -> Result<RenewCertificateOutput> {
        self.post("/renew-certificate", req).await
    }

    /// POST /request-access
    pub async fn request_access(
        &self,
        req: &RequestAccess,
    ) -> Result<RequestAccessOutput> {
        self.post("/request-access", req).await
    }

    /// POST /reset-access-key
    pub async fn reset_access_key(
        &self,
        req: &ResetAccessKey,
    ) -> Result<ResetAuthMethodAccessKeyOutput> {
        self.post("/reset-access-key", req).await
    }

    /// POST /reverse-rbac
    pub async fn reverse_rbac(
        &self,
        req: &ReverseRbac,
    ) -> Result<ReverseRbacOutput> {
        self.post("/reverse-rbac", req).await
    }

    /// POST /revoke-certificate
    pub async fn revoke_certificate(
        &self,
        req: &RevokeCertificate,
    ) -> Result<RevokeCertificateOutput> {
        self.post("/revoke-certificate", req).await
    }

    /// POST /revoke-creds
    pub async fn revoke_creds(
        &self,
    ) -> Result<RevokeCredsOutput> {
        self.post_empty("/revoke-creds").await
    }

    /// POST /rollback-secret
    pub async fn rollback_secret(
        &self,
        req: &RollbackSecret,
    ) -> Result<RollbackSecretOutput> {
        self.post("/rollback-secret", req).await
    }

    /// POST /rotate-key
    pub async fn rotate_key(
        &self,
        req: &RotateKey,
    ) -> Result<RotateKeyOutput> {
        self.post("/rotate-key", req).await
    }

    /// POST /rotate-oidc-client-secret
    pub async fn rotate_oidc_client_secret(
        &self,
        req: &RotateOidcClientSecret,
    ) -> Result<RotateOidcClientOutput> {
        self.post("/rotate-oidc-client-secret", req).await
    }

    /// POST /rotated-secret-create-aws
    pub async fn rotated_secret_create_aws(
        &self,
        req: &RotatedSecretCreateAws,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-aws", req).await
    }

    /// POST /rotated-secret-create-azure
    pub async fn rotated_secret_create_azure(
        &self,
        req: &RotatedSecretCreateAzure,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-azure", req).await
    }

    /// POST /rotated-secret-create-cassandra
    pub async fn rotated_secret_create_cassandra(
        &self,
        req: &RotatedSecretCreateCassandra,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-cassandra", req).await
    }

    /// POST /rotated-secret-create-custom
    pub async fn rotated_secret_create_custom(
        &self,
        req: &RotatedSecretCreateCustom,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-custom", req).await
    }

    /// POST /rotated-secret-create-dockerhub
    pub async fn rotated_secret_create_dockerhub(
        &self,
        req: &RotatedSecretCreateDockerhub,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-dockerhub", req).await
    }

    /// POST /rotated-secret-create-gcp
    pub async fn rotated_secret_create_gcp(
        &self,
        req: &RotatedSecretCreateGcp,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-gcp", req).await
    }

    /// POST /rotated-secret-create-hanadb
    pub async fn rotated_secret_create_hanadb(
        &self,
        req: &RotatedSecretCreateHanadb,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-hanadb", req).await
    }

    /// POST /rotated-secret-create-ldap
    pub async fn rotated_secret_create_ldap(
        &self,
        req: &RotatedSecretCreateLdap,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-ldap", req).await
    }

    /// POST /rotated-secret-create-mongodb
    pub async fn rotated_secret_create_mongodb(
        &self,
        req: &RotatedSecretCreateMongodb,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-mongodb", req).await
    }

    /// POST /rotated-secret-create-mssql
    pub async fn rotated_secret_create_mssql(
        &self,
        req: &RotatedSecretCreateMssql,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-mssql", req).await
    }

    /// POST /rotated-secret-create-mysql
    pub async fn rotated_secret_create_mysql(
        &self,
        req: &RotatedSecretCreateMysql,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-mysql", req).await
    }

    /// POST /rotated-secret-create-openai
    pub async fn rotated_secret_create_open_ai(
        &self,
        req: &RotatedSecretCreateOpenAi,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-openai", req).await
    }

    /// POST /rotated-secret-create-oracledb
    pub async fn rotated_secret_create_oracledb(
        &self,
        req: &RotatedSecretCreateOracledb,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-oracledb", req).await
    }

    /// POST /rotated-secret-create-postgresql
    pub async fn rotated_secret_create_postgresql(
        &self,
        req: &RotatedSecretCreatePostgresql,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-postgresql", req).await
    }

    /// POST /rotated-secret-create-redis
    pub async fn rotated_secret_create_redis(
        &self,
        req: &RotatedSecretCreateRedis,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-redis", req).await
    }

    /// POST /rotated-secret-create-redshift
    pub async fn rotated_secret_create_redshift(
        &self,
        req: &RotatedSecretCreateRedshift,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-redshift", req).await
    }

    /// POST /rotated-secret-create-snowflake
    pub async fn rotated_secret_create_snowflake(
        &self,
        req: &RotatedSecretCreateSnowflake,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-snowflake", req).await
    }

    /// POST /rotated-secret-create-splunk
    pub async fn rotated_secret_create_splunk(
        &self,
        req: &RotatedSecretCreateSplunk,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-splunk", req).await
    }

    /// POST /rotated-secret-create-ssh
    pub async fn rotated_secret_create_ssh(
        &self,
        req: &RotatedSecretCreateSsh,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-ssh", req).await
    }

    /// POST /rotated-secret-create-windows
    pub async fn rotated_secret_create_windows(
        &self,
        req: &RotatedSecretCreateWindows,
    ) -> Result<RotatedSecretCreateOutput> {
        self.post("/rotated-secret-create-windows", req).await
    }

    /// POST /rotated-secret-delete
    pub async fn rotated_secret_delete(
        &self,
        req: &RotatedSecretDelete,
    ) -> Result<DeleteItemOutput> {
        self.post("/rotated-secret-delete", req).await
    }

    /// POST /rotated-secret-delete-sync
    pub async fn rotated_secret_delete_sync(
        &self,
        req: &RotatedSecretDeleteSync,
    ) -> Result<RotatedSecretDeleteSyncOutput> {
        self.post("/rotated-secret-delete-sync", req).await
    }

    /// POST /rotated-secret-get-value
    pub async fn rotated_secret_get_value(
        &self,
        req: &RotatedSecretGetValue,
    ) -> Result<serde_json::Value> {
        self.post("/rotated-secret-get-value", req).await
    }

    /// POST /rotated-secret-list
    pub async fn rotated_secret_list(
        &self,
        req: &RotatedSecretList,
    ) -> Result<GetProducersListReplyObj> {
        self.post("/rotated-secret-list", req).await
    }

    /// POST /rotated-secret-sync
    pub async fn rotated_secret_sync(
        &self,
        req: &RotatedSecretSync,
    ) -> Result<RotatedSecretSyncOutput> {
        self.post("/rotated-secret-sync", req).await
    }

    /// POST /rotated-secret-update-aws
    pub async fn rotated_secret_update_aws(
        &self,
        req: &RotatedSecretUpdateAws,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-aws", req).await
    }

    /// POST /rotated-secret-update-azure
    pub async fn rotated_secret_update_azure(
        &self,
        req: &RotatedSecretUpdateAzure,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-azure", req).await
    }

    /// POST /rotated-secret-update-cassandra
    pub async fn rotated_secret_update_cassandra(
        &self,
        req: &RotatedSecretUpdateCassandra,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-cassandra", req).await
    }

    /// POST /rotated-secret-update-custom
    pub async fn rotated_secret_update_custom(
        &self,
        req: &RotatedSecretUpdateCustom,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-custom", req).await
    }

    /// POST /rotated-secret-update-dockerhub
    pub async fn rotated_secret_update_dockerhub(
        &self,
        req: &RotatedSecretUpdateDockerhub,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-dockerhub", req).await
    }

    /// POST /rotated-secret-update-gcp
    pub async fn rotated_secret_update_gcp(
        &self,
        req: &RotatedSecretUpdateGcp,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-gcp", req).await
    }

    /// POST /rotated-secret-update-hanadb
    pub async fn rotated_secret_update_hanadb(
        &self,
        req: &RotatedSecretUpdateHanadb,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-hanadb", req).await
    }

    /// POST /rotated-secret-update-ldap
    pub async fn rotated_secret_update_ldap(
        &self,
        req: &RotatedSecretUpdateLdap,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-ldap", req).await
    }

    /// POST /rotated-secret-update-mongodb
    pub async fn rotated_secret_update_mongodb(
        &self,
        req: &RotatedSecretUpdateMongodb,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-mongodb", req).await
    }

    /// POST /rotated-secret-update-mssql
    pub async fn rotated_secret_update_mssql(
        &self,
        req: &RotatedSecretUpdateMssql,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-mssql", req).await
    }

    /// POST /rotated-secret-update-mysql
    pub async fn rotated_secret_update_mysql(
        &self,
        req: &RotatedSecretUpdateMysql,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-mysql", req).await
    }

    /// POST /rotated-secret-update-openai
    pub async fn rotated_secret_update_open_ai(
        &self,
        req: &RotatedSecretUpdateOpenAi,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-openai", req).await
    }

    /// POST /rotated-secret-update-oracledb
    pub async fn rotated_secret_update_oracledb(
        &self,
        req: &RotatedSecretUpdateOracledb,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-oracledb", req).await
    }

    /// POST /rotated-secret-update-postgresql
    pub async fn rotated_secret_update_postgresql(
        &self,
        req: &RotatedSecretUpdatePostgresql,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-postgresql", req).await
    }

    /// POST /rotated-secret-update-redis
    pub async fn rotated_secret_update_redis(
        &self,
        req: &RotatedSecretUpdateRedis,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-redis", req).await
    }

    /// POST /rotated-secret-update-redshift
    pub async fn rotated_secret_update_redshift(
        &self,
        req: &RotatedSecretUpdateRedshift,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-redshift", req).await
    }

    /// POST /rotated-secret-update-snowflake
    pub async fn rotated_secret_update_snowflake(
        &self,
        req: &RotatedSecretUpdateSnowflake,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-snowflake", req).await
    }

    /// POST /rotated-secret-update-splunk
    pub async fn rotated_secret_update_splunk(
        &self,
        req: &RotatedSecretUpdateSplunk,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-splunk", req).await
    }

    /// POST /rotated-secret-update-ssh
    pub async fn rotated_secret_update_ssh(
        &self,
        req: &RotatedSecretUpdateSsh,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-ssh", req).await
    }

    /// POST /rotated-secret-update-windows
    pub async fn rotated_secret_update_windows(
        &self,
        req: &RotatedSecretUpdateWindows,
    ) -> Result<RotatedSecretUpdateOutput> {
        self.post("/rotated-secret-update-windows", req).await
    }

    /// POST /set-item-state
    pub async fn set_item_state(
        &self,
        req: &SetItemState,
    ) -> Result<SetItemStateOutput> {
        self.post("/set-item-state", req).await
    }

    /// POST /set-role-rule
    pub async fn set_role_rule(
        &self,
        req: &SetRoleRule,
    ) -> Result<SetRoleRuleOutput> {
        self.post("/set-role-rule", req).await
    }

    /// POST /share-item
    pub async fn share_item(
        &self,
        req: &ShareItem,
    ) -> Result<ShareItemOutput> {
        self.post("/share-item", req).await
    }

    /// POST /sign-data-with-classic-key
    pub async fn sign_data_with_classic_key(
        &self,
        req: &SignDataWithClassicKey,
    ) -> Result<SignOutput> {
        self.post("/sign-data-with-classic-key", req).await
    }

    /// POST /sign-ecdsa
    pub async fn sign_ec_dsa(
        &self,
        req: &SignEcDsa,
    ) -> Result<SignEcDsaOutput> {
        self.post("/sign-ecdsa", req).await
    }

    /// POST /sign-gpg
    pub async fn sign_gpg(
        &self,
        req: &SignGpg,
    ) -> Result<SignGpgOutput> {
        self.post("/sign-gpg", req).await
    }

    /// POST /sign-jwt-with-classic-key
    pub async fn sign_jwt_with_classic_key(
        &self,
        req: &SignJwtWithClassicKey,
    ) -> Result<SignJwtOutput> {
        self.post("/sign-jwt-with-classic-key", req).await
    }

    /// POST /sign-pkcs1
    pub async fn sign_pkcs1(
        &self,
        req: &SignPkcs1,
    ) -> Result<SignPkcs1Output> {
        self.post("/sign-pkcs1", req).await
    }

    /// POST /sign-pki-cert-with-classic-key
    pub async fn sign_pki_cert_with_classic_key(
        &self,
        req: &SignPkiCertWithClassicKey,
    ) -> Result<SignPkiCertOutput> {
        self.post("/sign-pki-cert-with-classic-key", req).await
    }

    /// POST /sign-rsassa-pss
    pub async fn sign_rsa_ssa_pss(
        &self,
        req: &SignRsaSsaPss,
    ) -> Result<SignRsaSsaPssOutput> {
        self.post("/sign-rsassa-pss", req).await
    }

    /// POST /static-creds-auth
    pub async fn static_creds_auth(
        &self,
        req: &StaticCredsAuth,
    ) -> Result<StaticCredsAuthOutput> {
        self.post("/static-creds-auth", req).await
    }

    /// POST /static-secret-delete-sync
    pub async fn static_secret_delete_sync(
        &self,
        req: &StaticSecretDeleteSync,
    ) -> Result<StaticSecretDeleteSyncOutput> {
        self.post("/static-secret-delete-sync", req).await
    }

    /// POST /static-secret-sync
    pub async fn static_secret_sync(
        &self,
        req: &StaticSecretSync,
    ) -> Result<SecretSyncOutput> {
        self.post("/static-secret-sync", req).await
    }

    /// POST /target-create-artifactory
    pub async fn target_create_artifactory(
        &self,
        req: &TargetCreateArtifactory,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-artifactory", req).await
    }

    /// POST /target-create-aws
    pub async fn target_create_aws(
        &self,
        req: &TargetCreateAws,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-aws", req).await
    }

    /// POST /target-create-azure
    pub async fn target_create_azure(
        &self,
        req: &TargetCreateAzure,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-azure", req).await
    }

    /// POST /target-create-db
    pub async fn target_create_db(
        &self,
        req: &TargetCreateDb,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-db", req).await
    }

    /// POST /target-create-dockerhub
    pub async fn target_create_dockerhub(
        &self,
        req: &TargetCreateDockerhub,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-dockerhub", req).await
    }

    /// POST /target-create-eks
    pub async fn target_create_eks(
        &self,
        req: &TargetCreateEks,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-eks", req).await
    }

    /// POST /target-create-gcp
    pub async fn target_create_gcp(
        &self,
        req: &TargetCreateGcp,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-gcp", req).await
    }

    /// POST /target-create-gemini
    pub async fn target_create_gemini(
        &self,
        req: &TargetCreateGemini,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-gemini", req).await
    }

    /// POST /target-create-github
    pub async fn target_create_github(
        &self,
        req: &TargetCreateGithub,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-github", req).await
    }

    /// POST /target-create-gitlab
    pub async fn target_create_gitlab(
        &self,
        req: &TargetCreateGitlab,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-gitlab", req).await
    }

    /// POST /target-create-gke
    pub async fn target_create_gke(
        &self,
        req: &TargetCreateGke,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-gke", req).await
    }

    /// POST /target-create-globalsign
    pub async fn target_create_global_sign(
        &self,
        req: &TargetCreateGlobalSign,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-globalsign", req).await
    }

    /// POST /target-create-globalsign-atlas
    pub async fn target_create_global_sign_atlas(
        &self,
        req: &TargetCreateGlobalSignAtlas,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-globalsign-atlas", req).await
    }

    /// POST /target-create-godaddy
    pub async fn target_create_godaddy(
        &self,
        req: &TargetCreateGodaddy,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-godaddy", req).await
    }

    /// POST /target-create-hashi-vault
    pub async fn target_create_hashi_vault(
        &self,
        req: &TargetCreateHashiVault,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-hashi-vault", req).await
    }

    /// POST /target-create-k8s
    pub async fn target_create_k8s(
        &self,
        req: &TargetCreateK8s,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-k8s", req).await
    }

    /// POST /target-create-ldap
    pub async fn target_create_ldap(
        &self,
        req: &TargetCreateLdap,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-ldap", req).await
    }

    /// POST /target-create-lets-encrypt
    pub async fn target_create_lets_encrypt(
        &self,
        req: &TargetCreateLetsEncrypt,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-lets-encrypt", req).await
    }

    /// POST /target-create-linked
    pub async fn target_create_linked(
        &self,
        req: &TargetCreateLinked,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-linked", req).await
    }

    /// POST /target-create-openai
    pub async fn target_create_open_ai(
        &self,
        req: &TargetCreateOpenAi,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-openai", req).await
    }

    /// POST /target-create-ping
    pub async fn target_create_ping(
        &self,
        req: &TargetCreatePing,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-ping", req).await
    }

    /// POST /target-create-rabbitmq
    pub async fn target_create_rabbit_mq(
        &self,
        req: &TargetCreateRabbitMq,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-rabbitmq", req).await
    }

    /// POST /target-create-salesforce
    pub async fn target_create_salesforce(
        &self,
        req: &TargetCreateSalesforce,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-salesforce", req).await
    }

    /// POST /target-create-sectigo
    pub async fn target_create_sectigo(
        &self,
        req: &TargetCreateSectigo,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-sectigo", req).await
    }

    /// POST /target-create-splunk
    pub async fn target_create_splunk(
        &self,
        req: &TargetCreateSplunk,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-splunk", req).await
    }

    /// POST /target-create-ssh
    pub async fn target_create_ssh(
        &self,
        req: &TargetCreateSsh,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-ssh", req).await
    }

    /// POST /target-create-web
    pub async fn target_create_web(
        &self,
        req: &TargetCreateWeb,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-web", req).await
    }

    /// POST /target-create-windows
    pub async fn target_create_windows(
        &self,
        req: &TargetCreateWindows,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-windows", req).await
    }

    /// POST /target-create-zerossl
    pub async fn target_create_zero_ssl(
        &self,
        req: &TargetCreateZeroSsl,
    ) -> Result<TargetCreateOutput> {
        self.post("/target-create-zerossl", req).await
    }

    /// POST /target-delete
    pub async fn target_delete(
        &self,
        req: &TargetDelete,
    ) -> Result<TargetDeleteOutput> {
        self.post("/target-delete", req).await
    }

    /// POST /target-get
    pub async fn target_get(
        &self,
        req: &TargetGet,
    ) -> Result<Target> {
        self.post("/target-get", req).await
    }

    /// POST /target-get-details
    pub async fn target_get_details(
        &self,
        req: &TargetGetDetails,
    ) -> Result<GetTargetDetailsOutput> {
        self.post("/target-get-details", req).await
    }

    /// POST /target-list
    pub async fn target_list(
        &self,
        req: &TargetList,
    ) -> Result<ListTargetsOutput> {
        self.post("/target-list", req).await
    }

    /// POST /target-update-artifactory
    pub async fn target_update_artifactory(
        &self,
        req: &TargetUpdateArtifactory,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-artifactory", req).await
    }

    /// POST /target-update-aws
    pub async fn target_update_aws(
        &self,
        req: &TargetUpdateAws,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-aws", req).await
    }

    /// POST /target-update-azure
    pub async fn target_update_azure(
        &self,
        req: &TargetUpdateAzure,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-azure", req).await
    }

    /// POST /target-update-db
    pub async fn target_update_db(
        &self,
        req: &TargetUpdateDb,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-db", req).await
    }

    /// POST /target-update-dockerhub
    pub async fn target_update_dockerhub(
        &self,
        req: &TargetUpdateDockerhub,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-dockerhub", req).await
    }

    /// POST /target-update-eks
    pub async fn target_update_eks(
        &self,
        req: &TargetUpdateEks,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-eks", req).await
    }

    /// POST /target-update-gcp
    pub async fn target_update_gcp(
        &self,
        req: &TargetUpdateGcp,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-gcp", req).await
    }

    /// POST /target-update-gemini
    pub async fn target_update_gemini(
        &self,
        req: &TargetUpdateGemini,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-gemini", req).await
    }

    /// POST /target-update-github
    pub async fn target_update_github(
        &self,
        req: &TargetUpdateGithub,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-github", req).await
    }

    /// POST /target-update-gitlab
    pub async fn target_update_gitlab(
        &self,
        req: &TargetUpdateGitlab,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-gitlab", req).await
    }

    /// POST /target-update-gke
    pub async fn target_update_gke(
        &self,
        req: &TargetUpdateGke,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-gke", req).await
    }

    /// POST /target-update-globalsign
    pub async fn target_update_global_sign(
        &self,
        req: &TargetUpdateGlobalSign,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-globalsign", req).await
    }

    /// POST /target-update-globalsign-atlas
    pub async fn target_update_global_sign_atlas(
        &self,
        req: &TargetUpdateGlobalSignAtlas,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-globalsign-atlas", req).await
    }

    /// POST /target-update-godaddy
    pub async fn target_update_godaddy(
        &self,
        req: &TargetUpdateGodaddy,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-godaddy", req).await
    }

    /// POST /target-update-hashi-vault
    pub async fn target_update_hashi_vault(
        &self,
        req: &TargetUpdateHashiVault,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-hashi-vault", req).await
    }

    /// POST /target-update-k8s
    pub async fn target_update_k8s(
        &self,
        req: &TargetUpdateK8s,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-k8s", req).await
    }

    /// POST /target-update-ldap
    pub async fn target_update_ldap(
        &self,
        req: &TargetUpdateLdap,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-ldap", req).await
    }

    /// POST /target-update-lets-encrypt
    pub async fn target_update_lets_encrypt(
        &self,
        req: &TargetUpdateLetsEncrypt,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-lets-encrypt", req).await
    }

    /// POST /target-update-linked
    pub async fn target_update_linked(
        &self,
        req: &TargetUpdateLinked,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-linked", req).await
    }

    /// POST /target-update-openai
    pub async fn target_update_open_ai(
        &self,
        req: &TargetUpdateOpenAi,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-openai", req).await
    }

    /// POST /target-update-ping
    pub async fn target_update_ping(
        &self,
        req: &TargetUpdatePing,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-ping", req).await
    }

    /// POST /target-update-rabbitmq
    pub async fn target_update_rabbit_mq(
        &self,
        req: &TargetUpdateRabbitMq,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-rabbitmq", req).await
    }

    /// POST /target-update-salesforce
    pub async fn target_update_salesforce(
        &self,
        req: &TargetUpdateSalesforce,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-salesforce", req).await
    }

    /// POST /target-update-sectigo
    pub async fn target_update_sectigo(
        &self,
        req: &TargetUpdateSectigo,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-sectigo", req).await
    }

    /// POST /target-update-ssh
    pub async fn target_update_ssh(
        &self,
        req: &TargetUpdateSsh,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-ssh", req).await
    }

    /// POST /target-update-web
    pub async fn target_update_web(
        &self,
        req: &TargetUpdateWeb,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-web", req).await
    }

    /// POST /target-update-windows
    pub async fn target_update_windows(
        &self,
        req: &TargetUpdateWindows,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-windows", req).await
    }

    /// POST /target-update-zerossl
    pub async fn target_update_zero_ssl(
        &self,
        req: &TargetUpdateZeroSsl,
    ) -> Result<TargetUpdateOutput> {
        self.post("/target-update-zerossl", req).await
    }

    /// POST /tokenize
    pub async fn tokenize(
        &self,
        req: &Tokenize,
    ) -> Result<TokenizeOutput> {
        self.post("/tokenize", req).await
    }

    /// POST /tokenize-batch
    pub async fn tokenize_batch(
        &self,
        req: &BatchTokenizationRequest,
    ) -> Result<TokenizeOutput> {
        self.post("/tokenize-batch", req).await
    }

    /// POST /uid-create-child-token
    pub async fn uid_create_child_token(
        &self,
        req: &UidCreateChildToken,
    ) -> Result<UidCreateChildTokenOutput> {
        self.post("/uid-create-child-token", req).await
    }

    /// POST /uid-generate-token
    pub async fn uid_generate_token(
        &self,
        req: &UidGenerateToken,
    ) -> Result<UidGenerateTokenOutput> {
        self.post("/uid-generate-token", req).await
    }

    /// POST /uid-list-children
    pub async fn uid_list_children(
        &self,
        req: &UidListChildren,
    ) -> Result<UniversalIdentityDetails> {
        self.post("/uid-list-children", req).await
    }

    /// POST /uid-revoke-token
    pub async fn uid_revoke_token(
        &self,
        req: &UidRevokeToken,
    ) -> Result<UidRevokeTokenOutput> {
        self.post("/uid-revoke-token", req).await
    }

    /// POST /uid-rotate-token
    pub async fn uid_rotate_token(
        &self,
        req: &UidRotateToken,
    ) -> Result<UidRotateTokenOutput> {
        self.post("/uid-rotate-token", req).await
    }

    /// POST /unwrap-token
    pub async fn unwrap_token(
        &self,
        req: &UnwrapToken,
    ) -> Result<UnwrapTokenOutput> {
        self.post("/unwrap-token", req).await
    }

    /// POST /update-account-settings
    pub async fn update_account_settings(
        &self,
        req: &UpdateAccountSettings,
    ) -> Result<UpdateAccountSettingsOutput> {
        self.post("/update-account-settings", req).await
    }

    /// POST /update-artifactory-target
    pub async fn update_artifactory_target(
        &self,
        req: &UpdateArtifactoryTarget,
    ) -> Result<UpdateArtifactoryTargetOutput> {
        self.post("/update-artifactory-target", req).await
    }

    /// POST /update-assoc
    pub async fn update_assoc(
        &self,
        req: &UpdateAssoc,
    ) -> Result<UpdateAssocOutput> {
        self.post("/update-assoc", req).await
    }

    /// POST /update-auth-method
    pub async fn update_auth_method(
        &self,
        req: &UpdateAuthMethod,
    ) -> Result<UpdateAuthMethodOutput> {
        self.post("/update-auth-method", req).await
    }

    /// POST /update-auth-method-aws-iam
    pub async fn update_auth_method_awsiam(
        &self,
        req: &UpdateAuthMethodAwsiam,
    ) -> Result<UpdateAuthMethodAwsiamOutput> {
        self.post("/update-auth-method-aws-iam", req).await
    }

    /// POST /update-auth-method-azure-ad
    pub async fn update_auth_method_azure_ad(
        &self,
        req: &UpdateAuthMethodAzureAd,
    ) -> Result<UpdateAuthMethodAzureAdOutput> {
        self.post("/update-auth-method-azure-ad", req).await
    }

    /// POST /update-auth-method-cert
    pub async fn update_auth_method_cert(
        &self,
        req: &UpdateAuthMethodCert,
    ) -> Result<UpdateAuthMethodCertOutput> {
        self.post("/update-auth-method-cert", req).await
    }

    /// POST /update-auth-method-gcp
    pub async fn update_auth_method_gcp(
        &self,
        req: &UpdateAuthMethodGcp,
    ) -> Result<UpdateAuthMethodGcpOutput> {
        self.post("/update-auth-method-gcp", req).await
    }

    /// POST /update-auth-method-k8s
    pub async fn update_auth_method_k8s(
        &self,
        req: &UpdateAuthMethodK8s,
    ) -> Result<UpdateAuthMethodK8sOutput> {
        self.post("/update-auth-method-k8s", req).await
    }

    /// POST /update-auth-method-ldap
    pub async fn update_auth_method_ldap(
        &self,
        req: &UpdateAuthMethodLdap,
    ) -> Result<UpdateAuthMethodLdapOutput> {
        self.post("/update-auth-method-ldap", req).await
    }

    /// POST /update-auth-method-oauth2
    pub async fn update_auth_method_o_auth2(
        &self,
        req: &UpdateAuthMethodOAuth2,
    ) -> Result<UpdateAuthMethodOAuth2Output> {
        self.post("/update-auth-method-oauth2", req).await
    }

    /// POST /update-auth-method-oci
    pub async fn update_auth_method_oci(
        &self,
        req: &UpdateAuthMethodOci,
    ) -> Result<UpdateAuthMethodOciOutput> {
        self.post("/update-auth-method-oci", req).await
    }

    /// POST /update-auth-method-oidc
    pub async fn update_auth_method_oidc(
        &self,
        req: &UpdateAuthMethodOidc,
    ) -> Result<UpdateAuthMethodOidcOutput> {
        self.post("/update-auth-method-oidc", req).await
    }

    /// POST /update-auth-method-saml
    pub async fn update_auth_method_saml(
        &self,
        req: &UpdateAuthMethodSaml,
    ) -> Result<UpdateAuthMethodSamlOutput> {
        self.post("/update-auth-method-saml", req).await
    }

    /// POST /update-auth-method-universal-identity
    pub async fn update_auth_method_universal_identity(
        &self,
        req: &UpdateAuthMethodUniversalIdentity,
    ) -> Result<UpdateAuthMethodUniversalIdentityOutput> {
        self.post("/update-auth-method-universal-identity", req).await
    }

    /// POST /update-aws-target
    pub async fn update_aws_target(
        &self,
        req: &UpdateAwsTarget,
    ) -> Result<UpdateAwsTargetOutput> {
        self.post("/update-aws-target", req).await
    }

    /// POST /update-aws-target-details
    pub async fn update_aws_target_details(
        &self,
        req: &UpdateAwsTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-aws-target-details", req).await
    }

    /// POST /update-azure-target
    pub async fn update_azure_target(
        &self,
        req: &UpdateAzureTarget,
    ) -> Result<UpdateAzureTargetOutput> {
        self.post("/update-azure-target", req).await
    }

    /// POST /update-certificate-value
    pub async fn update_certificate_value(
        &self,
        req: &UpdateCertificateValue,
    ) -> Result<UpdateCertificateOutput> {
        self.post("/update-certificate-value", req).await
    }

    /// POST /update-classic-key-certificate
    pub async fn update_classic_key_certificate(
        &self,
        req: &UpdateClassicKeyCertificate,
    ) -> Result<UpdateClassicKeyCertificateOutput> {
        self.post("/update-classic-key-certificate", req).await
    }

    /// POST /update-db-target
    pub async fn update_db_target(
        &self,
        req: &UpdateDbTarget,
    ) -> Result<UpdateDbTargetOutput> {
        self.post("/update-db-target", req).await
    }

    /// POST /update-db-target-details
    pub async fn update_db_target_details(
        &self,
        req: &UpdateDbTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-db-target-details", req).await
    }

    /// POST /update-dockerhub-target
    pub async fn update_dockerhub_target(
        &self,
        req: &UpdateDockerhubTarget,
    ) -> Result<UpdateDockerhubTargetOutput> {
        self.post("/update-dockerhub-target", req).await
    }

    /// POST /update-eks-target
    pub async fn update_eks_target(
        &self,
        req: &UpdateEksTarget,
    ) -> Result<UpdateEksTargetOutput> {
        self.post("/update-eks-target", req).await
    }

    /// POST /update-event-forwarder
    pub async fn update_event_forwarder(
        &self,
        req: &UpdateEventForwarder,
    ) -> Result<UpdateEventForwarderOutput> {
        self.post("/update-event-forwarder", req).await
    }

    /// POST /update-gcp-target
    pub async fn update_gcp_target(
        &self,
        req: &UpdateGcpTarget,
    ) -> Result<UpdateGcpTargetOutput> {
        self.post("/update-gcp-target", req).await
    }

    /// POST /update-github-target
    pub async fn update_github_target(
        &self,
        req: &UpdateGithubTarget,
    ) -> Result<UpdateGithubTargetOutput> {
        self.post("/update-github-target", req).await
    }

    /// POST /update-gitlab-target
    pub async fn update_gitlab_target(
        &self,
        req: &UpdateGitlabTarget,
    ) -> Result<UpdateGitlabTargetOutput> {
        self.post("/update-gitlab-target", req).await
    }

    /// POST /update-gke-target
    pub async fn update_gke_target(
        &self,
        req: &UpdateGkeTarget,
    ) -> Result<UpdateGkeTargetOutput> {
        self.post("/update-gke-target", req).await
    }

    /// POST /update-globalsign-atlas-target
    pub async fn update_global_sign_atlas_target(
        &self,
        req: &UpdateGlobalSignAtlasTarget,
    ) -> Result<UpdateGlobalSignAtlasTargetOutput> {
        self.post("/update-globalsign-atlas-target", req).await
    }

    /// POST /update-globalsign-target
    pub async fn update_global_sign_target(
        &self,
        req: &UpdateGlobalSignTarget,
    ) -> Result<UpdateGlobalSignTargetOutput> {
        self.post("/update-globalsign-target", req).await
    }

    /// POST /update-godaddy-target
    pub async fn update_godaddy_target(
        &self,
        req: &UpdateGodaddyTarget,
    ) -> Result<UpdateGodaddyTargetOutput> {
        self.post("/update-godaddy-target", req).await
    }

    /// POST /update-group
    pub async fn update_group(
        &self,
        req: &UpdateGroup,
    ) -> Result<UpdateGroupOutput> {
        self.post("/update-group", req).await
    }

    /// POST /update-hashi-vault-target
    pub async fn update_hashi_vault_target(
        &self,
        req: &UpdateHashiVaultTarget,
    ) -> Result<UpdateHashiVaultTargetOutput> {
        self.post("/update-hashi-vault-target", req).await
    }

    /// POST /update-item
    pub async fn update_item(
        &self,
        req: &UpdateItem,
    ) -> Result<UpdateItemOutput> {
        self.post("/update-item", req).await
    }

    /// POST /update-k8s-target
    pub async fn update_native_k8s_target(
        &self,
        req: &UpdateNativeK8sTarget,
    ) -> Result<UpdateNativeK8sTargetOutput> {
        self.post("/update-k8s-target", req).await
    }

    /// POST /update-ldap-target
    pub async fn update_ldap_target(
        &self,
        req: &UpdateLdapTarget,
    ) -> Result<UpdateLdapTargetOutput> {
        self.post("/update-ldap-target", req).await
    }

    /// POST /update-ldap-target-details
    pub async fn update_ldap_target_details(
        &self,
        req: &UpdateLdapTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-ldap-target-details", req).await
    }

    /// POST /update-linked-target
    pub async fn update_linked_target(
        &self,
        req: &UpdateLinkedTarget,
    ) -> Result<UpdateLinkedTargetOutput> {
        self.post("/update-linked-target", req).await
    }

    /// POST /update-oidc-app
    pub async fn update_oidc_app(
        &self,
        req: &UpdateOidcApp,
    ) -> Result<UpdateOidcAppOutput> {
        self.post("/update-oidc-app", req).await
    }

    /// POST /update-ping-target
    pub async fn update_ping_target(
        &self,
        req: &UpdatePingTarget,
    ) -> Result<UpdatePingTargetOutput> {
        self.post("/update-ping-target", req).await
    }

    /// POST /update-pki-cert-issuer
    pub async fn update_pki_cert_issuer(
        &self,
        req: &UpdatePkiCertIssuer,
    ) -> Result<UpdatePkiCertIssuerOutput> {
        self.post("/update-pki-cert-issuer", req).await
    }

    /// POST /update-rabbitmq-target
    pub async fn update_rabbit_mq_target(
        &self,
        req: &UpdateRabbitMqTarget,
    ) -> Result<UpdateRabbitMqTargetOutput> {
        self.post("/update-rabbitmq-target", req).await
    }

    /// POST /update-rabbitmq-target-details
    pub async fn update_rabbit_mq_target_details(
        &self,
        req: &UpdateRabbitMqTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-rabbitmq-target-details", req).await
    }

    /// POST /update-rdp-target-details
    pub async fn update_rdp_target_details(
        &self,
        req: &UpdateRdpTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-rdp-target-details", req).await
    }

    /// POST /update-role
    pub async fn update_role(
        &self,
        req: &UpdateRole,
    ) -> Result<UpdateRoleOutput> {
        self.post("/update-role", req).await
    }

    /// POST /update-rotated-secret
    pub async fn update_rotated_secret(
        &self,
        req: &UpdateRotatedSecret,
    ) -> Result<UpdateRotatedSecretOutput> {
        self.post("/update-rotated-secret", req).await
    }

    /// POST /update-rotation-settings
    pub async fn update_rotation_settings(
        &self,
        req: &UpdateRotationSettings,
    ) -> Result<RotateKeyOutput> {
        self.post("/update-rotation-settings", req).await
    }

    /// POST /update-salesforce-target
    pub async fn update_salesforce_target(
        &self,
        req: &UpdateSalesforceTarget,
    ) -> Result<UpdateSalesforceTargetOutput> {
        self.post("/update-salesforce-target", req).await
    }

    /// POST /update-secret-val
    pub async fn update_secret_val(
        &self,
        req: &UpdateSecretVal,
    ) -> Result<UpdateSecretValOutput> {
        self.post("/update-secret-val", req).await
    }

    /// POST /update-ssh-cert-issuer
    pub async fn update_ssh_cert_issuer(
        &self,
        req: &UpdateSshCertIssuer,
    ) -> Result<UpdateSshCertIssuerOutput> {
        self.post("/update-ssh-cert-issuer", req).await
    }

    /// POST /update-ssh-target
    pub async fn update_ssh_target(
        &self,
        req: &UpdateSshTarget,
    ) -> Result<UpdateSshTargetOutput> {
        self.post("/update-ssh-target", req).await
    }

    /// POST /update-ssh-target-details
    pub async fn update_ssh_target_details(
        &self,
        req: &UpdateSshTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-ssh-target-details", req).await
    }

    /// POST /update-target
    pub async fn update_target(
        &self,
        req: &UpdateTarget,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-target", req).await
    }

    /// POST /update-target-details
    pub async fn update_target_details(
        &self,
        req: &UpdateTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-target-details", req).await
    }

    /// POST /update-web-target
    pub async fn update_web_target(
        &self,
        req: &UpdateWebTarget,
    ) -> Result<UpdateWebTargetOutput> {
        self.post("/update-web-target", req).await
    }

    /// POST /update-web-target-details
    pub async fn update_web_target_details(
        &self,
        req: &UpdateWebTargetDetails,
    ) -> Result<UpdateTargetOutput> {
        self.post("/update-web-target-details", req).await
    }

    /// POST /update-windows-target
    pub async fn update_windows_target(
        &self,
        req: &UpdateWindowsTarget,
    ) -> Result<UpdateWindowsTargetOutput> {
        self.post("/update-windows-target", req).await
    }

    /// POST /update-zerossl-target
    pub async fn update_zero_ssl_target(
        &self,
        req: &UpdateZeroSslTarget,
    ) -> Result<UpdateZeroSslTargetOutput> {
        self.post("/update-zerossl-target", req).await
    }

    /// POST /upload-rsa
    pub async fn upload_rsa(
        &self,
        req: &UploadRsa,
    ) -> Result<UploadRsaOutput> {
        self.post("/upload-rsa", req).await
    }

    /// POST /usc-create
    pub async fn usc_create(
        &self,
        req: &UscCreate,
    ) -> Result<UscCreateSecretOutput> {
        self.post("/usc-create", req).await
    }

    /// POST /usc-delete
    pub async fn usc_delete(
        &self,
        req: &UscDelete,
    ) -> Result<UscDeleteSecretOutput> {
        self.post("/usc-delete", req).await
    }

    /// POST /usc-get
    pub async fn usc_get(
        &self,
        req: &UscGet,
    ) -> Result<UscGetSecretOutput> {
        self.post("/usc-get", req).await
    }

    /// POST /usc-list
    pub async fn usc_list(
        &self,
        req: &UscList,
    ) -> Result<UscListSecretsOutput> {
        self.post("/usc-list", req).await
    }

    /// POST /usc-update
    pub async fn usc_update(
        &self,
        req: &UscUpdate,
    ) -> Result<UscUpdateSecretOutput> {
        self.post("/usc-update", req).await
    }

    /// POST /user-event-last-status
    pub async fn get_last_user_event_status(
        &self,
        req: &GetLastUserEventStatus,
    ) -> Result<GetUserEventStatusOutput> {
        self.post("/user-event-last-status", req).await
    }

    /// POST /validate-certificate-challenge
    pub async fn validate_certificate_challenge(
        &self,
        req: &ValidateCertificateChallenge,
    ) -> Result<ValidateCertificateChallengeOutput> {
        self.post("/validate-certificate-challenge", req).await
    }

    /// POST /validate-token
    pub async fn validate_token(
        &self,
        req: &ValidateToken,
    ) -> Result<ValidateTokenOutput> {
        self.post("/validate-token", req).await
    }

    /// POST /vault-address
    pub async fn vault_address(
        &self,
        req: &VaultAddress,
    ) -> Result<VaultAddressOutput> {
        self.post("/vault-address", req).await
    }

    /// POST /verify-data-with-classic-key
    pub async fn verify_data_with_classic_key(
        &self,
        req: &VerifyDataWithClassicKey,
    ) -> Result<VerifyPkiCertOutput> {
        self.post("/verify-data-with-classic-key", req).await
    }

    /// POST /verify-ecdsa
    pub async fn verify_ec_dsa(
        &self,
        req: &VerifyEcDsa,
    ) -> Result<VerifyEcDsaOutput> {
        self.post("/verify-ecdsa", req).await
    }

    /// POST /verify-gpg
    pub async fn verify_gpg(
        &self,
        req: &VerifyGpg,
    ) -> Result<VerifyGpgOutput> {
        self.post("/verify-gpg", req).await
    }

    /// POST /verify-jwt-with-classic-key
    pub async fn verify_jwt_with_classic_key(
        &self,
        req: &VerifyJwtWithClassicKey,
    ) -> Result<VerifyJwtOutput> {
        self.post("/verify-jwt-with-classic-key", req).await
    }

    /// POST /verify-pkcs1
    pub async fn verify_pkcs1(
        &self,
        req: &VerifyPkcs1,
    ) -> Result<VerifyPkcs1Output> {
        self.post("/verify-pkcs1", req).await
    }

    /// POST /verify-pki-cert-with-classic-key
    pub async fn verify_pki_cert_with_classic_key(
        &self,
        req: &VerifyPkiCertWithClassicKey,
    ) -> Result<VerifyPkiCertOutput> {
        self.post("/verify-pki-cert-with-classic-key", req).await
    }

    /// POST /verify-rsassa-pss
    pub async fn verify_rsa_ssa_pss(
        &self,
        req: &VerifyRsaSsaPss,
    ) -> Result<VerifyRsaSsaPssOutput> {
        self.post("/verify-rsassa-pss", req).await
    }

}
