use axum::{Router, routing::get};
use std::net::SocketAddr;
use rand::Rng;

struct Metric<'a> {
    comment: Option<&'a str>,
    label: &'a str,
    value: String,
}

impl Metric<'_> {
    pub fn new<'a>(comment: Option<&'a str>, label: &'a str, value: String) -> Metric<'a> {
        Metric { comment, label, value }
    }
    pub fn update_value(&mut self, value: String) -> () {
        self.value = value;
    }
    pub fn display(&self) -> String {
        match self.comment {
            Some(c) => format!("# {}\n{} {}\n", c , self.label, self.value),
            None => return format!("{} {}\n", self.label, self.value),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/metrics", get(get_metrics));
    let addr = SocketAddr::from(([127,0,0,1], 9100));


    println!("Server running on http://localhost:9100");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_metrics() -> String {
    let random_value = rand::thread_rng().gen_range(0..100);
    let metrics: Vec<Metric> = vec![
        Metric::new(None, "random_metric", random_value.to_string()), 
        Metric::new(Some("Yet another random metric"), "another_random_metric", (random_value+1).to_string()),
    ];
    let result = metrics.iter().map(|m| m.display()).collect::<String>();
    result
}
