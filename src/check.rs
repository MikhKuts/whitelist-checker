use ping;
use std::{io::Error, net::ToSocketAddrs, time::Duration};

pub fn check(
    url: String,
) -> std::result::Result<std::result::Result<u128, ping::Error>, (String, Error)> {
    let address = match url.to_socket_addrs() {
        Ok(mut ok) => ok.next().unwrap().ip(),
        Err(err) => return Err((String::from("Ошибка разрешения имени"), err)),
    };

    match ping::new(address)
        .timeout(Duration::from_secs(2))
        .ttl(128)
        .send()
    {
        Ok(ok) => {
            return Ok(Ok(ok.rtt.as_millis()));
        }
        Err(e) => {
            return Ok(Err(e));
        }
    };
}
