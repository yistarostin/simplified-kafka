use phf::phf_map;

#[derive(Debug, Clone, PartialEq)]

// Types of actions to handle on new requests
pub enum RequestType {
    Subscribe,
    Publish,
}

pub static REQUEST_TYPES: phf::Map<&'static str, RequestType> = phf_map! {
    "subscribe" => RequestType::Subscribe,
    "publish"   => RequestType::Publish,
};
