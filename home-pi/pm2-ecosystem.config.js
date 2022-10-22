module.exports = {
  apps: [
    {
      name: "api",
      cwd: "/home/pi/dev/tgtapi/",
      script: "yarn",
      args: "start:prod",
      interpreter: "/bin/bash",
      env: {
        "SPOTIFY_CLIENT_ID": "a7e126eaee8b4c6f9e689a8b3b15efa5",
        "SPOTIFY_SECRET_ID": "7de3ad7d3a6a4669926a627b5c4588a8",
        "SPOTIFY_REDIRECT_URL": "http://localhost:6006/me/spotify-callback",
        "DB_URI": "mongodb+srv://admin:admin@cluster0-aligf.mongodb.net/devices?retryWrites=true",
        "HUE_URL": "http://192.168.1.183/api/aHJTvyHPP-Y6oANR3nVfZxRjX92lG0R-HcJso2KJ",
        "PORT": 3000
      }
    },
    {
      name: "web",
      cwd: "/home/pi/dev/website",
      script: "yarn",
      args: "start:prod",
      interpreter: "/bin/bash",
      env: {
        "NEXT_PUBLIC_API_URL": "https://api.tylertracy.com",
        "PORT": 5000
      }
    },
    {
      name: "craft",
      cwd: "/home/pi/dev/craft",
      script: "./scripts/web-start.sh",
      interpreter: "/bin/bash",
      env: {
        "DB_URL": "mongodb+srv://admin:admin@cluster0.aligf.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
        "PORT": 4000
      }
    }
  ]
}
