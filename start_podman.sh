if podman container exists rust_semantic_core; then
  echo "Container postgres_dev already exists, starting it..."
  podman start rust_semantic_core
else
  echo "Container postgres_dev does not exist, creating and starting it..."
  if [ -z "$POSTGRES_PASSWORD" ]; then
    echo "Error: Variable POSTGRES_PASSWORD is not set, please set it before proceeding"
    exit 1
  fi
  podman run -d \
    --name rust_semantic_core \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
    -e POSTGRES_DB=rust_semantic_core \
    -p 5432:5432 \
    -v $(pwd)/data/postgres \
    docker.io/library/postgres:18-alpine
fi
