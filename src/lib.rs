use crate::api::me::notifications::id::NotificationDeleteReturn;
use crate::api::me::UserEditArgs;
use crate::types::Ips;
use crate::types::users::{Membership, Notification, StudentInfo};

pub mod types;

pub const BASE_URL: &str = "https://homeworker.li/api/v2";

pub struct HomeworkerClient {
    access_token: String,
    refresh_token: Option<String>,
}

impl HomeworkerClient {
    pub fn new(access_token: String, refresh_token: Option<String>) -> HomeworkerClient {
        Self {
            access_token,
            refresh_token,
        }
    }

    pub fn get_me(&self, callback: fn(ehttp::Result<types::users::User>)) {
        api::me::get(&self.access_token, callback);
    }
    pub fn edit_me(&self, callback: fn(ehttp::Result<types::users::User>), (name, mail, mobile, birthday): (String, String, String, String)) {
        api::me::post(&self.access_token, callback, &UserEditArgs { name, mail, mobile, birthday });
    }

    pub fn get_access_token(&self, callback: fn(ehttp::Result<types::users::AccessTokenInfo>)) {
        api::me::access_token::get(&self.access_token, callback);
    }

    pub fn get_presence(&self, callback: fn(ehttp::Result<types::users::Presence>)) {
        api::me::presence::get(&self.access_token, callback);
    }
    pub fn set_presence(&self, callback: fn(ehttp::Result<types::users::Presence>), new_presence: &types::users::Presence) {
        api::me::presence::post(&self.access_token, callback, new_presence);
    }

    pub fn get_navigation(&self, callback: fn(ehttp::Result<types::users::Navigation>)) {
        api::me::navigation::get(&self.access_token, callback);
    }

    pub fn get_setting(&self, callback: fn(ehttp::Result<types::users::Setting>), setting_name: &str, default_value: &str) {
        api::me::settings::get(&self.access_token, callback, setting_name, default_value);
    }
    pub fn set_setting(&self, callback: fn(ehttp::Result<types::users::Setting>), new_setting: &types::users::Setting) {
        api::me::settings::post(&self.access_token, callback, new_setting);
    }

    pub fn get_notifications(&self, callback: fn(ehttp::Result<Vec<Notification>>)) {
        api::me::notifications::get(&self.access_token, callback);
    }
    pub fn get_unseen_notifications(&self, callback: fn(ehttp::Result<Vec<Notification>>)) {
        api::me::notifications::unseen::get(&self.access_token, callback);
    }
    pub fn get_notification(&self, callback: fn(ehttp::Result<Notification>), notification_id: u32) {
        api::me::notifications::id::get(&self.access_token, callback, notification_id);
    }
    pub fn delete_notification(&self, callback: fn(ehttp::Result<NotificationDeleteReturn>), notification_id: u32) {
        api::me::notifications::id::delete(&self.access_token, callback, notification_id);
    }
    pub fn mark_notification_as_seen(&self, callback: fn(ehttp::Result<()>), notification_id: u32) {
        api::me::notifications::id::mark_as_seen::post(&self.access_token, callback, notification_id);
    }

    pub fn get_memberships(&self, callback: fn(ehttp::Result<Vec<Membership>>)) {
        api::me::courses::memberships::get(&self.access_token, callback);
    }

    pub fn get_student(&self, callback: fn(ehttp::Result<StudentInfo>)) {
        api::me::student::get(&self.access_token, callback);
    }

    pub fn get_ips(&self, callback: fn(ehttp::Result<Ips>)) {
        api::ips::get(callback);
    }
}

pub mod api {
    use ehttp::Request;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    use crate::BASE_URL;

    fn fetch<T: DeserializeOwned + 'static>(request: Request, callback: fn(ehttp::Result<T>)) {
        ehttp::fetch(
            request,
            move |result: ehttp::Result<ehttp::Response>| {
                match result {
                    Ok(response) => {
                        let y = &response.bytes;
                        callback(Ok(serde_json::from_slice::<T>(y).unwrap()))
                    }
                    Err(error) => callback(Err(error)),
                }
            },
        );
    }

    fn get_request(endpoint: &str, token: &str) -> Request {
        let mut request = Request::get(BASE_URL.to_owned() + endpoint);
        request
            .headers
            .insert("Authorization".to_owned(), "Bearer ".to_owned() + token);
        request
    }

    fn delete_request(endpoint: &str, token: &str) -> Request {
        let mut request = Request {
            method: "DELETE".to_owned(),
            url: BASE_URL.to_owned() + endpoint,
            body: vec![],
            headers: ehttp::headers(&[("Accept", "*/*")]),
        };
        request
            .headers
            .insert("Authorization".to_owned(), "Bearer ".to_owned() + token);
        request
    }

    fn post_request(endpoint: &str, token: &str, body: &impl Serialize) -> Request {
        let mut request = Request::post(
            BASE_URL.to_owned() + endpoint,
            serde_json::to_vec(body).unwrap(),
        );
        request
            .headers
            .insert("Authorization".to_owned(), "Bearer ".to_owned() + token);
        request
    }

    pub mod me {
        use crate::{
            api::{fetch, get_request, post_request},
            types::users::User,
        };

        pub fn get(token: &str, callback: fn(ehttp::Result<User>)) {
            fetch(get_request("/me", token), callback);
        }

        #[derive(serde::Serialize, serde::Deserialize, Clone)]
        pub struct UserEditArgs {
            //TODO: Check for optional in /me POST
            pub name: String,
            pub mail: String,
            pub mobile: String,
            pub birthday: String,
        }

        pub fn post(token: &str, callback: fn(ehttp::Result<User>), edit_info: &UserEditArgs) {
            fetch(post_request("/me", token, edit_info), callback);
        }

        pub mod access_token {
            use crate::{
                api::{fetch, get_request},
                types::users::AccessTokenInfo,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<AccessTokenInfo>)) {
                fetch(get_request("/me/access_token", token), callback);
            }
        }

        pub mod presence {
            use crate::{
                api::{fetch, get_request, post_request},
                types::users::Presence,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<Presence>)) {
                fetch(get_request("/me/presence", token), callback);
            }

            pub fn post(token: &str, callback: fn(ehttp::Result<Presence>), new_presence: &Presence) {
                fetch(post_request("/me/presence", token, new_presence), callback);
            }
        }

        pub mod navigation {
            use crate::{
                api::{fetch, get_request},
                types::users::Navigation,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<Navigation>)) {
                fetch(get_request("/me/navigation", token), callback);
            }
        }

        pub mod settings {
            use crate::{
                api::{fetch, get_request, post_request},
                types::users::Setting,
            };

            //TODO: default_value option
            pub fn get(token: &str, callback: fn(ehttp::Result<Setting>), setting_name: &str, default_value: &str) {
                fetch(get_request(&format!("/me/settings?name={}&default={}", setting_name, default_value), token), callback);
            }

            pub fn post(token: &str, callback: fn(ehttp::Result<Setting>), new_setting: &Setting) {
                fetch(post_request("/me/settings", token, new_setting), callback);
            }
        }

        pub mod notifications {
            use crate::{
                api::{fetch, get_request},
                types::users::Notification,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<Vec<Notification>>)) {
                fetch(get_request("/me/notifications", token), callback);
            }

            pub mod unseen {
                use crate::{
                    api::{fetch, get_request},
                    types::users::Notification,
                };

                pub fn get(token: &str, callback: fn(ehttp::Result<Vec<Notification>>)) {
                    fetch(get_request("/me/notifications/unseen", token), callback);
                }
            }

            pub mod id {
                use crate::{
                    api::{delete_request, fetch, get_request},
                    types::users::Notification,
                };

                pub fn get(token: &str, callback: fn(ehttp::Result<Notification>), notification_id: u32) {
                    fetch(get_request(&format!("/me/notifications/{}", notification_id), token), callback);
                }

                #[derive(serde::Serialize, serde::Deserialize, Clone)]
                pub struct NotificationDeleteReturn {
                    pub success: bool,
                }

                pub fn delete(token: &str, callback: fn(ehttp::Result<NotificationDeleteReturn>), notification_id: u32) {
                    fetch(delete_request(&format!("/me/notifications/{}", notification_id), token), callback);
                }

                pub mod mark_as_seen {
                    use crate::{
                        api::{fetch, post_request},
                    };

                    pub fn post(token: &str, callback: fn(ehttp::Result<()>), notification_id: u32) {
                        fetch(post_request(&format!("/me/notifications/{}/mark_as_read", notification_id), token, &""), callback);
                    }
                }
            }
        }

        pub mod courses {
            pub mod memberships {
                use crate::{
                    api::{fetch, get_request},
                    types::users::Membership,
                };

                pub fn get(token: &str, callback: fn(ehttp::Result<Vec<Membership>>)) {
                    fetch(get_request("/me/courses/memberships", token), callback);
                }
            }
        }

        pub mod student {
            use crate::{
                api::{fetch, get_request},
                types::users::StudentInfo,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<StudentInfo>)) {
                fetch(get_request("/me/student", token), callback);
            }
        }
    }

    pub mod schools {
        use crate::{
            api::{fetch, get_request},
            types::schools::School,
        };

        pub fn get(token: &str, callback: fn(ehttp::Result<Vec<School>>)) {
            fetch(get_request("/schools", token), callback);
        }

        pub mod id {
            use crate::{
                api::{fetch, get_request},
                types::schools::School,
            };

            pub fn get(token: &str, callback: fn(ehttp::Result<School>), school_id: u32) {
                fetch(get_request(&format!("/schools/{}", school_id), token), callback);
            }
        }
    }

    pub mod ips {
        use crate::{
            api::fetch,
            BASE_URL,
            types::Ips,
        };

        pub fn get(callback: fn(ehttp::Result<Ips>)) {
            fetch(ehttp::Request::get(BASE_URL.to_owned() + "/ips"), callback);
        }
    }
}
