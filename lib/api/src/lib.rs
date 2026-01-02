pub mod feeds {
    pub const DESCRIPTOR: &[u8] = tonic::include_file_descriptor_set!("feeds_descriptor");

    tonic::include_proto!("bzd.feeds.feeds");
}
