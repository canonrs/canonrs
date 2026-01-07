# Canon Rule #14: Exit 0 ≠ Crash

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All containerized services

---

## 🎯 Objective

Understand that `exit 0` indicates successful program termination, not a crash or Docker/orchestration failure.

---

## 📋 The Misconception

### Common Incorrect Assumptions:

❌ "Docker killed my process"  
❌ "Tokio runtime crashed"  
❌ "Panic occurred but wasn't logged"  
❌ "Kubernetes evicted the pod"  
❌ "Out of memory"  

---

## 🧠 What Exit 0 Actually Means

**Exit Code 0:**
- Program terminated **successfully**
- `main()` returned `Ok(())` or implicit success
- No panic, no error, no signal

**Process Lifecycle:**
```rust
fn main() -> Result<()> {
    setup();
    do_work();
    Ok(())  // ← Program exits with code 0
}
```

Container sees: Process exited successfully → Container stops.

---

## 🔍 Real Causes of Exit 0

### 1. Main Function Returns Early
```rust
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting...");
    
    setup_logging();
    
    // ❌ No blocking await - main returns immediately
    tokio::spawn(async {
        run_server().await;
    });
    
    Ok(())  // ← Exit 0 here
}
```

**Fix:** Add blocking await:
```rust
Ok(())  // Remove this
run_server().await?;  // Add blocking call
Ok(())  // Now only reached on shutdown
```

### 2. Runtime Dependencies Missing
```rust
// Binary built dynamically
// Container lacks libc/openssl
// Process starts, fails to load libs, exits 0 (no error logged)
```

**Detection:**
```bash
ldd /app/binary
# Shows: libssl.so => not found
```

### 3. Configuration Error Returns Early
```rust
fn main() -> Result<()> {
    let config = load_config()?;  // Returns Err
    // Error not logged, just propagated
    start_server(config)?;
    Ok(())
}
```

**Improvement:**
```rust
fn main() -> Result<()> {
    let config = load_config()
        .context("Failed to load config")?;
    tracing::info!("Config loaded successfully");
    // ...
}
```

### 4. Axum/Tokio Returns Without Blocking
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/", get(handler));
    
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;  // ✅ This blocks
    
    Ok(())  // Only reached on graceful shutdown
}
```

If `.await?` missing or returns early → Exit 0.

---

## 🚨 How to Diagnose

### Check Exit Code:
```bash
# Run container
docker run --name test service

# Check exit code
docker inspect test --format='{{.State.ExitCode}}'
# Output: 0

# If 0: process terminated successfully
# If >0: process crashed/errored
```

### Common Exit Codes:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 137 | SIGKILL (OOM) |
| 139 | Segmentation fault |
| 143 | SIGTERM (graceful shutdown) |

### Verify No Panic Occurred:
```bash
# Run with backtrace
docker run --rm -e RUST_BACKTRACE=1 service

# If exit 0 and no backtrace: not a panic
```

---

## ✅ Correct Debugging Approach

### Step 1: Confirm Exit Code
```bash
echo $?
# If 0: not a crash
```

### Step 2: Add Explicit Logging
```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing::info!("Process starting");
    
    setup()?;
    tracing::info!("Setup complete");
    
    run_server().await?;
    tracing::info!("Server shutdown");  // Should never see this
    
    Ok(())
}
```

### Step 3: Check for Early Returns
```bash
# If logs show "Setup complete" but not "Server started":
# → Function returned early
```

### Step 4: Verify Binary Completeness
```bash
# Check binary size (Canon Rule #13)
ls -lh /app/service

# Check for dynamic linking
ldd /app/service
```

---

## 📝 Prevention Checklist

- [ ] Main function has blocking `.await` that never returns
- [ ] All errors are logged before returning
- [ ] Binary is statically linked (MUSL)
- [ ] Logging initialized before any logic
- [ ] Exit points explicitly logged

---

## 🔧 Forced Blocking Pattern
```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    tracing::info!("Starting service");
    
    // THIS MUST BLOCK FOREVER
    tokio::select! {
        result = run_server() => {
            tracing::error!("Server exited: {:?}", result);
            result?;
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received");
        }
    }
    
    tracing::info!("Exiting");
    Ok(())
}
```

---

## 🔗 Related Rules

- **Canon Rule #13** (Binary Size): Detecting incomplete binaries
- **Canon Rule #18** (Docker Cache): Stale binaries causing early exit
- **Canon Rule #20** (MUSL): Runtime dependency issues

---

**Key Insight:** Exit 0 is not mysterious. It means your program successfully ran to completion and returned. The question is: why did it complete so quickly?

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
