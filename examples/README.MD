# dysql examples readme

## 1. Setup database container
```
$docker-compose up -d
```
## 2. Prepare test data
```
$sqlx database create
$sqlx mig run
```
### 3. Goto the "with_tokio_postgres" folder, and run it
```
$cargo run
```
### 4. After all, you can uninstall database container
```
$docker-compose down
```
