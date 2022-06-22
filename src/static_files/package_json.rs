// package.json file describes the project
// with its information and dependencies

pub const PACKAGE_JSON: &str = 
r#"{
    "name": "PROJECT_NAME",
    "version": "0.0.1",
    "description": "This project srcis created by Creta",
    "main": "bin/index.js",
    "repository": "https://example.com/example/example.git",
    "author": "example",
    "license": "MIT",
    "scripts": {
        "start": "yarn tsc && node bin/index.js"
    },
    "devDependencies": {
        "typescript": "^4.7.3"
    }
}
"#;