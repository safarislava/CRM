use actix_governor::governor::clock::QuantaInstant;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};

pub fn login_governor() -> GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> {
    GovernorConfigBuilder::default()
        .seconds_per_request(1)
        .burst_size(5)
        .finish()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_login_governor_config() {
        let _config = login_governor();
    }
}
