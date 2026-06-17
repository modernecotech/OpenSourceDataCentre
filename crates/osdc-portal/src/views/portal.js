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

const portalCommandState = {
  surface: null,
  items: [],
  selectedId: null,
  counter: 0,
};

const customerCommandRuntime = {
  status: "draft",
  lastRecord: null,
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
    const command = recordPortalCommand({
      action: button.getAttribute("aria-label") || original || buttonId,
      message,
      payload: { button_id: buttonId, output_id: outputId ?? null },
      status: "submitted",
    });
    const output = outputId ? document.getElementById(outputId) : null;
    if (output) output.textContent = `${command.command_id}: ${message}`;
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
  let rows = [...tbody.querySelectorAll("tr:not([hidden])")].map((row) =>
    [...row.children]
      .map((cell) => `"${cell.textContent.trim().replaceAll('"', '""')}"`)
      .join(",")
  );
  if (!rows.length) {
    rows = [...tbody.children]
      .filter((child) => !child.hidden)
      .map((child) => `"${child.textContent.trim().replace(/\s+/g, " ").replaceAll('"', '""')}"`);
  }
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
    recordPortalCommand({
      action: `Export ${filename}`,
      message: `${filename} exported`,
      payload: { button_id: buttonId, table_id: tbodyId, filename },
      status: "validated",
      evidenceTarget: filename,
    });
  });
}

function downloadJson(filename, value) {
  const url = URL.createObjectURL(
    new Blob([`${JSON.stringify(value, null, 2)}\n`], { type: "application/json" })
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  link.click();
  URL.revokeObjectURL(url);
}

function portalSurface() {
  return document.body?.dataset.portal || "portal";
}

function portalSurfaceLabel(surface = portalSurface()) {
  return surface
    .split("-")
    .filter(Boolean)
    .map((part) => part[0]?.toUpperCase() + part.slice(1))
    .join(" ");
}

function portalCommandStorageKey(surface = portalSurface()) {
  return `osdc.portal.commands.${surface}`;
}

function loadPortalCommands() {
  const surface = portalSurface();
  if (portalCommandState.surface === surface) return;
  portalCommandState.surface = surface;
  portalCommandState.items = [];
  portalCommandState.selectedId = null;
  portalCommandState.counter = 0;
  try {
    const raw = window.localStorage?.getItem(portalCommandStorageKey(surface));
    if (raw) {
      const parsed = JSON.parse(raw);
      portalCommandState.items = Array.isArray(parsed.items) ? parsed.items : [];
      portalCommandState.counter = Number(parsed.counter || portalCommandState.items.length || 0);
      portalCommandState.selectedId = portalCommandState.items.at(-1)?.command_id ?? null;
    }
  } catch (_error) {
    portalCommandState.items = [];
  }
}

function savePortalCommands() {
  try {
    window.localStorage?.setItem(
      portalCommandStorageKey(),
      JSON.stringify({
        counter: portalCommandState.counter,
        items: portalCommandState.items.slice(-20),
      })
    );
  } catch (_error) {
    /* localStorage is optional in headless or locked-down browsers. */
  }
}

function defaultCommandEvidenceTarget(surface = portalSurface()) {
  return `target/assurance/commands/${surface}`;
}

function selectedPortalContext() {
  const selected = document.querySelector(".selected-row");
  if (selected?.textContent) {
    return selected.textContent.trim().replace(/\s+/g, " ").slice(0, 220);
  }
  return document.querySelector(".title-block h1")?.textContent?.trim() || portalSurfaceLabel();
}

function commandStatusKind(status) {
  const value = String(status ?? "").toLowerCase();
  if (!value) return "info";
  if (["approved", "validated", "completed"].includes(value)) return "normal";
  if (["draft", "submitted", "queued"].includes(value)) return "info";
  if (["blocked", "failed", "rejected"].includes(value)) return "danger";
  return "warn";
}

function commandId(surface) {
  portalCommandState.counter += 1;
  const prefix = surface.toUpperCase().replace(/[^A-Z0-9]+/g, "_");
  return `CMD-${prefix}-${String(portalCommandState.counter).padStart(4, "0")}`;
}

function ensurePortalCommandWorkspace() {
  const content = document.querySelector(".content");
  if (!content) return null;
  loadPortalCommands();
  let panel = document.getElementById("portal-command-workspace");
  if (panel) return panel;

  panel = document.createElement("section");
  panel.className = "panel command-workspace";
  panel.id = "portal-command-workspace";
  panel.innerHTML = `
    <div class="panel-header">
      <h2>Active Command Queue</h2>
      <div class="panel-tools">
        <span class="status info" id="portal-command-status">idle</span>
        <button id="portal-command-validate">Validate</button>
        <button id="portal-command-approve">Approve</button>
        <button id="portal-command-download">Download JSON</button>
        <button id="portal-command-clear">Clear</button>
      </div>
    </div>
    <div class="panel-body">
      <div class="flow-grid" id="portal-command-flow">
        <div class="flow-step"><strong>Command</strong><span>idle</span></div>
        <div class="flow-step"><strong>Validate</strong><span>pending</span></div>
        <div class="flow-step"><strong>Approve</strong><span>pending</span></div>
        <div class="flow-step"><strong>Evidence</strong><span>pending</span></div>
      </div>
      <div class="command-layout">
        <table>
          <thead><tr><th>Command</th><th>Action</th><th>Surface</th><th>State</th><th>Evidence</th></tr></thead>
          <tbody id="portal-command-queue"></tbody>
        </table>
        <table>
          <thead><tr><th>Field</th><th>Value</th></tr></thead>
          <tbody id="portal-command-detail"></tbody>
        </table>
      </div>
    </div>
  `;

  const metrics = content.querySelector(".metric-grid");
  if (metrics) {
    metrics.insertAdjacentElement("afterend", panel);
  } else {
    content.prepend(panel);
  }

  document.getElementById("portal-command-validate")?.addEventListener("click", (event) => {
    event.preventDefault();
    updateSelectedPortalCommand("validated");
  });
  document.getElementById("portal-command-approve")?.addEventListener("click", (event) => {
    event.preventDefault();
    const selected = selectedPortalCommand();
    updateSelectedPortalCommand(selected?.status === "validated" ? "approved" : "blocked");
  });
  document.getElementById("portal-command-download")?.addEventListener("click", (event) => {
    event.preventDefault();
    const selected = selectedPortalCommand();
    if (selected) downloadJson(`${selected.command_id.toLowerCase()}.json`, selected.payload);
  });
  document.getElementById("portal-command-clear")?.addEventListener("click", (event) => {
    event.preventDefault();
    portalCommandState.items = [];
    portalCommandState.selectedId = null;
    savePortalCommands();
    renderPortalCommandWorkspace();
  });

  renderPortalCommandWorkspace();
  return panel;
}

function selectedPortalCommand() {
  loadPortalCommands();
  return portalCommandState.items.find((item) => item.command_id === portalCommandState.selectedId)
    ?? portalCommandState.items.at(-1)
    ?? null;
}

function recordPortalCommand({
  action,
  message,
  payload = {},
  status = "submitted",
  evidenceTarget,
}) {
  ensurePortalCommandWorkspace();
  const surface = portalSurface();
  const command_id = commandId(surface);
  const created_at_utc = new Date().toISOString();
  const command = {
    command_id,
    surface,
    action,
    message,
    status,
    evidence_target: evidenceTarget || defaultCommandEvidenceTarget(surface),
    selected_context: selectedPortalContext(),
    created_at_utc,
    payload: {
      command_id,
      surface,
      action,
      status,
      message,
      source_route: window.location.pathname,
      selected_context: selectedPortalContext(),
      evidence_target: evidenceTarget || defaultCommandEvidenceTarget(surface),
      requested_at_utc: created_at_utc,
      validations: [
        { check: "catalogue contract", status: "queued" },
        { check: "approval gate", status: "pending" },
        { check: "evidence target", status: "pending" },
      ],
      ...payload,
    },
  };
  portalCommandState.items.push(command);
  portalCommandState.selectedId = command.command_id;
  savePortalCommands();
  renderPortalCommandWorkspace();
  return command;
}

function updateSelectedPortalCommand(nextStatus) {
  const command = selectedPortalCommand();
  if (!command) return null;
  command.status = nextStatus;
  command.payload.status = nextStatus;
  command.payload.validations = [
    { check: "catalogue contract", status: nextStatus === "blocked" ? "blocked" : "passed" },
    { check: "approval gate", status: nextStatus === "approved" ? "approved" : nextStatus === "blocked" ? "blocked" : "ready" },
    { check: "evidence target", status: nextStatus === "approved" ? "recorded" : "pending" },
  ];
  command.payload.updated_at_utc = new Date().toISOString();
  savePortalCommands();
  renderPortalCommandWorkspace();
  return command;
}

function renderPortalCommandWorkspace() {
  const panel = document.getElementById("portal-command-workspace");
  if (!panel) return;
  loadPortalCommands();
  const queue = document.getElementById("portal-command-queue");
  const detail = document.getElementById("portal-command-detail");
  const selected = selectedPortalCommand();
  const status = document.getElementById("portal-command-status");
  if (status) {
    status.className = statusClass(commandStatusKind(selected?.status));
    status.textContent = selected?.status || "idle";
  }

  renderFlow("portal-command-flow", [
    { label: "Command", value: selected?.command_id || portalSurfaceLabel() },
    { label: "Validate", value: selected?.status === "validated" || selected?.status === "approved" ? "passed" : "pending" },
    { label: "Approve", value: selected?.status === "approved" ? "approved" : selected?.status === "blocked" ? "blocked" : "pending" },
    { label: "Evidence", value: selected?.evidence_target || defaultCommandEvidenceTarget() },
  ]);

  if (queue) {
    clear(queue);
    if (!portalCommandState.items.length) {
      const row = document.createElement("tr");
      const cell = document.createElement("td");
      cell.colSpan = 5;
      cell.append(text("No command records"));
      row.append(cell);
      queue.append(row);
    }
    for (const item of portalCommandState.items.slice().reverse()) {
      const row = document.createElement("tr");
      if (item.command_id === selected?.command_id) row.className = "selected-row";
      row.tabIndex = 0;
      for (const field of [item.command_id, item.action, portalSurfaceLabel(item.surface)]) {
        const cell = document.createElement("td");
        cell.append(text(field));
        row.append(cell);
      }
      row.append(statusCell(item.status, commandStatusKind(item.status)));
      const evidence = document.createElement("td");
      appendValueOrRepoLink(evidence, item.evidence_target);
      row.append(evidence);
      row.addEventListener("click", () => {
        portalCommandState.selectedId = item.command_id;
        savePortalCommands();
        renderPortalCommandWorkspace();
      });
      row.addEventListener("keydown", (event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          portalCommandState.selectedId = item.command_id;
          savePortalCommands();
          renderPortalCommandWorkspace();
        }
      });
      queue.append(row);
    }
  }

  if (detail) {
    clear(detail);
    const rows = selected
      ? [
          ["command_id", selected.command_id],
          ["surface", portalSurfaceLabel(selected.surface)],
          ["action", selected.action],
          ["status", selected.status],
          ["context", selected.selected_context],
          ["message", selected.message],
          ["evidence", selected.evidence_target],
          ["created_at_utc", selected.created_at_utc],
        ]
      : [
          ["surface", portalSurfaceLabel()],
          ["status", "idle"],
          ["evidence", defaultCommandEvidenceTarget()],
        ];
    for (const [key, value] of rows) {
      const row = document.createElement("tr");
      const keyCell = document.createElement("td");
      keyCell.append(text(key));
      const valueCell = document.createElement("td");
      appendValueOrRepoLink(valueCell, value);
      row.append(keyCell, valueCell);
      detail.append(row);
    }
  }
}

function attachColumnFilter({ textInputId, selectId, tbodyId, columnIndex }) {
  const textInput = document.getElementById(textInputId);
  const select = document.getElementById(selectId);
  const tbody = document.getElementById(tbodyId);
  if (!tbody || (!textInput && !select)) return;

  const apply = () => {
    const query = (textInput?.value ?? "").trim().toLowerCase();
    const selected = (select?.value ?? "").trim().toLowerCase();
    for (const row of tbody.querySelectorAll("tr")) {
      const rowText = row.textContent.toLowerCase();
      const columnText = row.children[columnIndex]?.textContent.trim().toLowerCase() ?? "";
      row.hidden =
        Boolean(query && !rowText.includes(query)) ||
        Boolean(selected && columnText !== selected);
    }
  };

  textInput?.addEventListener("input", apply);
  select?.addEventListener("change", apply);
  apply();
  return apply;
}

function attachMultiTableFilter(inputId, tbodyIds) {
  const input = document.getElementById(inputId);
  const tbodies = tbodyIds.map((id) => document.getElementById(id)).filter(Boolean);
  if (!input || !tbodies.length) return;

  const apply = () => {
    const query = input.value.trim().toLowerCase();
    for (const tbody of tbodies) {
      for (const row of tbody.querySelectorAll("tr")) {
        row.hidden = Boolean(query && !row.textContent.toLowerCase().includes(query));
      }
    }
  };

  input.addEventListener("input", apply);
  apply();
  return apply;
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

function renderHardwarePipeline(targetId, stages) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const stage of stages) {
    const tr = document.createElement("tr");
    for (const field of [
      stage.stage_name,
      stage.purpose,
      stage.primary_system,
      stage.ui_action,
      stage.automation_hook,
      stage.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(stage.status, statusClassFromStatus(stage.status)));
    target.append(tr);
  }
}

function renderHardwareProfiles(targetId, profiles) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const profile of profiles) {
    const tr = document.createElement("tr");
    for (const field of [
      profile.profile_id,
      profile.workload_class,
      profile.node_role,
      profile.provisioner,
      profile.os_image,
      profile.network_profile,
      profile.target_pool,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(profile.status, statusClassFromStatus(profile.status)));
    target.append(tr);
  }
}

function renderHardwareRequests(targetId, requests) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const request of requests) {
    const tr = document.createElement("tr");
    for (const field of [
      request.request_id,
      request.requester,
      request.profile_id,
      request.count,
      request.site,
      request.approval_gate,
      request.current_stage,
      request.target_environment,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(request.status, statusClassFromStatus(request.status)));
    target.append(tr);
  }
}

function renderSystemConnectors(targetId, connectors) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const connector of connectors) {
    const tr = document.createElement("tr");
    for (const field of [
      connector.system_name,
      connector.capability,
      connector.adapter_pattern,
      connector.endpoint_pattern,
      connector.auth_model,
      connector.write_mode,
      connector.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(connector.status, statusClassFromStatus(connector.status)));
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

function repoHref(value) {
  if (typeof value !== "string") return null;
  if (
    value.startsWith("docs/") ||
    value.startsWith("crates/osdc-portal/migrations/") ||
    value.startsWith("data/") ||
    value.startsWith("examples/") ||
    value === "README.md" ||
    value === "LICENSE.md"
  ) {
    return `/${value}`;
  }
  return null;
}

function appendValueOrRepoLink(td, value) {
  if (typeof value === "string" && value.startsWith("vscode://")) {
    const link = document.createElement("a");
    link.href = value;
    link.append(text("Open in VS Code"));
    td.append(link);
    return;
  }
  if (typeof value === "string" && value.startsWith("https://")) {
    const link = document.createElement("a");
    link.href = value;
    link.target = "_blank";
    link.rel = "noreferrer";
    link.append(text(value));
    td.append(link);
    return;
  }

  const href = repoHref(value);
  if (!href) {
    td.append(text(value));
    return;
  }

  const link = document.createElement("a");
  link.href = href;
  link.target = "_blank";
  link.rel = "noreferrer";
  link.append(text(value));
  td.append(link);
}

function renderLifecycleStages(targetId, stages) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const stage of stages) {
    const card = document.createElement("div");
    card.className = "lifecycle-stage";
    const title = document.createElement("strong");
    title.append(text(stage.phase));
    const gate = document.createElement("span");
    gate.append(text(`${stage.gate_id} - ${stage.gate_name}`));
    const owner = document.createElement("small");
    owner.append(text(stage.owner));
    const focus = document.createElement("small");
    focus.append(text(stage.focus));
    const status = document.createElement("em");
    status.className = statusClass(stage.status_kind);
    status.append(text(stage.status));
    const evidence = document.createElement("a");
    evidence.href = repoHref(stage.evidence_path) ?? "#";
    evidence.target = "_blank";
    evidence.rel = "noreferrer";
    evidence.append(text(stage.evidence_path));
    card.append(title, gate, owner, focus, status, evidence);
    target.append(card);
  }
}

function renderDeveloperServices(targetId, services, compact = false) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tr = document.createElement("tr");
    const fields = compact
      ? [
          service.service_id,
          service.function,
          service.default_stack,
          service.vs_code_integration,
        ]
      : [
          service.service_id,
          service.function,
          service.default_stack,
          service.vs_code_integration,
          service.control,
        ];
    for (const field of fields) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(service.status, statusClassFromStatus(service.status)));
    if (!compact) {
      const evidence = document.createElement("td");
      appendValueOrRepoLink(evidence, service.evidence_path);
      tr.append(evidence);
    }
    target.append(tr);
  }
}

function statusClassFromStatus(status) {
  const value = String(status ?? "").toLowerCase();
  if (value === "production-baseline" || value === "implemented") return "normal";
  if (value === "pilot" || value === "template" || value === "preview") return "info";
  if (value === "blocked") return "danger";
  return "warn";
}

function renderDeveloperTemplates(targetId, templates) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const template of templates) {
    const tr = document.createElement("tr");
    for (const field of [template.name, template.language, template.runtime]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    for (const field of [
      template.devcontainer_path,
      template.pipeline_path,
      template.deploy_path,
      template.vscode_clone_uri,
    ]) {
      const td = document.createElement("td");
      appendValueOrRepoLink(td, field);
      tr.append(td);
    }
    tr.append(statusCell(template.status, statusClassFromStatus(template.status)));
    target.append(tr);
  }
}

function renderDeveloperEnvironments(targetId, environments) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const environment of environments) {
    const tr = document.createElement("tr");
    for (const field of [
      environment.name,
      environment.cluster,
      environment.namespace_pattern,
      environment.gitops_tool,
      environment.approval_policy,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(environment.status, statusClassFromStatus(environment.status)));
    target.append(tr);
  }
}

function renderDeveloperPromotionGates(targetId, gates) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const gate of gates) {
    const tr = document.createElement("tr");
    for (const field of [
      gate.gate_id,
      gate.from_environment,
      gate.to_environment,
      gate.required_checks,
      gate.approver,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(gate.status, statusClassFromStatus(gate.status)));
    target.append(tr);
  }
}

function renderVsCodeWorkflows(targetId, workflows) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const workflow of workflows) {
    const tr = document.createElement("tr");
    for (const field of [workflow.workflow_id, workflow.action, workflow.vs_code_surface]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    for (const field of [workflow.command_or_uri, workflow.artifact_path]) {
      const td = document.createElement("td");
      appendValueOrRepoLink(td, field);
      tr.append(td);
    }
    const action = document.createElement("td");
    action.append(text(workflow.portal_action));
    tr.append(action);
    tr.append(statusCell(workflow.status, statusClassFromStatus(workflow.status)));
    target.append(tr);
  }
}

function renderDataPlatformServices(targetId, services, compact = false) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tr = document.createElement("tr");
    const fields = compact
      ? [service.service_id, service.function, service.default_stack, service.control]
      : [service.service_id, service.function, service.default_stack, service.control];
    for (const field of fields) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(service.status, statusClassFromStatus(service.status)));
    if (!compact) {
      const evidence = document.createElement("td");
      appendValueOrRepoLink(evidence, service.evidence_path);
      tr.append(evidence);
    }
    target.append(tr);
  }
}

function renderDataProducts(targetId, products) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const product of products) {
    const tr = document.createElement("tr");
    for (const field of [
      product.name,
      product.domain,
      product.owner,
      product.lakehouse_table,
      product.ontology_object,
      product.quality_gate,
      product.access_policy,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(product.status, statusClassFromStatus(product.status)));
    target.append(tr);
  }
}

function renderDataPipelines(targetId, pipelines) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const pipeline of pipelines) {
    const tr = document.createElement("tr");
    for (const field of [
      pipeline.name,
      pipeline.engine,
      pipeline.source,
      pipeline.target,
      pipeline.schedule,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(pipeline.status, statusClassFromStatus(pipeline.status)));
    target.append(tr);
  }
}

function renderDataOntology(targetId, objects) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const object of objects) {
    const tr = document.createElement("tr");
    for (const field of [
      object.name,
      object.domain,
      object.description,
      object.relationships,
      object.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(object.status, statusClassFromStatus(object.status)));
    target.append(tr);
  }
}

function renderDataPolicies(targetId, policies) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const policy of policies) {
    const tr = document.createElement("tr");
    for (const field of [
      policy.policy_id,
      policy.scope,
      policy.subject,
      policy.allowed_actions,
      policy.conditions,
      policy.enforcement_point,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(policy.status, statusClassFromStatus(policy.status)));
    target.append(tr);
  }
}

function renderDataTemplates(targetId, templates) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const template of templates) {
    const tr = document.createElement("tr");
    for (const field of [template.name, template.template_type]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    for (const field of [
      template.repo_path,
      template.devcontainer_path,
      template.pipeline_path,
      template.preview_surface,
    ]) {
      const td = document.createElement("td");
      appendValueOrRepoLink(td, field);
      tr.append(td);
    }
    tr.append(statusCell(template.status, statusClassFromStatus(template.status)));
    target.append(tr);
  }
}

function attachCardFilter(inputId, containerId) {
  const input = document.getElementById(inputId);
  const container = document.getElementById(containerId);
  if (!input || !container) return;

  const apply = () => {
    const query = input.value.trim().toLowerCase();
    for (const card of container.children) {
      card.hidden = Boolean(query && !card.textContent.toLowerCase().includes(query));
    }
  };

  input.addEventListener("input", apply);
  apply();
}

function renderLifecycleWorkItems(targetId, items) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of items) {
    const tr = document.createElement("tr");
    for (const field of [
      item.item_type,
      item.id,
      item.phase,
      item.title,
      item.owner,
      item.priority,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, item.status_kind));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, item.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderLifecycleEvidence(targetId, items) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of items) {
    const tr = document.createElement("tr");
    for (const field of [item.source, item.id, item.domain, item.title, item.owner]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, item.status_kind));
    const artifact = document.createElement("td");
    appendValueOrRepoLink(artifact, item.artifact);
    tr.append(artifact);
    target.append(tr);
  }
}

function renderLifecycleServices(targetId, services) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const service of services) {
    const tr = document.createElement("tr");
    for (const field of [
      service.service_id,
      service.category,
      service.interface,
      service.implementation,
      service.workflow,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(service.status, service.status_kind));
    target.append(tr);
  }
}

function renderLifecycleDocuments(targetId, documents) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const documentItem of documents) {
    const tr = document.createElement("tr");
    for (const field of [documentItem.area, documentItem.title, documentItem.purpose]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    const path = document.createElement("td");
    appendValueOrRepoLink(path, documentItem.path);
    tr.append(path);
    target.append(tr);
  }
}

function renderCommercialGaps(targetId, gaps, includeCurrentState = false) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const gap of gaps) {
    const tr = document.createElement("tr");
    const fields = [
      gap.domain,
      gap.gap_id,
      gap.commercial_expectation,
      gap.priority,
    ];
    if (includeCurrentState) fields.splice(3, 0, gap.current_repo_state);
    for (const field of fields) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(gap.status, statusClassFromStatus(gap.status)));
    const artifact = document.createElement("td");
    appendValueOrRepoLink(artifact, gap.next_artifact);
    tr.append(artifact);
    target.append(tr);
  }
}

function renderCommercialRemoteHandsPricebook(targetId, items) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of items) {
    const tr = document.createElement("tr");
    for (const field of [
      item.task_class,
      item.pricebook_id,
      item.billing_unit,
      item.target_response,
      item.requires_approval,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, statusClassFromStatus(item.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, item.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderCommercialAccessRoles(targetId, roles) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const role of roles) {
    const tr = document.createElement("tr");
    for (const field of [
      role.scope,
      role.access_role_id,
      role.role_name,
      role.approval_owner,
      role.review_cadence,
      role.allowed_areas,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(role.status, statusClassFromStatus(role.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, role.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderCommercialMetrics({
  gaps,
  standards,
  colocation,
  crossConnects,
  remoteProducts,
  pricebook,
  accessRoles,
  auditEvidence,
}) {
  const openGaps = gaps.filter((gap) => String(gap.status).toLowerCase() !== "closed");
  const customerProducts =
    colocation.length + crossConnects.length + remoteProducts.length + pricebook.length;
  renderMetrics("commercial-metrics", [
    {
      label: "Open gaps",
      value: openGaps.length,
      detail: "commercial readiness",
    },
    {
      label: "Standards controls",
      value: standards.length,
      detail: "mapped to evidence",
    },
    {
      label: "Customer products",
      value: customerProducts,
      detail: "colo cross-connect hands",
    },
    {
      label: "Access and audit",
      value: accessRoles.length + auditEvidence.length,
      detail: "roles evidence cadence",
    },
  ]);
}

function renderCommercialStandards(targetId, standards) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of standards) {
    const tr = document.createElement("tr");
    for (const field of [
      item.standard_family,
      item.requirement_id,
      item.control_area,
      item.applies,
      item.osdc_design_response,
      item.responsible_party,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, statusClassFromStatus(item.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, item.evidence_file);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderCommercialSlas(targetId, slas) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const sla of slas) {
    const tr = document.createElement("tr");
    for (const field of [
      sla.sla_class_id,
      sla.service_scope,
      sla.target,
      sla.measurement_window,
      sla.credit_model,
      sla.exclusions,
      sla.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(sla.status, statusClassFromStatus(sla.status)));
    target.append(tr);
  }
}

function renderCommercialColocationProducts(targetId, products) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const product of products) {
    const tr = document.createElement("tr");
    for (const field of [
      product.product_id,
      product.product_type,
      product.unit,
      product.default_commitment,
      product.required_controls,
      product.demarcation,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    target.append(tr);
  }
}

function renderCommercialCrossConnectProducts(targetId, products) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const product of products) {
    const tr = document.createElement("tr");
    for (const field of [
      product.product_id,
      product.product_type,
      product.media,
      product.demarcation,
      product.workflow,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(product.status, statusClassFromStatus(product.status)));
    target.append(tr);
  }
}

function renderCommercialRemoteHandsProducts(targetId, products) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const product of products) {
    const tr = document.createElement("tr");
    for (const field of [
      product.product_id,
      product.task_class,
      product.response_target,
      product.requires_approval,
      product.scope_boundary,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(product.status, statusClassFromStatus(product.status)));
    target.append(tr);
  }
}

function renderCommercialAuditEvidence(targetId, evidence) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of evidence) {
    const tr = document.createElement("tr");
    for (const field of [
      item.domain,
      item.evidence_id,
      item.evidence_name,
      item.owner,
      item.cadence,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, statusClassFromStatus(item.status)));
    const path = document.createElement("td");
    appendValueOrRepoLink(path, item.evidence_path);
    tr.append(path);
    target.append(tr);
  }
}

function renderCustomerAccounts(targetId, accounts) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const account of accounts) {
    const tr = document.createElement("tr");
    for (const field of [
      `${account.customer_id} ${account.display_name}`,
      account.customer_type,
      account.residency_zone,
      account.primary_region,
      account.identity_realm,
      account.billing_account,
      account.support_tier,
      account.service_owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(account.status, statusClassFromStatus(account.status)));
    target.append(tr);
  }
}

function renderCustomerSites(targetId, sites) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const site of sites) {
    const tr = document.createElement("tr");
    for (const field of [
      site.site_id,
      site.customer_id,
      `${site.city} ${site.country}`,
      site.deployment_stage,
      `${site.it_load_kw} kW`,
      site.substrate,
      site.provisioner,
      site.source_of_truth,
      site.ops_owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(site.status, statusClassFromStatus(site.status)));
    target.append(tr);
  }
}

function renderCustomerWorkflows(targetId, workflows) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const workflow of workflows) {
    const tr = document.createElement("tr");
    for (const field of [
      `${workflow.workflow_id} ${workflow.workflow_name}`,
      workflow.customer_goal,
      workflow.connector_ids,
      workflow.required_mfa_policy,
      workflow.provisioning_system,
      workflow.billing_event,
      workflow.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(workflow.status, statusClassFromStatus(workflow.status)));
    target.append(tr);
  }
}

function renderCustomerMfaPolicies(targetId, policies) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const policy of policies) {
    const tr = document.createElement("tr");
    for (const field of [
      policy.policy_id,
      policy.scope,
      policy.provider_stack,
      policy.factors,
      policy.recovery_method,
      policy.enforcement_point,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(policy.status, statusClassFromStatus(policy.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, policy.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderCustomerBillingPlans(targetId, plans) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const plan of plans) {
    const tr = document.createElement("tr");
    for (const field of [
      plan.plan_id,
      plan.customer_segment,
      plan.services_included,
      plan.rating_engine,
      plan.invoice_engine,
      formatUsd(plan.minimum_commit_usd),
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(plan.status, statusClassFromStatus(plan.status)));
    target.append(tr);
  }
}

function renderCustomerUsageMeters(targetId, meters) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const meter of meters) {
    const tr = document.createElement("tr");
    for (const field of [
      meter.meter_id,
      meter.service_domain,
      meter.source_system,
      meter.metric_name,
      meter.unit,
      meter.collection_cadence,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(meter.status, statusClassFromStatus(meter.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, meter.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderCustomerInvoices(targetId, invoices) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const invoice of invoices) {
    const tr = document.createElement("tr");
    for (const field of [
      invoice.invoice_id,
      invoice.customer_id,
      invoice.billing_period,
      invoice.plan_id,
      invoice.usage_summary,
      formatUsd(invoice.amount_usd),
      formatUsd(invoice.credits_usd),
      formatUsd(invoice.tax_usd),
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(invoice.status, statusClassFromStatus(invoice.status)));
    target.append(tr);
  }
}

function renderCustomerConnectors(targetId, connectors) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const connector of connectors) {
    const tr = document.createElement("tr");
    for (const field of [
      connector.connector_id,
      connector.system_name,
      connector.domain,
      connector.capability,
      connector.adapter_pattern,
      connector.write_mode,
      connector.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(connector.status, statusClassFromStatus(connector.status)));
    target.append(tr);
  }
}

function populateCustomerSelects(overview) {
  renderOptions(
    "customer-select",
    overview.accounts.map((account) => ({
      id: account.customer_id,
      label: `${account.display_name} - ${account.customer_type}`,
    })),
    (option) => option.label
  );
  renderOptions(
    "customer-workflow-select",
    overview.workflows.map((workflow) => ({
      id: workflow.workflow_id,
      label: `${workflow.workflow_name} - ${workflow.status}`,
    })),
    (option) => option.label
  );
  renderOptions(
    "customer-mfa-select",
    overview.mfa_policies.map((policy) => ({
      id: policy.policy_id,
      label: `${policy.policy_id} - ${policy.scope}`,
    })),
    (option) => option.label
  );
  renderOptions(
    "customer-plan-select",
    overview.billing_plans.map((plan) => ({
      id: plan.plan_id,
      label: `${plan.plan_id} - ${plan.customer_segment}`,
    })),
    (option) => option.label
  );
  populateCustomerSiteSelect(overview);
}

function populateCustomerSiteSelect(overview) {
  const customerId = document.getElementById("customer-select")?.value;
  const sites = overview.site_instances.filter((site) => !customerId || site.customer_id === customerId);
  const options = (sites.length ? sites : overview.site_instances).map((site) => ({
    id: site.site_id,
    label: `${site.site_id} - ${site.deployment_stage}`,
  }));
  renderOptions("customer-site-select", options, (option) => option.label);
}

function customerContext(overview) {
  const account = byId(
    overview.accounts,
    "customer_id",
    document.getElementById("customer-select")?.value
  );
  const matchingSites = overview.site_instances.filter((site) => site.customer_id === account?.customer_id);
  const site = byId(
    matchingSites.length ? matchingSites : overview.site_instances,
    "site_id",
    document.getElementById("customer-site-select")?.value
  );
  const workflow = byId(
    overview.workflows,
    "workflow_id",
    document.getElementById("customer-workflow-select")?.value
  );
  const mfa = byId(
    overview.mfa_policies,
    "policy_id",
    document.getElementById("customer-mfa-select")?.value || workflow?.required_mfa_policy
  );
  const plan = byId(
    overview.billing_plans,
    "plan_id",
    document.getElementById("customer-plan-select")?.value
  );
  const invoices = overview.invoice_preview.filter(
    (invoice) => invoice.customer_id === account?.customer_id
  );
  const connectorIds = new Set(splitTokenList(workflow?.connector_ids));
  const connectors = overview.connectors.filter((connector) => connectorIds.has(connector.connector_id));
  const meters = overview.usage_meters.filter(
    (meter) => meter.rating_plan === plan?.plan_id || workflow?.billing_event?.includes("usage")
  );

  return {
    account,
    site,
    workflow,
    mfa,
    plan,
    invoices,
    connectors,
    meters,
    operator: document.getElementById("customer-operator")?.value || "customer-ops",
  };
}

function customerEvidencePath(context) {
  const customer = infrastructureIdSegment(context.account?.customer_id || "customer");
  const workflow = infrastructureIdSegment(context.workflow?.workflow_id || "workflow");
  return `target/assurance/customers/${customer}/${workflow}`;
}

function customerCommandRecord(context, status = customerCommandRuntime.status) {
  const commandId = `CUST-${context.account?.customer_id || "UNKNOWN"}-${context.workflow?.workflow_id || "WORKFLOW"}`;
  return {
    command_id: commandId,
    customer_id: context.account?.customer_id,
    customer_name: context.account?.display_name,
    customer_type: context.account?.customer_type,
    status,
    workflow_id: context.workflow?.workflow_id,
    workflow_name: context.workflow?.workflow_name,
    site_id: context.site?.site_id,
    deployment_stage: context.site?.deployment_stage,
    substrate: context.site?.substrate,
    residency_zone: context.site?.data_residency_zone || context.account?.residency_zone,
    identity_realm: context.account?.identity_realm,
    mfa_policy: context.mfa?.policy_id,
    mfa_provider_stack: context.mfa?.provider_stack,
    billing_account: context.account?.billing_account,
    billing_plan: context.plan?.plan_id,
    invoice_previews: context.invoices.map((invoice) => invoice.invoice_id),
    meters: context.meters.map((meter) => meter.meter_id),
    connectors: context.connectors.map((connector) => connector.connector_id),
    operator: context.operator,
    evidence_bundle: customerEvidencePath(context),
    persistence_table: "osdc_portal.infrastructure_requests",
    generated_at_utc: new Date().toISOString(),
  };
}

function renderCustomerCommandWorkspace(overview, nextStatus = customerCommandRuntime.status) {
  const context = customerContext(overview);
  if (!context.account || !context.workflow) return null;
  customerCommandRuntime.status = nextStatus;
  const record = customerCommandRecord(context, nextStatus);
  customerCommandRuntime.lastRecord = record;

  const status = document.getElementById("customer-command-status");
  if (status) {
    status.className = statusClass(commandStatusKind(nextStatus));
    status.textContent = nextStatus;
  }
  renderFlow("customer-execution-flow", [
    { label: "Customer", value: context.account.display_name },
    { label: "MFA", value: `${context.mfa?.policy_id ?? "policy"} via ${context.mfa?.provider_stack ?? "identity"}` },
    { label: "Provision", value: `${context.site?.deployment_stage ?? "site"} on ${context.site?.substrate ?? "substrate"}` },
    { label: "Billing", value: `${context.plan?.plan_id ?? "plan"} / ${context.meters.length} meters` },
    { label: "Evidence", value: record.evidence_bundle },
  ]);
  renderKeyValueTable("customer-command-record", [
    ["command_id", record.command_id],
    ["customer", `${record.customer_id} ${record.customer_name}`],
    ["workflow", `${record.workflow_id} ${record.workflow_name}`],
    ["site", record.site_id],
    ["mfa_policy", record.mfa_policy],
    ["billing_plan", record.billing_plan],
    ["connectors", record.connectors],
    ["status", record.status],
  ]);
  renderKeyValueTable("customer-evidence-bundle", [
    ["evidence_bundle", record.evidence_bundle],
    ["workflow_docs", context.workflow.evidence_path],
    ["mfa_docs", context.mfa?.evidence_path],
    ["billing_docs", "docs/commercial/billing-and-metering.md"],
    ["persistence_table", record.persistence_table],
  ]);
  renderCustomerExecutionSteps(context);

  const output = document.getElementById("customer-action-output");
  if (output) {
    output.textContent = `${record.command_id}: ${context.workflow.customer_goal}`;
  }
  return { context, record };
}

function renderCustomerExecutionSteps(context) {
  const target = document.getElementById("customer-execution-steps");
  if (!target) return;
  clear(target);
  const rows = [
    {
      stage: "identity",
      system: context.mfa?.provider_stack ?? "Keycloak",
      action: `enforce ${context.mfa?.policy_id ?? "MFA policy"}`,
      owner: context.mfa?.owner ?? "identity-owner",
      status: customerCommandRuntime.status === "approved" ? "approved" : context.mfa?.status ?? "template",
      evidence: context.mfa?.evidence_path ?? "docs/security/open-source-mfa.md",
    },
    {
      stage: "provision",
      system: context.workflow?.provisioning_system ?? context.site?.provisioner,
      action: `${context.site?.site_id ?? "site"} on ${context.site?.substrate ?? "substrate"}`,
      owner: context.site?.ops_owner ?? context.workflow?.owner,
      status: customerCommandRuntime.status === "approved" ? "ready" : context.site?.status ?? "template",
      evidence: context.workflow?.evidence_path,
    },
    {
      stage: "billing",
      system: `${context.plan?.rating_engine ?? "rating"} + ${context.plan?.invoice_engine ?? "invoice"}`,
      action: `${context.plan?.plan_id ?? "plan"} with ${context.meters.length} meters`,
      owner: context.plan?.approval_owner ?? "finance-owner",
      status: customerCommandRuntime.status === "approved" ? "ready" : context.plan?.status ?? "template",
      evidence: "docs/commercial/billing-and-metering.md",
    },
    ...context.connectors.map((connector) => ({
      stage: "connector",
      system: connector.system_name,
      action: connector.capability,
      owner: connector.owner,
      status: connector.status,
      evidence: connector.evidence_path,
    })),
  ];

  for (const rowData of rows) {
    const tr = document.createElement("tr");
    for (const field of [rowData.stage, rowData.system, rowData.action, rowData.owner]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(rowData.status, statusClassFromStatus(rowData.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, rowData.evidence);
    tr.append(evidence);
    target.append(tr);
  }
}

function recordCustomerCommand(overview, action, nextStatus = "submitted") {
  const rendered = renderCustomerCommandWorkspace(overview, nextStatus);
  if (!rendered) return null;
  const { context, record } = rendered;
  return recordPortalCommand({
    action,
    message: `${action}: ${record.customer_id} ${record.workflow_id}`,
    payload: {
      ...record,
      workflow_goal: context.workflow.customer_goal,
      billing_event: context.workflow.billing_event,
      required_mfa_policy: context.workflow.required_mfa_policy,
      tax_policy: context.plan?.tax_policy,
    },
    status: nextStatus,
    evidenceTarget: record.evidence_bundle,
  });
}

function renderAssuranceAutomationJobs(targetId, jobs) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const job of jobs) {
    const tr = document.createElement("tr");
    for (const field of [
      job.job_id,
      job.purpose,
      job.command,
      job.trigger,
      job.evidence_output,
      job.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(job.status, statusClassFromStatus(job.status)));
    target.append(tr);
  }
}

function renderAssuranceTests(targetId, tests) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of tests) {
    const tr = document.createElement("tr");
    for (const field of [
      item.function_area,
      item.test_id,
      item.target,
      item.test_type,
      item.tool_stack,
      item.trigger,
      item.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, statusClassFromStatus(item.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, item.required_evidence);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderAssuranceRings(targetId, rings) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const ring of rings) {
    const tr = document.createElement("tr");
    for (const field of [
      ring.ring_id,
      ring.scope,
      ring.cadence,
      ring.entry_criteria,
      ring.automated_tests,
      ring.promotion_gate,
      ring.rollback_strategy,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(ring.status, statusClassFromStatus(ring.status)));
    target.append(tr);
  }
}

function renderAssuranceGates(targetId, gates) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const gate of gates) {
    const tr = document.createElement("tr");
    for (const field of [
      gate.stage,
      gate.gate_id,
      gate.applies_to,
      gate.required_checks,
      gate.automation_tool,
      gate.blocking,
      gate.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(gate.status, statusClassFromStatus(gate.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, gate.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderAssuranceThreatStack(targetId, components) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const component of components) {
    const tr = document.createElement("tr");
    for (const field of [
      component.capability,
      component.component_id,
      component.wiz_like_function,
      component.open_source_stack,
      component.integration_surface,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(component.status, statusClassFromStatus(component.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, component.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderAssuranceScannerCoverage(targetId, scanners) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const scanner of scanners) {
    const tr = document.createElement("tr");
    for (const field of [
      scanner.scan_domain,
      scanner.scanner_id,
      scanner.target,
      scanner.default_tool,
      scanner.trigger,
      scanner.output,
      scanner.aggregation,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(scanner.status, statusClassFromStatus(scanner.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, scanner.evidence_path);
    tr.append(evidence);
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

function renderConfigScriptEditor(scripts) {
  const tbody = document.getElementById("edge-script-list");
  const editor = document.getElementById("edge-script-editor");
  const title = document.getElementById("edge-script-title");
  const meta = document.getElementById("edge-script-meta");
  const output = document.getElementById("edge-script-output");
  if (!tbody || !editor || !title || !meta) return;

  let selectedId = scripts[0]?.id;

  const selectScript = (scriptId) => {
    selectedId = scriptId;
    const script = scripts.find((item) => item.id === selectedId) ?? scripts[0];
    if (!script) return;
    title.textContent = `${script.tool} ${script.path}`;
    meta.textContent = `${script.validation_command} | ${script.rollout_target} | ${script.edit_mode}`;
    editor.value = script.content;
    for (const row of tbody.querySelectorAll("tr")) {
      row.classList.toggle("selected-row", row.dataset.scriptId === selectedId);
    }
    if (output) output.textContent = `${script.tool} loaded. Prototype edits model a GitOps review; no live service is changed.`;
  };

  clear(tbody);
  for (const script of scripts) {
    const tr = document.createElement("tr");
    tr.dataset.scriptId = script.id;
    tr.tabIndex = 0;
    for (const field of [script.tool, script.path, script.edit_mode, script.risk]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.addEventListener("click", () => selectScript(script.id));
    tr.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        selectScript(script.id);
      }
    });
    tbody.append(tr);
  }

  selectScript(selectedId);

  document.getElementById("edge-script-validate")?.addEventListener("click", (event) => {
    event.preventDefault();
    const script = scripts.find((item) => item.id === selectedId);
    if (output && script) {
      const message = `Prototype validation queued: ${script.validation_command}`;
      const command = recordPortalCommand({
        action: "Validate edge config script",
        message,
        payload: {
          script_id: script.id,
          path: script.path,
          validation_command: script.validation_command,
          owner: script.owner,
        },
        status: "submitted",
        evidenceTarget: script.evidence_path,
      });
      output.textContent = `${command.command_id}: ${message}`;
    }
  });

  document.getElementById("edge-script-stage")?.addEventListener("click", (event) => {
    event.preventDefault();
    const script = scripts.find((item) => item.id === selectedId);
    if (output && script) {
      const message = `Prototype GitOps change staged for ${script.path}; production flow must open a reviewed change request before rollout.`;
      const command = recordPortalCommand({
        action: "Stage edge GitOps change",
        message,
        payload: {
          script_id: script.id,
          path: script.path,
          rollout_path: script.rollout_path,
          validation_command: script.validation_command,
          owner: script.owner,
        },
        status: "submitted",
        evidenceTarget: script.evidence_path,
      });
      output.textContent = `${command.command_id}: ${message}`;
    }
  });
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

function renderDeploymentStackProfiles(targetId, profiles) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const profile of profiles) {
    const tr = document.createElement("tr");
    for (const field of [
      `${profile.stage} / ${profile.it_load_kw.toLocaleString()} kW`,
      profile.default_cloud_substrate,
      profile.alternate_cloud_substrate,
      profile.storage_substrate,
      profile.bare_metal_lifecycle,
      profile.developer_gitops,
      profile.when_to_use,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    const status = document.createElement("td");
    status.append(badge(profile.maturity, profile.maturity === "pilot" ? "info" : "normal"));
    tr.append(status);
    target.append(tr);
  }
}

function splitTokenList(value) {
  return String(value ?? "")
    .split("+")
    .map((item) => item.trim())
    .filter(Boolean);
}

function byId(rows, key, id) {
  return rows.find((row) => row[key] === id) ?? rows[0];
}

function renderInfrastructureOptions(workbench) {
  renderOptions(
    "infra-workflow-select",
    workbench.workflows.map((workflow) => ({
      id: workflow.workflow_id,
      label: `${workflow.workflow_name} - ${workflow.service_domain}`,
    })),
    (option) => option.label
  );
  renderOptions(
    "infra-stack-select",
    workbench.stack_profiles.map((profile) => ({
      id: profile.profile_id,
      label: `${profile.stage} - ${profile.default_cloud_substrate}`,
    })),
    (option) => option.label
  );

  const workflowSelect = document.getElementById("infra-workflow-select");
  if (workflowSelect && workbench.workflows.some((workflow) => workflow.workflow_id === "WF_PROVISION_VM")) {
    workflowSelect.value = "WF_PROVISION_VM";
  }
  const stackSelect = document.getElementById("infra-stack-select");
  if (stackSelect && workbench.stack_profiles.some((profile) => profile.profile_id === "DSP_250KW_REGIONAL")) {
    stackSelect.value = "DSP_250KW_REGIONAL";
  }
}

function renderInfrastructureWorkflows(targetId, workflows, selectedId, onSelect) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const workflow of workflows) {
    const tr = document.createElement("tr");
    tr.dataset.workflowId = workflow.workflow_id;
    if (workflow.workflow_id === selectedId) tr.className = "selected-row";
    tr.tabIndex = 0;
    for (const field of [
      `${workflow.workflow_id} ${workflow.workflow_name}`,
      workflow.user_goal,
      workflow.service_domain,
      workflow.default_substrate,
      workflow.connector_ids,
      workflow.required_test_ids,
      workflow.required_gate_ids,
      workflow.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(workflow.status, statusClassFromStatus(workflow.status)));
    tr.addEventListener("click", () => onSelect(workflow.workflow_id));
    tr.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        onSelect(workflow.workflow_id);
      }
    });
    target.append(tr);
  }
}

function renderInfrastructureConnectors(targetId, connectors) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const connector of connectors) {
    const tr = document.createElement("tr");
    for (const field of [
      connector.connector_id,
      connector.system_name,
      connector.capability,
      connector.adapter_pattern,
      connector.write_mode,
      connector.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(connector.status, statusClassFromStatus(connector.status)));
    target.append(tr);
  }
}

function renderInfrastructureAdapters(targetId, milestones, selectedWorkflowId) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  const sorted = [...milestones].sort((left, right) => {
    const leftSelected = splitTokenList(left.workflow_ids).includes(selectedWorkflowId);
    const rightSelected = splitTokenList(right.workflow_ids).includes(selectedWorkflowId);
    if (leftSelected !== rightSelected) return leftSelected ? -1 : 1;
    return Number(left.priority) - Number(right.priority);
  });
  for (const milestone of sorted) {
    const tr = document.createElement("tr");
    if (splitTokenList(milestone.workflow_ids).includes(selectedWorkflowId)) {
      tr.className = "selected-row";
    }
    for (const field of [
      milestone.priority,
      `${milestone.milestone_id} ${milestone.adapter_target}`,
      milestone.backend_system,
      milestone.initial_mode,
      milestone.production_write_path,
      milestone.first_capability,
      milestone.workflow_ids,
      milestone.proof_command,
      milestone.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(milestone.status, statusClassFromStatus(milestone.status)));
    const next = document.createElement("td");
    next.append(text(milestone.next_step));
    tr.append(next);
    target.append(tr);
  }
}

function renderInfrastructureProofs(targetId, proofs, milestones, selectedWorkflowId) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  const selectedMilestones = new Set(
    milestones
      .filter((milestone) => splitTokenList(milestone.workflow_ids).includes(selectedWorkflowId))
      .map((milestone) => milestone.milestone_id)
  );
  const sorted = [...proofs].sort((left, right) => {
    const leftSelected = selectedMilestones.has(left.milestone_id);
    const rightSelected = selectedMilestones.has(right.milestone_id);
    if (leftSelected !== rightSelected) return leftSelected ? -1 : 1;
    return left.milestone_id.localeCompare(right.milestone_id);
  });
  for (const proof of sorted) {
    const tr = document.createElement("tr");
    if (selectedMilestones.has(proof.milestone_id)) tr.className = "selected-row";
    for (const field of [
      proof.proof_id,
      proof.milestone_id,
      proof.adapter_target,
      proof.proof_command,
      proof.mode,
      proof.scope,
      proof.required_env,
      proof.evidence_output,
      proof.required_gate,
      proof.owner,
    ]) {
      const td = document.createElement("td");
      if (field === proof.evidence_output) {
        appendValueOrRepoLink(td, field);
      } else {
        td.append(text(field));
      }
      tr.append(td);
    }
    tr.append(statusCell(proof.status, statusClassFromStatus(proof.status)));
    target.append(tr);
  }
}

function renderInfrastructurePersistence(schema) {
  const target = document.getElementById("infra-persistence-schema");
  if (!target || !schema) return;
  clear(target);

  renderFlow("infra-persistence-flow", [
    { label: "Schema", value: schema.schema_name },
    { label: "Migration", value: schema.migration_path },
    { label: "Docs", value: schema.docs_path },
    { label: "Boundary", value: schema.boundary },
  ]);

  for (const table of schema.tables ?? []) {
    const tr = document.createElement("tr");
    const name = document.createElement("td");
    name.append(text(table.table_name));
    tr.append(name);
    const purpose = document.createElement("td");
    purpose.append(text(table.purpose));
    tr.append(purpose);
    const columns = document.createElement("td");
    columns.append(text((table.columns ?? []).join(", ")));
    tr.append(columns);
    const count = document.createElement("td");
    count.append(text(table.column_count));
    tr.append(count);
    target.append(tr);
  }
}

function renderInfrastructureTests(targetId, tests) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const item of tests) {
    const tr = document.createElement("tr");
    for (const field of [
      item.test_id,
      item.function_area,
      item.target,
      item.test_type,
      item.tool_stack,
      item.trigger,
      item.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(item.status, statusClassFromStatus(item.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, item.required_evidence);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderInfrastructureGates(targetId, gates) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const gate of gates) {
    const tr = document.createElement("tr");
    for (const field of [
      gate.gate_id,
      gate.stage,
      gate.applies_to,
      gate.required_checks,
      gate.automation_tool,
      gate.blocking,
      gate.owner,
    ]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(gate.status, statusClassFromStatus(gate.status)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, gate.evidence_path);
    tr.append(evidence);
    target.append(tr);
  }
}

function infrastructureIdSegment(value) {
  return String(value ?? "resource")
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "") || "resource";
}

function resetInfrastructureRuntime(runtime) {
  runtime.status = "draft";
  runtime.submitted = false;
  runtime.checksRun = false;
  runtime.approved = false;
  runtime.generatedAt = new Date().toISOString();
}

function infrastructureContext(workbench) {
  const workflowId = document.getElementById("infra-workflow-select")?.value;
  const stackId = document.getElementById("infra-stack-select")?.value;
  const workflow = byId(workbench.workflows, "workflow_id", workflowId);
  const stack = byId(workbench.stack_profiles, "profile_id", stackId);
  if (!workflow || !stack) return null;

  const connectorIds = new Set(splitTokenList(workflow.connector_ids));
  const testIds = new Set(splitTokenList(workflow.required_test_ids));
  const gateIds = new Set(splitTokenList(workflow.required_gate_ids));
  const adapterMilestones = workbench.adapter_milestones.filter((milestone) =>
    splitTokenList(milestone.workflow_ids).includes(workflow.workflow_id)
  );
  const selectedMilestoneIds = new Set(adapterMilestones.map((milestone) => milestone.milestone_id));
  const adapterProofs = (workbench.adapter_proofs ?? []).filter((proof) =>
    selectedMilestoneIds.has(proof.milestone_id)
  );
  const resourceName = document.getElementById("infra-resource-name")?.value || "new-resource";

  return {
    workflow,
    stack,
    connectors: workbench.connectors.filter((connector) => connectorIds.has(connector.connector_id)),
    tests: workbench.test_harnesses.filter((item) => testIds.has(item.test_id)),
    gates: workbench.upgrade_gates.filter((gate) => gateIds.has(gate.gate_id)),
    adapterMilestones,
    adapterProofs,
    job: workbench.automation_jobs.find((item) => item.job_id === workflow.automation_job_id),
    changeMode: document.getElementById("infra-change-mode")?.value ?? "gitops-pr",
    environment: document.getElementById("infra-environment")?.value ?? "development",
    resourceName,
    resourceSlug: infrastructureIdSegment(resourceName),
    owner: document.getElementById("infra-owner")?.value || workflow.owner,
  };
}

function infrastructureRisk(environment) {
  if (environment === "production") return "high";
  if (environment === "production-canary" || environment === "staging") return "medium";
  return "low";
}

function infrastructureChangeRecord(context, runtime) {
  const changeId = `CR-${context.workflow.workflow_id.replace(/^WF_/, "")}-${context.resourceSlug}`.toUpperCase();
  const evidencePath = `target/assurance/changes/${changeId.toLowerCase()}`;
  return {
    change_id: changeId,
    request_id: `REQ-${context.resourceSlug.toUpperCase()}`,
    title: `${context.workflow.workflow_name}: ${context.resourceName}`,
    requester: context.owner,
    target_system: context.workflow.service_domain,
    target_environment: context.environment,
    change_type: "infrastructure-plan",
    risk: infrastructureRisk(context.environment),
    status: runtime.status,
    workflow_id: context.workflow.workflow_id,
    deployment_profile: context.stack.profile_id,
    change_mode: context.changeMode,
    connectors: context.connectors.map((connector) => connector.connector_id),
    adapter_milestones: context.adapterMilestones.map((milestone) => milestone.milestone_id),
    adapter_proofs: context.adapterProofs.map((proof) => proof.proof_id),
    required_tests: context.tests.map((test) => test.test_id),
    required_gates: context.gates.map((gate) => gate.gate_id),
    automation_job: context.job?.job_id ?? context.workflow.automation_job_id,
    automation_command: context.job?.command ?? "",
    rollout_plan: {
      strategy: context.changeMode,
      stages: ["plan", "validate", "approve", "rollout", "evidence"],
    },
    rollback_plan: {
      trigger_conditions: ["failed gate", "failed health check", "owner rollback"],
      restore_actions: ["revert GitOps change", "restore previous declared state"],
    },
    evidence_bundle: evidencePath,
    persistence_table: "osdc_portal.change_requests",
    generated_at_utc: runtime.generatedAt,
  };
}

function renderKeyValueTable(targetId, rows) {
  const target = document.getElementById(targetId);
  if (!target) return;
  clear(target);
  for (const [key, value] of rows) {
    const tr = document.createElement("tr");
    const keyCell = document.createElement("td");
    keyCell.append(text(key));
    const valueCell = document.createElement("td");
    const display = Array.isArray(value) ? value.join(", ") : String(value ?? "");
    appendValueOrRepoLink(valueCell, display);
    tr.append(keyCell, valueCell);
    target.append(tr);
  }
}

function executionStatusKind(status) {
  const value = String(status ?? "").toLowerCase();
  if (["passed", "approved", "ready", "recorded", "submitted"].includes(value)) return "normal";
  if (["queued", "draft", "planned", "pilot", "generated", "validated"].includes(value)) return "info";
  if (["blocked", "failed", "rejected"].includes(value)) return "danger";
  return "warn";
}

function renderInfrastructureExecutionSteps(context, runtime, record) {
  const target = document.getElementById("infra-execution-steps");
  if (!target) return;
  clear(target);

  const rows = [
    ...context.connectors.map((connector) => ({
      stage: "read",
      item: connector.system_name,
      owner: connector.owner,
      action: connector.capability,
      state: connector.status,
      evidence: connector.evidence_path,
    })),
    ...context.adapterProofs.map((proof) => ({
      stage: "proof",
      item: proof.proof_id,
      owner: proof.owner,
      action: proof.proof_command,
      state: runtime.checksRun ? "recorded" : proof.status,
      evidence: proof.evidence_output,
    })),
    ...context.tests.map((test) => ({
      stage: "validate",
      item: test.test_id,
      owner: test.tool_stack,
      action: test.test_type,
      state: runtime.checksRun ? "passed" : "queued",
      evidence: test.required_evidence,
    })),
    ...context.gates.map((gate) => ({
      stage: "gate",
      item: gate.gate_id,
      owner: gate.owner,
      action: gate.required_checks,
      state: runtime.approved && gate.gate_id === "GATE_APPROVAL" ? "approved" : runtime.checksRun ? "ready" : "blocked",
      evidence: gate.evidence_path,
    })),
    {
      stage: "persist",
      item: "change request",
      owner: "osdc_portal",
      action: record.persistence_table,
      state: runtime.submitted ? "submitted" : "generated",
      evidence: "crates/osdc-portal/migrations/0001_osdc_portal_state.sql",
    },
  ];

  for (const row of rows) {
    const tr = document.createElement("tr");
    for (const field of [row.stage, row.item, row.owner, row.action]) {
      const td = document.createElement("td");
      td.append(text(field));
      tr.append(td);
    }
    tr.append(statusCell(row.state, executionStatusKind(row.state)));
    const evidence = document.createElement("td");
    appendValueOrRepoLink(evidence, row.evidence);
    tr.append(evidence);
    target.append(tr);
  }
}

function renderInfrastructureWorkspace(context, runtime) {
  const record = infrastructureChangeRecord(context, runtime);
  const status = document.getElementById("infra-change-status");
  if (status) {
    status.className = statusClass(executionStatusKind(runtime.status));
    status.textContent = runtime.status;
  }

  renderFlow("infra-execution-flow", [
    { label: "Request", value: `${record.change_id} / ${runtime.status}` },
    { label: "Validate", value: runtime.checksRun ? `${context.tests.length} passed` : `${context.tests.length} queued` },
    { label: "Approve", value: runtime.approved ? "approved" : `${context.gates.length} gates` },
    { label: "Rollout", value: runtime.approved ? context.changeMode : "not staged" },
    { label: "Evidence", value: record.evidence_bundle },
  ]);

  renderKeyValueTable("infra-change-record", [
    ["change_id", record.change_id],
    ["request_id", record.request_id],
    ["status", record.status],
    ["workflow", `${context.workflow.workflow_id} ${context.workflow.workflow_name}`],
    ["resource", context.resourceName],
    ["environment", context.environment],
    ["owner", context.owner],
    ["risk", record.risk],
    ["change_mode", context.changeMode],
    ["automation", record.automation_command || record.automation_job],
  ]);
  renderKeyValueTable("infra-evidence-bundle", [
    ["bundle_path", record.evidence_bundle],
    ["workflow_evidence", context.workflow.evidence_path],
    ["tests", record.required_tests],
    ["gates", record.required_gates],
    ["connectors", record.connectors],
    ["adapter_proofs", record.adapter_proofs],
    ["persistence", record.persistence_table],
  ]);
  renderInfrastructureExecutionSteps(context, runtime, record);
  return record;
}

function renderInfrastructureSelection(workbench, applyFilters = {}, runtime = { status: "draft" }) {
  const context = infrastructureContext(workbench);
  if (!context) return null;
  const { workflow, stack, connectors, tests, gates, adapterMilestones, job, changeMode, environment, resourceName, owner } =
    context;
  const record = renderInfrastructureWorkspace(context, runtime);

  renderInfrastructureWorkflows("infra-workflows", workbench.workflows, workflow.workflow_id, (nextId) => {
    const select = document.getElementById("infra-workflow-select");
    if (select) select.value = nextId;
    resetInfrastructureRuntime(runtime);
    renderInfrastructureSelection(workbench, applyFilters, runtime);
  });
  renderInfrastructureConnectors("infra-connectors", connectors);
  renderInfrastructureAdapters(
    "infra-adapters",
    workbench.adapter_milestones,
    workflow.workflow_id
  );
  renderInfrastructureProofs(
    "infra-adapter-proofs",
    workbench.adapter_proofs,
    workbench.adapter_milestones,
    workflow.workflow_id
  );
  renderInfrastructurePersistence(workbench.persistence_schema);
  renderInfrastructureTests("infra-tests", tests);
  renderInfrastructureGates("infra-gates", gates);

  const selectedStatus = document.getElementById("infra-selected-status");
  if (selectedStatus) {
    selectedStatus.className = statusClass(statusClassFromStatus(workflow.status));
    selectedStatus.textContent = workflow.status;
  }
  const stackStatus = document.getElementById("infra-stack-status");
  if (stackStatus) {
    stackStatus.className = statusClass(statusClassFromStatus(stack.maturity));
    stackStatus.textContent = stack.maturity;
  }

  renderFlow("infra-plan-flow", [
    { label: "Request", value: `${resourceName} / ${workflow.workflow_name}` },
    { label: "Policy", value: `${owner} owns ${workflow.service_domain}` },
    { label: "Tests", value: `${tests.length} harnesses / ${gates.length} gates` },
    { label: "Rollout", value: `${changeMode} to ${environment}` },
    { label: "Audit", value: workflow.evidence_path },
  ]);
  renderFlow("infra-stack-flow", [
    { label: "Cloud", value: stack.default_cloud_substrate },
    { label: "Storage", value: stack.storage_substrate },
    { label: "Source", value: stack.source_of_truth },
    { label: "Security", value: stack.edge_security },
    { label: "GitOps", value: stack.developer_gitops },
  ]);

  const output = document.getElementById("infra-action-output");
  if (output) {
    const jobText = job ? `${job.job_id} (${job.command})` : workflow.automation_job_id;
    output.textContent = [
      `${record.change_id}: ${workflow.user_goal} for ${resourceName} in ${environment}.`,
      `Use ${stack.stage} on ${stack.default_cloud_substrate}.`,
      `Stage through ${changeMode} with ${connectors.length} connectors, ${adapterMilestones.length} adapter milestones, ${tests.length} tests, ${gates.length} gates, and automation ${jobText}.`,
      `Evidence target: ${record.evidence_bundle}.`,
    ].join(" ");
  }

  applyFilters.workflows?.();
  applyFilters.connectors?.();
  applyFilters.adapters?.();
  applyFilters.proofs?.();
  applyFilters.tests?.();
  applyFilters.gates?.();
  return record;
}

async function hydrateInfrastructure() {
  const workbench = await api("/api/infrastructure/workbench");
  const runtime = {
    status: "draft",
    submitted: false,
    checksRun: false,
    approved: false,
    generatedAt: new Date().toISOString(),
  };
  renderMetrics("infra-metrics", workbench.metrics);
  renderInfrastructureOptions(workbench);

  const applyFilters = {
    workflows: attachTableFilters({
      textInputId: "infra-workflow-filter",
      statusSelectId: "infra-workflow-status",
      tbodyId: "infra-workflows",
    }),
    connectors: attachTableFilters({
      textInputId: "infra-connector-filter",
      tbodyId: "infra-connectors",
    }),
    adapters: attachTableFilters({
      textInputId: "infra-adapter-filter",
      statusSelectId: "infra-adapter-status",
      tbodyId: "infra-adapters",
    }),
    proofs: attachTableFilters({
      textInputId: "infra-proof-filter",
      statusSelectId: "infra-proof-status",
      tbodyId: "infra-adapter-proofs",
    }),
    tests: attachTableFilters({
      textInputId: "infra-test-filter",
      tbodyId: "infra-tests",
    }),
    gates: attachTableFilters({
      textInputId: "infra-gate-filter",
      tbodyId: "infra-gates",
    }),
  };

  const renderSelected = (reset = false) => {
    if (reset) resetInfrastructureRuntime(runtime);
    return renderInfrastructureSelection(workbench, applyFilters, runtime);
  };

  renderSelected();

  for (const id of [
    "infra-workflow-select",
    "infra-stack-select",
    "infra-environment",
    "infra-resource-name",
    "infra-owner",
    "infra-change-mode",
  ]) {
    document
      .getElementById(id)
      ?.addEventListener("change", () => renderSelected(true));
  }
  document
    .getElementById("infra-resource-name")
    ?.addEventListener("input", () => renderSelected(true));
  document.getElementById("infra-preview")?.addEventListener("click", (event) => {
    event.preventDefault();
    renderSelected(true);
  });

  document.getElementById("infra-refresh")?.addEventListener("click", (event) => {
    event.preventDefault();
    const record = renderSelected();
    const output = document.getElementById("infra-action-output");
    if (output && record) output.textContent = `${record.change_id}: workbench state refreshed.`;
  });
  document.getElementById("infra-open-change")?.addEventListener("click", (event) => {
    event.preventDefault();
    runtime.submitted = true;
    runtime.status = "submitted";
    const record = renderSelected();
    if (!record) return;
    const command = recordPortalCommand({
      action: "Open infrastructure change",
      message: `${record.change_id} submitted to ${record.persistence_table}`,
      payload: { change_request: record },
      status: "submitted",
      evidenceTarget: record.evidence_bundle,
    });
    const output = document.getElementById("infra-action-output");
    if (output && record) output.textContent = `${command.command_id}: ${record.change_id}: submitted to ${record.persistence_table}; rollout mode ${record.change_mode}.`;
  });
  document.getElementById("infra-run-tests")?.addEventListener("click", (event) => {
    event.preventDefault();
    runtime.submitted = true;
    runtime.checksRun = true;
    runtime.status = "validated";
    const record = renderSelected();
    if (!record) return;
    const command = recordPortalCommand({
      action: "Run infrastructure tests",
      message: `${record.required_tests.length} tests passed; ${record.required_gates.length} gates ready`,
      payload: { change_request: record },
      status: "validated",
      evidenceTarget: record.evidence_bundle,
    });
    const output = document.getElementById("infra-action-output");
    if (output && record) output.textContent = `${command.command_id}: ${record.change_id}: ${record.required_tests.length} tests passed and ${record.required_gates.length} gates are ready for approval.`;
  });
  document.getElementById("infra-approve-change")?.addEventListener("click", (event) => {
    event.preventDefault();
    runtime.submitted = true;
    if (runtime.checksRun) {
      runtime.approved = true;
      runtime.status = "approved";
    } else {
      runtime.approved = false;
      runtime.status = "blocked";
    }
    const record = renderSelected();
    if (!record) return;
    const command = recordPortalCommand({
      action: "Approve infrastructure rollout",
      message: runtime.approved
        ? `${record.change_id} approved for ${record.change_mode}`
        : `${record.change_id} blocked by queued validation`,
      payload: { change_request: record },
      status: runtime.approved ? "approved" : "blocked",
      evidenceTarget: record.evidence_bundle,
    });
    const output = document.getElementById("infra-action-output");
    if (output && record) {
      output.textContent = runtime.approved
        ? `${command.command_id}: ${record.change_id}: approved for ${record.change_mode}; evidence bundle ${record.evidence_bundle}.`
        : `${command.command_id}: ${record.change_id}: approval blocked by queued validation.`;
    }
  });
  document.getElementById("infra-download-change")?.addEventListener("click", (event) => {
    event.preventDefault();
    const context = infrastructureContext(workbench);
    if (!context) return;
    const record = infrastructureChangeRecord(context, runtime);
    downloadJson(`${record.change_id.toLowerCase()}.json`, record);
    const command = recordPortalCommand({
      action: "Export infrastructure change JSON",
      message: `${record.change_id} JSON change record generated`,
      payload: { change_request: record },
      status: "validated",
      evidenceTarget: record.evidence_bundle,
    });
    const output = document.getElementById("infra-action-output");
    if (output) output.textContent = `${command.command_id}: ${record.change_id}: JSON change record generated.`;
  });
  wireDownloadButton("infra-export-workflows", "infra-workflows", "osdc-infrastructure-workflows.csv");
  wireDownloadButton("infra-export-adapters", "infra-adapters", "osdc-live-adapter-roadmap.csv");
  wireDownloadButton("infra-export-proofs", "infra-adapter-proofs", "osdc-live-adapter-proofs.csv");
  wireDownloadButton("infra-export-persistence", "infra-persistence-schema", "osdc-portal-persistence-schema.csv");
  wireDownloadButton("infra-export-tests", "infra-tests", "osdc-infrastructure-required-tests.csv");
  wireDownloadButton("infra-export-gates", "infra-gates", "osdc-infrastructure-required-gates.csv");
}

async function hydratePlanner() {
  const planning = await api("/api/cost/planning");
  let selectedId = "S2";

  renderMetrics("planner-metrics", planning.metrics);
  renderPriceBasis("planner-price-basis", planning.price_basis);
  renderDeploymentStackProfiles("planner-stack-profiles", planning.deployment_profiles);
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
  attachTableFilters({
    textInputId: "planner-stack-filter",
    tbodyId: "planner-stack-profiles",
  });
  wireActionButton("planner-refresh", "Planner data refreshed", "planner-action-output");
  wireDownloadButton("planner-export", "planner-scenarios", "osdc-cost-scenarios.csv");
  wireDownloadButton("planner-category-export", "planner-categories", "osdc-cost-categories.csv");
  wireDownloadButton("planner-price-export", "planner-price-basis", "osdc-price-basis.csv");
  wireDownloadButton(
    "planner-stack-export",
    "planner-stack-profiles",
    "osdc-deployment-stack-profiles.csv"
  );
  document.getElementById("planner-select-pilot")?.addEventListener("click", () => {
    selectedId = "S2";
    renderSelected();
    recordPortalCommand({
      action: "Select regional pilot scenario",
      message: "250 kW regional pilot selected for planning",
      payload: {
        scenario_id: "S2",
        surface: "planner",
      },
      status: "validated",
      evidenceTarget: "data/costing/scenario-costs-2026.csv",
    });
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
    recordPortalCommand({
      action: "Start tenant provision request",
      message: "Tenant provisioning request opened",
      payload: {
        service_id: document.getElementById("provision-service")?.value,
        shape_id: document.getElementById("provision-shape")?.value,
        environment: "tenant",
      },
      status: "draft",
      evidenceTarget: "target/assurance/commands/tenant",
    });
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

async function hydrateLifecycle() {
  const [overview, developer, dataPlatform, commercialGaps, remoteHandsPricebook, accessRoles] = await Promise.all([
    api("/api/lifecycle/overview"),
    api("/api/developer/platform"),
    api("/api/data-platform/overview"),
    api("/api/commercial/gaps"),
    api("/api/commercial/remote-hands-pricebook"),
    api("/api/commercial/access-roles"),
  ]);

  renderMetrics("lifecycle-metrics", overview.metrics);
  renderLifecycleStages("lifecycle-stages", overview.stages);
  renderLifecycleWorkItems("lifecycle-work", overview.work_items);
  renderLifecycleEvidence("lifecycle-evidence", overview.evidence);
  renderLifecycleServices("lifecycle-services", overview.services);
  renderLifecycleDocuments("lifecycle-documents", overview.documents);
  renderDeveloperServices("lifecycle-developer", developer.services, true);
  renderDataPlatformServices("lifecycle-data-platform", dataPlatform.services, true);
  renderCommercialGaps("lifecycle-commercial-gaps", commercialGaps);
  renderCommercialRemoteHandsPricebook(
    "lifecycle-commercial-remote-hands",
    remoteHandsPricebook
  );
  renderCommercialAccessRoles("lifecycle-commercial-access", accessRoles);

  attachCardFilter("lifecycle-stage-filter", "lifecycle-stages");
  attachTableFilters({
    textInputId: "lifecycle-work-filter",
    statusSelectId: "lifecycle-work-status",
    tbodyId: "lifecycle-work",
  });
  attachColumnFilter({
    textInputId: "lifecycle-evidence-filter",
    selectId: "lifecycle-evidence-source",
    tbodyId: "lifecycle-evidence",
    columnIndex: 0,
  });
  attachTableFilters({
    textInputId: "lifecycle-service-filter",
    statusSelectId: "lifecycle-service-status",
    tbodyId: "lifecycle-services",
  });
  attachTableFilters({
    textInputId: "lifecycle-doc-filter",
    tbodyId: "lifecycle-documents",
  });
  attachTableFilters({
    textInputId: "lifecycle-developer-filter",
    tbodyId: "lifecycle-developer",
  });
  attachTableFilters({
    textInputId: "lifecycle-data-filter",
    tbodyId: "lifecycle-data-platform",
  });
  attachMultiTableFilter("lifecycle-commercial-filter", [
    "lifecycle-commercial-gaps",
    "lifecycle-commercial-remote-hands",
    "lifecycle-commercial-access",
  ]);

  wireActionButton(
    "lifecycle-refresh",
    "Lifecycle data refreshed.",
    "lifecycle-action-output"
  );
  wireActionButton(
    "lifecycle-open-change",
    "Lifecycle change request staged.",
    "lifecycle-action-output"
  );
  wireActionButton(
    "lifecycle-stage-gate",
    "Gate review bundle staged.",
    "lifecycle-action-output"
  );
  wireActionButton(
    "lifecycle-create-evidence",
    "Evidence request staged.",
    "lifecycle-action-output"
  );
  wireActionButton(
    "lifecycle-commissioning-pack",
    "Commissioning pack assembled.",
    "lifecycle-action-output"
  );
  wireActionButton(
    "lifecycle-handover-pack",
    "Handover pack assembled.",
    "lifecycle-action-output"
  );

  wireDownloadButton("lifecycle-export-stages", "lifecycle-stages", "osdc-lifecycle-stages.csv");
  wireDownloadButton("lifecycle-export-work", "lifecycle-work", "osdc-lifecycle-work.csv");
  wireDownloadButton(
    "lifecycle-export-evidence",
    "lifecycle-evidence",
    "osdc-lifecycle-evidence.csv"
  );
  wireDownloadButton(
    "lifecycle-export-services",
    "lifecycle-services",
    "osdc-lifecycle-services.csv"
  );
  wireDownloadButton("lifecycle-export-docs", "lifecycle-documents", "osdc-lifecycle-docs.csv");
  wireDownloadButton(
    "lifecycle-export-commercial-gaps",
    "lifecycle-commercial-gaps",
    "osdc-commercial-gaps.csv"
  );
  wireDownloadButton(
    "lifecycle-export-commercial-remote-hands",
    "lifecycle-commercial-remote-hands",
    "osdc-remote-hands-pricebook.csv"
  );
  wireDownloadButton(
    "lifecycle-export-commercial-access",
    "lifecycle-commercial-access",
    "osdc-access-roles.csv"
  );
}

async function hydrateDeveloper() {
  const platform = await api("/api/developer/platform");

  renderMetrics("developer-metrics", platform.metrics);
  renderDeveloperServices("developer-services", platform.services);
  renderDeveloperTemplates("developer-templates", platform.templates);
  renderDeveloperEnvironments("developer-environments", platform.environments);
  renderDeveloperPromotionGates("developer-gates", platform.promotion_gates);
  renderVsCodeWorkflows("developer-vscode", platform.vscode_workflows);

  attachTableFilters({
    textInputId: "developer-service-filter",
    statusSelectId: "developer-service-status",
    tbodyId: "developer-services",
  });
  attachTableFilters({
    textInputId: "developer-template-filter",
    statusSelectId: "developer-template-status",
    tbodyId: "developer-templates",
  });
  attachTableFilters({
    textInputId: "developer-environment-filter",
    tbodyId: "developer-environments",
  });
  attachTableFilters({
    textInputId: "developer-gate-filter",
    tbodyId: "developer-gates",
  });
  attachTableFilters({
    textInputId: "developer-vscode-filter",
    tbodyId: "developer-vscode",
  });

  wireActionButton(
    "developer-refresh",
    "Developer platform data refreshed.",
    "developer-action-output"
  );
  wireActionButton(
    "developer-create-service",
    "Service template request staged.",
    "developer-action-output"
  );
  wireActionButton(
    "developer-create-repo",
    "Forgejo repository request staged.",
    "developer-action-output"
  );
  wireActionButton(
    "developer-run-pipeline",
    "Pipeline run request staged.",
    "developer-action-output"
  );
  wireActionButton(
    "developer-open-gitops",
    "GitOps pull request staged.",
    "developer-action-output"
  );
  wireActionButton(
    "developer-open-vscode",
    "Use any VS Code link in the templates or workflows table.",
    "developer-action-output"
  );

  wireDownloadButton("developer-export-templates", "developer-templates", "osdc-dev-templates.csv");
  wireDownloadButton("developer-export-vscode", "developer-vscode", "osdc-vscode-workflows.csv");
}

async function hydrateCommercial() {
  const [
    gaps,
    standards,
    slas,
    colocation,
    crossConnects,
    remoteProducts,
    pricebook,
    accessRoles,
    auditEvidence,
  ] = await Promise.all([
    api("/api/commercial/gaps"),
    api("/api/commercial/standards"),
    api("/api/commercial/sla-classes"),
    api("/api/commercial/colocation-products"),
    api("/api/commercial/cross-connect-products"),
    api("/api/commercial/remote-hands-products"),
    api("/api/commercial/remote-hands-pricebook"),
    api("/api/commercial/access-roles"),
    api("/api/commercial/audit-evidence"),
  ]);

  renderCommercialMetrics({
    gaps,
    standards,
    colocation,
    crossConnects,
    remoteProducts,
    pricebook,
    accessRoles,
    auditEvidence,
  });
  renderCommercialGaps("commercial-gaps", gaps, true);
  renderCommercialStandards("commercial-standards", standards);
  renderCommercialSlas("commercial-slas", slas);
  renderCommercialColocationProducts("commercial-colocation", colocation);
  renderCommercialCrossConnectProducts("commercial-cross-connects", crossConnects);
  renderCommercialRemoteHandsProducts("commercial-remote-products", remoteProducts);
  renderCommercialRemoteHandsPricebook("commercial-pricebook", pricebook);
  renderCommercialAccessRoles("commercial-access", accessRoles);
  renderCommercialAuditEvidence("commercial-audit", auditEvidence);

  attachTableFilters({
    textInputId: "commercial-gap-filter",
    statusSelectId: "commercial-gap-status",
    tbodyId: "commercial-gaps",
  });
  attachTableFilters({
    textInputId: "commercial-standard-filter",
    tbodyId: "commercial-standards",
  });
  attachTableFilters({
    textInputId: "commercial-sla-filter",
    tbodyId: "commercial-slas",
  });
  attachTableFilters({
    textInputId: "commercial-colocation-filter",
    tbodyId: "commercial-colocation",
  });
  attachTableFilters({
    textInputId: "commercial-cross-connect-filter",
    tbodyId: "commercial-cross-connects",
  });
  attachTableFilters({
    textInputId: "commercial-remote-filter",
    tbodyId: "commercial-remote-products",
  });
  attachTableFilters({
    textInputId: "commercial-pricebook-filter",
    tbodyId: "commercial-pricebook",
  });
  attachTableFilters({
    textInputId: "commercial-access-filter",
    tbodyId: "commercial-access",
  });
  attachTableFilters({
    textInputId: "commercial-audit-filter",
    tbodyId: "commercial-audit",
  });

  wireActionButton("commercial-refresh", "Commercial data refreshed.", "commercial-action-output");
  wireActionButton(
    "commercial-create-gap",
    "Commercial gap review staged.",
    "commercial-action-output"
  );
  wireActionButton(
    "commercial-open-sla",
    "SLA pack assembled from current templates.",
    "commercial-action-output"
  );
  wireActionButton(
    "commercial-open-access",
    "Access review staged for commercial and security owners.",
    "commercial-action-output"
  );
  wireActionButton(
    "commercial-open-audit",
    "Audit evidence pack assembled.",
    "commercial-action-output"
  );

  wireDownloadButton("commercial-export-gaps", "commercial-gaps", "osdc-commercial-gaps.csv");
  wireDownloadButton(
    "commercial-export-standards",
    "commercial-standards",
    "osdc-commercial-standards.csv"
  );
  wireDownloadButton("commercial-export-slas", "commercial-slas", "osdc-sla-classes.csv");
  wireDownloadButton(
    "commercial-export-pricebook",
    "commercial-pricebook",
    "osdc-remote-hands-pricebook.csv"
  );
  wireDownloadButton("commercial-export-access", "commercial-access", "osdc-access-roles.csv");
  wireDownloadButton("commercial-export-audit", "commercial-audit", "osdc-audit-evidence.csv");
}

async function hydrateCustomers() {
  const overview = await api("/api/customers/overview");

  renderMetrics("customer-metrics", overview.metrics);
  renderCustomerAccounts("customer-accounts", overview.accounts);
  renderCustomerSites("customer-sites", overview.site_instances);
  renderCustomerWorkflows("customer-workflows", overview.workflows);
  renderCustomerMfaPolicies("customer-mfa-policies", overview.mfa_policies);
  renderCustomerBillingPlans("customer-billing-plans", overview.billing_plans);
  renderCustomerUsageMeters("customer-usage-meters", overview.usage_meters);
  renderCustomerInvoices("customer-invoices", overview.invoice_preview);
  renderCustomerConnectors("customer-connectors", overview.connectors);
  populateCustomerSelects(overview);
  renderCustomerCommandWorkspace(overview, "draft");

  for (const id of [
    "customer-select",
    "customer-site-select",
    "customer-workflow-select",
    "customer-mfa-select",
    "customer-plan-select",
    "customer-operator",
  ]) {
    document.getElementById(id)?.addEventListener("change", () => {
      if (id === "customer-select") populateCustomerSiteSelect(overview);
      renderCustomerCommandWorkspace(overview, "draft");
    });
    document
      .getElementById(id)
      ?.addEventListener("input", () => renderCustomerCommandWorkspace(overview, "draft"));
  }

  attachTableFilters({ textInputId: "customer-account-filter", tbodyId: "customer-accounts" });
  attachTableFilters({ textInputId: "customer-site-filter", tbodyId: "customer-sites" });
  attachTableFilters({ textInputId: "customer-mfa-filter", tbodyId: "customer-mfa-policies" });
  attachTableFilters({ textInputId: "customer-workflow-filter", tbodyId: "customer-workflows" });
  attachMultiTableFilter("customer-billing-filter", [
    "customer-billing-plans",
    "customer-usage-meters",
  ]);
  attachTableFilters({ textInputId: "customer-invoice-filter", tbodyId: "customer-invoices" });
  attachTableFilters({ textInputId: "customer-connector-filter", tbodyId: "customer-connectors" });

  document.getElementById("customers-refresh")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Refresh customer operations", "submitted");
  });
  document.getElementById("customers-preview")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Preview customer command", "validated");
  });
  document.getElementById("customers-open-onboarding")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Open customer onboarding", "submitted");
  });
  document.getElementById("customers-enforce-mfa")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Enforce open-source MFA", "submitted");
  });
  document.getElementById("customers-provision-site")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Provision customer site", "submitted");
  });
  document.getElementById("customers-preview-invoice")?.addEventListener("click", (event) => {
    event.preventDefault();
    recordCustomerCommand(overview, "Preview invoice pack", "validated");
  });
  document.getElementById("customers-download-command")?.addEventListener("click", (event) => {
    event.preventDefault();
    const rendered = customerCommandRuntime.lastRecord
      ? { record: customerCommandRuntime.lastRecord }
      : renderCustomerCommandWorkspace(overview, "draft");
    if (rendered?.record) {
      downloadJson(`${rendered.record.command_id.toLowerCase()}.json`, rendered.record);
    }
  });

  wireDownloadButton("customers-export-accounts", "customer-accounts", "osdc-customer-accounts.csv");
  wireDownloadButton("customers-export-sites", "customer-sites", "osdc-customer-sites.csv");
  wireDownloadButton("customers-export-billing", "customer-billing-plans", "osdc-billing-plans.csv");
  wireDownloadButton("customers-export-invoices", "customer-invoices", "osdc-invoice-preview.csv");
}

async function hydrateAssurance() {
  const overview = await api("/api/assurance/overview");

  renderMetrics("assurance-metrics", overview.metrics);
  renderAssuranceAutomationJobs("assurance-jobs", overview.automation_jobs);
  renderAssuranceTests("assurance-tests", overview.test_harnesses);
  renderAssuranceRings("assurance-rings", overview.upgrade_rings);
  renderAssuranceGates("assurance-gates", overview.upgrade_gates);
  renderAssuranceThreatStack("assurance-threat-stack", overview.threat_stack);
  renderAssuranceScannerCoverage("assurance-scanners", overview.scanner_coverage);

  attachTableFilters({
    textInputId: "assurance-job-filter",
    tbodyId: "assurance-jobs",
  });
  attachTableFilters({
    textInputId: "assurance-test-filter",
    statusSelectId: "assurance-test-status",
    tbodyId: "assurance-tests",
  });
  attachTableFilters({
    textInputId: "assurance-ring-filter",
    tbodyId: "assurance-rings",
  });
  attachTableFilters({
    textInputId: "assurance-gate-filter",
    tbodyId: "assurance-gates",
  });
  attachTableFilters({
    textInputId: "assurance-threat-filter",
    tbodyId: "assurance-threat-stack",
  });
  attachTableFilters({
    textInputId: "assurance-scanner-filter",
    tbodyId: "assurance-scanners",
  });

  wireActionButton("assurance-refresh", "Assurance data refreshed.", "assurance-action-output");
  wireActionButton(
    "assurance-run-tests",
    "Test gate run staged.",
    "assurance-action-output"
  );
  wireActionButton(
    "assurance-open-upgrade",
    "Upgrade pull request staged with assurance gates.",
    "assurance-action-output"
  );
  wireActionButton(
    "assurance-run-scan",
    "Scanner bundle staged for DefectDojo and Dependency-Track ingestion.",
    "assurance-action-output"
  );
  wireActionButton(
    "assurance-create-waiver",
    "Waiver request staged with owner and expiry required.",
    "assurance-action-output"
  );

  wireDownloadButton("assurance-export-jobs", "assurance-jobs", "osdc-assurance-jobs.csv");
  wireDownloadButton("assurance-export-tests", "assurance-tests", "osdc-assurance-tests.csv");
  wireDownloadButton("assurance-export-rings", "assurance-rings", "osdc-upgrade-rings.csv");
  wireDownloadButton("assurance-export-gates", "assurance-gates", "osdc-upgrade-gates.csv");
  wireDownloadButton(
    "assurance-export-scanners",
    "assurance-scanners",
    "osdc-scanner-coverage.csv"
  );
}

async function hydrateDataPlatform() {
  const overview = await api("/api/data-platform/overview");

  renderMetrics("data-metrics", overview.metrics);
  renderDataPlatformServices("data-services", overview.services);
  renderDataProducts("data-products", overview.products);
  renderDataPipelines("data-pipelines", overview.pipelines);
  renderDataOntology("data-ontology", overview.ontology);
  renderDataPolicies("data-policies", overview.access_policies);
  renderDataTemplates("data-templates", overview.templates);

  attachTableFilters({
    textInputId: "data-service-filter",
    statusSelectId: "data-service-status",
    tbodyId: "data-services",
  });
  attachTableFilters({
    textInputId: "data-product-filter",
    statusSelectId: "data-product-status",
    tbodyId: "data-products",
  });
  attachTableFilters({
    textInputId: "data-pipeline-filter",
    tbodyId: "data-pipelines",
  });
  attachTableFilters({
    textInputId: "data-ontology-filter",
    tbodyId: "data-ontology",
  });
  attachTableFilters({
    textInputId: "data-policy-filter",
    tbodyId: "data-policies",
  });
  attachTableFilters({
    textInputId: "data-template-filter",
    tbodyId: "data-templates",
  });

  wireActionButton("data-refresh", "Data platform refreshed.", "data-action-output");
  wireActionButton(
    "data-create-product",
    "Data product request staged.",
    "data-action-output"
  );
  wireActionButton(
    "data-register-source",
    "Source registration request staged.",
    "data-action-output"
  );
  wireActionButton(
    "data-create-pipeline",
    "Pipeline template request staged.",
    "data-action-output"
  );
  wireActionButton(
    "data-publish-dashboard",
    "Dashboard publication request staged.",
    "data-action-output"
  );
  wireActionButton(
    "data-open-ontology",
    "Ontology browser request staged.",
    "data-action-output"
  );

  wireDownloadButton("data-export-products", "data-products", "osdc-data-products.csv");
  wireDownloadButton("data-export-policies", "data-policies", "osdc-data-policies.csv");
  wireDownloadButton("data-export-templates", "data-templates", "osdc-data-templates.csv");
}

function populateHardwareProfileSelect(profiles) {
  const select = document.getElementById("hardware-profile-select");
  if (!select) return;
  clear(select);
  for (const profile of profiles) {
    const option = document.createElement("option");
    option.value = profile.profile_id;
    option.append(text(`${profile.profile_id} - ${profile.node_role}`));
    select.append(option);
  }
}

function selectedHardwareProfile(profiles) {
  const select = document.getElementById("hardware-profile-select");
  return profiles.find((profile) => profile.profile_id === select?.value) ?? profiles[0];
}

function renderHardwareRequestPreview(profiles) {
  const profile = selectedHardwareProfile(profiles);
  const count = document.getElementById("hardware-count")?.value || "1";
  const site = document.getElementById("hardware-site")?.value || "site";
  const network = document.getElementById("hardware-network-zone")?.value || profile?.network_profile;
  const rackPolicy = document.getElementById("hardware-rack-policy")?.value || "rack-policy";
  const target = document.getElementById("hardware-target")?.value || profile?.target_pool;
  const output = document.getElementById("hardware-action-output");
  if (!profile || !output) return;

  output.textContent = [
    `Preview: ${count} x ${profile.node_role}`,
    `via ${profile.provisioner}`,
    `at ${site}`,
    `for ${target}.`,
    `Reserve in NetBox, validate BMC with Redfish/OpenBMC, deploy ${profile.os_image}, attach ${network}, use ${rackPolicy}, enroll security, then hand off to ${profile.target_pool}.`,
  ].join(" ");
}

async function hydrateHardware() {
  const overview = await api("/api/hardware/provisioning");

  renderMetrics("hardware-metrics", overview.metrics);
  renderHardwarePipeline("hardware-pipeline", overview.pipeline);
  renderHardwareProfiles("hardware-profiles", overview.profiles);
  renderHardwareRequests("hardware-requests", overview.requests);
  renderSystemConnectors("hardware-connectors", overview.connectors);
  populateHardwareProfileSelect(overview.profiles);
  renderHardwareRequestPreview(overview.profiles);

  for (const id of [
    "hardware-profile-select",
    "hardware-count",
    "hardware-site",
    "hardware-network-zone",
    "hardware-rack-policy",
    "hardware-target",
  ]) {
    document
      .getElementById(id)
      ?.addEventListener("change", () => renderHardwareRequestPreview(overview.profiles));
  }

  attachTableFilters({
    textInputId: "hardware-pipeline-filter",
    tbodyId: "hardware-pipeline",
  });
  attachTableFilters({
    textInputId: "hardware-profile-filter",
    tbodyId: "hardware-profiles",
  });
  attachTableFilters({
    textInputId: "hardware-request-filter",
    tbodyId: "hardware-requests",
  });
  attachTableFilters({
    textInputId: "hardware-connector-filter",
    tbodyId: "hardware-connectors",
  });

  wireActionButton("hardware-refresh", "Hardware provisioning data refreshed.", "hardware-action-output");
  wireActionButton(
    "hardware-preview-request",
    "Hardware request preview refreshed. NetBox reservation and BMC validation are the first actions.",
    "hardware-action-output"
  );
  wireActionButton(
    "hardware-open-change",
    "GitOps hardware provisioning change staged with source-of-truth reservation.",
    "hardware-action-output"
  );
  wireDownloadButton("hardware-export-requests", "hardware-requests", "osdc-hardware-requests.csv");
  wireDownloadButton("hardware-export-profiles", "hardware-profiles", "osdc-hardware-profiles.csv");
}

async function hydrateEdge() {
  const [status, config, scripts] = await Promise.all([
    api("/api/edge/status"),
    api("/api/edge/config-preview"),
    api("/api/config/scripts"),
  ]);

  renderMetrics("edge-metrics", status.metrics);
  renderEdgeServices("edge-services", status.services);
  renderEdgeConfig("edge-config-preview", config.generated_files);
  renderChecks("edge-config-checks", config.rollout_checks);
  renderConfigScriptEditor(scripts);
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
  attachTableFilters({
    textInputId: "edge-script-filter",
    tbodyId: "edge-script-list",
  });
  wireActionButton("edge-detection-mode", "Detection mode staged");
  wireActionButton("edge-stage-rollout", "Rollout staged");
  wireActionButton("edge-generate-configs", "Configs generated");
  wireActionButton("edge-ssh-policy", "SSH policy review queued");
  wireActionButton("edge-audit", "Audit queued");
}

document.addEventListener("DOMContentLoaded", () => {
  ensurePortalCommandWorkspace();
  if (document.body.dataset.portal === "infrastructure") {
    hydrateInfrastructure().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "tenant") {
    hydrateTenant().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "operator") {
    hydrateOperator().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "lifecycle") {
    hydrateLifecycle().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "hardware") {
    hydrateHardware().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "developer") {
    hydrateDeveloper().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "commercial") {
    hydrateCommercial().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "customers") {
    hydrateCustomers().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "assurance") {
    hydrateAssurance().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "data-platform") {
    hydrateDataPlatform().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "edge") {
    hydrateEdge().catch((error) => console.error(error));
  }
  if (document.body.dataset.portal === "planner") {
    hydratePlanner().catch((error) => console.error(error));
  }
});
