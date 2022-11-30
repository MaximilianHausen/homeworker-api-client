use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::types::Ips;
use crate::types::schools::*;
use crate::types::timetables::TimetableDay;
use crate::types::users::*;

pub mod types;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ApiError(types::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}

impl From<types::Error> for Error {
    fn from(error: types::Error) -> Self {
        Self::ApiError(error)
    }
}

pub mod auth {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct TokenResponse {
        pub access_token: String,
        pub expires_in: u32,
        pub scope: String,
        pub token_type: String,
        pub refresh_token: String,
    }

    pub async fn exchange_token(client_id: String, client_secret: String, code: String) -> crate::Result<TokenResponse> {
        #[derive(Serialize)]
        struct CodeExchangeQuery {
            client_id: String,
            client_secret: String,
            code: String,
            grant_type: String,
        }

        let response = Client::new().get("https://homeworker.li/api/v2/oauth2/token".to_owned())
            .query(&CodeExchangeQuery {
                client_id,
                client_secret,
                code,
                grant_type: "authorization_code".to_owned(),
            })
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<TokenResponse>().await?)
        } else {
            Err(response.json::<crate::types::Error>().await?.into())
        }
    }
}

pub struct HomeworkerClient {
    client: Client,
    base_url: String,
    pub access_token: String,
    pub client_header: String,
}

impl HomeworkerClient {
    pub fn new(access_token: String, client_header: String) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://homeworker.li/api/v2".to_owned(),
            access_token,
            client_header
        }
    }

    pub fn with_custom_url(access_token: String, client_header: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            access_token,
            client_header
        }
    }

    // ========== GENERAL FETCH ==========

    async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.fetch(self.client.get(self.base_url.clone() + endpoint)).await
    }
    async fn post<T: Serialize, Q: DeserializeOwned>(&self, endpoint: &str, body: &T) -> Result<Q> {
        self.fetch(self.client.post(self.base_url.clone() + endpoint).json(&body)).await
    }
    async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.fetch(self.client.delete(self.base_url.clone() + endpoint)).await
    }

    async fn fetch<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("X-Client", &self.client_header)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(response.json::<types::Error>().await?.into())
        }
    }

    // ========== API ROUTES ==========

    // ---------- /me ----------

    pub async fn get_me(&self) -> Result<User> {
        self.get("/me").await
    }
    pub async fn edit_me(&self, name: String, mail: String, mobile: String, birthday: String) -> Result<User> {
        #[derive(serde::Serialize)]
        pub struct UserEditArgs {
            //TODO: Check for optional in /me POST
            pub name: String,
            pub mail: String,
            pub mobile: String,
            pub birthday: String,
        }
        self.post("/me", &UserEditArgs { name, mail, mobile, birthday }).await
    }

    pub async fn get_token_info(&self) -> Result<AccessTokenInfo> {
        self.get("/me/access_token").await
    }

    pub async fn get_presence(&self) -> Result<Presence> {
        self.get("/me/presence").await
    }
    pub async fn set_presence(&self, is_online: bool, expires_in: i32) -> Result<Presence> {
        self.post("/me/presence", &Presence { is_online, expires_in }).await
    }

    pub async fn get_navigation(&self) -> Result<Navigation> {
        self.get("/me/navigation").await
    }

    //TODO: default_value option
    pub async fn get_setting(&self, setting_name: &str, default_value: &str) -> Result<Setting> {
        self.get(&format!("/me/settings?name={}&default={}", setting_name, default_value)).await
    }
    pub async fn set_setting(&self, name: String, value: String) -> Result<Setting> {
        self.post("/me/settings", &Setting { name, value: Some(value) }).await
    }

    pub async fn get_all_notifications(&self) -> Result<Vec<Notification>> {
        self.get("/me/notifications").await
    }
    pub async fn get_unseen_notifications(&self) -> Result<Vec<Notification>> {
        self.get("/me/notifications/unseen").await
    }
    pub async fn get_notification(&self, notification_id: u32) -> Result<Notification> {
        self.get(&format!("/me/notifications/{}", notification_id)).await
    }
    pub async fn delete_notification(&self, notification_id: u32) -> Result<bool> {
        #[derive(serde::Deserialize)]
        pub struct NotificationDeleteReturn {
            pub success: bool,
        }
        self.delete::<NotificationDeleteReturn>(&format!("/me/notifications/{}", notification_id)).await
            .map(|x| x.success)
    }
    pub async fn mark_notification_as_seen(&self, notification_id: u32) -> Result<()> {
        self.post(&format!("/me/notifications/{}/mark_as_read", notification_id), &"").await
    }

    pub async fn get_course_memberships(&self) -> Result<Vec<Membership>> {
        self.get("/me/courses/memberships").await
    }

    pub async fn get_student_info(&self) -> Result<StudentInfo> {
        self.get("/me/student").await
    }

    // ---------- /schools ----------

    pub async fn get_all_schools(&self) -> Result<Vec<School>> {
        self.get("/schools").await
    }

    pub async fn get_school(&self, school_id: u32) -> Result<School> {
        self.get(&format!("/schools/{}", school_id)).await
    }

    // ---------- /courses ----------

    pub async fn get_timetable(&self, course_id: u32) -> Result<Vec<TimetableDay>> {
        self.get(&format!("/courses/{}/timetable/timeline", course_id)).await
    }

    // ---------- /ips ----------

    pub async fn get_ips(&self) -> Result<Ips> {
        self.get("/ips").await
    }
}
