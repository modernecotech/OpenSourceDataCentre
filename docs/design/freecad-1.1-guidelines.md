# FreeCAD 1.1 Design Guidelines

FreeCAD files are source artifacts, not screenshots. Every mechanical part or assembly should be reviewable, reproducible, and exportable using free tools.

## Directory Convention

Use this structure once CAD artifacts are added:

```text
cad/
  racks/
  cooling/
  power/
  security/
  building/
  shared-parts/
exports/
  step/
  stl/
  ifc/
  drawings/
  bom/
```

## File Naming

Use stable names that survive manufacturing and procurement:

```text
<system>-<assembly>-<part>-<revision>.FCStd
```

Examples:

- `rack-eia19-pdu-bracket-r01.FCStd`
- `cooling-groundloop-manifold-r01.FCStd`
- `cooling-thermalspine-row-segment-r01.FCStd`
- `cooling-rack-reardoor-hx-r01.FCStd`
- `security-door-controller-enclosure-r01.FCStd`

## Required Metadata

Each part should include:

- Part number.
- Revision.
- Material.
- Finish or coating.
- Manufacturing method.
- Tolerance class.
- Mass.
- Source license.
- Export date for generated STEP/STL/IFC files.

## Rack Design Rules

- Model 19-inch EIA, Open19, OCP Open Rack V3, and Open Rack Wide as separate reference envelopes.
- Keep adapter plates and shelves as separate assemblies.
- Include cable-bend radius and service-clearance geometry.
- Include front, rear, side, and overhead access zones.
- Model center of gravity for mobile or seismic variants.
- Add maximum static and dynamic load attributes.

## Cooling Design Rules

- Separate air-cooled, direct-to-chip, rear-door heat-exchanger, and immersion components.
- Include leak detection, drip paths, isolation valves, strain relief, and service loops in the mechanical envelope.
- Mark all liquid paths with material, working pressure, test pressure, and compatible coolant.
- For earth-based cooling, separate underground loops, surface plant, manifolds, filtration, and heat exchangers.
- For rack thermal-spine cooling, separate rack capture modules, row manifolds, underfloor/service-trench segments, sensor cassettes, heat-exchanger skids, sorption chiller envelopes, and bypass paths.

## Export Rules

- Export STEP for fabrication and supplier review.
- Export STL only for 3D printing or visualization.
- Export IFC for building coordination.
- Export CSV BOM for calculators and procurement.
- Export PDF drawings for human review, including critical dimensions and tolerances.

Generated exports should be reproducible from the `.FCStd` source and should include the source file revision in the export metadata.

## Review Checklist

- Does the part fit at least one documented rack standard or clearly declare `Custom`?
- Is the design serviceable with locally available tools?
- Are fasteners and materials regionally available?
- Are sharp edges, pinch points, grounding, and liquid risks documented?
- Does the BOM avoid vendor-locked consumables?
- Are thermal and structural assumptions linked to tests or simulations?
