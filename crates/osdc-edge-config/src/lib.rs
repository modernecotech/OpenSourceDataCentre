use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeDeploymentProfile {
    pub profile_id: String,
    pub name: String,
    pub region: String,
    pub deployment_class: String,
    pub nodes: Vec<EdgeNode>,
    pub origin_tunnel: OriginTunnelProfile,
    pub threat_model: EdgeThreatModel,
    pub cache_policy: EdgeCachePolicy,
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeNode {
    pub id: String,
    pub role: String,
    pub board: String,
    pub services: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginTunnelProfile {
    pub mode: String,
    pub private_origins_required: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeThreatModel {
    pub upstream_ddos_provider_required: bool,
    pub volumetric_ddos_claim_allowed: bool,
    pub community_intelligence_mode: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheDefaultMode {
    NoStore,
    PublicStaticOnly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeCachePolicy {
    pub default_mode: CacheDefaultMode,
    pub public_static_ttl_seconds: u64,
    pub cache_authenticated_responses: bool,
}

#[derive(Debug, Error, PartialEq)]
pub enum EdgeConfigError {
    #[error("profile must include at least one edge node")]
    NoNodes,
    #[error("regional or public-sector profiles should include at least three edge nodes")]
    TooFewRegionalNodes,
    #[error("node {node_id} has no services")]
    NodeWithoutServices { node_id: String },
    #[error("profile must include authoritative DNS on at least one node")]
    MissingAuthoritativeDns,
    #[error("profile must include observability on at least one node")]
    MissingObservability,
    #[error("authenticated response caching must remain disabled by default")]
    AuthenticatedCachingEnabled,
    #[error("volumetric DDoS claims are not allowed for OSDC Edge Shield profiles")]
    VolumetricDdosClaimAllowed,
}

pub fn validate_profile(profile: &EdgeDeploymentProfile) -> Result<(), EdgeConfigError> {
    if profile.nodes.is_empty() {
        return Err(EdgeConfigError::NoNodes);
    }

    if matches!(
        profile.deployment_class.as_str(),
        "regional" | "public-sector"
    ) && profile.nodes.len() < 3
    {
        return Err(EdgeConfigError::TooFewRegionalNodes);
    }

    for node in &profile.nodes {
        if node.services.is_empty() {
            return Err(EdgeConfigError::NodeWithoutServices {
                node_id: node.id.clone(),
            });
        }
    }

    if !has_service(profile, "dns_authoritative") {
        return Err(EdgeConfigError::MissingAuthoritativeDns);
    }

    if !has_service(profile, "observability") {
        return Err(EdgeConfigError::MissingObservability);
    }

    if profile.cache_policy.cache_authenticated_responses {
        return Err(EdgeConfigError::AuthenticatedCachingEnabled);
    }

    if profile.threat_model.volumetric_ddos_claim_allowed {
        return Err(EdgeConfigError::VolumetricDdosClaimAllowed);
    }

    Ok(())
}

fn has_service(profile: &EdgeDeploymentProfile, service_id: &str) -> bool {
    profile
        .nodes
        .iter()
        .any(|node| node.services.iter().any(|service| service == service_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(raw: &str) -> EdgeDeploymentProfile {
        serde_json::from_str(raw).unwrap()
    }

    #[test]
    fn validates_edge_shield_examples() {
        let examples = [
            include_str!("../../../examples/edge-shield/3-node-regional-edge.json"),
            include_str!("../../../examples/edge-shield/school-hospital-edge.json"),
            include_str!("../../../examples/edge-shield/ministry-public-service-edge.json"),
        ];

        for raw in examples {
            validate_profile(&parse(raw)).unwrap();
        }
    }

    #[test]
    fn rejects_regional_profile_with_too_few_nodes() {
        let mut profile = parse(include_str!(
            "../../../examples/edge-shield/3-node-regional-edge.json"
        ));
        profile.nodes.truncate(2);

        assert_eq!(
            validate_profile(&profile).unwrap_err(),
            EdgeConfigError::TooFewRegionalNodes
        );
    }

    #[test]
    fn rejects_volumetric_ddos_claims() {
        let mut profile = parse(include_str!(
            "../../../examples/edge-shield/school-hospital-edge.json"
        ));
        profile.threat_model.volumetric_ddos_claim_allowed = true;

        assert_eq!(
            validate_profile(&profile).unwrap_err(),
            EdgeConfigError::VolumetricDdosClaimAllowed
        );
    }
}
