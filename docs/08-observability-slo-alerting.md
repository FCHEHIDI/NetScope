# 08 — Observabilité opérationnelle: SLI/SLO, alerting et dashboards

## 1) Objectif

Formaliser le modèle d’observabilité opérationnelle du MVP NetScope pour:
- mesurer la santé réelle du pipeline,
- déclencher les alertes utiles,
- guider le diagnostic incident sans bruit excessif.

## 2) SLI retenus (MVP)

## 2.1 Performance
- `request_latency_ms_p50`
- `request_latency_ms_p95`
- `request_latency_ms_p99`
- `requests_per_sec`

## 2.2 Fiabilité
- `error_rate` = `errors_total / requests_total`
- `timeout_rate` = `timeouts_total / requests_total`
- `retry_rate` = `retries_total / requests_total`

## 2.3 Capacité / saturation
- `pool_saturation`
- `inflight_requests`
- `queue_depth`
- `circuit_state` (closed/half-open/open)

## 2.4 Auditabilité
- `audit_writes_total`
- `audit_write_errors_total`
- `audit_backlog_depth`
- `audit_flush_lag_seconds`

## 3) SLO initiaux (MVP)

- `latency_p95` < 400 ms en nominal.
- `error_rate` < 2% sur fenêtre 5 min.
- `dashboard_event_delay` < 2 s en nominal.
- `critical_audit_loss` = 0 (écriture directe ou buffer durable).
- `circuit_open_duration` < 5 min continu en nominal.

## 4) Politique d’alerting

## 4.1 Niveaux
- **Info**: signal utile sans action immédiate.
- **Warning**: dégradation à surveiller.
- **Critical**: impact utilisateur/traçabilité en cours.

## 4.2 Règles minimales

- `LatencyP95High` (Warning)
  - condition: `latency_p95 > 400ms` pendant 10 min
- `ErrorRateHigh` (Critical)
  - condition: `error_rate > 2%` pendant 5 min
- `AuditBacklogGrowing` (Critical)
  - condition: `audit_backlog_depth` croissant pendant 15 min
- `CircuitBreakerOpenLong` (Warning/Critical)
  - condition: `circuit_state=open` > 5 min (warning), > 15 min (critical)
- `WsDropRateHigh` (Warning)
  - condition: drop events WS au-dessus du seuil défini

## 5) Dashboards recommandés

## 5.1 Dashboard "Executive"
- débit global,
- latence p50/p95/p99,
- taux d’erreur,
- état circuit breaker,
- état audit (OK / dégradé).

## 5.2 Dashboard "SRE Deep Dive"
- latence par `route_group` et `upstream`,
- retries/timeouts par fenêtre,
- queue depth et inflight,
- saturation pool,
- backlog audit + erreurs d’écriture RGW.

## 5.3 Dashboard "Realtime Events" (WS)
- flux des événements corrélés (`trace_id`),
- mises en évidence erreurs/timeout,
- événements `system.ws.drop_notice`.

## 6) Bonnes pratiques de bruit d’alerte

- Ajouter des fenêtres temporelles (pas de déclenchement instantané bruité).
- Grouper par impact (service, upstream, route_group).
- Dédupliquer les alertes corrélées.
- Exiger un runbook lié pour chaque alerte critique.

## 7) Vérification en phase de tests de charge

- Vérifier déclenchement correct des alertes sur scénarios injectés.
- Vérifier rétablissement automatique des alertes après retour nominal.
- Mesurer faux positifs / faux négatifs et ajuster seuils.

## 8) Livrables d’exploitation

- Fichier de règles d’alerting versionné.
- JSON dashboards versionnés.
- Checklist de revue mensuelle des seuils et cardinalité.
