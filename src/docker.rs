#[path = "./values.rs"] mod values;
#[path = "./scan.rs"] mod scan;
use std::process::{Command, Output};
use std::fs::OpenOptions;
use std::io::Write;

pub fn build_and_run(image_name: &str) -> Output {
    Command::new("docker")
    .args(["run", "--rm", image_name, "printenv"])
    .output()
    .expect("failed to run image")
}

async fn fetch(url: &str) -> Result<values::Root, reqwest::Error> {
    reqwest::get(url).await?.json::<values::Root>().await
}

async fn get_container_names(url: &str) -> Result<Vec<String>, reqwest::Error> {
    Ok(fetch(url).await?.get_summary_names())
}

pub async fn export_secrets(secrets: Vec<&str>) -> Result<(), reqwest::Error> {
    for secret in secrets {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("secrets.txt")
            .unwrap();
    
        writeln!(file, "{}", secret).unwrap();
    }
    Ok(())
}

pub async fn process(url: &str) -> Result<Vec<&str>, reqwest::Error> {
    let mut secrets_list: Vec<&str> = vec![];

    for name in get_container_names(url).await?.iter() {
        let docker_output = build_and_run(name).stdout;
        let result = scan::find_secrets(docker_output);
        for secrets in result.unwrap() {
            secrets_list.push(secrets);
        }
    }
    Ok(secrets_list)
}
