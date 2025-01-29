use std::sync::Arc;

use async_trait::async_trait;
use pingora_core::prelude::*;
use pingora_core::server::Server;
use pingora_load_balancing::{selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};
struct Lb(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for Lb {
    // just to satificed trait bound.
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    // this will be called for every request;  as per blog post
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstreams = self.0.select(b"", 256).unwrap();

        println!("Upstream peer is {:?}", upstreams);
        let peer = Box::new(HttpPeer::new(
            upstreams,
            false,
            "one.one.one.one".to_string(),
        ));

        Ok(peer)
    }

    // motify/filter the request context

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstram_request: &mut pingora_http::RequestHeader,
        _ctx: &mut (),
    ) -> Result<()> {

        upstram_request.insert_header("Host", "one.one.one.one").unwrap();

        Ok(())

    }
}

fn main() {
    // create upstream Service
    let upstreams =
        Arc::new(LoadBalancer::try_from_iter(["127.0.0.1:3000", "127.0.0.1:3001"]).unwrap());

    // TODO: create background healthcheck

    // create https server => that will listen the incomming request
    let mut lb_server = Server::new(None).unwrap();

    // create proxy instance
    let mut lb = pingora_proxy::http_proxy_service(&lb_server.configuration, Lb(upstreams));
    // set the lb port
    lb.add_tcp("0.0.0.0:6789");

    lb_server.add_service(lb);
    lb_server.run_forever()
}
