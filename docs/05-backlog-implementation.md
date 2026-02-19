# 05 — Backlog d’implémentation (S1 → S6)

## 1) Cadre d’exécution

- **Objectif**: transformer le blueprint méthodologique en MVP exécutable, sans over-engineering.
- **Rythme proposé**: 6 lots séquentiels, chacun livrable et vérifiable.
- **Principe**: chaque lot ferme une boucle complète « code + observabilité + validation ».

## 2) Vue macro des lots

| Lot | Intitulé | Résultat concret | Dépendances |
|---|---|---|---|
| S1 | Network Probe | Mesure fiable des requêtes sortantes | aucune |
| S2 | Metrics Engine | Exposition des métriques p50/p95/p99, erreurs, débit | S1 |
| S3 | API REST Axum | Endpoints santé, métriques et état résilience | S2 |
| S4 | Dashboard WebSocket | Flux temps réel des événements corrélés | S2, S3 |
| S5 | Intégration Ceph | Audit JSONL persistant sur RGW | S1, S3 |
| S6 | Charge & Validation | Rapport de conformité invariants/SLO | S1→S5 |

## 3) Backlog détaillé

## S1 — Network Probe

### User stories
- En tant qu’ingénieur SRE, je veux mesurer chaque requête sortante pour calculer la latence et diagnostiquer les anomalies.
- En tant qu’exploitant, je veux corréler chaque mesure avec `trace_id` et `request_id`.

### Tâches
- Ajouter une couche d’interception HTTP sortante (middleware client).
- Capturer: `started_at`, `elapsed_ms`, `status_code`, `bytes_sent`, `bytes_received`.
- Enrichir avec `internal_ip`, `external_ip` (simulation NAT), `route`, `method`.
- Émettre un événement interne normalisé (probe event).

### Definition of Done
- 100% des appels sortants produisent un probe event.
- Aucun impact > 5% sur la latence médiane en test de non-régression.
- Les événements invalides sont rejetés explicitement avec log structuré.

### Risques / mitigations
- **Risque**: surcharge CPU due au logging trop verbeux.
- **Mitigation**: niveau de log dynamique + sampling DEBUG hors incident.

## S2 — Metrics Engine

### User stories
- En tant qu’équipe SRE, je veux disposer des métriques réseau clés (latence, erreurs, retries, timeouts, saturation, débit).

### Tâches
- Implémenter compteurs, jauges et histogrammes.
- Définir buckets stables de latence (ms) et labels contrôlés.
- Calculer p50/p95/p99 sur fenêtre glissante.
- Exposer un endpoint scrape compatible Prometheus.

### Definition of Done
- Métriques visibles et cohérentes entre 2 runs identiques.
- p50/p95/p99 conformes aux jeux de tests contrôlés.
- Absence de cardinalité non maîtrisée sur les labels.

### Risques / mitigations
- **Risque**: explosion de cardinalité (`route`, `error_message`).
- **Mitigation**: normalisation stricte des labels + listes autorisées.

## S3 — API REST (Axum)

### User stories
- En tant qu’opérateur, je veux consulter l’état du service et les métriques consolidées via API.

### Tâches
- Créer endpoints:
  - `GET /health/live`
  - `GET /health/ready`
  - `GET /metrics/network`
  - `GET /resilience/state`
- Standardiser les réponses JSON et codes HTTP.
- Ajouter middlewares Tower: tracing, timeout serveur, limitation simple.

### Definition of Done
- Contrat API versionné et documenté.
- Couverture tests d’intégration endpoints critiques.
- Temps de réponse API interne p95 dans cible locale définie.

### Risques / mitigations
- **Risque**: duplication logique entre handlers.
- **Mitigation**: couche service dédiée + DTOs stricts.

## S4 — Dashboard WebSocket

### User stories
- En tant qu’opérateur NOC, je veux visualiser en quasi temps réel les événements réseau corrélés.

### Tâches
- Implémenter gateway WS avec diffusion d’événements normalisés.
- Ajouter filtre minimal côté serveur (`severity`, `route`).
- Gérer heartbeat/ping-pong et reconnexion client.

### Definition of Done
- Délai de publication bout en bout < 2 s en charge nominale.
- Aucun événement WS sans `trace_id`.
- Dégradation contrôlée en cas de clients lents (drop policy explicite).

### Risques / mitigations
- **Risque**: accumulation mémoire avec clients lents.
- **Mitigation**: buffer borné par client + stratégie drop oldest.

## S5 — Intégration Ceph (RGW)

### User stories
- En tant qu’auditeur, je veux un historique immuable des événements critiques dans Ceph.

### Tâches
- Implémenter writer objet S3-compatible RGW (batch JSONL).
- Définir convention de clés par partition temporelle.
- Ajouter buffer local borné et reprise en cas d’indisponibilité RGW.
- Vérifier checksum/accusé de succès écriture.

### Definition of Done
- 100% des événements critiques persistés (ou bufferisés durablement).
- Reprise automatique validée après indisponibilité RGW simulée.
- Relecture ciblée possible par intervalle temporel et service.

### Risques / mitigations
- **Risque**: latence RGW impacte le chemin critique.
- **Mitigation**: écriture asynchrone + queue durable + circuit de protection.

## S6 — Charge & Validation réseau

### User stories
- En tant que lead SRE, je veux prouver la tenue des invariants et SLO en charge nominale et dégradée.

### Tâches
- Préparer jeux de charge paliers (faible, nominal, pic).
- Injecter incidents: latence, timeout, 5xx, pertes réseau.
- Mesurer p95/p99, erreurs, retries, timeouts, saturation pool, état circuit breaker.
- Produire rapport final « conformité / non-conformité ».

### Definition of Done
- Rapport signé avec résultats par invariant I1→I5.
- SLO MVP validés ou écarts documentés avec plan d’action.
- Reproductibilité des tests (scripts + paramètres versionnés).

### Risques / mitigations
- **Risque**: biais de benchmark local mono-machine.
- **Mitigation**: campagne complémentaire sur VMs VirtualBox multi-nœuds.

## 4) Dépendances critiques

- Kafka opérationnel pour ingestion continue.
- Ceph RGW disponible avec credentials dédiés projet.
- Prometheus/Grafana prêts pour vérification croisée des métriques.
- Environnement de test réseau (Docker local puis VMs VirtualBox).

## 5) Politique de commits atomiques (recommandée)

- 1 commit par lot fonctionnel majeur.
- Message impératif et explicite (`feat:`, `docs:`, `test:`, `chore:`).
- Inclure dans chaque lot: code, config, doc et preuve de validation.

## 6) Critères de passage entre lots

- **Go S(n+1)** seulement si:
  - DoD du lot S(n) entièrement validée.
  - Invariants impactés re-testés et conformes.
  - Documentation mise à jour (schéma, runbook, limites connues).
