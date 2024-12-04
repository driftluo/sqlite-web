# sqlite-web

This library is a highly abstract one - it's a binding for [sqljs](https://github.com/sql-js/sql.js), which itself is a binding for sqlite-wasm. In simple terms, this provides a way for Rust to manipulate data in the browser using SQLite syntax. All data is stored in memory, and can be exported and saved to IndexedDB, enabling database operations in the browser using SQLite syntax through Rust.


## Usage

```bash
$ npm install && npm run serve
```

and then open your browser on `http://localhost:8080/`, open console, you can find it's output.
