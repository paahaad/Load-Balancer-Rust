# Pingora Load Balancer

A simple load balancer built using [Pingora](https://github.com/cloudflare/pingora), a Rust-based framework for building high-performance proxies and load balancers. This project demonstrates how to set up a basic load balancer that distributes traffic across multiple backend servers.

---

## Features

- **Round Robin Load Balancing**: Distributes requests evenly across backend servers.
- **Health Checks**: Ensures traffic is only routed to healthy upstream servers.
- **Customizable**: Easily extendable to support more advanced load balancing strategies.

---

## Prerequisites

Before running the project, ensure you have the following installed:

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
2. **Backend Servers**: Two or more HTTP servers to act as upstream servers (e.g., Express.js servers).

---

## Setup

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/pingora-lb.git
cd pingora-lb
```

### 2. Build the Project
```bash
cargo build --release
```

### 3. Set Up Backend Servers
Run two or more HTTP servers on different ports. For example, using Express.js:

```javascript
// server.mjs
import express from "express";

const app = express();
const PORT = 3000; // Change this for each server

app.get("/", (_, res) => res.end(`Hello from server on port ${PORT}`));

app.listen(PORT, () => console.log(`Server running on port ${PORT}`));
```

Start the servers:
```bash
node server.mjs  # On port 3000
node server.mjs  # On port 3001
```

### 4. Configure the Load Balancer
Update the `main.rs` file to point to your backend servers:
```rust
let upstreams =
    Arc::new(LoadBalancer::try_from_iter(["127.0.0.1:3000", "127.0.0.1:3001"]).unwrap());
```

### 5. Run the Load Balancer
```bash
cargo run
```

The load balancer will start on `0.0.0.0:6789`.

---

## Usage

### Test the Load Balancer
Send a request to the load balancer:
```bash
curl http://localhost:6789
```

You should see responses alternating between your backend servers:
```
Hello from server on port 3000
Hello from server on port 3001
```

### Health Checks
The load balancer periodically checks the health of the upstream servers. Unhealthy servers are automatically removed from the pool.

---

## Configuration

### Upstream Servers
Edit the `upstreams` array in `main.rs` to add or remove backend servers:
```rust
let upstreams =
    Arc::new(LoadBalancer::try_from_iter(["127.0.0.1:3000", "127.0.0.1:3001", "127.0.0.1:3002"]).unwrap());
```

### Load Balancing Strategy
By default, the load balancer uses **Round Robin**. You can implement other strategies by modifying the `selection` module.

---

## Logs
The load balancer logs incoming requests and upstream server health checks. Check the logs for debugging:
```bash
tail -f pingora-lb.log
```

---

## Questions?
If you have any questions or need help, feel free to open an issue or contact the parvat.raj2@gmail.com.
