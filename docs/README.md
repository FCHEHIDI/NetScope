# NetScope — Dossier méthodologique

Ce dossier formalise la méthode de conception du MVP d’observabilité réseau (Ceph-first) pour un pipeline distribué inspiré télécom/SRE.

## Contenu

1. [01-methodologie.md](01-methodologie.md) — cadre méthodologique, périmètre, invariants, principes d’architecture
2. [02-topologie-flux.md](02-topologie-flux.md) — topologies cibles, flux réseau, séquences, gestion d’erreur
3. [03-modelisation-donnees.md](03-modelisation-donnees.md) — modélisation relationnelle/logique des métriques, traces, audits
4. [04-plan-validation.md](04-plan-validation.md) — stratégie de validation, charge, critères d’acceptation, gouvernance
5. [05-backlog-implementation.md](05-backlog-implementation.md) — backlog d’implémentation S1→S6, user stories, DoD, risques/mitigations
6. [06-runbook-mvp.md](06-runbook-mvp.md) — procédures d’exploitation, incidents, replay et seuils d’alerte MVP
7. [07-architecture-workspace-rust.md](07-architecture-workspace-rust.md) — architecture de référence des crates Rust, contrats et testabilité
8. [adr/README.md](adr/README.md) — registre ADR des décisions d’architecture

## Positionnement

- Priorité MVP: robustesse fonctionnelle, traçabilité, mesure réseau fiable
- Stockage objet: Ceph RGW dès la V1
- Évolution prévue: industrialisation Kubernetes et montée en charge multi-VM

## Résultat attendu

À l’issue de cette phase documentaire, l’équipe dispose d’un blueprint exécutable qui limite l’over-engineering et sécurise l’implémentation incrémentale.
