# ADR-0003 — Modèle métrique orienté stabilité de cardinalité

- **Statut**: accepted
- **Date**: 2026-02-20
- **Décideurs**: NetScope Core Team

## Contexte

Le MVP doit fournir p50/p95/p99, erreurs, retries, timeouts, saturation et débit.
Une cardinalité de labels mal contrôlée dégraderait Prometheus et les dashboards.

## Décision

Adopter un modèle métrique minimal, stable et borné:
- compteurs: `requests_total`, `errors_total`, `timeouts_total`, `retries_total`
- jauges: `queue_depth`, `inflight_requests`, `pool_saturation`, `circuit_state`
- histogramme: `request_latency_ms`
- labels contrôlés: `route_group`, `upstream`, `status_class`

Interdire les labels à haute cardinalité (`trace_id`, `request_id`, messages d’erreur bruts).

## Alternatives considérées

1. Labels détaillés par route complète + code + message
   - Rejetée: cardinalité explosive.
2. Agrégation hors Prometheus seulement
   - Rejetée: perte de standard observabilité.

## Conséquences

### Positives
- Coût métrique prévisible.
- Dashboards stables en volumétrie.
- Exploitation simplifiée en incident.

### Négatives
- Granularité de diagnostic réduite côté métriques.
- Dépendance plus forte aux logs/traces pour analyses fines.

## Mesures de contrôle

- Politique de labels whitelistée.
- Revue obligatoire de tout nouveau label.
- Tests de charge orientés cardinalité.

## Impacts

- Modules impactés: `metrics-engine`, `api-server`, `runbook`.
- Invariants impactés: I3 (cohérence métrique), I4 (corrélation via logs/traces).
