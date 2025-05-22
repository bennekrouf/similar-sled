module.exports = {
  apps: [{
    name: "similar-service",
    cwd: "/home/similar/production/similar",
    script: "./similar",
    watch: false,
    autorestart: true,
    max_restarts: 10,
    env: {
      APP_ENV: "production"
    }
  }]
}
