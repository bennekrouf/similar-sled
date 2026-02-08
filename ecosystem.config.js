module.exports = {
  apps: [{
    name: "similar-service",
    script: "./target/release/similar",
    watch: false,
    autorestart: true,
    max_restarts: 10,
    env: {
      APP_ENV: "production"
    }
  }]
}
