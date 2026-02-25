use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

/// Injecte un UUID v4 dans `x-request-id` sur chaque requête entrante
/// qui n'en possède pas déjà un.
///
/// La couche s'appuie sur `tower_http::request_id::MakeRequestUuid` :
/// aucune dépendance externe, UUID généré via `uuid::Uuid::new_v4()`.
pub fn set_request_id_layer() -> SetRequestIdLayer<MakeRequestUuid> {
    SetRequestIdLayer::x_request_id(MakeRequestUuid::default())
}

/// Recopie `x-request-id` de la requête vers la réponse afin que
/// le client puisse corréler la réponse avec son identifiant.
pub fn propagate_request_id_layer() -> PropagateRequestIdLayer {
    PropagateRequestIdLayer::x_request_id()
}
