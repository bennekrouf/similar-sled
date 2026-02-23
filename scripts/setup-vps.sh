#!/bin/bash
# =============================================================================
# Similar-Sled API VPS Setup Script for Debian (running alongside Mayorana)
# =============================================================================
#
# This script will:
#   1. Install Rust (cargo) for the mayorana user if not present
#   2. Clone/pull similar-sled repository
#   3. Build similar-sled in release mode
#   4. Start similar-sled with PM2
#   5. Configure Nginx for similar.mayorana.ch
#   6. Optionally, setup SSL via Certbot (DNS needed)
#

set -euo pipefail

DOMAIN="similar.mayorana.ch"
REPO="git@github.com:bennekrouf/similar-sled.git"
APP_DIR="/var/www/similar-sled"
APP_PORT=8000
APP_USER="mayorana"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[!]${NC} $1"; }
err()  { echo -e "${RED}[X]${NC} $1"; exit 1; }
step() { echo -e "\n${CYAN}=== $1 ===${NC}\n"; }

if [ "$EUID" -ne 0 ]; then
  err "Please run as root: sudo bash scripts/setup-vps.sh"
fi

step "1/5 — Install Rust for $APP_USER"

# Install curl and build-essential just in case
apt install -y curl build-essential

# We need to install Rust for the APP_USER
sudo -u "$APP_USER" HOME=/var/www bash -c "
  if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  fi
"
log "Rust installed for $APP_USER"

step "2/5 — Clone similar-sled repo"

# Generating an SSH key for mayorana if it doesn't exist (it should already be there from the mayorana setup)
if [ ! -f "/var/www/.ssh/id_ed25519" ]; then
  warn "No SSH key found in /var/www/.ssh for $APP_USER! Make sure mayorana setup was run first."
fi

if [ ! -d "$APP_DIR" ]; then
  ssh-keyscan github.com >> /var/www/.ssh/known_hosts 2>/dev/null || true
  chown "$APP_USER:$APP_USER" /var/www/.ssh/known_hosts || true
  sudo -u "$APP_USER" -H git clone "$REPO" "$APP_DIR"
  log "Repository cloned to $APP_DIR"
else
  warn "Directory $APP_DIR already exists, pulling latest..."
  sudo -u "$APP_USER" -H git -C "$APP_DIR" pull
fi

step "3/5 — Build and Start with PM2"

# Ensure Cargo is in PATH and build
sudo -u "$APP_USER" HOME=/var/www bash -c "
  source /var/www/.cargo/env
  cd $APP_DIR
  cargo build --release
  if [ -f ecosystem.config.js ]; then
    \$(which pm2) start ecosystem.config.js
  else
    echo \"[!] ecosystem.config.js not found in \$APP_DIR yet. You may need to git push it first!\"
  fi
  \$(which pm2) save
"
log "similar-sled compiled and started on PM2"

step "4/5 — Configure Nginx"

cat > "/etc/nginx/sites-available/similar-sled" <<EOF
server {
    listen 80;
    server_name $DOMAIN;

    access_log /var/log/nginx/similar_access.log;
    error_log /var/log/nginx/similar_error.log;

    location / {
        # Using the same general rate limiting zone defined in mayorana
        limit_req zone=general burst=10 nodelay;
        limit_conn addr 20;

        limit_req_status 429;
        limit_conn_status 429;

        proxy_pass http://localhost:$APP_PORT;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        proxy_connect_timeout 5s;
        proxy_send_timeout 10s;
        proxy_read_timeout 30s;
        
        client_max_body_size 5m;
        
        # Security headers
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header X-XSS-Protection "1; mode=block" always;
        add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    }
}
EOF

ln -sf /etc/nginx/sites-available/similar-sled /etc/nginx/sites-enabled/similar-sled
nginx -t || err "nginx config test failed!"
systemctl reload nginx
log "Nginx configured for $DOMAIN"

step "5/5 — SSL with Certbot"

read -p "Have you configured DNS A records for $DOMAIN? (y/n) " dns_ready

if [ "\$dns_ready" != "y" ] && [ "\$dns_ready" != "Y" ]; then
  warn "Skipping certbot. Run this later when DNS is ready:"
  warn "  sudo certbot --nginx -d $DOMAIN --non-interactive --agree-tos -m your@email.com"
else
  read -p "Enter your email for Let's Encrypt notifications: " le_email
  certbot --nginx -d "$DOMAIN" --non-interactive --agree-tos -m "\$le_email" --redirect
  log "SSL certificate installed"
fi

step "Setup complete!"
echo "similar-sled is now deployed alongside mayorana."
echo "API Route: https://$DOMAIN"
