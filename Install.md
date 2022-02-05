Step 1 – 
Run the graph node itself:
Git clone: https://github.com/graphprotocol/graph-node.git
cd docker
./setup.sh
Now configure the docker-compose.yml file by changing the following line – 
ethereum: 'mainnet:http://host.docker.internal:8545'
to 
ethereum: 'hpb:https://hpbnode.com'

now run 
docker-compose up, to get the graph node running
