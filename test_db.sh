docker run \
  -e POSTGRES_USER=dim  -e POSTGRES_PASSWORD=dim -e POSTGRES_DB=db \
  -p 3269:5432 -d \
  --name "postgres_test" \
  postgres