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
 
