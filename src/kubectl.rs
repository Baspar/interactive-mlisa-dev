use serde::Deserialize;

#[derive(Deserialize,Debug,Clone)]
pub struct Status {
    pub phase: String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct PodLabels {
    #[serde(alias = "app.kubernetes.io/component")]
    pub component: String,
    #[serde(alias = "app.kubernetes.io/instance")]
    pub instance: String,
    #[serde(alias = "app.kubernetes.io/managed-by")]
    pub managed_by: String,
    #[serde(alias = "app.kubernetes.io/name")]
    pub name: String,
    #[serde(alias = "app.kubernetes.io/version")]
    pub version: String,
    pub patched: Option<String>,
}
#[derive(Deserialize,Debug)]
#[serde(untagged)]
pub enum Labels {
    PodLabels (PodLabels),
    OtherLabels {},
}

#[derive(Deserialize,Debug)]
pub struct MetaData<T> {
    pub name: String,
    pub labels: T,
}

#[derive(Deserialize,Debug,Clone)]
pub struct Spec {
}

#[derive(Deserialize,Debug)]
pub struct Pod<T> {
    pub metadata: MetaData<T>,
    pub spec: Spec,
    pub status: Status,
}

#[derive(Deserialize,Debug)]
pub struct Response<T> {
    pub items: Vec<Pod<T>>,
}
