# なぜ migration file を残すのか？

別のシステムでもデータベースを再構築できるから

# rust schema table はなんの役に立つ？

DSLで書かれている

データベース構造をパット見で確認するのに役立つ

# Queryable と Insertable の役割は？

struct type と DB をマッチさせるための trait

# .find() .select() といったメソッドが返すものは？

SQL Builder object

実際は、これを実行してDBにクエリを送る必要がある

# .load() .get_result() メソッドが必要とする引数は？

DB Connection オブジェクト