use serde::{Deserialize, Serialize};
use std::env;
use crate::ai::prompt::system_instruction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionRecommendation {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiChatResponse {
    pub explanation: String,
    pub recommended_actions: Vec<ActionRecommendation>,
}

pub async fn analyze_user_query(query: &str) -> AiChatResponse {
    let api_key = env::var("GEMINI_API_KEY").unwrap_or_default();

    if !api_key.is_empty() {
        if let Ok(res) = call_gemini_api(&api_key, query).await {
            return res;
        }
    }

    // Fallback: Smart Local AI Parser
    smart_local_ai_parser(query)
}

pub async fn analyze_system_logs(logs_text: &str) -> AiChatResponse {
    let api_key = env::var("GEMINI_API_KEY").unwrap_or_default();

    if !api_key.is_empty() {
        let prompt = format!(
            "Analisislah log Windows berikut dan tunjukkan akar masalahnya secara singkat:\n\n{}",
            logs_text
        );
        if let Ok(res) = call_gemini_api(&api_key, &prompt).await {
            return res;
        }
    }

    let mut actions = Vec::new();
    let lower = logs_text.to_lowercase();

    if lower.contains("spooler") || lower.contains("print") {
        actions.push(ActionRecommendation {
            id: "fix_spooler".into(),
            label: "Clear Printer Queue & Restart Service".into(),
        });
    }
    if lower.contains("net") || lower.contains("dns") || lower.contains("dhcp") {
        actions.push(ActionRecommendation {
            id: "fix_network".into(),
            label: "Reset Network & Flush DNS".into(),
        });
    }
    if lower.contains("corrupt") || lower.contains("disk") || lower.contains("ntfs") || lower.contains("bugcheck") {
        actions.push(ActionRecommendation {
            id: "run_sfc".into(),
            label: "Run SFC & DISM System Repair".into(),
        });
    }

    if actions.is_empty() {
        actions.push(ActionRecommendation {
            id: "run_sfc".into(),
            label: "Run SFC System Repair".into(),
        });
    }

    AiChatResponse {
        explanation: "Hasil Analisis Nantara AI Engine:\nDitemukan beberapa peristiwa peringatan/error pada Windows Log. Berikut rekomendasi tindakan perbaikan otomatis yang disarankan:".to_string(),
        recommended_actions: actions,
    }
}

fn smart_local_ai_parser(query: &str) -> AiChatResponse {
    let q = query.to_lowercase();
    let mut actions = Vec::new();
    let mut explanation_parts = Vec::new();

    if q.contains("internet") || q.contains("wifi") || q.contains("koneksi") || q.contains("dns") || q.contains("ping") {
        actions.push(ActionRecommendation {
            id: "fix_network".into(),
            label: "Fix Network & Flush DNS".into(),
        });
        explanation_parts.push("Saya mendeteksi potensi kendala pada konfigurasi jaringan atau cache DNS Anda.");
    }

    if q.contains("print") || q.contains("printer") || q.contains("cetak") || q.contains("macet") || q.contains("spooler") {
        actions.push(ActionRecommendation {
            id: "fix_spooler".into(),
            label: "Clear Printer Spooler Queue".into(),
        });
        explanation_parts.push("Terdeteksi kendala antrean cetak dokumen di Printer Spooler Service.");
    }

    if q.contains("lemot") || q.contains("lag") || q.contains("hang") || q.contains("ram") || q.contains("memori") || q.contains("freeze") {
        actions.push(ActionRecommendation {
            id: "kill_heavy_apps".into(),
            label: "Free Up RAM & Kill Heavy Apps".into(),
        });
        explanation_parts.push("Penggunaan resource sistem cukup tinggi atau ada aplikasi yang frozen.");
    }

    if q.contains("bsod") || q.contains("blue screen") || q.contains("error") || q.contains("corrupt") || q.contains("rusak") {
        actions.push(ActionRecommendation {
            id: "run_sfc".into(),
            label: "Run SFC & DISM Repair".into(),
        });
        explanation_parts.push("Sistem menunjukkan tanda-tanda integritas file Windows bermasalah.");
    }

    if actions.is_empty() {
        actions.push(ActionRecommendation {
            id: "fix_network".into(),
            label: "Check Network Connection".into(),
        });
        actions.push(ActionRecommendation {
            id: "run_sfc".into(),
            label: "Run System Integrity Repair".into(),
        });
        explanation_parts.push("Halo! Saya telah menganalisis keluhan Anda. Berikut adalah langkah diagnostik otomatis yang direkomendasikan:");
    }

    AiChatResponse {
        explanation: explanation_parts.join(" "),
        recommended_actions: actions,
    }
}

async fn call_gemini_api(api_key: &str, prompt: &str) -> Result<AiChatResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let payload = serde_json::json!({
        "contents": [{
            "parts": [{ "text": format!("{}\n\nUser Question: {}", system_instruction(), prompt) }]
        }]
    });

    let res = client.post(&url).json(&payload).send().await?;
    let json: serde_json::Value = res.json().await?;

    let text_content = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("");

    if let Ok(parsed) = serde_json::from_str::<AiChatResponse>(text_content) {
        return Ok(parsed);
    }

    Ok(AiChatResponse {
        explanation: text_content.to_string(),
        recommended_actions: vec![],
    })
}
