# 03 — Modélisation des données et schémas

## 1) Modèle relationnel logique (audit + métriques)

> Modèle relationnel de référence pour la compréhension métier.
> En MVP, le stockage principal d’audit est objet (Ceph) au format JSONL.

```mermaid
erDiagram
    REQUEST_EVENT ||--o{ RETRY_EVENT : has
    REQUEST_EVENT ||--o{ TIMEOUT_EVENT : has
    REQUEST_EVENT ||--|| RESILIENCE_STATE : yields
    REQUEST_EVENT ||--o{ METRIC_POINT : emits
    REQUEST_EVENT ||--o{ AUDIT_RECORD : persists

    REQUEST_EVENT {
      string request_id PK
      string trace_id
      string span_id
      string route
      string method
      string internal_ip
      string external_ip
      datetime started_at
      int payload_in_bytes
      int payload_out_bytes
      string idempotency_key
      string status
    }

    RETRY_EVENT {
      string retry_id PK
      string request_id FK
      int attempt
      int backoff_ms
      string reason
      datetime at
    }

    TIMEOUT_EVENT {
      string timeout_id PK
      string request_id FK
      string timeout_type
      int elapsed_ms
      datetime at
    }

    RESILIENCE_STATE {
      string request_id PK,FK
      string circuit_state
      int inflight_requests
      int queue_depth
      bool backpressure_applied
    }

    METRIC_POINT {
      string metric_id PK
      string request_id FK
      string metric_name
      float metric_value
      string unit
      datetime at
    }

    AUDIT_RECORD {
      string audit_id PK
      string request_id FK
      string severity
      string event_type
      string object_key
      datetime written_at
    }
```

## 2) Schéma JSON d’un événement d’audit (MVP)

```json
{
  "event_version": "1.0",
  "event_type": "request.completed",
  "severity": "INFO",
  "timestamp": "2026-02-19T20:00:00Z",
  "trace_id": "2b6f...",
  "span_id": "8a1c...",
  "request_id": "req-...",
  "idempotency_key": "idem-...",
  "route": "/v1/partner/send",
  "method": "POST",
  "internal_ip": "10.0.0.12",
  "external_ip": "203.0.113.20",
  "upstream": {
    "host": "partner.example.net",
    "status_code": 200,
    "latency_ms": 142
  },
  "resilience": {
    "retry_count": 1,
    "timeout_count": 0,
    "circuit_state": "closed",
    "backpressure": false
  },
  "network": {
    "bytes_sent": 812,
    "bytes_received": 1210
  },
  "result": {
    "outcome": "success",
    "error_code": null,
    "error_message": null
  }
}
```

## 3) Convention de clé objet Ceph (RGW)

Format recommandé:

`netscope-audit/{yyyy}/{mm}/{dd}/{service}/{route_hash}/{hour}/events-{node}-{seq}.jsonl`

Bénéfices:
- partition temporelle simple
- distribution de charge RGW
- relecture ciblée par service/route

## 4) Modèle d’agrégation métrique

- **Compteurs**: `requests_total`, `errors_total`, `timeouts_total`, `retries_total`
- **Jauges**: `pool_saturation`, `queue_depth`, `inflight_requests`, `circuit_state`
- **Histogrammes**: `request_latency_ms` (bucket stable)
- **Débit**: dérivé `requests_total / fenêtre`

## 5) Exigences de qualité des données

- Champs obligatoires: `trace_id`, `request_id`, `timestamp`, `route`, `outcome`
- Horodatage UTC ISO-8601
- Schéma versionné (`event_version`) pour compatibilité ascendante
- Rejet explicite des événements invalides vers une file de quarantaine
