FROM microsoft/dotnet:2.1-sdk-alpine

WORKDIR /app


COPY . ./
RUN dotnet restore
RUN dotnet publish -c Release -o out

ENTRYPOINT ["dotnet", "app.Server/out/app.Server.dll"]
