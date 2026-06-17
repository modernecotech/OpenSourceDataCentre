use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RouteSensitivity {
    PublicStatic,
    PublicApi,
    PublicService,
    AuthenticatedApp,
    Admin,
    Health,
    Legal,
    Payment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheMode {
    NoStore,
    Public,
    Revalidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeRoutePolicyInput {
    pub sensitivity: RouteSensitivity,
    pub explicitly_cacheable: bool,
    pub authenticated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeRoutePolicyDecision {
    pub cache_mode: CacheMode,
    pub waf_required: bool,
    pub zero_trust_required: bool,
    pub bot_friction_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DdosClaim {
    pub upstream_capacity_planned: bool,
    pub scrubbing_provider_planned: bool,
    pub claims_volumetric_absorption: bool,
}

#[derive(Debug, Error, PartialEq)]
pub enum EdgePolicyError {
    #[error("OSDC Edge Shield must not claim volumetric DDoS absorption without upstream or scrubbing capacity")]
    UnsupportedDdosClaim,
}

pub fn route_policy(input: EdgeRoutePolicyInput) -> EdgeRoutePolicyDecision {
    let cache_mode = match input.sensitivity {
        RouteSensitivity::PublicStatic if input.explicitly_cacheable && !input.authenticated => {
            CacheMode::Public
        }
        RouteSensitivity::PublicApi if input.explicitly_cacheable && !input.authenticated => {
            CacheMode::Revalidate
        }
        _ => CacheMode::NoStore,
    };

    EdgeRoutePolicyDecision {
        cache_mode,
        waf_required: matches!(
            input.sensitivity,
            RouteSensitivity::PublicStatic
                | RouteSensitivity::PublicApi
                | RouteSensitivity::PublicService
                | RouteSensitivity::AuthenticatedApp
                | RouteSensitivity::Admin
        ),
        zero_trust_required: matches!(
            input.sensitivity,
            RouteSensitivity::AuthenticatedApp | RouteSensitivity::Admin
        ),
        bot_friction_allowed: matches!(
            input.sensitivity,
            RouteSensitivity::PublicApi | RouteSensitivity::PublicService
        ),
    }
}

pub fn validate_ddos_claim(claim: DdosClaim) -> Result<(), EdgePolicyError> {
    if claim.claims_volumetric_absorption
        && !(claim.upstream_capacity_planned || claim.scrubbing_provider_planned)
    {
        Err(EdgePolicyError::UnsupportedDdosClaim)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caches_only_explicit_public_static_content() {
        let decision = route_policy(EdgeRoutePolicyInput {
            sensitivity: RouteSensitivity::PublicStatic,
            explicitly_cacheable: true,
            authenticated: false,
        });

        assert_eq!(decision.cache_mode, CacheMode::Public);
        assert!(decision.waf_required);
        assert!(!decision.zero_trust_required);
    }

    #[test]
    fn never_caches_authenticated_admin_routes() {
        let decision = route_policy(EdgeRoutePolicyInput {
            sensitivity: RouteSensitivity::Admin,
            explicitly_cacheable: true,
            authenticated: true,
        });

        assert_eq!(decision.cache_mode, CacheMode::NoStore);
        assert!(decision.waf_required);
        assert!(decision.zero_trust_required);
    }

    #[test]
    fn rejects_unsupported_volumetric_ddos_claims() {
        let claim = DdosClaim {
            upstream_capacity_planned: false,
            scrubbing_provider_planned: false,
            claims_volumetric_absorption: true,
        };

        assert_eq!(
            validate_ddos_claim(claim).unwrap_err(),
            EdgePolicyError::UnsupportedDdosClaim
        );
    }
}
