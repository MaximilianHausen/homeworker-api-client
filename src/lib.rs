use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::types::Ips;
use crate::types::schools::*;
use crate::types::users::*;

pub mod types;

pub const BASE_URL: &str = "https://homeworker.li/api/v2";

type Result<T> = std::result::Result<T, Error>;

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

pub struct HomeworkerClient {
    client: Client,
    pub access_token: String,
}

impl HomeworkerClient {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    // ========== GENERAL FETCH ==========

    async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.fetch(self.client.get(BASE_URL.to_owned() + endpoint)).await
    }
    async fn post<T: Serialize, Q: DeserializeOwned>(&self, endpoint: &str, body: &T) -> Result<Q> {
        self.fetch(self.client.post(BASE_URL.to_owned() + endpoint).json(&body)).await
    }
    async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.fetch(self.client.delete(BASE_URL.to_owned() + endpoint)).await
    }


    async fn fetch<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request
            .header("Authorization", format!("Bearer {}", self.access_token))
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

    // ---------- /ips ----------

    pub async fn get_ips(&self) -> Result<Ips> {
        self.get("/ips").await
    }
}
