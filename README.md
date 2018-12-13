# music-player
simple music player for YouTube and SoundCloud


### Settings

- `.env`

```
LOGINSRV_JWT_ALGO=HS512
LOGINSRV_JWT_SECRET=[YOUR JWT SECRET KEY]
LOGINSRV_SUCCESS_URL=http://localhost/
LOGINSRV_GITHUB=client_id=[YOUR CLIENT ID],client_secret=[YOUR CLIENT SECRET]
LOGINSRV_GOOGLE=client_id=[YOUR CLIENT ID],client_secret=[YOUR CLIENT SECRET],scope=https://www.googleapis.com/auth/userinfo.email
YOUTUBE_API_KEY=[YOUR YOUTUVE API KEY]
MYSQL_RANDOM_ROOT_PASSWORD=yes
MYSQL_DATABASE=mysql
MYSQL_USER=user
MYSQL_PASSWORD=password
DB_HOST=db
DB_PORT=3306
DATA_API_PORT=80
```


- `./app/app.Server/appsettings.json`

```json
{
  "Logging": {
    "LogLevel": {
      "Default": "Warning"
    }
  },
  "AllowedHosts": "*",
  "LOGINSRV_JWT_SECRET": "[YOUR JWT SECRET KEY]"
}
```
