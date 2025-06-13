#[derive(Debug)]
pub struct IpInfo {
    pub ip: String,
    pub country: String,
}

pub async fn get_ip_info() -> IpInfo {
    use reqwest::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    let client = Client::builder().timeout(Duration::from_secs(5)).build();
    let client = match client {
        Ok(c) => c,
        Err(_) => {
            return IpInfo {
                ip: "Error creating HTTP client".to_string(),
                country: "Unknown".to_string(),
            };
        }
    };

    for _ in 0..12 {
        match client
            .get("https://check.torproject.org/api/ip")
            .send()
            .await
        {
            Ok(resp) => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(ip) = json.get("IP").and_then(|v| v.as_str()) {
                        let country = get_country_from_ip(&client, ip).await;
                        return IpInfo {
                            ip: ip.to_string(),
                            country,
                        };
                    }
                }
            }
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }

    match client.get("https://ident.me").send().await {
        Ok(resp) => {
            if let Ok(ip) = resp.text().await {
                let ip = ip.trim().to_string();
                let country = get_country_from_ip(&client, &ip).await;
                return IpInfo { ip, country };
            }
        }
        Err(_) => {}
    }

    IpInfo {
        ip: "Error obtaining IP".to_string(),
        country: "Unknown".to_string(),
    }
}

async fn get_country_from_ip(client: &reqwest::Client, ip: &str) -> String {
    let url = format!("https://ipapi.co/{}/country_name/", ip);
    match client.get(&url).send().await {
        Ok(resp) => {
            if let Ok(country) = resp.text().await {
                let country = country.trim();
                if !country.is_empty() && !country.to_lowercase().contains("error") {
                    return country.to_string();
                }
            }
        }
        Err(_) => {}
    }

    let url = format!("http://ip-api.com/json/{}", ip);
    match client.get(&url).send().await {
        Ok(resp) => {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(country) = json.get("country").and_then(|v| v.as_str()) {
                    if json.get("status").and_then(|v| v.as_str()) == Some("success") {
                        return country.to_string();
                    }
                }
            }
        }
        Err(_) => {}
    }

    "Unknown".to_string()
}
