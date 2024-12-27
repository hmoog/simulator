pub trait MessageType: Send + Sync + Clone + 'static {}
impl <T: Send + Sync + Clone + 'static> MessageType for T {}