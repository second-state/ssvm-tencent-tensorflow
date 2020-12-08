

```
$ ssvmup build
$ ./ssvm-tensorflow-lite pkg/mobilenet_bg.wasm < bird.json
$ ./ssvm-tensorflow-lite pkg/mobilenet_bg.wasm < PurpleGallinule.json

$ base64 -w 0 bird.jpg | curl -d @- -X POST https://service-aj0plx8u-1302315972.hk.apigw.tencentcs.com/release/my_hk
$ base64 -w 0 PurpleGallinule.jpg | curl -d @- -X POST https://service-aj0plx8u-1302315972.hk.apigw.tencentcs.com/release/my_hk
```
