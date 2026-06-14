const statusClass = (kind) => {
  if (kind === "warn") return "status warn";
  if (kind === "danger") return "status danger";
  if (kind === "info") return "status info";
  return "status";
};

const text = (value) => document.createTextNode(value ?? "");

const clear = (node) => {
  while (node.firstChild) node.removeChild(node.firstChild);
};

async function api(path) {
  const response = await fetch(path, { headers: { Accept: "application/json" } });
  if (!response.ok) throw new Error(`${path} returned ${response.status}`);
  return response.json();
}

function renderMetrics(targetId, metrics) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const metric of metrics) {
    const card = document.createElement("div");
    card.className = "metric";
    const label = document.createElement("span");
    label.append(text(metric.label));
    const value = document.createElement("strong");
    value.append(text(metric.value));
    const detail = document.createElement("em");
    detail.append(text(metric.detail));
    card.append(label, value, detail);
    target.append(card);
  }
}

function renderFlow(targetId, steps) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const step of steps) {
    const item = document.createElement("div");
    item.className = "flow-step";
    const label = document.createElement("strong");
    label.append(text(step.label));
    const value = document.createElement("span");
    value.append(text(step.value));
    item.append(label, value);
    target.append(item);
  }
}

function renderServices(targetId, services) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tile = document.createElement("div");
    tile.className = "service-tile";
    const name = document.createElement("strong");
    name.append(text(service.name ?? service.domain));
    const stack = document.createElement("span");
    stack.append(text(service.stack));
    tile.append(name, stack);
    target.append(tile);
  }
}

function renderOptions(selectId, options, labelFn) {
  const target = document.getElementById(selectId);
  if (!target) return;
  clear(target);
  for (const option of options) {
    const element = document.createElement("option");
    element.value = typeof option === "string" ? option : option.id;
    element.append(text(labelFn(option)));
    target.append(element);
  }
}

function attachTableFilters({ textInputId, statusSelectId, tbodyId }) {
  const textInput = document.getElementById(textInputId);
  const statusSelect = statusSelectId ? document.getElementById(statusSelectId) : null;
  const tbody = document.getElementById(tbodyId);
  if (!tbody || (!textInput && !statusSelect)) return;

  const apply = () => {
    const query = (textInput?.value ?? "").trim().toLowerCase();
    const status = (statusSelect?.value ?? "").trim().toLowerCase();
    for (const row of tbody.querySelectorAll("tr")) {
      const rowText = row.textContent.toLowerCase();
      const badgeText = row.querySelector(".status")?.textContent.trim().toLowerCase() ?? "";
      row.hidden = Boolean(query && !rowText.includes(query)) || Boolean(status && badgeText !== status);
    }
  };

  textInput?.addEventListener("input", apply);
  statusSelect?.addEventListener("change", apply);
  apply();
  return apply;
}

function wireActionButton(buttonId, message, outputId) {
  const button = document.getElementById(buttonId);
  if (!button) return;
  const original = button.textContent;

  button.addEventListener("click", (event) => {
    event.preventDefault();
    const output = outputId ? document.getElementById(outputId) : null;
    if (output) output.textContent = message;
    if (!output && !button.classList.contains("icon-button")) {
      button.textContent = message;
      setTimeout(() => {
        button.textContent = original;
      }, 1400);
    }
  });
}

function downloadTableCsv(tbodyId, filename) {
  const tbody = document.getElementById(tbodyId);
  if (!tbody) return;
  const rows = [...tbody.querySelectorAll("tr:not([hidden])")].map((row) =>
    [...row.children]
      .map((cell) => `"${cell.textContent.trim().replaceAll('"', '""')}"`)
      .join(",")
  );
  if (!rows.length) return;
  const url = URL.createObjectURL(new Blob([`${rows.join("\n")}\n`], { type: "text/csv" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  link.click();
  URL.revokeObjectURL(url);
}

function wireDownloadButton(buttonId, tbodyId, filename) {
  const button = document.getElementById(buttonId);
  if (!button) return;
  button.addEventListener("click", (event) => {
    event.preventDefault();
    downloadTableCsv(tbodyId, filename);
  });
}

function statusCell(label, kind) {
  const td = document.createElement("td");
  const badge = document.createElement("span");
  badge.className = statusClass(kind);
  badge.append(text(label));
  td.append(badge);
  return td;
}

function renderRows(targetId, rows, columns) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const row of rows) {
    const tr = document.createElement("tr");
    for (const column of columns) {
      if (column.status) {
        tr.append(statusCell(row[column.label], row[column.kind]));
      } else {
        const td = document.createElement("td");
        td.append(text(row[column]));
        tr.append(td);
      }
    }
    target.append(tr);
  }
}

function renderCoreServiceRows(targetId, services, mode) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tr = document.createElement("tr");
    const fields =
      mode === "operator"
        ? [
            service.display_name,
            service.open_source_stack,
            service.default_shape,
            service.provisionable ? "yes" : "no",
          ]
        : [
            service.display_name,
            service.aws_equivalent,
            service.azure_equivalent,
            service.open_source_stack,
          ];
    for (const field of fields) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(service.status, service.status_kind));
    target.append(tr);
  }
}

function renderBlueprintRows(targetId, blueprints) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const blueprint of blueprints) {
    const tr = document.createElement("tr");
    for (const field of [
      blueprint.display_name,
      blueprint.service_id,
      blueprint.default_shape,
      blueprint.backing_stack,
      blueprint.operator_checks.join(", "),
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    target.append(tr);
  }
}

function renderEdgeServices(targetId, services) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tr = document.createElement("tr");
    for (const field of [
      service.id,
      service.cloudflare_equivalent,
      service.open_source_stack,
      service.radxa_role,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(service.status, service.status_kind));
    target.append(tr);
  }
}

function renderPreview(targetId, preview) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  const items = [
    ["Request", preview.request_id],
    ["Service", preview.service_id],
    ["Shape", preview.shape],
    ["Linux", preview.linux_image],
    ["Backend", preview.backing_stack],
    ["Power", `${preview.estimated_power_w} W`],
    ["Estimate", `$${preview.estimated_monthly_usd}/mo`],
    ["Checks", preview.operator_checks.join(", ")],
  ];
  for (const [labelText, valueText] of items) {
    const item = document.createElement("div");
    item.className = "flow-step";
    const label = document.createElement("strong");
    label.append(text(labelText));
    const value = document.createElement("span");
    value.append(text(valueText));
    item.append(label, value);
    target.append(item);
  }
}

function tenantServiceProfile(serviceName) {
  const profiles = {
    "VM instance": {
      id: "compute_vm",
      backend: "OpenStack Nova + Cinder + Neutron",
      checks: ["quota", "image signature", "rack power", "Ceph health"],
    },
    "Kubernetes cluster": {
      id: "kubernetes",
      backend: "Cluster API + Metal3 + Cilium + Harbor",
      checks: ["node capacity", "image policy", "network quota", "backup policy"],
    },
    "AI model endpoint": {
      id: "ai_batch",
      backend: "Kueue + KServe + vLLM/SGLang + Ceph model cache",
      checks: ["GPU pool capacity", "model license class", "thermal headroom", "tenant quota"],
    },
    "PostgreSQL database": {
      id: "database",
      backend: "CloudNativePG + Ceph RBD + OpenBao",
      checks: ["backup policy", "restore test", "storage quota", "secret rotation"],
    },
    "Object bucket": {
      id: "object_storage",
      backend: "Ceph RGW + Keycloak/OPA policy",
      checks: ["bucket policy", "quota", "replication", "public access review"],
    },
    "Bare-metal reservation": {
      id: "compute_bare_metal",
      backend: "OpenStack Ironic + Metal3 + Redfish",
      checks: ["BMC reachability", "rack power", "firmware baseline", "tenant isolation"],
    },
    "Serverless function": {
      id: "serverless",
      backend: "Knative + KEDA + Kubernetes",
      checks: ["image policy", "scale limit", "network policy", "cold-start budget"],
    },
    "Message queue": {
      id: "messaging",
      backend: "NATS + Strimzi/Kafka + RabbitMQ",
      checks: ["retention policy", "quota", "replication", "consumer limits"],
    },
    "Secrets vault": {
      id: "secrets",
      backend: "OpenBao + cert-manager + Smallstep",
      checks: ["project policy", "rotation policy", "backup seal", "audit sink"],
    },
    "Backup policy": {
      id: "backup",
      backend: "Velero + Restic/Kopia + Ceph snapshots",
      checks: ["schedule", "retention", "restore target", "offline copy window"],
    },
  };
  return profiles[serviceName] ?? profiles["VM instance"];
}

function tenantShapePower(shape) {
  const watts = {
    "gpu-open.1x16g": 220,
    "gpu-lowpower.1x16g": 96,
    "edge-arm.small": 30,
    "cpu.standard": 85,
  };
  return watts[shape] ?? 85;
}

function buildTenantPreview() {
  const serviceName = document.getElementById("provision-service")?.value ?? "VM instance";
  const shape = document.getElementById("provision-shape")?.value ?? "cpu.standard";
  const linuxImage = document.getElementById("provision-linux")?.value ?? "Debian stable";
  const profile = tenantServiceProfile(serviceName);
  const power = tenantShapePower(shape);

  return {
    request_id: "local-preview",
    service_id: profile.id,
    shape,
    linux_image: linuxImage,
    backing_stack: profile.backend,
    estimated_power_w: power,
    estimated_monthly_usd: Math.max(8, Math.round(power * 0.42)),
    operator_checks: profile.checks,
    next_api_path: "POST /api/provisioning/requests",
  };
}

function wireTenantPreviewForm(preview) {
  const serviceSelect = document.getElementById("provision-service");
  const shapeSelect = document.getElementById("provision-shape");
  const linuxSelect = document.getElementById("provision-linux");

  if (serviceSelect) serviceSelect.value = "AI model endpoint";
  if (shapeSelect) shapeSelect.value = preview.shape;
  if (linuxSelect) linuxSelect.value = preview.linux_image;

  const refresh = () => renderPreview("tenant-provision-preview", buildTenantPreview());
  for (const id of [
    "provision-name",
    "provision-service",
    "provision-shape",
    "provision-linux",
    "provision-storage",
    "provision-network",
  ]) {
    document.getElementById(id)?.addEventListener("input", refresh);
    document.getElementById(id)?.addEventListener("change", refresh);
  }
  refresh();
}

function renderEdgeConfig(targetId, files) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const file of files) {
    const tr = document.createElement("tr");
    for (const field of [file.path, file.owner, file.purpose]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    target.append(tr);
  }
}

function renderChecks(targetId, checks) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const check of checks) {
    const item = document.createElement("div");
    item.className = "flow-step";
    const label = document.createElement("strong");
    label.append(text("Check"));
    const value = document.createElement("span");
    value.append(text(check));
    item.append(label, value);
    target.append(item);
  }
}

function formatUsd(value) {
  if (value >= 1_000_000) {
    return `$${(value / 1_000_000).toFixed(value % 1_000_000 === 0 ? 0 : 2)}M`;
  }
  if (value >= 1_000) return `$${Math.round(value / 1_000)}k`;
  if (value > 0 && value < 1) return `$${value.toFixed(2)}`;
  return `$${value.toLocaleString()}`;
}

function formatUsdRange(low, high) {
  return `${formatUsd(low)}-${formatUsd(high)}`;
}

function scenarioById(scenarios, id) {
  return scenarios.find((scenario) => scenario.id === id) ?? scenarios[0];
}

function renderPlannerScenarios(targetId, scenarios, selectedId, onSelect) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const scenario of scenarios) {
    const tr = document.createElement("tr");
    tr.dataset.scenarioId = scenario.id;
    if (scenario.id === selectedId) tr.className = "selected-row";
    tr.tabIndex = 0;
    const fields = [
      `${scenario.id} ${scenario.name}`,
      `${scenario.it_load_kw.toLocaleString()} kW`,
      scenario.racks.toLocaleString(),
      `${scenario.building_area_m2.toLocaleString()} m2`,
      formatUsdRange(scenario.core_facility_low_usd, scenario.core_facility_high_usd),
      formatUsdRange(scenario.starter_it_low_usd, scenario.starter_it_high_usd),
      formatUsdRange(scenario.total_with_it_low_usd, scenario.total_with_it_high_usd),
      `${scenario.build_time_low_weeks}-${scenario.build_time_high_weeks} weeks`,
    ];
    for (const field of fields) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.addEventListener("click", () => onSelect(scenario.id));
    tr.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        onSelect(scenario.id);
      }
    });
    target.append(tr);
  }
}

function renderPlannerSelection(scenario) {
  const selected = document.getElementById("planner-selected-id");
  if (selected) selected.textContent = scenario.id;
  renderFlow("planner-selected-summary", [
    {
      label: "Total",
      value: formatUsdRange(scenario.total_with_it_low_usd, scenario.total_with_it_high_usd),
    },
    {
      label: "Facility",
      value: formatUsdRange(scenario.core_facility_low_usd, scenario.core_facility_high_usd),
    },
    {
      label: "Starter IT",
      value: formatUsdRange(scenario.starter_it_low_usd, scenario.starter_it_high_usd),
    },
    {
      label: "Build",
      value: `${scenario.build_time_low_weeks}-${scenario.build_time_high_weeks} weeks`,
    },
    {
      label: "Racks",
      value: `${scenario.racks} racks / ${scenario.it_load_kw.toLocaleString()} kW`,
    },
  ]);
  const output = document.getElementById("planner-action-output");
  if (output) output.textContent = scenario.default_building_system;
}

function renderPlannerCategories(targetId, categories) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const category of categories) {
    const tr = document.createElement("tr");
    for (const field of [
      category.category,
      formatUsd(category.low_usd),
      formatUsd(category.high_usd),
      category.notes,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    target.append(tr);
  }
}

function renderPriceBasis(targetId, items) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of items) {
    const tr = document.createElement("tr");
    for (const field of [
      item.item_family,
      item.unit,
      formatUsd(item.low_usd),
      formatUsd(item.high_usd),
      formatUsd(item.planning_selected_usd),
      item.source_marketplace,
      item.project_use,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    target.append(tr);
  }
}

async function hydratePlanner() {
  const planning = await api("/api/cost/planning");
  let selectedId = "S2";

  renderMetrics("planner-metrics", planning.metrics);
  renderPriceBasis("planner-price-basis", planning.price_basis);
  const applyCategoryFilter = attachTableFilters({
    textInputId: "planner-category-filter",
    tbodyId: "planner-categories",
  });

  const renderSelected = () => {
    const selected = scenarioById(planning.scenarios, selectedId);
    renderPlannerScenarios("planner-scenarios", planning.scenarios, selectedId, (nextId) => {
      selectedId = nextId;
      renderSelected();
    });
    renderPlannerSelection(selected);
    renderPlannerCategories(
      "planner-categories",
      planning.categories.filter((category) => category.scenario_id === selected.id)
    );
    applyCategoryFilter?.();
  };

  renderSelected();
  attachTableFilters({
    textInputId: "planner-scenario-filter",
    tbodyId: "planner-scenarios",
  });
  attachTableFilters({
    textInputId: "planner-price-filter",
    tbodyId: "planner-price-basis",
  });
  wireActionButton("planner-refresh", "Planner data refreshed", "planner-action-output");
  wireDownloadButton("planner-export", "planner-scenarios", "osdc-cost-scenarios.csv");
  wireDownloadButton("planner-category-export", "planner-categories", "osdc-cost-categories.csv");
  wireDownloadButton("planner-price-export", "planner-price-basis", "osdc-price-basis.csv");
  document.getElementById("planner-select-pilot")?.addEventListener("click", () => {
    selectedId = "S2";
    renderSelected();
  });
}

async function hydrateTenant() {
  const [summary, options, blueprints, preview] = await Promise.all([
    api("/api/tenant/summary"),
    api("/api/provisioning/options"),
    api("/api/catalog/blueprints"),
    api("/api/provisioning/preview"),
  ]);

  renderMetrics("tenant-metrics", summary.metrics);
  renderFlow("tenant-site-flow", summary.site_flow);
  renderServices("tenant-services", summary.services);
  renderRows("tenant-resources", summary.resources, [
    "name",
    "resource_type",
    "backing_stack",
    "power_class",
    { status: true, label: "status", kind: "status_kind" },
    "monthly_estimate",
  ]);
  renderCoreServiceRows("tenant-core-services", summary.core_services, "tenant");
  renderBlueprintRows("tenant-blueprints", blueprints);

  renderOptions("provision-service", options.services, (option) => option);
  renderOptions(
    "provision-shape",
    options.shapes,
    (option) => `${option.id} - ${option.label}`
  );
  renderOptions("provision-linux", options.linux_images, (option) => option);
  renderOptions("provision-storage", options.storage, (option) => option);
  renderOptions("provision-network", options.networks, (option) => option);
  wireTenantPreviewForm(preview);

  attachTableFilters({
    textInputId: "tenant-service-filter",
    statusSelectId: "tenant-status-filter",
    tbodyId: "tenant-core-services",
  });
  attachTableFilters({
    textInputId: "tenant-blueprint-filter",
    tbodyId: "tenant-blueprints",
  });
  attachTableFilters({
    textInputId: "tenant-resource-filter",
    tbodyId: "tenant-resources",
  });
  wireActionButton(
    "tenant-submit",
    "Provisioning request staged for operator approval.",
    "tenant-action-output"
  );
  wireActionButton("tenant-save-template", "Template saved in this browser session.", "tenant-action-output");
  wireActionButton("tenant-refresh", "Tenant data refreshed.", "tenant-action-output");
  wireActionButton("tenant-api-tokens", "API token panel queued for implementation.", "tenant-action-output");
  wireActionButton("tenant-api-schema", "OpenAPI schema export queued.", "tenant-action-output");
  wireActionButton("tenant-opentofu", "OpenTofu module export queued.", "tenant-action-output");
  wireActionButton("tenant-preview-refresh", "Preview recalculated.", "tenant-action-output");
  wireDownloadButton("tenant-download-csv", "tenant-resources", "osdc-tenant-resources.csv");

  document.getElementById("tenant-provision-focus")?.addEventListener("click", () => {
    document.getElementById("provision-name")?.focus();
  });
}

async function hydrateOperator() {
  const status = await api("/api/operator/status");

  renderMetrics("operator-metrics", status.metrics);
  renderFlow("operator-power-flow", status.power_flow);
  renderFlow("operator-thermal-flow", status.thermal_flow);
  renderRows("operator-hardware", status.hardware_pools, [
    "pool",
    "hardware",
    "power",
    "allocated",
    { status: true, label: "status", kind: "status_kind" },
  ]);
  renderRows("operator-stack", status.cloud_stack, [
    "domain",
    "stack",
    { status: true, label: "health", kind: "health_kind" },
  ]);
  renderCoreServiceRows("operator-core-services", status.core_services, "operator");
  renderRows("operator-operations", status.operations, [
    "time",
    "system",
    "action",
    "owner",
    "risk",
    { status: true, label: "status", kind: "status_kind" },
  ]);

  attachTableFilters({
    textInputId: "operator-hardware-filter",
    tbodyId: "operator-hardware",
  });
  attachTableFilters({
    textInputId: "operator-stack-filter",
    tbodyId: "operator-stack",
  });
  attachTableFilters({
    textInputId: "operator-service-filter",
    statusSelectId: "operator-status-filter",
    tbodyId: "operator-core-services",
  });
  attachTableFilters({
    textInputId: "operator-operation-filter",
    tbodyId: "operator-operations",
  });
  wireActionButton("operator-shed", "Shed plan queued");
  wireActionButton("operator-open-change", "Change opened");
  wireActionButton("operator-quotas", "Quota review queued");
  wireActionButton("operator-sync", "Sync queued");
  wireActionButton("operator-audit", "Audit queued");
  wireDownloadButton("operator-export-log", "operator-operations", "osdc-operations.csv");
}

async function hydrateEdge() {
  const [status, config] = await Promise.all([
    api("/api/edge/status"),
    api("/api/edge/config-preview"),
  ]);

  renderMetrics("edge-metrics", status.metrics);
  renderEdgeServices("edge-services", status.services);
  renderEdgeConfig("edge-config-preview", config.generated_files);
  renderChecks("edge-config-checks", config.rollout_checks);
  const role = document.getElementById("edge-config-role");
  if (role) role.textContent = config.node_role;
  renderRows("edge-nodes", status.nodes, [
    "name",
    "board",
    "role",
    "power",
    { status: true, label: "status", kind: "status_kind" },
  ]);
  renderRows("edge-rollout", status.rollout, [
    "time",
    "system",
    "action",
    "owner",
    "risk",
    { status: true, label: "status", kind: "status_kind" },
  ]);

  attachTableFilters({
    textInputId: "edge-service-filter",
    statusSelectId: "edge-status-filter",
    tbodyId: "edge-services",
  });
  attachTableFilters({
    textInputId: "edge-node-filter",
    tbodyId: "edge-nodes",
  });
  attachTableFilters({
    textInputId: "edge-rollout-filter",
    tbodyId: "edge-rollout",
  });
  wireActionButton("edge-detection-mode", "Detection mode staged");
  wireActionButton("edge-stage-rollout", "Rollout staged");
  wireActionButton("edge-generate-configs", "Configs generated");
  wireActionButton("edge-ssh-policy", "SSH policy review queued");
  wireActionButton("edge-audit", "Audit queued");
}

document.addEventListener("DOMContentLoaded", () => {
  if (document.body.dataset.portal === "tenant") {
    hydrateTenant().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "operator") {
    hydrateOperator().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "edge") {
    hydrateEdge().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "planner") {
    hydratePlanner().catch((error) => console.error(error));
  }
});
