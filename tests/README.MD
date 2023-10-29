# dysql examples readme

## 1. Setup database container
```
$docker-compose up -d
```
## 2. Prepare test data 
```
$cd <workspace>/db_init/<db folder>
$sqlx database create
$sqlx mig run
```
### 3. test it
```
$cargo test
```
### 4. After all, you can uninstall database container
```
$docker-compose down
```
