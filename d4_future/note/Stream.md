# Stream

Stream = 流れるデータ構造、ネットワークストリーム、イベントストリーム、データストリーム

`Stream` trait を実装すると、構造体を stream として扱えるようになる　

`Stream`
- `type Item`
- `fn poll_next`: `Pin -> &mut Context -> Poll<Option<Self::Item>>`




