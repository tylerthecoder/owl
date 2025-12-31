# Secrets Management

This setup manages sensitive credentials and API keys using [1Password CLI](https://developer.1password.com/docs/cli/).

## How It Works

1. **Templates**: `example.secrets.sh` contains 1Password references (not actual secrets)
2. **Injection**: `setup.sh` uses `op inject` to replace references with actual values
3. **Output**: `secrets.sh` is created with actual secrets (gitignored, mode 600)
4. **Loading**: `secrets.sh` is sourced automatically via owl's rc scripts

```
example.secrets.sh          setup.sh (op inject)         secrets.sh
------------------          --------------------         ----------
op://Dev/Openai/credential  →  reads from 1Password  →  sk-proj-xxx...
op://Dev/Anthropic/...      →  replaces references   →  sk-ant-xxx...
```

## Initial Setup

### 1. Install 1Password CLI

**Arch Linux**:
```bash
yay -S 1password-cli
```

**Ubuntu/Debian**:
```bash
# Follow instructions at:
# https://developer.1password.com/docs/cli/get-started/
```

### 2. Authenticate with 1Password

```bash
op account add --shorthand personal
op signin
```

### 3. Add Secrets to 1Password Vault

Create items in your 1Password vault (e.g., "Dev" vault):
- **Openai** item with credential field
- **Anthropic** item with credential field
- **Npm** item with credential field

### 4. Generate secrets.sh

```bash
cd ~/owl/setups/secrets
./setup.sh
```

This will:
- ✅ Check if 1Password CLI is installed
- ✅ Verify authentication
- ✅ Inject secrets from 1Password
- ✅ Create `secrets.sh` with mode 600
- ✅ Validate successful creation

## Adding New Secrets

### 1. Add to 1Password Vault

Create a new item in your 1Password vault with the credential.

### 2. Update example.secrets.sh

```bash
# Add the new reference using 1Password secret reference format
export NEW_API_KEY="op://Dev/ServiceName/credential"
```

### 3. Regenerate secrets.sh

```bash
./setup.sh
```

### 4. Verify

```bash
# Source the secrets
source secrets.sh

# Check the variable is set
echo $NEW_API_KEY
```

## Rotating Secrets

When you need to rotate compromised or expired secrets:

### 1. Update in 1Password

Go to 1Password and update the credential value.

### 2. Regenerate secrets.sh

```bash
cd ~/owl/setups/secrets
./setup.sh
```

### 3. Reload Your Shell

```bash
# Reload owl environment
source ~/owl/owl-start.sh

# Or restart your terminal
```

## Security Best Practices

### ✅ What's Protected

- ✅ `secrets.sh` is gitignored (never committed)
- ✅ `secrets.sh` has mode 600 (owner read/write only)
- ✅ Actual secrets stored in 1Password (encrypted vault)
- ✅ Only 1Password references committed to git

### ⚠️ Important Warnings

- **Never** commit `secrets.sh` to git
- **Never** share `secrets.sh` via chat/email/slack
- **Always** use 1Password references in templates
- **Immediately** rotate secrets if accidentally exposed

### 🔒 File Permissions

```bash
-rw-------  secrets.sh           # Mode 600: owner only
-rw-r--r--  example.secrets.sh   # Mode 644: template (safe to commit)
-rwxr-xr-x  setup.sh             # Mode 755: executable script
```

## Troubleshooting

### "1Password CLI not found"

Install 1Password CLI for your platform:
- Arch: `yay -S 1password-cli`
- Ubuntu: See https://developer.1password.com/docs/cli/get-started/

### "1Password not authenticated"

Sign in to 1Password:
```bash
op signin
```

### "Failed to inject secrets"

1. Check you're authenticated: `op account list`
2. Verify the vault/item exists in 1Password
3. Ensure the reference format is correct: `op://Vault/Item/field`

### "Permission denied" when sourcing secrets.sh

Fix permissions:
```bash
chmod 600 secrets.sh
```

## Emergency Recovery

If `secrets.sh` is lost or corrupted:

1. **Don't panic** - secrets are safely stored in 1Password
2. Regenerate from 1Password:
   ```bash
   cd ~/owl/setups/secrets
   ./setup.sh
   ```

## Files in This Setup

```
setups/secrets/
├── README.md              # This file
├── example.secrets.sh     # Template with 1Password references (committed)
├── secrets.sh            # Generated file with actual secrets (gitignored)
├── setup.sh              # Script to inject secrets from 1Password
├── install.sh            # One-time setup (install 1Password CLI)
└── setup.json            # Owl setup configuration
```

## Related Documentation

- [1Password CLI Documentation](https://developer.1password.com/docs/cli/)
- [1Password Secret References](https://developer.1password.com/docs/cli/secrets-reference-syntax/)
- [Owl Documentation](../../README.md)
