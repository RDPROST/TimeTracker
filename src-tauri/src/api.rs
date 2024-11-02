use base64::prelude::*;
use colored::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_xml_rs::from_str;
use std::collections::HashMap;
use tauri::command;
use dotenv::dotenv;
use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BASE_URL: String = {
        dotenv().ok();
        env::var("BASE_URL").unwrap_or_else(|_| "https://default.url.com".to_string())
    };

    pub static ref USER_AGENT_VALUE: String = {
        env::var("USER_AGENT_VALUE").unwrap_or_else(|_| "".to_string())
    };
}

fn get_url(resource_type: &str) -> String {
    match resource_type {
        "auth" => format!("{}?m=ttlp", *BASE_URL),
        "projects" => format!("{}?m=ttlp", *BASE_URL),
        "tasks" => format!("{}?m=ttlt", *BASE_URL),
        "startTimer" => format!("{}?m=ttr", *BASE_URL),
        "stopTimer" => format!("{}?m=tts", *BASE_URL),
        "timer" => format!("{}?m=ttget", *BASE_URL),
        _ => format!("{}?m={}", *BASE_URL, resource_type),
    }
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "error")]
    error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorDetail {
    #[serde(rename = "value")]
    value: String,
    #[serde(rename = "$value")]
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataProjects {
    #[serde(rename = "project", default)]
    projects: Vec<Project>,
    #[serde(rename = "error")]
    error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "org_id")]
    org_id: String,
    #[serde(rename = "org_name")]
    org_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataTasks {
    #[serde(rename = "task", default)]
    projects: Vec<Task>,
    #[serde(rename = "error")]
    error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "phase_id")]
    phase_id: String,
    #[serde(rename = "phase_name")]
    phase_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DataTimer {
    #[serde(rename = "progress")]
    timer: Option<Timer>,
    #[serde(rename = "servertime")]
    servertime: Option<String>,
    #[serde(rename = "error")]
    error: ErrorDetail,
}

#[derive(Debug, Deserialize, Serialize)]
struct Timer {
    #[serde(rename = "time")]
    id: Option<String>,
    #[serde(rename = "org_id")]
    org_id: Option<String>,
    #[serde(rename = "org_name")]
    org_name: Option<String>,
    #[serde(rename = "task_id")]
    task_id: Option<String>,
    #[serde(rename = "task_name")]
    task_name: Option<String>,
    #[serde(rename = "project_id")]
    project_id: Option<String>,
    #[serde(rename = "project_name")]
    project_name: Option<String>,
    #[serde(rename = "start_date")]
    start_date: Option<String>,
    #[serde(rename = "description")]
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MyError {
    message: String,
    code: Option<String>,
}

async fn get_response(
    username: &str,
    password: &str,
    resource_type: &str,
    params: Option<HashMap<String, String>>,
) -> Result<String, MyError> {
    let credentials = format!("{}:{}", username, password);
    let encoded_credentials = BASE64_STANDARD.encode(credentials);
    let auth_header_value = format!("Basic {}", encoded_credentials);

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header_value).unwrap(),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(&*USER_AGENT_VALUE));
	println!("Заголовки: {:?}", headers);

    let mut url = get_url(resource_type);

    if let Some(parameters) = params {
        let query_string: String = parameters
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");

        url = format!("{}&{}", url, query_string);
    }

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| MyError {
            message: format!("{}", e),
            code: Some("500".to_string()),
        })?;

    let status = response.status();

    let text = response.text().await.map_err(|e| MyError {
        message: format!("{}", e),
        code: Some("500".to_string()),
    })?;

    println!("Ответ сервера '{}': {}", resource_type.green(), text);

    if let Ok(error_response) = from_str::<ErrorResponse>(&text) {
        let error = MyError {
            message: format!("{}", error_response.error.message),
            code: Some(error_response.error.value),
        };

        if error.code == Some("0".to_string()) {
            return Ok(text);
        } else {
            return Err(error);
        }
    }

    if status.is_success() {
        let new_string = text.clone().replace("<br/>", "\n");

        return Ok(new_string);
    } else {
        return Err(MyError {
            message: format!("{}", text),
            code: None,
        });
    }
}

#[command]
pub async fn authorize(username: &str, password: &str) -> Result<String, MyError> {
    let text = get_response(username, password, "auth", None).await;

    match text {
        Ok(text) => Ok(text),
        Err(err) => Err(err),
    }
}

#[command]
pub async fn get_projects(username: &str, password: &str) -> Result<serde_json::Value, MyError> {
    let text = get_response(username, password, "projects", None).await?;

    let data: DataProjects = from_str(&text).map_err(|e| MyError {
        message: format!("Ошибка парсинга XML: {}", e),
        code: Some("500".to_string()),
    })?;

    if data.error.value != "0" {
        return Err(MyError {
            message: data.error.message,
            code: Some(data.error.value),
        });
    }

    Ok(json!(data.projects))
}

#[command]
pub async fn get_tasks(
    username: &str,
    password: &str,
    project_id: &str,
) -> Result<serde_json::Value, MyError> {
    let params = HashMap::from([("project_id".to_string(), project_id.to_string())]);

    let text = get_response(username, password, "tasks", Some(params)).await?;

    let data: DataTasks = from_str(&text).map_err(|e| MyError {
        message: format!("Ошибка парсинга XML: {}", e),
        code: Some("500".to_string()),
    })?;

    if data.error.value != "0" {
        return Err(MyError {
            message: data.error.message,
            code: Some(data.error.value),
        });
    }

    Ok(json!(data.projects))
}

#[command]
pub async fn start_timer(username: &str, password: &str, task_id: &str) -> Result<String, MyError> {
    let params = HashMap::from([("t_id".to_string(), task_id.to_string())]);

    let text = get_response(username, password, "startTimer", Some(params)).await?;

    let data: DataTimer = from_str(&text).map_err(|e| MyError {
        message: format!("Ошибка парсинга XML: {}", e),
        code: Some("500".to_string()),
    })?;

    if data.error.value != "0" {
        return Err(MyError {
            message: data.error.message,
            code: Some(data.error.value),
        });
    }

    Ok("true".to_string())
}

#[command]
pub async fn stop_timer(username: &str, password: &str) -> Result<String, MyError> {
    let text = get_response(username, password, "stopTimer", None).await?;

    let data: DataTimer = from_str(&text).map_err(|e| MyError {
        message: format!("Ошибка парсинга XML: {}", e),
        code: Some("500".to_string()),
    })?;

    if data.error.value != "0" {
        return Err(MyError {
            message: data.error.message,
            code: Some(data.error.value),
        });
    }

    Ok("true".to_string())
}

#[command]
pub async fn get_timer(username: &str, password: &str) -> Result<serde_json::Value, MyError> {
    let text = get_response(username, password, "timer", None).await?;

    let data: DataTimer = from_str(&text).map_err(|e| MyError {
        message: format!("Ошибка парсинга XML: {}", e),
        code: Some("500".to_string()),
    })?;

    if data.error.value != "0" {
        return Err(MyError {
            message: data.error.message,
            code: Some(data.error.value),
        });
    }

    let result = json!({
        "timer": data.timer,
        "servertime": data.servertime,
    });

    Ok(result)
}
