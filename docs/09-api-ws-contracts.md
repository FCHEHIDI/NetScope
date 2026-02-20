# 09 — Spécification des contrats API REST & WebSocket

## 1) Objectif

Figer les interfaces du MVP avant bootstrap code Rust/Axum:
- endpoints REST opérationnels,
- modèles de payload,
- conventions d’erreurs,
- protocole WebSocket temps réel.

## 2) Principes de contrat

- Version d’API explicite: préfixe `/v1`.
- Réponses JSON structurées et homogènes.
- Corrélation systématique via `trace_id` et `request_id`.
- Erreurs déterministes (`error.code` stable).
- Compatibilité ascendante: ajout de champs permis, suppression interdite en v1.

## 3) Enveloppe standard de réponse

## 3.1 Succès

```json
{
  "status": "ok",
  "data": {},
  "meta": {
    "trace_id": "2b6f...",
    "timestamp": "2026-02-20T09:00:00Z"
  }
}
```

## 3.2 Erreur

```json
{
  "status": "error",
  "error": {
    "code": "UPSTREAM_TIMEOUT",
    "message": "Timeout during outbound request",
    "details": {
      "timeout_type": "read",
      "elapsed_ms": 1200
    }
  },
  "meta": {
    "trace_id": "2b6f...",
    "timestamp": "2026-02-20T09:00:00Z"
  }
}
```

## 4) Contrats REST (MVP)

## 4.1 GET /v1/health/live

### Description
Liveness simple du service process.

### Réponse `200`

```json
{
  "status": "ok",
  "data": {
    "service": "netscope",
    "state": "live"
  },
  "meta": {
    "trace_id": "...",
    "timestamp": "..."
  }
}
```

### Erreurs
- Aucune en nominal.

## 4.2 GET /v1/health/ready

### Description
Readiness incluant dépendances critiques (Kafka, Ceph RGW, metrics engine).

### Réponse `200`

```json
{
  "status": "ok",
  "data": {
    "service": "netscope",
    "state": "ready",
    "dependencies": {
      "kafka": "up",
      "ceph_rgw": "up",
      "metrics_engine": "up"
    }
  },
  "meta": {
    "trace_id": "...",
    "timestamp": "..."
  }
}
```

### Réponse `503`

```json
{
  "status": "error",
  "error": {
    "code": "DEPENDENCY_UNAVAILABLE",
    "message": "One or more dependencies are down",
    "details": {
      "kafka": "down",
      "ceph_rgw": "up",
      "metrics_engine": "up"
    }
  },
  "meta": {
    "trace_id": "...",
    "timestamp": "..."
  }
}
```

## 4.3 GET /v1/metrics/network

### Description
Expose les métriques réseau consolidées pour lecture applicative.

### Paramètres query (optionnels)
- `window`: `1m|5m|15m` (défaut `5m`)
- `route_group`: filtre logique route

### Réponse `200`

```json
{
  "status": "ok",
  "data": {
    "window": "5m",
    "requests_per_sec": 128.4,
    "latency_ms": {
      "p50": 84,
      "p95": 212,
      "p99": 410
    },
    "error_rate": 0.012,
    "timeouts_total": 43,
    "retries_total": 188,
    "pool_saturation": 0.67,
    "inflight_requests": 54,
    "queue_depth": 22,
    "circuit_state": "closed"
  },
  "meta": {
    "trace_id": "...",
    "timestamp": "..."
  }
}
```

### Réponse `400`
- `INVALID_QUERY_PARAM` (fenêtre non supportée).

## 4.4 GET /v1/resilience/state

### Description
Expose l’état runtime des mécanismes de résilience.

### Réponse `200`

```json
{
  "status": "ok",
  "data": {
    "timeouts": {
      "connect_ms": 200,
      "read_ms": 1000,
      "global_ms": 1500
    },
    "retry": {
      "max_attempts": 2,
      "backoff_base_ms": 50,
      "jitter": true,
      "idempotent_only": true
    },
    "circuit_breaker": {
      "state": "half-open",
      "failure_threshold": 0.2,
      "open_remaining_ms": 4000
    },
    "backpressure": {
      "queue_capacity": 2048,
      "queue_depth": 389,
      "drop_policy": "reject-new"
    }
  },
  "meta": {
    "trace_id": "...",
    "timestamp": "..."
  }
}
```

### Réponse `500`
- `RESILIENCE_STATE_UNAVAILABLE`.

## 5) Endpoint de scrape Prometheus

## GET /metrics

- Format texte Prometheus exposition standard.
- Hors enveloppe JSON (contrat Prometheus natif).
- Protégé réseau interne (pas d’exposition internet directe).

## 6) Contrat WebSocket (MVP)

## 6.1 Endpoint

- `GET /v1/ws/events`
- Upgrade HTTP vers WebSocket.

## 6.2 Query params

- `severity`: `INFO|WARN|ERROR` (optionnel)
- `route_group`: filtre logique (optionnel)

## 6.3 Événement standard

```json
{
  "event_type": "network.request.completed",
  "event_version": "1.0",
  "timestamp": "2026-02-20T09:10:00Z",
  "trace_id": "2b6f...",
  "request_id": "req-...",
  "route_group": "partner-send",
  "internal_ip": "10.0.0.12",
  "external_ip": "203.0.113.20",
  "status_code": 200,
  "latency_ms": 142,
  "retry_count": 1,
  "timeout_count": 0,
  "circuit_state": "closed",
  "severity": "INFO"
}
```

## 6.4 Événements de contrôle

- `system.heartbeat`
- `system.backpressure.applied`
- `system.ws.drop_notice`

Exemple `drop_notice`:

```json
{
  "event_type": "system.ws.drop_notice",
  "event_version": "1.0",
  "timestamp": "2026-02-20T09:11:00Z",
  "trace_id": "...",
  "dropped_events": 120,
  "reason": "client_slow_consumer"
}
```

## 7) Codes d’erreur applicatifs

| Code | HTTP | Description |
|---|---:|---|
| `INVALID_QUERY_PARAM` | 400 | Paramètre invalide |
| `DEPENDENCY_UNAVAILABLE` | 503 | Dépendance critique indisponible |
| `UPSTREAM_TIMEOUT` | 504 | Timeout appel externe |
| `UPSTREAM_ERROR` | 502 | Erreur upstream non-timeout |
| `RESILIENCE_STATE_UNAVAILABLE` | 500 | État résilience indisponible |
| `INTERNAL_ERROR` | 500 | Erreur interne non classée |

## 8) En-têtes standard

Requêtes entrantes:
- `x-request-id` (optionnel, généré si absent)
- `traceparent` (W3C, optionnel mais recommandé)

Réponses:
- `x-request-id`
- `x-trace-id`
- `content-type: application/json` (hors `/metrics`)

## 9) Contraintes de compatibilité

- Les champs marqués obligatoires en v1 ne changent pas de type.
- Ajout de champs autorisé si non-breaking.
- Toute rupture nécessite v2 + ADR dédiée.

## 10) Critères d’acceptation de contrat

- Les endpoints REST répondent selon enveloppe standard.
- Les erreurs sont mappées aux codes contractuels.
- Chaque événement WS porte `trace_id` et `event_version`.
- Les exemples de payload sont validés via tests de contrat.
