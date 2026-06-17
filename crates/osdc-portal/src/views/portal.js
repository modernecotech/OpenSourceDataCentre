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
  });
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
      output.textContent = `Prototype validation queued: ${script.validation_command}`;
    }
  });

  document.getElementById("edge-script-stage")?.addEventListener("click", (event) => {
    event.preventDefault();
    const script = scripts.find((item) => item.id === selectedId);
    if (output && script) {
      output.textContent = `Prototype GitOps change staged for ${script.path}; production flow must open a reviewed change request before rollout.`;
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
