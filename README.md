# node-api skia example

A rust project that shows how one can call rust functions to tie to skia via the node api.

The project uses rust  +  n



## getting dependencies
 
Instructions on installing rust : https://rustup.rs/

Instructions on installing npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm


Install `napi` which contains bindings to the language 

```shell
npm install -g napi
```
## running
This will output an image called `encoded.png` containing a black line, the code to generate that
is in `hello.mjs`
```shell
npm run build && node hello.mjs
```