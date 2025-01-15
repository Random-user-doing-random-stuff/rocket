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

### Step 1: Update System

Update your system packages:

```bash
sudo apt update
```

### Step 2: Install Git

Install Git if not already installed:

```bash
sudo apt install git
```

### Step 3: Clone the Repository

Clone the project repository:

```bash
git clone https://github.com/Random-user-doing-random-stuff/rocket.git
```

### Step 4: Install Curl

Install Curl if not already installed:

```bash
sudo apt install curl
```

### Step 5: Install Rust

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 6: Install Build Tools

Install the necessary build tools:

```bash
sudo apt install build-essential
```

### Step 7: Compile Libraries

Compile the project libraries:

```bash
cargo build --release
```

### Step 8: Install Docker and Docker Compose

#### Install Dependencies

Install necessary dependencies:

```bash
sudo apt install -y apt-transport-https ca-certificates curl gnupg lsb-release
```

#### Add Docker's GPG Key

Add Docker’s official GPG key:

```bash
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
```

#### Add Docker Repository

Add the Docker repository:

```bash
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

#### Install Docker Engine

Update package lists and install Docker:

```bash
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io
```

#### Verify Docker Installation

Check Docker version:

```bash
sudo docker --version
```

#### Enable Docker on Boot

Enable Docker to start on boot:

```bash
sudo systemctl enable docker
```

#### Install Docker Compose

Download and install Docker Compose:

```bash
sudo curl -L "https://github.com/docker/compose/releases/download/$(curl -s https://api.github.com/repos/docker/compose/releases/latest | jq -r .tag_name)/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### Verify Docker Compose Installation

Check Docker Compose version:

```bash
docker-compose --version
```

#### Add User to Docker Group (Optional)

To run Docker commands without `sudo`, add your user to the Docker group:

```bash
sudo usermod -aG docker $USER
```

Log out and log back in for the group change to take effect.

### Step 9: Create Docker Compose File

Create a `docker-compose.yml` file with the following content:

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

### Step 10: Start Docker Compose

Start the Docker container:

```bash
docker-compose up
```

### Step 11: Configure Environment Variables

Add the following to your `.env` file:

```text
DATABASE_URL=postgres://user:password@localhost/db
```

Replace the values with those set in the `docker-compose.yml` file.

### Step 12: Install Diesel CLI

Install Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features postgres
sudo apt install libpq-dev
```

### Step 13: Restart System (Optional)

Restart your system:

```bash
shutdown -h now
```