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

async function hydrateTenant() {
  const [summary, options] = await Promise.all([
    api("/api/tenant/summary"),
    api("/api/provisioning/options"),
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

  renderOptions("provision-service", options.services, (option) => option);
  renderOptions(
    "provision-shape",
    options.shapes,
    (option) => `${option.id} - ${option.label}`
  );
  renderOptions("provision-linux", options.linux_images, (option) => option);
  renderOptions("provision-storage", options.storage, (option) => option);
  renderOptions("provision-network", options.networks, (option) => option);
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
  renderRows("operator-operations", status.operations, [
    "time",
    "system",
    "action",
    "owner",
    "risk",
    { status: true, label: "status", kind: "status_kind" },
  ]);
}

document.addEventListener("DOMContentLoaded", () => {
  if (document.body.dataset.portal === "tenant") {
    hydrateTenant().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "operator") {
    hydrateOperator().catch((error) => console.error(error));
  }
});
