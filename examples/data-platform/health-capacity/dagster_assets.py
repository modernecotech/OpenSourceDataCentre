import dagster as dg
import pandas as pd


@dg.asset(group_name="health_capacity")
def raw_capacity_export() -> pd.DataFrame:
    return pd.DataFrame(
        [
            {"region": "north", "facility_count": 4, "available_beds": 32},
            {"region": "south", "facility_count": 3, "available_beds": 18},
        ]
    )


@dg.asset(group_name="health_capacity", deps=[raw_capacity_export])
def capacity_daily(raw_capacity_export: pd.DataFrame) -> pd.DataFrame:
    return raw_capacity_export.assign(reporting_level="region")


defs = dg.Definitions(assets=[raw_capacity_export, capacity_daily])
