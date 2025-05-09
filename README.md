## .envファイルの一部変更
Submodulesで入れてるものを
```
POSTGRES_DB=backend
DATABASE_URL=postgres://loco:loco@loco_discord_db:5432/backend
```
に変更する（変更部分だけ記載）

また、Dockerfileもこう変更を加える
```
    depends_on: 
      - db
  db:
    hostname: loco_discord_db
```

また、composeの名前付きボリュームは`postgres-data`から`postgres-fitbit-data`にリネーム