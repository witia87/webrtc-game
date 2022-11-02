### super-ecs example

To run the example,

```
npm i
npm run start
```

Protobuf:
npm install ts-proto

Compile Protobuf to TypeScript
protoc --plugin=./node_modules/.bin/protoc-gen-ts_proto --proto_path=../proto/ --ts_proto_out=./src/messages/ ../proto/*.proto
