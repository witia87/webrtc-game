### webrtc-client setup

Project startup is based on:
https://github.com/goldenratio/super-ecs-example



Install protobuf:
```
npm install ts-proto
```

Compile Protobuf to TypeScript
```
protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --proto_path=../proto/ --ts_proto_out=./src/comms/messages/ ../proto/*.proto
```

Install dependencies

```
npm i
```

Start hosting a client app

```
npm run start
```
