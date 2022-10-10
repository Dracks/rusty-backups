use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use chrono::prelude::{Local};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct YamlCommand {
    exec: String,
    output: String,
}

struct BackupInstance {
    host: crate::backup::connection::Host,
    commands: Vec<YamlCommand>,
    output_path: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
struct YamlHost {
    username: String,
    hostname: String,
    private_key: String,
    output_path: Option<String>,
    commands: Vec<YamlCommand>,
}

pub struct Backup {
    instances: Vec<BackupInstance>,
}

fn check_parent_folder(path: &Path) {
	let base = path.parent().unwrap();
	if !base.is_dir(){
		std::fs::create_dir_all(base).unwrap()
	}
}

impl Backup {
    pub fn new(file: &str) -> Backup {
        let f = std::fs::File::open(file).expect("Could not open file");
        let instances: Vec<YamlHost> = serde_yaml::from_reader(f).expect("Could not read values");

        let mut v2 = Vec::new();
        for i in instances {
            v2.push(BackupInstance {
                host: crate::backup::connection::Host {
                    username: i.username,
                    hostname: i.hostname,
                    private_file: i.private_key,
                },
                output_path: i.output_path,
                commands: i.commands,
            })
        }
        return Backup { instances: v2 };
    }

    pub fn print(&self) {
        for instance in &self.instances {
            let username = &instance.host.username;
            let hostname = &instance.host.hostname;
            println!("{username}@{hostname}")
        }
    }

    pub fn execute(&self) {
    	let now = Local::now().format("%Y-%m-%d").to_string();
        for instance in &self.instances {
            let connection = crate::backup::connection::Connection::new(&instance.host).unwrap();
            for cmd in &instance.commands {
            	let output_path =match &instance.output_path {
            		None => cmd.output.clone(),
            		Some(path)=> match cmd.output.starts_with('/') {
            			true => cmd.output.clone(),
            			false => format!("{}/{}", path, cmd.output)
            		}
            	};

            	let file_str = output_path.as_str().replace("<date>", &now);
            	let file_path = Path::new(&file_str);
            	check_parent_folder(file_path);
            	
                println!(
                    "Executing {} on {}@{}",
                    cmd.exec, instance.host.username, instance.host.hostname
                );
                let std_err = connection.execute(&cmd.exec, file_path);
                println!("stderr {}", std_err);
            }
            connection.close();
        }
    }
}
