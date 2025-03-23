# Key-Value Store API (Rust + Axum + MySQL)

🚀 **A lightweight and efficient key-value store built with Rust, Axum, and MySQL.**

## 📌 Features
- RESTful API for storing, retrieving, and deleting key-value pairs
- MySQL as the storage backend with SQLx for database interactions
- Built using **Axum** for high-performance asynchronous request handling
- Supports JSON-based input and output

## 🛠️ Setup Instructions

### 1️⃣ Clone the Repository
```sh
$ git clone https://github.com/yourusername/key-value-store.git
$ cd key-value-store
```

### 2️⃣ Set Up the MySQL Database
Ensure you have MySQL installed and running. Then, create a database and user:
```sql
CREATE DATABASE key_value_db;
CREATE USER 'rust_user'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON key_value_db.* TO 'rust_user'@'localhost';
FLUSH PRIVILEGES;
```

### 3️⃣ Configure Environment Variables
Create a `.env` file in the project root:
```sh
DATABASE_URL="mysql://rust_user:your_password@localhost/key_value_db"
```

### 4️⃣ Install Dependencies & Run the Server
```sh
$ cargo build
$ cargo run
```

## 🔥 API Endpoints

### ✅ Store a Key-Value Pair
**POST** `/set`
```sh
curl -X POST http://localhost:3000/set \
     -H "Content-Type: application/json" \
     -d '{"key": "name", "value": "John"}'
```
**Response:**
```json
"Key set successfully"
```

### ✅ Retrieve a Value by Key
**GET** `/get/{key}`
```sh
curl -X GET http://localhost:3000/get/name
```
**Response:**
```json
{"key": "name", "value": "John"}
```

### ✅ Delete a Key-Value Pair
**DELETE** `/delete/{key}`
```sh
curl -X DELETE http://localhost:3000/delete/name
```
**Response:**
```json
"Key deleted successfully"
```

## 🛠 Technologies Used
- **Rust** (Axum, Tokio, SQLx)
- **MySQL** (Data storage)
- **Docker (Optional)** for containerization

## 📌 Next Steps
- [ ] Add TTL expiration for keys
- [ ] Improve error handling and logging
- [ ] Deploy on a cloud platform


---
🚀 **Contributions & feedback are welcome!** Happy coding!

