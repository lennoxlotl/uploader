# uploader

Very simple service for uploading files onto a server to share them with the general public.<br>
⚠️ This is very much an **experimental** project, please use with caution as bugs are expected.

## Getting started
Want to contribute to the project?

1. Install Rust through your preferred package manager or [here](https://rustup.rs/)
2. Clone the repository (captain obvious)
3. Make sure you have a PostgreSQL instance running (you can use [this](docker/development/docker-compose.yml) Docker Compose file to easily spin one up)
4. Create a configuration file in the root directory (example config can be found [here](docker/production/Rocketr.toml))
5. Build or run using `cargo` (`cargo build` / `cargo run`)

## Deployment 
*Pre-built Docker images for Linux (x86, arm) are provided using GitHub Packages (ghcr.io)*

### Docker Compose
For an easy setup experience you can use the Docker Compose file provided [here](docker/production/docker-compose.yml).<br>
Make sure you carefully read through the compose file as you should change several things such as credentials and ports.<br>

#### Configuration
Before being able to run the containers you need to create a configuration file. An example can be found [here](docker/production/Rocket.toml).<br>
Just like with the compose file make sure to carefully read through the config file and change values to your liking.


## Roadmap
This roadmap is constantly updated with new ideas and features that are planned to be added in the future

#### API
- [ ] Multiple auth-keys stored in Postgres
- [ ] Simple CLI application for creating auth-keys and administrating the service
- [x] Support for multiple storage "drivers" (e.g. local file system)

#### General
- [ ] Issue and Pull Request templates to make contribution easier
- [ ] Provide setup guide for Kubernetes
