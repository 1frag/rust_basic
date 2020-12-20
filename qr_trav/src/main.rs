use {
    async_trait::async_trait,
    base64,
    mobc::Manager,
    qrcode_generator::QrCodeEcc,
    rand::Rng,
    redis,
    serde_json::json,
    tide::{
        Request, Middleware,
        http::Cookie,
    },
    uuid::Uuid,
};

struct Site<'a> {
    proto: &'a str,
    hostname: &'a str,
    port: i32,
}

impl Site<'_> {
    fn host(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    fn at(&self, path: &str, overridden_hostname: Option<&str>) -> String {
        let host = &(self.host() + "/");
        let host = overridden_hostname.unwrap_or(host);
        format!("{}://{}{}", self.proto, host, path)
    }
}

const CURRENT_SITE: Site = Site {
    proto: "http",
    hostname: "0.0.0.0",
    port: 8080,
};

#[derive(Clone)]
struct AppState {
    redis_pool: mobc::Pool<RedisManager>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let redis_pool = mobc::Pool::builder()
        .max_open(100)
        .build(RedisManager);
    let mut app = tide::with_state(AppState { redis_pool });

    app.at("/:path").get(main_handler);
    app.with(AuthenticateMiddleware);
    app.listen(CURRENT_SITE.host()).await?;
    Ok(())
}

fn add_start_again_html(resp: &mut tide::Response) {
    // language=HTML
    resp.set_body(r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Step failed</title>
</head>
<body>
    You are wrong, start again to pass succesfully.
</body>
</html>"##);
}

fn add_puzzle_into_html(resp: &mut tide::Response, answer: &str, host: Option<&str>) {
    let next_path = CURRENT_SITE.at(answer, host);
    let qr_code = qrcode_generator::to_png_to_vec(
        next_path, QrCodeEcc::Low, 1024
    ).unwrap();
    let qr_code = base64::encode(qr_code);

    // language=HTML
    resp.set_body(format!(r##"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Start</title>
</head>
<body>
    <img src="data:image/jpeg;base64, {}" height="300px" alt="qr_code">
</body>
</html>"##, qr_code));
}

async fn is_incorrect_answer(state: &AppState, user: &str, path: &str) -> Option<bool> {
    let conn: mobc::Connection<RedisManager> = state.redis_pool.get().await.ok()?;
    let mut conn: redis::aio::Connection = conn.into_inner();
    let user_data: Option<String> = redis::cmd("GET")
        .arg(user)
        .query_async(&mut conn)
        .await.unwrap();
    if user_data.is_none() { return None }
    let user_data: serde_json::Value = serde_json::from_str(&user_data?).ok()?;
    let user_data = user_data.get("path").unwrap().as_str()?;
    println!(" path={:?} user_data['path']={:?}", &path, user_data);
    Some(path != user_data)
}

async fn expect_in_future(state: &AppState, uuid: &str, answer: &str) -> Option<()> {
    let conn: mobc::Connection<RedisManager> = state.redis_pool.get().await.ok()?;
    let mut conn: redis::aio::Connection = conn.into_inner();
    let user_data = json!({"path": answer});
    let user_data = user_data.to_string();
    let _: redis::RedisResult<()> = redis::cmd("SET")
        .arg(uuid)
        .arg(&user_data)
        .query_async(&mut conn)
        .await;
    println!("{}", user_data);
    Some(())
}

async fn main_handler(req: Request<AppState>) -> tide::Result {
    let user: &UserUUID = req.ext().unwrap();
    let mst: &MileStoneTag = req.ext().unwrap();
    if *mst == MileStoneTag::Other {
        return Ok(tide::Response::new(404));
    }

    let mut resp = tide::Response::new(200);
    resp.insert_header("Content-Type", "text/html");
    if MileStoneTag::Step == *mst && is_incorrect_answer(
        &req.state(), &user.uuid, &req.url().path()
    ).await.unwrap_or(true) {
        add_start_again_html(&mut resp);
    } else {
        let answer = format!("/step_{}", rand::thread_rng().gen_range(100000..1000000));
        add_puzzle_into_html(&mut resp, &answer, req.local_addr());
        expect_in_future(&req.state(), &user.uuid, &answer).await;
    }
    Ok(resp)
}

struct AuthenticateMiddleware;

#[derive(Debug)]
struct UserUUID { uuid: String }

#[derive(Debug, PartialEq)]
enum MileStoneTag { Start, Step, Other }


#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for AuthenticateMiddleware {
    async fn handle(
        &self,
        mut req: Request<State>,
        next: tide::Next<'_, State>,
    ) -> tide::Result {
        let mut update_cookie = None;
        req.set_ext(UserUUID {
            uuid: match req.cookie("user_id") {
                Some(c) => c.value().to_string(),
                None => {
                    let uuid = Uuid::new_v4().to_string();
                    update_cookie = Some(Cookie::new("user_id", uuid.clone()));
                    uuid
                }
            }
        });
        use regex::Regex;
        let re = Regex::new(r"/(start)|(step_\d+)").unwrap();
        let fs = match re.captures(req.url().path()) {
            Some(t) => {
                match t.get(2) {
                    Some(_) => MileStoneTag::Step,
                    None => MileStoneTag::Start,
                }
            }, None => MileStoneTag::Other
        };
        req.set_ext(fs);

        let mut resp: tide::Response = next.run(req).await;
        if update_cookie.is_some() {
            resp.insert_cookie(update_cookie.unwrap());
        }
        Ok(resp)
    }
}

struct RedisManager;

#[async_trait]
impl Manager for RedisManager {
    type Connection = redis::aio::Connection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = redis::Client::open("redis://:redis@redis/").unwrap();
        let conn = client.get_async_connection().await?;
        Ok(conn)
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}
