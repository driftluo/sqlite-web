export async function initSql(data) {
  const SQL = await window.initSqlJs({
    // Required to load the wasm binary asynchronously. Of course, you can host it wherever you want
    // You can omit locateFile completely when running in node
    locateFile: (file) => `https://sql.js.org/dist/${file}`,
  });
  return new Database(SQL, data);
}

export class Database {
  constructor(SQL, data) {
    this._db = new SQL.Database(data);
  }

  run(sql) {
    this._db.run(sql);
  }

  exec(sql) {
    return this._db.exec(sql);
  }

  export() {
    return this._db.export();
  }
}
