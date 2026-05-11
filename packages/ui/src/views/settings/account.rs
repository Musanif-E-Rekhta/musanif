use dioxus::prelude::*;

use crate::components::AuthField;
use crate::models::{Profile, UpdateProfileInput};
use crate::{api, Route, CURRENT_USER};

#[component]
pub fn AccountPrefs() -> Element {
    let nav = use_navigator();
    let current_user = CURRENT_USER.read();
    let is_authenticated = current_user.is_some();
    let initial = current_user
        .as_ref()
        .map(|u| u.username.chars().next().unwrap_or('?').to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "G".into());
    let display_name = current_user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "Guest".into());
    let display_meta = current_user
        .as_ref()
        .map(|u| u.email.clone())
        .unwrap_or_else(|| "Not signed in".into());
    let plan_label = current_user
        .as_ref()
        .and_then(|u| u.plan_tier.clone())
        .unwrap_or_else(|| "Free".into());
    drop(current_user);

    let sign_out = move |_| {
        let refresh = api::get_refresh_token();
        *CURRENT_USER.write() = None;
        api::clear_all_tokens();
        if let Some(refresh) = refresh {
            spawn(async move {
                let _ = api::logout_user(refresh).await;
            });
        }
    };

    let delete_account = move |_| {
        spawn(async move {
            if api::delete_me().await {
                *CURRENT_USER.write() = None;
                api::clear_all_tokens();
                nav.replace(Route::Home {});
            }
        });
    };

    rsx! {
        div {
            h3 { class: "settings-h3", "Account" }

            div { class: "settings-account-card",
                div { class: "settings-account-avatar", "{initial}" }
                div { style: "flex: 1",
                    p { class: "settings-account-name", "{display_name}" }
                    p { class: "settings-account-meta", "{display_meta}" }
                }
                if is_authenticated {
                    button {
                        class: "is-btn",
                        onclick: sign_out,
                        "Sign out"
                    }
                } else {
                    Link {
                        to: Route::Login {},
                        class: "is-btn is-btn--primary",
                        "Sign In"
                    }
                }
            }

            label { class: "settings-field-label", "Plan" }
            div { class: "settings-plan-row",
                div {
                    p { class: "settings-plan-name", "Musanif Reader" }
                    p { class: "settings-plan-tier", "{plan_label}" }
                }
            }

            if is_authenticated {
                ProfileEditor {}

                ChangePasswordForm {}
            }

            label { class: "settings-field-label", "Two-factor authentication" }
            p { class: "settings-field-hint", "Extra security on sign-in" }
            if is_authenticated {
                Link {
                    to: Route::Setup2fa {},
                    class: "is-btn",
                    "Manage 2FA"
                }
            } else {
                p { class: "settings-field-hint", "Sign in to manage 2FA." }
            }

            if is_authenticated {
                div { class: "settings-actions",
                    button {
                        class: "is-btn is-btn--danger",
                        onclick: delete_account,
                        "Delete account"
                    }
                }
            }
        }
    }
}

#[component]
fn ProfileEditor() -> Element {
    let me = use_resource(|| async { api::fetch_me().await });
    // Snapshot the loaded profile into editable signals once.
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut bio = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut hydrated = use_signal(|| false);
    let mut status = use_signal(|| ProfileSaveState::Idle);

    if !*hydrated.peek() {
        if let Some(Some(payload)) = me.read().as_ref() {
            if let Some(p) = payload.profile.clone() {
                first_name.set(p.first_name.unwrap_or_default());
                last_name.set(p.last_name.unwrap_or_default());
                display_name.set(p.display_name.unwrap_or_default());
                bio.set(p.bio.unwrap_or_default());
                website.set(p.website.unwrap_or_default());
            }
            hydrated.set(true);
        }
    }

    let save = move |e: Event<FormData>| {
        e.prevent_default();
        let input = UpdateProfileInput {
            first_name: blank_to_none(first_name.cloned()),
            last_name: blank_to_none(last_name.cloned()),
            display_name: blank_to_none(display_name.cloned()),
            bio: blank_to_none(bio.cloned()),
            website: blank_to_none(website.cloned()),
            ..Default::default()
        };
        status.set(ProfileSaveState::Saving);
        spawn(async move {
            match api::update_profile(input).await {
                Some(_) => status.set(ProfileSaveState::Saved),
                None => status.set(ProfileSaveState::Error),
            }
        });
    };

    rsx! {
        form {
            class: "settings-section",
            onsubmit: save,

            label { class: "settings-field-label", "Display name" }
            AuthField {
                id: "display_name",
                label: "",
                input_type: "text",
                placeholder: "How you'd like to be addressed",
                value: display_name,
            }

            label { class: "settings-field-label", "First name" }
            AuthField {
                id: "first_name",
                label: "",
                input_type: "text",
                placeholder: "First",
                value: first_name,
            }

            label { class: "settings-field-label", "Last name" }
            AuthField {
                id: "last_name",
                label: "",
                input_type: "text",
                placeholder: "Last",
                value: last_name,
            }

            label { class: "settings-field-label", "Website" }
            AuthField {
                id: "website",
                label: "",
                input_type: "url",
                placeholder: "https://",
                value: website,
            }

            label { class: "settings-field-label", "Bio" }
            textarea {
                class: "settings-textarea",
                rows: 4,
                value: "{bio}",
                oninput: move |e| bio.set(e.value()),
            }

            div { class: "settings-actions",
                button {
                    class: "is-btn is-btn--primary",
                    r#type: "submit",
                    {match *status.read() {
                        ProfileSaveState::Saving => "Saving…",
                        ProfileSaveState::Saved => "Saved ✓",
                        ProfileSaveState::Error => "Save failed — retry",
                        ProfileSaveState::Idle => "Save changes",
                    }}
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ProfileSaveState {
    Idle,
    Saving,
    Saved,
    Error,
}

#[component]
fn ChangePasswordForm() -> Element {
    let old_pw = use_signal(String::new);
    let new_pw = use_signal(String::new);
    let confirm_pw = use_signal(String::new);
    let mut status = use_signal(|| PasswordSaveState::Idle);

    let submit = move |e: Event<FormData>| {
        e.prevent_default();
        let old = old_pw.cloned();
        let new = new_pw.cloned();
        let confirm = confirm_pw.cloned();
        if new.is_empty() || old.is_empty() {
            status.set(PasswordSaveState::Error("Both fields are required".into()));
            return;
        }
        if new != confirm {
            status.set(PasswordSaveState::Error(
                "New password and confirmation don't match".into(),
            ));
            return;
        }
        status.set(PasswordSaveState::Saving);
        spawn(async move {
            if api::change_password(old, new).await {
                status.set(PasswordSaveState::Saved);
            } else {
                status.set(PasswordSaveState::Error("Server rejected the change".into()));
            }
        });
    };

    rsx! {
        form {
            class: "settings-section",
            onsubmit: submit,

            label { class: "settings-field-label", "Change password" }
            AuthField {
                id: "old_password",
                label: "",
                input_type: "password",
                placeholder: "Current password",
                value: old_pw,
            }
            AuthField {
                id: "new_password",
                label: "",
                input_type: "password",
                placeholder: "New password",
                value: new_pw,
            }
            AuthField {
                id: "confirm_password",
                label: "",
                input_type: "password",
                placeholder: "Confirm new password",
                value: confirm_pw,
            }

            div { class: "settings-actions",
                button {
                    class: "is-btn is-btn--primary",
                    r#type: "submit",
                    {match &*status.read() {
                        PasswordSaveState::Saving => "Updating…".to_string(),
                        PasswordSaveState::Saved => "Password updated ✓".to_string(),
                        PasswordSaveState::Error(msg) => format!("Error: {msg}"),
                        PasswordSaveState::Idle => "Update password".to_string(),
                    }}
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum PasswordSaveState {
    Idle,
    Saving,
    Saved,
    Error(String),
}

fn blank_to_none(s: String) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// Suppress dead-code warning: the `Profile` import is used by trait
// resolution through `UserMe.profile` even though it isn't named
// directly in this file.
#[allow(dead_code)]
fn _force_use(_: Profile) {}
