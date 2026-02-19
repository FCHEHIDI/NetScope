# ADR-0002 — Politiques de résilience centralisées sur l’outbound

- **Statut**: accepted
- **Date**: 2026-02-20
- **Décideurs**: NetScope Core Team

## Contexte

Le pipeline effectue des appels vers une API externe sujette à latence et erreurs transitoires.
Disperser la résilience dans plusieurs handlers créerait des divergences de comportement.

## Décision

Centraliser les politiques de résilience dans le crate `outbound-client`:
- timeouts distincts (connect/read/global),
- retries idempotents uniquement,
- backoff exponentiel avec jitter,
- circuit breaker avec états `closed/half-open/open`,
- backpressure par files bornées.

## Alternatives considérées

1. Résilience au niveau de chaque appel métier
   - Rejetée: duplication logique, observabilité incohérente.
2. Gateway externe unique pour résilience
   - Rejetée au MVP: ajoute couche opérationnelle prématurée.

## Conséquences

### Positives
- Comportement uniforme et testable.
- Corrélation métriques/résilience simplifiée.
- Réduction des erreurs de configuration locale.

### Négatives
- Fort couplage initial autour d’un module critique.
- Nécessite tests exhaustifs des politiques.

## Mesures de contrôle

- Tests unitaires dédiés par politique.
- Seuils et timeouts configurables + validés au boot.
- Exposition API de l’état résilience pour diagnostic.

## Impacts

- Modules impactés: `outbound-client`, `metrics-engine`, `api-server`, `domain`.
- Invariants impactés: I1 (idempotence), I2 (timeouts), I3 (cohérence métrique).
