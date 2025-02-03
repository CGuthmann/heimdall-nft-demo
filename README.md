# Setup

1. start the ssh-agent with `eval "$(ssh-agent -s)"`
2. add your key to the agent with `ssh-add PATH-TO-PRIVATE_KEY`
3. Fetch submodules:  `git submodule update --init --recursive --remote`
4. Add ssh private key named `id_docker` to `heimdall-nft-server/.ssh`
4. Create Docker with: `docker-compose up`
5. run Demo with: `cargo run` 