use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::SystemTime;

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
}
