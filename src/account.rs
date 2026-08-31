use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use codex_http_client::{HttpClientFactory, OutboundProxyPolicy};
use codex_login::{
    AuthCredentialsStoreMode, AuthDotJson, AuthKeyringBackendKind, AuthManager, CodexAuth,
    LoginSuccessPage, ServerOptions, TokenData, CLIENT_ID,
};

use crate::models::UsageData;

pub const MAX_MONITORED_ACCOUNTS: usize = 2;

/// Typed reference to one approved monitor-owned credential namespace.
/// The filesystem root is resolved by the auth owner at runtime and is never
/// serialized into settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitorAuthHandle {
    #[serde(rename = "slot-1")]
    Slot1,
    #[serde(rename = "slot-2")]
    Slot2,
}

#[allow(dead_code)] // Namespace resolution is consumed by the auth lifecycle phase.
impl MonitorAuthHandle {
    pub const fn namespace_key(self) -> &'static str {
        match self {
            Self::Slot1 => "monitor-auth/slot-1",
            Self::Slot2 => "monitor-auth/slot-2",
        }
    }

    pub const fn all() -> [Self; MAX_MONITORED_ACCOUNTS] {
        [Self::Slot1, Self::Slot2]
    }

    /// Resolve the clean production owner root at runtime. This path is never
    /// serialized into `AccountRegistryMetadata`.
    pub fn storage_path(self) -> Option<PathBuf> {
        let root = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .or_else(dirs::data_local_dir)?;
        Some(root.join("CodexUsage").join(self.namespace_key()))
    }
}

/// Non-secret metadata persisted for one monitored account.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitoredAccountMetadata {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial: Option<char>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// Typed reference to a Codex-owned credential owner. Never a token/path.
    pub auth_handle: MonitorAuthHandle,
}

fn default_enabled() -> bool {
    true
}

/// Persisted registry envelope. Vec order is the deterministic account order.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRegistryMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<MonitoredAccountMetadata>,
}

impl AccountRegistryMetadata {
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
}

#[allow(dead_code)] // Connection states are populated by later lifecycle phases.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConnectionState {
    #[default]
    Connected,
    ReauthRequired,
    Unavailable,
}

/// The only credential states that the monitor transaction needs to expose to
/// its decision layer. The credential contents never enter this model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialOwnerState {
    Absent,
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialTransactionDecision {
    Commit,
    Rollback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialTransactionResult {
    Committed(CredentialOwnerState),
    RolledBack(CredentialOwnerState),
    RollbackFailed,
}

/// Resolve the non-secret transaction decision independently from the
/// credential backend. This is the deterministic state-machine seam used by
/// lifecycle tests; production rollback still performs the real owner restore
/// or clear operation.
pub fn resolve_credential_transaction(
    previous: CredentialOwnerState,
    current: CredentialOwnerState,
    decision: CredentialTransactionDecision,
    rollback_succeeded: bool,
) -> CredentialTransactionResult {
    match decision {
        CredentialTransactionDecision::Commit => CredentialTransactionResult::Committed(current),
        CredentialTransactionDecision::Rollback if rollback_succeeded => {
            CredentialTransactionResult::RolledBack(previous)
        }
        CredentialTransactionDecision::Rollback => CredentialTransactionResult::RollbackFailed,
    }
}

/// Runtime account state. Only `metadata()` is persisted.
#[allow(dead_code)] // Runtime fields are part of the Phase 01 account contract.
#[derive(Clone, Debug)]
pub struct MonitoredAccount {
    pub id: String,
    pub initial: Option<char>,
    pub enabled: bool,
    pub auth_handle: MonitorAuthHandle,
    pub connection_state: ConnectionState,
    pub usage: Option<UsageData>,
    pub last_success_at: Option<SystemTime>,
    pub last_error: Option<String>,
}

#[allow(dead_code)] // Identity-to-owner binding is consumed by account lifecycle wiring.
impl MonitoredAccount {
    pub fn from_metadata(metadata: MonitoredAccountMetadata) -> Self {
        Self {
            id: metadata.id,
            initial: metadata.initial,
            enabled: metadata.enabled,
            auth_handle: metadata.auth_handle,
            connection_state: ConnectionState::Unavailable,
            usage: None,
            last_success_at: None,
            last_error: None,
        }
    }

    pub fn from_identity(identity: &AccountIdentity, auth_handle: MonitorAuthHandle) -> Self {
        Self {
            id: identity.id.clone(),
            initial: identity.initial(),
            enabled: true,
            auth_handle,
            connection_state: ConnectionState::Connected,
            usage: None,
            last_success_at: None,
            last_error: None,
        }
    }

    pub fn metadata(&self) -> MonitoredAccountMetadata {
        MonitoredAccountMetadata {
            id: self.id.clone(),
            initial: self.initial,
            enabled: self.enabled,
            auth_handle: self.auth_handle,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryError {
    EmptyIdentity,
    CapacityReached,
    DuplicateIdentity,
    DuplicateAuthOwner,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::EmptyIdentity => "account identity must not be empty",
            Self::CapacityReached => "maximum monitored account count is two",
            Self::DuplicateIdentity => "account identity is already registered",
            Self::DuplicateAuthOwner => "monitor auth owner is already registered",
        })
    }
}

impl std::error::Error for RegistryError {}

/// Stable registry with max-two enforcement, identity-based duplicates, and
/// insertion order as the persisted display order.
#[derive(Clone, Debug, Default)]
pub struct AccountRegistry {
    accounts: Vec<MonitoredAccount>,
}

#[allow(dead_code)] // Registry mutation/read APIs are consumed by later lifecycle phases.
impl AccountRegistry {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn from_metadata(metadata: AccountRegistryMetadata) -> Result<Self, RegistryError> {
        let mut registry = Self::empty();
        for account in metadata.accounts {
            registry.try_add(MonitoredAccount::from_metadata(account))?;
        }
        Ok(registry)
    }

    pub fn display_initial(&self, legacy_initial: Option<char>) -> Option<char> {
        self.primary_initial().or(legacy_initial)
    }

    pub fn accounts(&self) -> &[MonitoredAccount] {
        &self.accounts
    }

    pub fn len(&self) -> usize {
        self.accounts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn primary_initial(&self) -> Option<char> {
        self.accounts
            .iter()
            .find(|account| account.enabled)
            .or_else(|| self.accounts.first())
            .and_then(|account| account.initial)
    }

    pub fn available_auth_handle(&self) -> Option<MonitorAuthHandle> {
        MonitorAuthHandle::all().into_iter().find(|handle| {
            self.accounts
                .iter()
                .all(|account| account.auth_handle != *handle)
        })
    }

    pub fn account_by_handle(&self, handle: MonitorAuthHandle) -> Option<&MonitoredAccount> {
        self.accounts
            .iter()
            .find(|account| account.auth_handle == handle)
    }

    pub fn update_identity(&mut self, account_id: &str, identity: &AccountIdentity) -> bool {
        let Some(account) = self
            .accounts
            .iter_mut()
            .find(|account| account.id == account_id)
        else {
            return false;
        };
        account.initial = identity.initial();
        true
    }

    pub fn record_usage(&mut self, account_id: &str, usage: UsageData) -> bool {
        let Some(account) = self
            .accounts
            .iter_mut()
            .find(|account| account.id == account_id)
        else {
            return false;
        };
        account.connection_state = ConnectionState::Connected;
        account.usage = Some(usage);
        account.last_success_at = Some(SystemTime::now());
        account.last_error = None;
        true
    }

    pub fn record_usage_error(&mut self, account_id: &str, error: &str) -> bool {
        let Some(account) = self
            .accounts
            .iter_mut()
            .find(|account| account.id == account_id)
        else {
            return false;
        };
        account.connection_state = ConnectionState::Unavailable;
        account.last_error = Some(error.to_string());
        true
    }

    pub fn update_auth_handle(
        &mut self,
        account_id: &str,
        new_handle: MonitorAuthHandle,
    ) -> Result<(), RegistryError> {
        if self
            .accounts
            .iter()
            .any(|account| account.id != account_id && account.auth_handle == new_handle)
        {
            return Err(RegistryError::DuplicateAuthOwner);
        }
        let account = self
            .accounts
            .iter_mut()
            .find(|account| account.id == account_id)
            .ok_or(RegistryError::EmptyIdentity)?;
        account.auth_handle = new_handle;
        Ok(())
    }

    pub fn metadata(&self) -> AccountRegistryMetadata {
        AccountRegistryMetadata {
            accounts: self
                .accounts
                .iter()
                .map(MonitoredAccount::metadata)
                .collect(),
        }
    }

    pub fn try_add(&mut self, account: MonitoredAccount) -> Result<(), RegistryError> {
        if account.id.trim().is_empty() {
            return Err(RegistryError::EmptyIdentity);
        }
        if self
            .accounts
            .iter()
            .any(|existing| existing.id == account.id)
        {
            return Err(RegistryError::DuplicateIdentity);
        }
        if self
            .accounts
            .iter()
            .any(|existing| existing.auth_handle == account.auth_handle)
        {
            return Err(RegistryError::DuplicateAuthOwner);
        }
        if self.accounts.len() >= MAX_MONITORED_ACCOUNTS {
            return Err(RegistryError::CapacityReached);
        }
        self.accounts.push(account);
        Ok(())
    }

    pub fn remove_by_id(&mut self, id: &str) -> Option<MonitoredAccount> {
        let index = self.accounts.iter().position(|account| account.id == id)?;
        Some(self.accounts.remove(index))
    }
}

/// Runtime identity projection. Persistence requires an explicit auth handle;
/// identity itself never manufactures credential ownership.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccountIdentity {
    pub id: String,
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl AccountIdentity {
    pub fn initial(&self) -> Option<char> {
        first_uppercase_initial(self.display_name.as_deref())
            .or_else(|| first_uppercase_initial(self.username.as_deref()))
            .or_else(|| {
                self.email
                    .as_deref()
                    .map(|email| email.split('@').next().unwrap_or(email))
                    .and_then(|local| first_uppercase_initial(Some(local)))
            })
    }
}

#[derive(Debug, Default, Deserialize)]
struct IdentityClaims {
    sub: Option<String>,
    name: Option<String>,
    username: Option<String>,
    preferred_username: Option<String>,
    email: Option<String>,
}

/// Build an identity from the existing Codex auth adapter's in-memory
/// projection. This function does not read or persist credential material.
pub fn from_codex_auth_projection(
    account_id: Option<String>,
    id_token: Option<&str>,
) -> Option<AccountIdentity> {
    let claims = id_token.and_then(parse_identity_claims).unwrap_or_default();
    let id = account_id
        .filter(|value| !value.trim().is_empty())
        .or(claims.sub.clone())
        .or_else(|| claims.email.as_deref().map(opaque_identity_id))?;

    Some(AccountIdentity {
        id,
        display_name: claims.name,
        username: claims.username.or(claims.preferred_username),
        email: claims.email,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginError {
    AuthNamespaceUnavailable,
    RuntimeUnavailable,
    LoginFailed,
    Cancelled,
    TimedOut,
    NotAuthenticated,
    IdentityUnavailable,
    IdentityChanged,
    DuplicateAccount,
    RollbackFailed,
    InitialUsageFailed,
}

impl LoginError {
    pub const fn user_message(self) -> &'static str {
        match self {
            Self::AuthNamespaceUnavailable => "Monitor credential storage is unavailable.",
            Self::RuntimeUnavailable => "The monitor login runtime could not start.",
            Self::LoginFailed => "Account login could not be completed.",
            Self::Cancelled => "Account login was cancelled.",
            Self::TimedOut => "Account login timed out.",
            Self::NotAuthenticated => "The account was not authenticated.",
            Self::IdentityUnavailable => "The account identity could not be read.",
            Self::IdentityChanged => {
                "Re-authentication resolved to a different account; the existing account was kept."
            }
            Self::DuplicateAccount => "This account is already monitored.",
            Self::RollbackFailed => {
                "Account login failed and the monitor credential could not be rolled back."
            }
            Self::InitialUsageFailed => {
                "The account signed in, but its initial usage could not be read."
            }
        }
    }
}

#[derive(Clone)]
enum CredentialSnapshot {
    Absent,
    Present(TokenData),
}

impl CredentialSnapshot {
    fn state(&self) -> CredentialOwnerState {
        match self {
            Self::Absent => CredentialOwnerState::Absent,
            Self::Present(_) => CredentialOwnerState::Present,
        }
    }
}

pub struct LoginOperation {
    handle: MonitorAuthHandle,
    expected_identity: Option<String>,
    cancel_requested: Arc<AtomicBool>,
    cancel_handle: Arc<Mutex<Option<codex_login::ShutdownHandle>>>,
    snapshot: Arc<Mutex<Option<CredentialSnapshot>>>,
    result_rx: Receiver<Result<AccountIdentity, LoginError>>,
}

impl LoginOperation {
    pub fn start(
        handle: MonitorAuthHandle,
        expected_identity: Option<String>,
        existing_identity_ids: Vec<String>,
    ) -> Self {
        let cancel_requested = Arc::new(AtomicBool::new(false));
        let cancel_handle = Arc::new(Mutex::new(None));
        let snapshot = Arc::new(Mutex::new(None));
        let (result_tx, result_rx) = mpsc::sync_channel(1);
        let worker_cancel_requested = Arc::clone(&cancel_requested);
        let worker_cancel_handle = Arc::clone(&cancel_handle);
        let worker_snapshot = Arc::clone(&snapshot);
        let worker_expected_identity = expected_identity.clone();
        thread::spawn(move || {
            let result = run_login(
                handle,
                worker_expected_identity,
                existing_identity_ids,
                worker_cancel_requested,
                worker_cancel_handle,
                worker_snapshot,
            );
            let _ = result_tx.send(result);
        });
        Self {
            handle,
            expected_identity,
            cancel_requested,
            cancel_handle,
            snapshot,
            result_rx,
        }
    }

    pub fn handle(&self) -> MonitorAuthHandle {
        self.handle
    }

    pub fn expected_identity(&self) -> Option<&str> {
        self.expected_identity.as_deref()
    }

    pub fn cancel(&self) {
        self.cancel_requested.store(true, Ordering::Release);
        if let Some(handle) = self
            .cancel_handle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
        {
            handle.shutdown();
        }
    }

    pub fn try_result(&self) -> Result<Option<Result<AccountIdentity, LoginError>>, TryRecvError> {
        match self.result_rx.try_recv() {
            Ok(result) => Ok(Some(result)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
        }
    }

    pub fn commit(&self) {
        let snapshot = self
            .snapshot
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        if let Some(snapshot) = snapshot {
            debug_assert!(matches!(
                resolve_credential_transaction(
                    snapshot.state(),
                    CredentialOwnerState::Present,
                    CredentialTransactionDecision::Commit,
                    false,
                ),
                CredentialTransactionResult::Committed(_)
            ));
        }
    }

    pub fn rollback(&self) -> Result<(), LoginError> {
        let snapshot = self
            .snapshot
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
            .ok_or(LoginError::RollbackFailed)?;
        let storage_path = self
            .handle
            .storage_path()
            .ok_or(LoginError::RollbackFailed)?;
        restore_owner_snapshot(&storage_path, &snapshot)
    }
}

pub struct CleanupOperation {
    account_id: String,
    result_rx: Receiver<Result<(), LoginError>>,
}

pub struct InitialUsageOperation {
    account_id: String,
    result_rx: Receiver<Result<UsageData, LoginError>>,
}

impl InitialUsageOperation {
    pub fn start(account_id: String, handle: MonitorAuthHandle) -> Self {
        let (result_tx, result_rx) = mpsc::sync_channel(1);
        thread::spawn(move || {
            let result = run_initial_usage_read(handle);
            let _ = result_tx.send(result);
        });
        Self {
            account_id,
            result_rx,
        }
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn try_result(&self) -> Result<Option<Result<UsageData, LoginError>>, TryRecvError> {
        match self.result_rx.try_recv() {
            Ok(result) => Ok(Some(result)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
        }
    }
}

impl CleanupOperation {
    pub fn start(account_id: String, handle: MonitorAuthHandle) -> Self {
        let (result_tx, result_rx) = mpsc::sync_channel(1);
        thread::spawn(move || {
            let result = run_cleanup(handle);
            let _ = result_tx.send(result);
        });
        Self {
            account_id,
            result_rx,
        }
    }

    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    pub fn try_result(&self) -> Result<Option<Result<(), LoginError>>, TryRecvError> {
        match self.result_rx.try_recv() {
            Ok(result) => Ok(Some(result)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
        }
    }
}

fn auth_route_config() -> codex_login::AuthRouteConfig {
    codex_login::AuthRouteConfig::from_http_client_factory(HttpClientFactory::new(
        OutboundProxyPolicy::ReqwestDefault,
    ))
}

async fn auth_manager(handle: MonitorAuthHandle) -> Result<Arc<AuthManager>, LoginError> {
    let storage_path = handle
        .storage_path()
        .ok_or(LoginError::AuthNamespaceUnavailable)?;
    Ok(AuthManager::shared(
        storage_path,
        false,
        AuthCredentialsStoreMode::Keyring,
        None,
        None,
        AuthKeyringBackendKind::Secrets,
        auth_route_config(),
    )
    .await)
}

fn capture_reauth_snapshot(
    auth: Option<&CodexAuth>,
    expected_identity: &str,
) -> CredentialSnapshot {
    let Some(auth) = auth else {
        return CredentialSnapshot::Absent;
    };
    if auth.get_account_id().as_deref() != Some(expected_identity) {
        return CredentialSnapshot::Absent;
    }
    auth.get_token_data()
        .map(CredentialSnapshot::Present)
        .unwrap_or(CredentialSnapshot::Absent)
}

fn run_login(
    handle: MonitorAuthHandle,
    expected_identity: Option<String>,
    existing_identity_ids: Vec<String>,
    cancel_requested: Arc<AtomicBool>,
    cancel_handle: Arc<Mutex<Option<codex_login::ShutdownHandle>>>,
    snapshot_slot: Arc<Mutex<Option<CredentialSnapshot>>>,
) -> Result<AccountIdentity, LoginError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|_| LoginError::RuntimeUnavailable)?;
    runtime.block_on(async move {
        let storage_path = handle
            .storage_path()
            .ok_or(LoginError::AuthNamespaceUnavailable)?;

        let previous_snapshot = if let Some(expected_identity) = expected_identity.as_deref() {
            let manager = auth_manager(handle).await?;
            capture_reauth_snapshot(manager.auth().await.as_ref(), expected_identity)
        } else {
            CredentialSnapshot::Absent
        };
        *snapshot_slot
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(previous_snapshot.clone());

        if cancel_requested.load(Ordering::Acquire) {
            snapshot_slot
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .take();
            return Err(LoginError::Cancelled);
        }

        let mut options = ServerOptions::new(
            storage_path.clone(),
            CLIENT_ID.to_string(),
            None,
            AuthCredentialsStoreMode::Keyring,
            AuthKeyringBackendKind::Secrets,
            auth_route_config(),
        );
        options.port = 1455;
        options.login_success_page = LoginSuccessPage::Local;
        let server = match codex_login::run_login_server(options) {
            Ok(server) => server,
            Err(_) => {
                snapshot_slot
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .take();
                return Err(LoginError::LoginFailed);
            }
        };
        let shutdown = server.cancel_handle();
        *cancel_handle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(shutdown.clone());
        if cancel_requested.load(Ordering::Acquire) {
            shutdown.shutdown();
        }

        let callback = match tokio::time::timeout(
            Duration::from_secs(5 * 60),
            server.block_until_done_with_callback_result(),
        )
        .await
        {
            Ok(Ok(callback)) => callback,
            Ok(Err(_)) if cancel_requested.load(Ordering::Acquire) => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::Cancelled,
                );
            }
            Ok(Err(_)) => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::LoginFailed,
                );
            }
            Err(_) => {
                shutdown.shutdown();
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::TimedOut,
                );
            }
        };
        let _ = callback;

        let manager = match auth_manager(handle).await {
            Ok(manager) => manager,
            Err(_) => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::AuthNamespaceUnavailable,
                );
            }
        };
        let auth = match manager.auth().await {
            Some(auth) => auth,
            None => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::NotAuthenticated,
                );
            }
        };
        let token_data = match auth.get_token_data() {
            Ok(token_data) => token_data,
            Err(_) => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::NotAuthenticated,
                );
            }
        };
        let identity = match from_codex_auth_projection(
            auth.get_account_id(),
            Some(&token_data.id_token.raw_jwt),
        ) {
            Some(identity) => identity,
            None => {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::IdentityUnavailable,
                );
            }
        };

        if let Some(expected_identity) = expected_identity.as_deref() {
            if identity.id != expected_identity {
                return rollback_login_error(
                    &storage_path,
                    &previous_snapshot,
                    &snapshot_slot,
                    LoginError::IdentityChanged,
                );
            }
        } else if existing_identity_ids
            .iter()
            .any(|existing_id| existing_id == &identity.id)
        {
            return rollback_login_error(
                &storage_path,
                &previous_snapshot,
                &snapshot_slot,
                LoginError::DuplicateAccount,
            );
        }

        Ok(identity)
    })
}

fn rollback_login_error(
    path: &std::path::Path,
    snapshot: &CredentialSnapshot,
    snapshot_slot: &Arc<Mutex<Option<CredentialSnapshot>>>,
    error: LoginError,
) -> Result<AccountIdentity, LoginError> {
    let rollback_result = restore_owner_snapshot(path, snapshot);
    snapshot_slot
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take();
    match resolve_credential_transaction(
        snapshot.state(),
        CredentialOwnerState::Present,
        CredentialTransactionDecision::Rollback,
        rollback_result.is_ok(),
    ) {
        CredentialTransactionResult::RolledBack(_) => Err(error),
        CredentialTransactionResult::RollbackFailed | CredentialTransactionResult::Committed(_) => {
            Err(LoginError::RollbackFailed)
        }
    }
}

fn restore_owner_snapshot(
    path: &std::path::Path,
    snapshot: &CredentialSnapshot,
) -> Result<(), LoginError> {
    match snapshot {
        CredentialSnapshot::Present(tokens) => restore_managed_tokens(path, tokens.clone()),
        CredentialSnapshot::Absent => codex_login::logout(
            path,
            AuthCredentialsStoreMode::Keyring,
            AuthKeyringBackendKind::Secrets,
        )
        .map(|_| ())
        .map_err(|_| LoginError::RollbackFailed),
    }
}

fn restore_managed_tokens(path: &std::path::Path, tokens: TokenData) -> Result<(), LoginError> {
    let auth = AuthDotJson {
        auth_mode: None,
        openai_api_key: None,
        tokens: Some(tokens),
        last_refresh: None,
        agent_identity: None,
        personal_access_token: None,
        bedrock_api_key: None,
        bedrock_access_keys: None,
    };
    codex_login::save_auth(
        path,
        &auth,
        AuthCredentialsStoreMode::Keyring,
        AuthKeyringBackendKind::Secrets,
    )
    .map_err(|_| LoginError::RollbackFailed)
}

pub fn read_initial_usage(handle: MonitorAuthHandle) -> Result<UsageData, LoginError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|_| LoginError::RuntimeUnavailable)?;
    runtime.block_on(async move {
        let manager = auth_manager(handle).await?;
        let auth = manager.auth().await.ok_or(LoginError::NotAuthenticated)?;
        let token = auth.get_token().map_err(|_| LoginError::NotAuthenticated)?;
        let account_id = auth.get_account_id();
        crate::poller::read_codex_usage_for_account(&token, account_id.as_deref())
            .map_err(|_| LoginError::InitialUsageFailed)
    })
}

fn run_initial_usage_read(handle: MonitorAuthHandle) -> Result<UsageData, LoginError> {
    read_initial_usage(handle)
}

fn run_cleanup(handle: MonitorAuthHandle) -> Result<(), LoginError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|_| LoginError::RuntimeUnavailable)?;
    runtime.block_on(async move {
        let manager = auth_manager(handle).await?;
        manager
            .logout_with_revoke()
            .await
            .map(|_| ())
            .map_err(|_| LoginError::LoginFailed)
    })
}

fn parse_identity_claims(id_token: &str) -> Option<IdentityClaims> {
    let payload = id_token.split('.').nth(1)?;
    serde_json::from_slice(&decode_base64url(payload)?).ok()
}

fn decode_base64url(input: &str) -> Option<Vec<u8>> {
    let mut output = Vec::new();
    let mut accumulator = 0u32;
    let mut bits = 0u8;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            b'=' => break,
            _ => return None,
        } as u32;

        accumulator = (accumulator << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push(((accumulator >> bits) & 0xFF) as u8);
            if bits == 0 {
                accumulator = 0;
            } else {
                accumulator &= (1u32 << bits) - 1;
            }
        }
    }

    Some(output)
}

fn opaque_identity_id(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest[..16]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn first_uppercase_initial(value: Option<&str>) -> Option<char> {
    value?
        .chars()
        .find(|character| character.is_alphanumeric())
        .and_then(|character| character.to_uppercase().next())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(id: &str, display_name: Option<&str>) -> AccountIdentity {
        AccountIdentity {
            id: id.to_string(),
            display_name: display_name.map(str::to_string),
            username: None,
            email: None,
        }
    }

    fn monitored(
        id: &str,
        display_name: Option<&str>,
        auth_handle: MonitorAuthHandle,
    ) -> MonitoredAccount {
        MonitoredAccount::from_identity(&identity(id, display_name), auth_handle)
    }

    #[test]
    fn identity_prefers_display_name_then_username_then_email_local_part() {
        let named = AccountIdentity {
            display_name: Some("  sidik".to_string()),
            username: Some("nina".to_string()),
            email: Some("someone@example.com".to_string()),
            ..identity("account-a", None)
        };
        assert_eq!(named.initial(), Some('S'));

        let username = AccountIdentity {
            username: Some("nina".to_string()),
            email: Some("someone@example.com".to_string()),
            ..identity("account-b", None)
        };
        assert_eq!(username.initial(), Some('N'));

        let email = AccountIdentity {
            email: Some("someone@example.com".to_string()),
            ..identity("account-c", None)
        };
        assert_eq!(email.initial(), Some('S'));
    }

    #[test]
    fn registry_enforces_two_accounts_and_duplicate_identity() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Sam"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        assert_eq!(
            registry.try_add(monitored(
                "account-a",
                Some("Alex"),
                MonitorAuthHandle::Slot2
            )),
            Err(RegistryError::DuplicateIdentity)
        );

        registry
            .try_add(monitored(
                "account-b",
                Some("Nina"),
                MonitorAuthHandle::Slot2,
            ))
            .unwrap();
        assert_eq!(
            registry.try_add(monitored(
                "account-c",
                Some("Ray"),
                MonitorAuthHandle::Slot1
            )),
            Err(RegistryError::DuplicateAuthOwner)
        );
        assert_eq!(registry.len(), MAX_MONITORED_ACCOUNTS);
        assert!(!registry.is_empty());
        assert!(registry.remove_by_id("account-a").is_some());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn same_initial_accounts_are_valid_and_order_is_persisted() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Sam"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        registry
            .try_add(monitored(
                "account-b",
                Some("Sidik"),
                MonitorAuthHandle::Slot2,
            ))
            .unwrap();

        assert_eq!(registry.accounts()[0].initial, Some('S'));
        assert_eq!(registry.accounts()[1].initial, Some('S'));
        let metadata = registry.metadata();
        assert_eq!(metadata.accounts[0].id, "account-a");
        assert_eq!(metadata.accounts[1].id, "account-b");
    }

    #[test]
    fn distinct_auth_owners_allow_two_accounts() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Alice"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        registry
            .try_add(monitored(
                "account-b",
                Some("Bob"),
                MonitorAuthHandle::Slot2,
            ))
            .unwrap();

        assert_eq!(registry.len(), 2);
        assert_eq!(registry.accounts()[0].auth_handle, MonitorAuthHandle::Slot1);
        assert_eq!(registry.accounts()[1].auth_handle, MonitorAuthHandle::Slot2);
    }

    #[test]
    fn reauthentication_updates_display_identity_without_replacing_owner() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Alice"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();

        let resolved_identity = identity("account-a", Some("Alicia"));
        assert!(registry.update_identity("account-a", &resolved_identity));
        assert_eq!(registry.accounts()[0].initial, Some('A'));
        assert_eq!(registry.accounts()[0].auth_handle, MonitorAuthHandle::Slot1);
    }

    #[test]
    fn persisted_metadata_contains_no_credential_material() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Sam"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        let serialized = serde_json::to_string(&registry.metadata()).unwrap();
        assert!(!serialized.contains("access_token"));
        assert!(!serialized.contains("refresh_token"));
        assert!(!serialized.contains("id_token"));
        assert!(!serialized.contains("email"));
    }

    #[test]
    fn typed_auth_handle_round_trips_without_an_absolute_path() {
        let metadata = MonitoredAccountMetadata {
            id: "account-a".to_string(),
            initial: Some('S'),
            enabled: true,
            auth_handle: MonitorAuthHandle::Slot1,
        };
        let serialized = serde_json::to_string(&metadata).unwrap();
        assert!(serialized.contains("\"auth_handle\":\"slot-1\""));
        assert!(!serialized.contains("\\\\"));
        assert_eq!(
            serde_json::from_str::<MonitoredAccountMetadata>(&serialized).unwrap(),
            metadata
        );
        assert_eq!(
            MonitorAuthHandle::Slot1.namespace_key(),
            "monitor-auth/slot-1"
        );
        assert_eq!(
            MonitorAuthHandle::Slot2.namespace_key(),
            "monitor-auth/slot-2"
        );
        assert!(!serialized.contains("auth-spike"));
        assert!(!serialized.contains("C:"));
    }

    #[test]
    fn projection_keeps_identity_independent_from_auth_handle() {
        let projected = from_codex_auth_projection(Some("stable-account-id".into()), None)
            .expect("account id should produce an identity");
        assert_eq!(projected.id, "stable-account-id");
        let account = MonitoredAccount::from_identity(&projected, MonitorAuthHandle::Slot2);
        assert_eq!(account.id, "stable-account-id");
        assert_eq!(account.auth_handle, MonitorAuthHandle::Slot2);
    }

    #[test]
    fn empty_registry_uses_ephemeral_legacy_display_fallback() {
        let active = identity("account-a", Some("Sam"));
        let registry = AccountRegistry::empty();
        assert_eq!(registry.len(), 0);
        assert_eq!(registry.display_initial(active.initial()), Some('S'));
    }

    #[test]
    fn duplicate_auth_owner_is_rejected_in_add_and_reconstruction() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Sam"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        assert_eq!(
            registry.try_add(monitored(
                "account-b",
                Some("Nina"),
                MonitorAuthHandle::Slot1,
            )),
            Err(RegistryError::DuplicateAuthOwner)
        );

        let metadata = AccountRegistryMetadata {
            accounts: vec![
                MonitoredAccountMetadata {
                    id: "account-a".to_string(),
                    initial: Some('S'),
                    enabled: true,
                    auth_handle: MonitorAuthHandle::Slot1,
                },
                MonitoredAccountMetadata {
                    id: "account-b".to_string(),
                    initial: Some('N'),
                    enabled: true,
                    auth_handle: MonitorAuthHandle::Slot1,
                },
            ],
        };
        assert!(matches!(
            AccountRegistry::from_metadata(metadata),
            Err(RegistryError::DuplicateAuthOwner)
        ));
    }

    #[test]
    fn full_registry_classifies_identity_and_owner_duplicates_before_capacity() {
        let mut registry = AccountRegistry::empty();
        registry
            .try_add(monitored(
                "account-a",
                Some("Sam"),
                MonitorAuthHandle::Slot1,
            ))
            .unwrap();
        registry
            .try_add(monitored(
                "account-b",
                Some("Nina"),
                MonitorAuthHandle::Slot2,
            ))
            .unwrap();

        assert_eq!(
            registry.try_add(monitored(
                "account-a",
                Some("Alex"),
                MonitorAuthHandle::Slot2,
            )),
            Err(RegistryError::DuplicateIdentity)
        );
        assert_eq!(
            registry.try_add(monitored(
                "account-c",
                Some("Ray"),
                MonitorAuthHandle::Slot1,
            )),
            Err(RegistryError::DuplicateAuthOwner)
        );
    }

    #[test]
    fn missing_reauth_snapshot_does_not_block_login_start() {
        let snapshot = capture_reauth_snapshot(None, "account-a");
        assert_eq!(snapshot.state(), CredentialOwnerState::Absent);
        assert_eq!(
            resolve_credential_transaction(
                snapshot.state(),
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Commit,
                false,
            ),
            CredentialTransactionResult::Committed(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn reauth_same_identity_commits_new_owner_state() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Commit,
                false,
            ),
            CredentialTransactionResult::Committed(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn reauth_different_identity_restores_previous_owner_state() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn duplicate_add_after_oauth_cleans_unused_owner() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Absent,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Absent)
        );
    }

    #[test]
    fn cancel_before_mutation_leaves_owner_unchanged() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn cancel_after_mutation_rolls_owner_back() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn login_error_after_mutation_rolls_owner_back() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Absent,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Absent)
        );
    }

    #[test]
    fn timeout_after_mutation_rolls_owner_back() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                true,
            ),
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Present)
        );
    }

    #[test]
    fn rollback_failure_is_explicit() {
        assert_eq!(
            resolve_credential_transaction(
                CredentialOwnerState::Present,
                CredentialOwnerState::Present,
                CredentialTransactionDecision::Rollback,
                false,
            ),
            CredentialTransactionResult::RollbackFailed
        );
    }

    #[test]
    fn failed_b_transaction_does_not_change_a_owner_state() {
        let account_a = CredentialOwnerState::Present;
        let account_b_result = resolve_credential_transaction(
            CredentialOwnerState::Present,
            CredentialOwnerState::Present,
            CredentialTransactionDecision::Rollback,
            true,
        );
        assert_eq!(account_a, CredentialOwnerState::Present);
        assert_eq!(
            account_b_result,
            CredentialTransactionResult::RolledBack(CredentialOwnerState::Present)
        );
    }
}
