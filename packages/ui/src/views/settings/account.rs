use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdShieldCheck, Icon};

use crate::components::AuthField;
use crate::models::{Profile, TwoFactorStatus, UpdateProfileInput};
use crate::{api, Route, CURRENT_USER};

/// Account section island contents (used inside the SettingsSection
/// wrapper, which provides the eyebrow). Three sub-blocks for the
/// signed-in user, in order of frequency-of-use:
///
/// - Identity header (avatar, display name, email, plan, sign out).
/// - Profile auto-save form (display name, bio, website).
/// - Password change (explicit submit because auth demands it).
/// - 2FA status + manage button.
/// - Danger zone (inline two-step delete confirmation).
///
/// Signed-out users see only the identity header collapsed to a single
/// invite line. Reading / Display / Library / About panes still work
/// for signed-out users; only this section gates.
#[component]
pub fn AccountPrefs() -> Element {
    let signed_in = CURRENT_USER.read().is_some();

    if !signed_in {
        return rsx! { SignedOutCard {} };
    }

    rsx! {
        IdentityHeader {}
        ProfileEditor {}
        ChangePasswordForm {}
        TwoFactorRow {}
        DangerZone {}
    }
}

#[component]
fn SignedOutCard() -> Element {
    rsx! {
        div { class: "settings-signed-out",
            p { class: "settings-signed-out-headline", "Sign in to manage your account." }
            p { class: "settings-signed-out-body",
                "Preferences below work without an account; profile, password, and 2FA need one."
            }
            div { class: "settings-signed-out-actions",
                Link {
                    to: Route::Login {},
                    class: "is-btn is-btn--primary",
                    "Sign in"
                }
                Link {
                    to: Route::Signup {},
                    class: "is-btn",
                    "Create an account"
                }
            }
        }
    }
}

#[component]
fn IdentityHeader() -> Element {
    let user = CURRENT_USER.read();
    let initial = user
        .as_ref()
        .map(|u| {
            u.username
                .chars()
                .next()
                .unwrap_or('?')
                .to_ascii_uppercase()
                .to_string()
        })
        .unwrap_or_else(|| "G".into());
    let name = user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_else(|| "Guest".into());
    let email = user
        .as_ref()
        .map(|u| u.email.clone())
        .unwrap_or_else(|| "Not signed in".into());
    let plan = user
        .as_ref()
        .and_then(|u| u.plan_tier.clone())
        .unwrap_or_else(|| "Free".into());
    drop(user);

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

    rsx! {
        div { class: "settings-identity",
            div { class: "settings-identity-avatar", "{initial}" }
            div { class: "settings-identity-body",
                p { class: "settings-identity-name", "{name}" }
                p { class: "settings-identity-meta",
                    "{email}"
                    span { class: "settings-identity-sep", "·" }
                    span { class: "settings-identity-plan", "{plan}" }
                }
            }
            button {
                class: "is-btn",
                onclick: sign_out,
                "Sign out"
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SaveState {
    Idle,
    Saving,
    Saved,
    Failed,
}

#[component]
fn ProfileEditor() -> Element {
    let me = use_resource(|| async { api::fetch_me().await });
    let mut display_name = use_signal(String::new);
    let mut bio = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut hydrated = use_signal(|| false);
    let mut state = use_signal(|| SaveState::Idle);

    if !*hydrated.peek() {
        if let Some(Some(payload)) = me.read().as_ref() {
            if let Some(p) = payload.profile.clone() {
                display_name.set(p.display_name.unwrap_or_default());
                bio.set(p.bio.unwrap_or_default());
                website.set(p.website.unwrap_or_default());
            }
            hydrated.set(true);
        }
    }

    let label = match *state.read() {
        SaveState::Idle => "",
        SaveState::Saving => "Saving",
        SaveState::Saved => "Saved",
        SaveState::Failed => "Save failed, retry",
    };

    rsx! {
        div { class: "settings-subblock",
            div { class: "settings-subblock-head",
                h3 { class: "settings-subblock-title", "Profile" }
                if !label.is_empty() {
                    SaveChip { state: *state.read(), label: label.to_string() }
                }
            }

            // Single onfocusout on the wrapper: any field losing focus
            // inside this div triggers an auto-save. focusout bubbles
            // from children, so we don't need per-field handlers.
            div {
                class: "settings-profile-form",
                onfocusout: move |_| {
                    let input = UpdateProfileInput {
                        display_name: blank_to_none(display_name.cloned()),
                        bio: blank_to_none(bio.cloned()),
                        website: blank_to_none(website.cloned()),
                        ..Default::default()
                    };
                    state.set(SaveState::Saving);
                    spawn(async move {
                        match api::update_profile(input).await {
                            Some(_) => state.set(SaveState::Saved),
                            None => state.set(SaveState::Failed),
                        }
                    });
                },

                label { class: "settings-field-label", "Display name" }
                AuthField {
                    id: "display_name",
                    label: "",
                    input_type: "text",
                    placeholder: "What we call you in the app",
                    value: display_name,
                }

                label { class: "settings-field-label", "Bio" }
                textarea {
                    class: "settings-textarea",
                    rows: 3,
                    placeholder: "A line or two about you.",
                    value: "{bio}",
                    oninput: move |e| bio.set(e.value()),
                }

                label { class: "settings-field-label", "Website" }
                AuthField {
                    id: "website",
                    label: "",
                    input_type: "url",
                    placeholder: "https://",
                    value: website,
                }
            }
        }
    }
}

#[component]
fn SaveChip(state: SaveState, label: String) -> Element {
    let class = match state {
        SaveState::Saving => "save-chip save-chip--saving",
        SaveState::Saved => "save-chip save-chip--saved",
        SaveState::Failed => "save-chip save-chip--failed",
        SaveState::Idle => "save-chip",
    };
    rsx! { span { class: "{class}", "{label}" } }
}

#[derive(Clone, PartialEq)]
enum PasswordState {
    Idle,
    Saving,
    Saved,
    Error(String),
}

#[component]
fn ChangePasswordForm() -> Element {
    let mut old_pw = use_signal(String::new);
    let mut new_pw = use_signal(String::new);
    let mut confirm_pw = use_signal(String::new);
    let mut state = use_signal(|| PasswordState::Idle);

    let submit = move |e: Event<FormData>| {
        e.prevent_default();
        let old = old_pw.cloned();
        let new = new_pw.cloned();
        let confirm = confirm_pw.cloned();
        if old.is_empty() || new.is_empty() {
            state.set(PasswordState::Error("Fill both fields.".into()));
            return;
        }
        if new != confirm {
            state.set(PasswordState::Error("New password and confirm don't match.".into()));
            return;
        }
        state.set(PasswordState::Saving);
        spawn(async move {
            if api::change_password(old, new).await {
                state.set(PasswordState::Saved);
                old_pw.set(String::new());
                new_pw.set(String::new());
                confirm_pw.set(String::new());
            } else {
                state.set(PasswordState::Error("Server rejected the change.".into()));
            }
        });
    };

    let button_label = match &*state.read() {
        PasswordState::Saving => "Updating".to_string(),
        PasswordState::Saved => "Password updated".to_string(),
        PasswordState::Error(_) => "Update password".to_string(),
        PasswordState::Idle => "Update password".to_string(),
    };
    let error = match &*state.read() {
        PasswordState::Error(msg) => Some(msg.clone()),
        _ => None,
    };

    rsx! {
        div { class: "settings-subblock",
            div { class: "settings-subblock-head",
                h3 { class: "settings-subblock-title", "Change password" }
            }

            form {
                onsubmit: submit,

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

                if let Some(msg) = error {
                    p { class: "settings-inline-error", "{msg}" }
                }

                div { class: "settings-actions",
                    button {
                        class: "is-btn is-btn--primary",
                        r#type: "submit",
                        "{button_label}"
                    }
                }
            }
        }
    }
}

#[component]
fn TwoFactorRow() -> Element {
    let status = use_resource(|| async { api::fetch_my_2fa_status().await });

    let enabled = matches!(
        &*status.read(),
        Some(Some(TwoFactorStatus { enabled: true, .. }))
    );
    let loading = status.read().is_none();

    let dot_class = if enabled {
        "settings-twofa-dot settings-twofa-dot--on"
    } else {
        "settings-twofa-dot"
    };
    let state_label = if loading {
        "Checking"
    } else if enabled {
        "On"
    } else {
        "Off"
    };
    let manage_label = if enabled { "Manage" } else { "Set up" };

    rsx! {
        div { class: "settings-subblock",
            div { class: "settings-subblock-head",
                h3 { class: "settings-subblock-title", "Two-factor authentication" }
            }

            div { class: "settings-twofa",
                div { class: "settings-twofa-status",
                    Icon {
                        icon: LdShieldCheck,
                        width: 18,
                        height: 18,
                        class: "settings-twofa-icon",
                    }
                    span { class: "{dot_class}" }
                    span { class: "settings-twofa-label", "{state_label}" }
                }
                Link {
                    to: Route::Setup2fa {},
                    class: "is-btn",
                    "{manage_label}"
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DeleteState {
    Idle,
    Confirming,
    Deleting,
}

#[component]
fn DangerZone() -> Element {
    let nav = use_navigator();
    let mut state = use_signal(|| DeleteState::Idle);

    let start_confirm = move |_| state.set(DeleteState::Confirming);
    let cancel = move |_| state.set(DeleteState::Idle);
    let confirm_delete = move |_| {
        state.set(DeleteState::Deleting);
        spawn(async move {
            if api::delete_me().await {
                *CURRENT_USER.write() = None;
                api::clear_all_tokens();
                nav.replace(Route::Home {});
            } else {
                state.set(DeleteState::Idle);
            }
        });
    };

    rsx! {
        div { class: "settings-subblock settings-danger",
            div { class: "settings-subblock-head",
                h3 { class: "settings-subblock-title", "Danger zone" }
            }
            p { class: "settings-danger-body",
                "Deleting your account removes your profile, bookmarks, and highlights. The action cannot be undone."
            }

            match *state.read() {
                DeleteState::Idle => rsx! {
                    button {
                        class: "is-btn is-btn--danger",
                        onclick: start_confirm,
                        "Delete account"
                    }
                },
                DeleteState::Confirming => rsx! {
                    div { class: "settings-danger-confirm",
                        button {
                            class: "is-btn is-btn--danger",
                            onclick: confirm_delete,
                            "Confirm delete"
                        }
                        button {
                            class: "is-btn",
                            onclick: cancel,
                            "Cancel"
                        }
                    }
                },
                DeleteState::Deleting => rsx! {
                    div { class: "settings-danger-confirm",
                        button {
                            class: "is-btn is-btn--danger",
                            disabled: true,
                            "Deleting"
                        }
                    }
                },
            }
        }
    }
}

fn blank_to_none(s: String) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[allow(dead_code)]
fn _force_use(_: Profile) {}
