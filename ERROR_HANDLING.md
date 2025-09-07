# Sea-ORM 错误处理系统

这个项目实现了一个完善的错误处理系统，专门为 Sea-ORM 数据库操作设计，使用 `thiserror` 提供类型安全的错误处理。

## 特性

- 🎯 **类型安全**: 使用 Rust 的类型系统确保错误处理的正确性
- 🔄 **自动转换**: 自动将 `sea-orm::DbErr` 转换为自定义错误类型
- 🌐 **HTTP 集成**: 与 Actix-Web 完美集成，自动返回适当的 HTTP 状态码
- 📝 **详细错误信息**: 提供中文错误消息和错误类型标识符
- 🛠️ **便利方法**: 提供静态方法快速创建各种类型的错误

## 错误类型

### 数据库错误
- `DatabaseConnectionError`: 数据库连接错误
- `DatabaseQueryError`: 数据库查询错误
- `DatabaseConstraintError`: 数据库约束错误
- `DatabaseTransactionError`: 数据库事务错误
- `DatabaseRecordNotFound`: 数据库记录未找到
- `DatabaseUniqueConstraintError`: 唯一约束冲突
- `DatabaseForeignKeyError`: 外键约束错误
- `DatabaseTimeout`: 数据库超时
- `DatabaseConfigError`: 数据库配置错误

### 应用错误
- `InternalError`: 内部服务器错误
- `BadClientData`: 客户端请求错误
- `Timeout`: 请求超时
- `ValidationError`: 验证错误
- `NotFound`: 资源未找到
- `Unauthorized`: 未授权访问
- `Forbidden`: 禁止访问

## 使用方法

### 1. 基本使用

```rust
use crate::libs::{MyError, Result, DbResult};

// 创建错误
let err = MyError::bad_request("用户名不能为空");
let err = MyError::database_connection_error("数据库连接失败");

// 使用 ? 操作符自动转换
fn process_data() -> Result<String> {
    let data = std::fs::read_to_string("file.txt")?; // 自动转换为 MyError
    Ok(data)
}
```

### 2. 数据库操作

```rust
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

async fn find_user(db: &DatabaseConnection, user_id: i32) -> DbResult<Option<User>> {
    // Sea-ORM 错误会自动转换为 MyError
    let user = User::find()
        .filter(user::Column::Id.eq(user_id))
        .one(db)
        .await?; // 自动转换 sea-orm::DbErr 为 MyError
    
    Ok(user)
}
```

### 3. 错误处理

```rust
match find_user(db, 1).await {
    Ok(Some(user)) => println!("找到用户: {:?}", user),
    Ok(None) => println!("用户不存在"),
    Err(MyError::DatabaseRecordNotFound(msg)) => {
        println!("数据库记录未找到: {}", msg);
    }
    Err(MyError::DatabaseConnectionError(msg)) => {
        println!("数据库连接错误: {}", msg);
    }
    Err(err) => {
        println!("其他错误: {}", err);
    }
}
```

### 4. HTTP 响应

错误会自动转换为适当的 HTTP 响应：

```json
{
    "error": "数据库记录未找到: 用户ID 999 不存在",
    "status": 404,
    "error_type": "database_record_not_found"
}
```

## 错误转换规则

| Sea-ORM 错误 | 自定义错误 | HTTP 状态码 |
|-------------|-----------|------------|
| `ConnectionAcquire` | `DatabaseConnectionError` | 503 |
| `Conn` | `DatabaseConnectionError` | 503 |
| `Exec` (包含 "duplicate") | `DatabaseUniqueConstraintError` | 409 |
| `Exec` (包含 "foreign key") | `DatabaseForeignKeyError` | 400 |
| `Exec` (包含 "not found") | `DatabaseRecordNotFound` | 404 |
| `Exec` (包含 "timeout") | `DatabaseTimeout` | 504 |
| `Exec` (其他) | `DatabaseQueryError` | 500 |
| `Query` | `DatabaseQueryError` | 500 |
| `RecordNotFound` | `DatabaseRecordNotFound` | 404 |
| `Migration` | `DatabaseConfigError` | 500 |
| `RecordNotInserted` | `DatabaseConstraintError` | 400 |
| `RecordNotUpdated` | `DatabaseRecordNotFound` | 404 |

## 最佳实践

### 1. 使用类型别名

```rust
use crate::libs::{Result, DbResult};

// 使用 Result<T> 而不是 std::result::Result<T, MyError>
fn process_data() -> Result<String> {
    // ...
}

// 使用 DbResult<T> 表示数据库操作结果
async fn find_user(id: i32) -> DbResult<Option<User>> {
    // ...
}
```

### 2. 错误链和上下文

```rust
async fn complex_operation(db: &DatabaseConnection) -> Result<()> {
    let user = find_user(db, 1)
        .await
        .map_err(|err| MyError::internal_error(
            format!("查找用户失败: {}", err)
        ))?;
    
    // 处理用户数据...
    Ok(())
}
```

### 3. 验证输入数据

```rust
fn validate_user_input(data: &Value) -> Result<()> {
    let name = data.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MyError::validation_error("用户名不能为空"))?;
    
    if name.is_empty() {
        return Err(MyError::validation_error("用户名不能为空"));
    }
    
    Ok(())
}
```

### 4. 批量操作错误处理

```rust
async fn batch_operation(items: Vec<Value>) -> Result<Vec<Value>> {
    let mut results = Vec::new();
    
    for (index, item) in items.into_iter().enumerate() {
        match process_item(item).await {
            Ok(result) => results.push(result),
            Err(err) => {
                return Err(MyError::internal_error(
                    format!("批量操作失败，第 {} 项处理失败: {}", index + 1, err)
                ));
            }
        }
    }
    
    Ok(results)
}
```

## 扩展错误类型

如果需要添加新的错误类型，只需在 `MyError` 枚举中添加新的变体：

```rust
#[derive(Debug, Error)]
pub enum MyError {
    // 现有错误类型...
    
    #[error("新的错误类型: {0}")]
    NewErrorType(String),
}

impl MyError {
    pub fn new_error_type(msg: impl Into<String>) -> Self {
        Self::NewErrorType(msg.into())
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            // 现有匹配...
            MyError::NewErrorType(_) => StatusCode::BAD_REQUEST,
        }
    }
}
```

这个错误处理系统为您的 Sea-ORM 应用提供了强大而灵活的错误处理能力，确保错误信息清晰、类型安全，并且与 HTTP API 完美集成。

