use futures::prelude::*;

/// Common trait for GraphQL subscription servers.
pub trait SubscriptionServer {
    type ServeError;

    /// Returns a Future that, when spawned, brings up the GraphQL subscription server.
    fn serve(
        &mut self,
        port: u16,
    ) -> Result<Box<Future<Item = (), Error = ()> + Send>, Self::ServeError>;
}
