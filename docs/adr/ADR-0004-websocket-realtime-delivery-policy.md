# ADR-0004 — Politique de diffusion WebSocket en temps réel

- **Statut**: accepted
- **Date**: 2026-02-20
- **Décideurs**: NetScope Core Team

## Contexte

Le dashboard temps réel doit recevoir les événements réseau rapidement sans compromettre la stabilité du service.
Les clients lents peuvent provoquer accumulation mémoire et dégradation globale.

## Décision

Implémenter une diffusion WS avec politiques de protection:
- buffer borné par client,
- heartbeat ping/pong,
- stratégie `drop-oldest` sur saturation,
- événements minimaux corrélés (`trace_id`, `route`, `status`, `latency_ms`, `severity`),
- objectif de latence de diffusion < 2s en nominal.

## Alternatives considérées

1. Buffer non borné
   - Rejetée: risque OOM.
2. Déconnexion immédiate client lent sans buffer
   - Rejetée: expérience opérateur dégradée.

## Conséquences

### Positives
- Stabilité mémoire préservée.
- Comportement prévisible sous charge.
- Politique explicite de dégradation.

### Négatives
- Possibilité de perte d’événements non critiques côté client lent.
- Complexité légère de gestion d’état de connexion.

## Mesures de contrôle

- Métrique `ws_dropped_events_total`.
- Alerte si taux de drop dépasse seuil.
- Reconnexion client avec reprise visuelle côté dashboard.

## Impacts

- Modules impactés: `api-server` (ws), `metrics-engine`, `runbook`.
- Invariants impactés: I4 (traçabilité), I3 (cohérence signal temps réel).
