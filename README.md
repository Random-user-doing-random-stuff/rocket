# Installation Guide

This guide provides instructions for setting up the project on Debian 12.8.0 with KDE Plasma.

PostgreSQL is running in a Docker container. The version tested was 13.18, and Docker was version 20.10.24+dfsg1, build 297e128.

Note: KDE Plasma was used on a laptop, while GNOME was used on a virtual machine due to compatibility issues with VirtualBox.

## Requirements

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Diesel CLI](https://diesel.rs/)

## Features

- Password hashing (coming soon)

## Installation
use linux for this setup( might as well using linux and searching for tutorials for everything u need to make rather than using windows)

update all ur shit
```bash
sudo apt update
```

in case yout dumbass dont got git installed
```bash
sudo apt install git
```
clone the repository
```bash
git clone https://github.com/Random-user-doing-random-stuff/rocket.git
```
in case u dont have curl installed, u dumb fuck
```bash
sudo apt install curl
```

install rust (this will be the happiest day of ur life) (this is for linux and wsl's btw, bc if u're still using windows in the big 25 u might as well kill yourself)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

install the building tools for cargo (cc, gcc and blah blah blah, just need cc(i think))
```bash
sudo apt install build-essential
```

compile libraries (this will mainly install them in ur project) (use release flag if you'd like or remove if it if u dont duuh)
```bash
cargo build --release
```

Then follow all these annmoying steps to install docker and docker-composer 
 thank chat gpt for making this tutorial for me, otherwise you'd be fucked
### Step 2: Install dependencies
Install necessary dependencies:
 
```bash
sudo apt install -y apt-transport-https ca-certificates curl gnupg lsb-release
```
 
### Step 3: Add Docker's official GPG key
Add Docker’s official GPG key to verify the authenticity of the packages:
 
```bash
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
```
 
### Step 4: Add Docker's official repository
Add the Docker repository to your package sources:
 
```bash
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```
 
### Step 5: Install Docker Engine
Update the package lists and install Docker:
 
```bash
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io
```
 
### Step 6: Verify Docker installation
Check if Docker was successfully installed:
 
```bash
sudo docker --version
```
 
You should see the version of Docker displayed.
 
### Step 7: Enable Docker to start on boot
Enable Docker to start automatically on system boot:
 
```bash
sudo systemctl enable docker
```
 
### Step 8: Install Docker Compose
Now, to install Docker Compose, follow these steps:
 
1. Download the latest stable version of Docker Compose:
 
```bash
sudo curl -L "https://github.com/docker/compose/releases/download/$(curl -s https://api.github.com/repos/docker/compose/releases/latest | jq -r .tag_name)/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
```
 
2. Make Docker Compose executable:
 
```bash
sudo chmod +x /usr/local/bin/docker-compose
```
 
### Step 9: Verify Docker Compose installation
Check if Docker Compose is installed properly:
 
```bash
docker-compose --version
```
 
You should see the version of Docker Compose displayed.
 
### Step 10: Add your user to the Docker group (optional)
To run Docker commands without `sudo`, add your user to the Docker group:
 
```bash
sudo usermod -aG docker $USER
```
 
You will need to log out and log back in for the group change to take effect.
 
---
i restarted my pc,  so ill put the command here anyways
```bash
shutdown -h now
```

in case u are lost and dont have a `docker-compose.yml` file here is a template.
ofc you should replace the environment variables for what you'd like
```yml
services:
  db:
    image: postgres:13
    restart: always
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: mydb
    ports:
      - "5432:5432"
``` 

create the docker container or image or whatever it is with this command
```
docker-compose up
```

now in the `.env` file you will put the following url
```text
DATABASE_URL=postgres://user:password@localhost/db
```
and dont forget to put the values u set in the yml file

dont forget to install diesel cli
```bash
cargo install diesel_cli --no-default-features --features postgres
```
```bash
sudo apt install libpq-dev
```