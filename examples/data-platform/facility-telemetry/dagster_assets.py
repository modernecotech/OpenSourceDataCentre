import dagster as dg


@dg.asset(group_name="facility_telemetry")
def prometheus_export_manifest() -> dict:
    return {
        "source": "prometheus",
        "metrics": [
            "osdc_dc_bus_voltage",
            "osdc_rack_power_kw",
            "osdc_cooling_loop_temp_c",
        ],
        "target": "iceberg.osdc.facility_telemetry",
    }


defs = dg.Definitions(assets=[prometheus_export_manifest])
