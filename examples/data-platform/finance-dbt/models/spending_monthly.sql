select
  agency_id,
  programme_id,
  date_trunc('month', spend_date) as spend_month,
  sum(amount_local) as amount_local
from {{ source('raw_finance', 'spending') }}
group by 1, 2, 3
