use std::fs;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Package {
    name: String,
    version: String,
    description: String,
    main: String,
    private: bool,
    scripts: HashMap<String, String>,
    author: String,
    license: String,
    contributors: Vec<String>,
    dependencies: HashMap<String, String>,
}

#[derive(Copy, Clone)]
pub struct Generator {}

impl Generator {
    pub fn generate(self, path: &str) -> std::io::Result<()> {
        self.generate_root(path);
        self.generate_package(path);
        self.generate_readme(path);
        self.generate_npmrc(path);
        self.generate_server(path);
        self.generate_app(path);
        Ok(())
    }
    pub fn generate_root(self, path: &str) -> std::io::Result<()> {
        fs::create_dir(path)?;
        fs::create_dir(format!("{}/src", path))?;
        fs::create_dir(format!("{}/src/modules", path))?;
        fs::create_dir(format!("{}/src/modules/common", path))?;
        fs::create_dir(format!("{}/build", path))?;
        fs::create_dir(format!("{}/docs", path))?;
        fs::create_dir(format!("{}/configs", path))?;
        fs::create_dir(format!("{}/terraform", path))?;
        Ok(())
    }

    pub fn generate_package(self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(format!("{}/package.json", path))?;
        let mut package = Package {
            name: "".to_string(),
            version: "".to_string(),
            description: "".to_string(),
            main: "".to_string(),
            private: false,
            scripts: [
                ("start".to_string(), "node ./src/server.js".to_string())
            ].iter().cloned().collect(),
            author: "".to_string(),
            license: "".to_string(),
            contributors: vec!["mkubasz@gmail.com".to_string()],
            dependencies: [
                ("koa".to_string(), "^2.11.0".to_string())
            ].iter().cloned().collect(),
        };
        serde_json::to_writer(&file, &package)?;
        Ok(())
    }

    pub fn generate_readme(self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(format!("{}/README.md", path))?;
        Ok(())
    }

    pub fn generate_npmrc(self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(format!("{}/.npmrc", path))?;
        Ok(())
    }

    pub fn generate_server(self, path: &str) -> std::io::Result<()> {
        let source = br#"
const app = require('./modules/app');

app.listen(3000);
        "#;
        let mut file = File::create(format!("{}/src/server.js", path))?;
        file.write_all(source);
        Ok(())
    }

    pub fn generate_app(self, path: &str) -> std::io::Result<()> {
        let source = br#"
const Koa = require('koa');
const app = new Koa();

app.use(async ctx => {
  ctx.body = 'Hello World';
});

module.exports = app;
        "#;
        let mut file = File::create(format!("{}/src/modules/app.js", path))?;
        file.write_all(source);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let generator = Generator{};
    generator.generate("/home/tumnus/Projects/test");

    Ok(())
}