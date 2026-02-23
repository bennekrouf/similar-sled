module.exports = {
  apps: [{
    name: 'similar-sled',
    script: './target/release/similar',
    cwd: process.cwd(),
    instances: 1,
    exec_mode: 'fork',
    watch: false,
    max_memory_restart: '1G',
    autorestart: true,
    max_restarts: 10,
    min_uptime: '10s',
    env: {
      APP_ENV: 'production',
      PORT: 8000
    }
  }]
};
