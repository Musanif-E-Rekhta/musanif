//! Authentication GraphQL ops.
//!
//! Each public function takes/returns the existing `models::*` types so
//! views are insulated from cynic. Internally we build a strongly typed
//! cynic operation, send it via `client::run`, and project the response
//! into the domain model.

use cynic::MutationBuilder;

use super::client::run;
use super::schema::schema;
use crate::models;

// ── Cynic types ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct UserResponseGql {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub plan_tier: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct AuthPayload {
    pub token: String,
    pub refresh_token: Option<String>,
    /// async-graphql renames `requires_2fa` to `requires2Fa`
    /// (digit-then-letter boundary capitalises the next letter), which
    /// cynic's default lowerCamelCase rule doesn't reproduce — so we
    /// pin the schema name explicitly.
    #[cynic(rename = "requires2Fa")]
    pub requires_2fa: bool,
    pub challenge: Option<String>,
    pub user: UserResponseGql,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct Setup2FaPayload {
    pub secret: String,
    pub otpauth_url: String,
    pub recovery_codes: Vec<String>,
}

// ── Variables ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct RegisterVars {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct LoginVars {
    pub email: String,
    pub password: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct Login2FaCompleteVars {
    pub challenge: String,
    pub code: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct EmailVars {
    pub email: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ResetPasswordVars {
    pub token: String,
    pub new_password: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TokenVars {
    pub token: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct RefreshTokenVars {
    pub refresh_token: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct Verify2FaVars {
    pub code: String,
    pub recovery_codes: Vec<String>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CodeVars {
    pub code: String,
}

// ── Mutations ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "RegisterVars")]
pub struct RegisterMutation {
    #[arguments(username: $username, email: $email, password: $password)]
    pub register_user: AuthPayload,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "LoginVars")]
pub struct LoginMutation {
    #[arguments(email: $email, password: $password)]
    pub login_user: AuthPayload,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "Login2FaCompleteVars")]
pub struct Login2FaCompleteMutation {
    #[arguments(challenge: $challenge, code: $code)]
    pub login_2_fa_complete: AuthPayload,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "EmailVars")]
pub struct ForgotPasswordMutation {
    #[arguments(email: $email)]
    pub forgot_password: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "ResetPasswordVars")]
pub struct ResetPasswordMutation {
    #[arguments(token: $token, newPassword: $new_password)]
    pub reset_password_with_token: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot")]
pub struct RequestEmailVerificationMutation {
    pub request_email_verification: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "TokenVars")]
pub struct VerifyEmailMutation {
    #[arguments(token: $token)]
    pub verify_email: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "RefreshTokenVars")]
pub struct RefreshTokenMutation {
    #[arguments(refreshToken: $refresh_token)]
    pub refresh_token: AuthPayload,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "RefreshTokenVars")]
pub struct LogoutMutation {
    #[arguments(refreshToken: $refresh_token)]
    pub logout_user: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot")]
pub struct Setup2FaMutation {
    pub setup_2_fa: Setup2FaPayload,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "Verify2FaVars")]
pub struct Verify2FaMutation {
    #[arguments(code: $code, recoveryCodes: $recovery_codes)]
    pub verify_2_fa: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "CodeVars")]
pub struct Disable2FaMutation {
    #[arguments(code: $code)]
    pub disable_2_fa: bool,
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<UserResponseGql> for models::User {
    fn from(u: UserResponseGql) -> Self {
        models::User {
            id: u.id,
            username: u.username,
            email: u.email,
            is_active: u.is_active,
            is_verified: u.is_verified,
            plan_tier: Some(u.plan_tier),
        }
    }
}

impl From<AuthPayload> for models::AuthPayload {
    fn from(p: AuthPayload) -> Self {
        models::AuthPayload {
            token: p.token,
            refresh_token: p.refresh_token,
            requires_2fa: p.requires_2fa,
            challenge: p.challenge,
            user: p.user.into(),
        }
    }
}

impl From<Setup2FaPayload> for models::Setup2faPayload {
    fn from(p: Setup2FaPayload) -> Self {
        models::Setup2faPayload {
            secret: p.secret,
            otpauth_url: p.otpauth_url,
            recovery_codes: p.recovery_codes,
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn register(
    username: String,
    email: String,
    password: String,
) -> Option<models::AuthPayload> {
    let op = RegisterMutation::build(RegisterVars {
        username,
        email,
        password,
    });
    run(op).await.map(|d| d.register_user.into())
}

pub async fn login(email: String, password: String) -> Option<models::AuthPayload> {
    let op = LoginMutation::build(LoginVars { email, password });
    run(op).await.map(|d| d.login_user.into())
}

pub async fn login_2fa_complete(
    challenge: String,
    code: String,
) -> Option<models::AuthPayload> {
    let op = Login2FaCompleteMutation::build(Login2FaCompleteVars { challenge, code });
    run(op).await.map(|d| d.login_2_fa_complete.into())
}

pub async fn forgot_password(email: String) -> bool {
    let op = ForgotPasswordMutation::build(EmailVars { email });
    run(op).await.map(|d| d.forgot_password).unwrap_or(false)
}

pub async fn reset_password_with_token(token: String, new_password: String) -> bool {
    let op = ResetPasswordMutation::build(ResetPasswordVars {
        token,
        new_password,
    });
    run(op)
        .await
        .map(|d| d.reset_password_with_token)
        .unwrap_or(false)
}

pub async fn request_email_verification() -> bool {
    let op = RequestEmailVerificationMutation::build(());
    run(op)
        .await
        .map(|d| d.request_email_verification)
        .unwrap_or(false)
}

pub async fn verify_email(token: String) -> bool {
    let op = VerifyEmailMutation::build(TokenVars { token });
    run(op).await.map(|d| d.verify_email).unwrap_or(false)
}

pub async fn refresh_token(refresh_token: String) -> Option<models::AuthPayload> {
    let op = RefreshTokenMutation::build(RefreshTokenVars { refresh_token });
    run(op).await.map(|d| d.refresh_token.into())
}

pub async fn logout_user(refresh_token: String) -> bool {
    let op = LogoutMutation::build(RefreshTokenVars { refresh_token });
    run(op).await.map(|d| d.logout_user).unwrap_or(false)
}

pub async fn setup_2fa() -> Option<models::Setup2faPayload> {
    let op = Setup2FaMutation::build(());
    run(op).await.map(|d| d.setup_2_fa.into())
}

pub async fn verify_2fa(code: String, recovery_codes: Vec<String>) -> bool {
    let op = Verify2FaMutation::build(Verify2FaVars {
        code,
        recovery_codes,
    });
    run(op).await.map(|d| d.verify_2_fa).unwrap_or(false)
}

pub async fn disable_2fa(code: String) -> bool {
    let op = Disable2FaMutation::build(CodeVars { code });
    run(op).await.map(|d| d.disable_2_fa).unwrap_or(false)
}
