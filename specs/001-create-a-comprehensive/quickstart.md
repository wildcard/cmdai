# Quickstart Guide

## Installation & First Run

### Prerequisites
- Rust 1.75+ (for building from source)
- macOS with Apple Silicon (M1/M2/M3/M4) for optimal performance
- Linux x86_64/ARM64 or Windows for alternative platforms

### Installation Options

#### Option 1: Pre-compiled Binary (Recommended)
```bash
# Download latest release for your platform
curl -L https://github.com/user/cmdai/releases/latest/download/cmdai-$(uname -s)-$(uname -m) -o cmdai
chmod +x cmdai
sudo mv cmdai /usr/local/bin/
```

#### Option 2: Package Manager
```bash
# macOS with Homebrew
brew install cmdai

# Linux with apt
sudo apt update && sudo apt install cmdai

# Rust developers
cargo install cmdai
```

#### Option 3: Build from Source
```bash
git clone https://github.com/user/cmdai.git
cd cmdai
cargo build --release
./target/release/cmdai --version
```

## First Time Setup

### 1. Verify Installation
```bash
# Check version and basic functionality
cmdai --version
cmdai --help
```

### 2. Initialize Configuration
```bash
# Create default configuration
cmdai config show

# Set your preferred safety level
cmdai config set safety_level moderate

# Configure default shell
cmdai config set default_shell zsh
```

### 3. Test Backend Availability
```bash
# Check which backends are available
cmdai backends status

# Test specific backend
cmdai backends test mlx
cmdai backends test ollama
```

## Basic Usage Examples

### 1. Simple File Operations
```bash
# Find files
cmdai "find all PDF files larger than 10MB in Downloads"

# List directory contents
cmdai "show me the 10 largest files in current directory"

# Archive operations
cmdai "create a tar.gz archive of all .txt files"
```

### 2. System Information
```bash
# Disk usage
cmdai "show disk usage by directory"

# Process information
cmdai "find processes using the most memory"

# Network information
cmdai "show all open network connections"
```

### 3. Text Processing
```bash
# Search and filter
cmdai "find lines containing 'error' in all log files"

# Sort and count
cmdai "count unique IP addresses in access.log"

# Format conversion
cmdai "convert CSV file to JSON"
```

## Safety Features Demo

### 1. Safe Command Generation
```bash
# This will generate a safe command with green indicator
cmdai "list files in current directory"
# Expected output: ls -la (displayed in green)
```

### 2. Moderate Risk Commands
```bash
# This will show yellow warning and ask for confirmation
cmdai "delete all .tmp files in current directory"
# Expected: find . -name "*.tmp" -delete (displayed in yellow with confirmation)
```

### 3. High Risk Commands
```bash
# This will show red warning and require explicit confirmation
cmdai "delete all files in this directory"
# Expected: Red warning, detailed explanation, explicit confirmation required
```

### 4. Safety Override
```bash
# Force execution of normally blocked commands (expert users only)
cmdai --allow-dangerous "remove everything in /tmp"
```

## Configuration Examples

### 1. Backend Preferences
```bash
# Set preferred backend order
cmdai config set preferred_backends '["mlx", "ollama", "vllm"]'

# Configure specific backend
cmdai config set backends.mlx.model_path "/Users/you/.cmdai/models/mlx"
cmdai config set backends.ollama.endpoint "http://localhost:11434"
```

### 2. Safety Settings
```bash
# Strict safety (blocks all moderate+ risk commands)
cmdai config set safety_level strict

# Permissive safety (allows more commands)
cmdai config set safety_level permissive

# Custom confirmation timeout
cmdai config set confirmation_timeout 30
```

### 3. Output Preferences
```bash
# Always use JSON output
cmdai config set default_format json

# Enable verbose logging
cmdai config set log_level debug

# Disable command history
cmdai config set log_commands false
```

## Model Management

### 1. List Available Models
```bash
# Show all cached models
cmdai models list

# Show cache usage
cmdai models cache
```

### 2. Download Models
```bash
# Download specific model
cmdai models download "microsoft/DialoGPT-medium"

# Download recommended model for backend
cmdai models download --backend mlx
```

### 3. Cache Management
```bash
# Clean up old models
cmdai models cache --clean

# Remove specific model
cmdai models remove "old-model-id"
```

## Advanced Usage

### 1. Custom Backends
```bash
# Use specific backend
cmdai --backend vllm "process this data"

# Use custom endpoint
cmdai --backend vllm --endpoint "http://my-server:8000" "analyze logs"
```

### 2. Batch Processing
```bash
# Process multiple commands from file
cat commands.txt | xargs -I {} cmdai "{}"

# Generate commands without execution
cmdai --explain "complex operation"
```

### 3. Integration with Scripts
```bash
#!/bin/bash
# Generate and execute command programmatically
COMMAND=$(cmdai --format json "find large files" | jq -r '.command')
echo "Executing: $COMMAND"
eval "$COMMAND"
```

## Performance Validation

### 1. Startup Time Test
```bash
# Measure cold start time (should be <100ms)
time cmdai --version
```

### 2. Generation Speed Test
```bash
# Measure inference time (should be <2s)
time cmdai "simple file listing command"
```

### 3. Memory Usage Test
```bash
# Monitor memory usage during operation
cmdai benchmark --count 10
```

## Troubleshooting

### 1. Backend Issues
```bash
# Check backend status
cmdai backends status

# Test connectivity
cmdai backends test --all

# Reset backend configuration
cmdai config reset backends
```

### 2. Model Problems
```bash
# Verify model integrity
cmdai models validate

# Re-download corrupted models
cmdai models update --force

# Clear model cache
cmdai models cache --clear
```

### 3. Configuration Issues
```bash
# Validate current configuration
cmdai config validate

# Reset to defaults
cmdai config reset

# Show configuration hierarchy
cmdai config show --verbose
```

## Integration Tests

### User Story Validation
Each acceptance scenario from the specification should work as described:

1. **File Finding**: `cmdai "find all PDF files larger than 10MB in Downloads folder"`
   - Should generate: `find ~/Downloads -name "*.pdf" -size +10M -type f`
   - Display with green safety indicator
   - Ask for confirmation before execution

2. **Dangerous Operation**: `cmdai "delete all files in this directory"`
   - Should generate appropriate rm command
   - Display RED warning indicator
   - Require explicit confirmation with safety prompts

3. **Ambiguous Request**: `cmdai "compress files"`
   - Should ask clarifying questions
   - Should not generate command until clarified

4. **Safety Refusal**: `cmdai "format my hard drive"`
   - Should refuse to generate command
   - Should suggest alternative approaches

5. **Exploration**: `cmdai "show me disk usage by directory"`
   - Should generate: `du -sh */ | sort -hr`
   - Should explain command purpose
   - Should execute after confirmation

## Performance Benchmarks

### Expected Performance Targets
- **Startup time**: <100ms cold start
- **Generation time**: <2s for typical requests
- **Memory usage**: <500MB during operation
- **Binary size**: <50MB
- **Accuracy**: >90% for common user intents

### Validation Commands
```bash
# Run comprehensive benchmark
cmdai benchmark --count 100 --backend all

# Measure startup performance
for i in {1..10}; do time cmdai --version; done

# Test accuracy with known commands
cmdai "list files" | grep -q "ls" && echo "PASS" || echo "FAIL"
```