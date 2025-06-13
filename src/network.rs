pub async fn get_public_ip() -> String {
    use reqwest::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build();
    let client = match client {
        Ok(c) => c,
        Err(_) => return "Erro ao criar client".to_string(),
    };
    for _ in 0..12 {
        match client.get("https://check.torproject.org/api/ip").send().await {
            Ok(resp) => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(ip) = json.get("IP").and_then(|v| v.as_str()) {
                        return ip.to_string();
                    }
                }
            },
            Err(_) => {
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        }
    }
    match client.get("https://ident.me").send().await {
        Ok(resp) => {
            if let Ok(ip) = resp.text().await {
                return ip.trim().to_string();
            }
        },
        Err(_) => {}
    }
    "Erro ao obter IP".to_string()
}
