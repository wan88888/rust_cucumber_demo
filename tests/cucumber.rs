use cucumber::{World, WriterExt};
use std::{path::PathBuf, fs::File};

mod login_steps;
use login_steps::LoginWorld;

#[tokio::main]
async fn main() {
    let output_dir = PathBuf::from("target/cucumber-reports");
    std::fs::create_dir_all(&output_dir).unwrap();
    
    let json_file = File::create(output_dir.join("cucumber.json")).unwrap();
    
    LoginWorld::cucumber()
        .with_writer(
            cucumber::writer::Basic::stdout()
                .summarized()
                .normalized()
                .tee(
                    cucumber::writer::Json::new(json_file)
                )
        )
        .run("features/login.feature")
        .await;
}
