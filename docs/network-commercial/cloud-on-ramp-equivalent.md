# Cloud-On-Ramp Equivalent

For sovereign deployments, a cloud-on-ramp equivalent may connect tenants to national cloud regions, public-sector networks, research networks, or approved external cloud providers.

Design questions:

- Which network owns the route?
- Where is the demarcation?
- What encryption or private circuit is required?
- Which firewall and logging controls apply?
- What data-residency restrictions apply?
- How are route leaks prevented?
- What SLA applies if the upstream cloud or carrier fails?

The product should be treated like a cross-connect plus a routed service, with explicit responsibility boundaries.
