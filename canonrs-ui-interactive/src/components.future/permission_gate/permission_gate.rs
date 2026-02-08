use leptos::callback::Callback;
//! PermissionGate Component
//! Controle de acesso visual baseado em permissões

use leptos::prelude::*;

pub type PermissionChecker = Callback<&'static str, bool>;

#[derive(Clone, Copy)]
pub struct PermissionContext {
    checker: RwSignal<PermissionChecker>,
}

impl PermissionContext {
    pub fn new(checker: PermissionChecker) -> Self {
        Self {
            checker: RwSignal::new(checker),
        }
    }

    pub fn has_permission(&self, permission: &'static str) -> bool {
        self.checker.get().run(permission)
    }

    pub fn set_checker(&self, checker: PermissionChecker) {
        self.checker.set(checker);
    }
}

pub fn use_permissions() -> PermissionContext {
    use_context::<PermissionContext>()
        .expect("PermissionContext must be provided")
}

#[component]
pub fn PermissionProvider(
    children: Children,
    checker: PermissionChecker,
) -> impl IntoView {
    let context = PermissionContext::new(checker);
    provide_context(context);

    view! { {children()} }
}

#[component]
pub fn PermissionGate(
    permission: &'static str,
    children: Children,
    #[prop(optional)] fallback: Option<Children>,
) -> impl IntoView {
    let context = use_permissions();
    let has_perm = context.has_permission(permission);

    if has_perm {
        children().into_any()
    } else {
        fallback.map(|f| f()).unwrap_or_else(|| view! {}.into_any())
    }
}

#[component]
pub fn RequireAnyPermission(
    permissions: &'static [&'static str],
    children: Children,
    #[prop(optional)] fallback: Option<Children>,
) -> impl IntoView {
    let context = use_permissions();
    let has_any = permissions.iter().any(|p| context.has_permission(p));

    if has_any {
        children().into_any()
    } else {
        fallback.map(|f| f()).unwrap_or_else(|| view! {}.into_any())
    }
}

#[component]
pub fn RequireAllPermissions(
    permissions: &'static [&'static str],
    children: Children,
    #[prop(optional)] fallback: Option<Children>,
) -> impl IntoView {
    let context = use_permissions();
    let has_all = permissions.iter().all(|p| context.has_permission(p));

    if has_all {
        children().into_any()
    } else {
        fallback.map(|f| f()).unwrap_or_else(|| view! {}.into_any())
    }
}
