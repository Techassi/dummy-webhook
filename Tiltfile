local_resource('compile', 'cargo build --release', deps=['Cargo.lock', 'src/'])
docker_build('dummy-webhook', '.', dockerfile='docker/Dockerfile.ubuntu')

# Replace the above (local_resource, docker_build) with just this:
# docker_build('dummy-webhook', '.', dockerfile='docker/Dockerfile')

local_resource('load_image', 'kind load docker-image dummy-webhook')
k8s_yaml("deploy/webhook.yaml")
